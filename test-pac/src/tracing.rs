/*
CC0 1.0 Universal

CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE LEGAL SERVICES.
DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN ATTORNEY-CLIENT RELATIONSHIP. CREATIVE
COMMONS PROVIDES THIS INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED HEREUNDER, AND
DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM THE USE OF THIS DOCUMENT OR THE INFORMATION
OR WORKS PROVIDED HEREUNDER.
*/
use std::sync::OnceLock;

thread_local! {
    /// Function that will be called when reading from a register using
    /// though the PAC API.
    ///
    /// The function parameters are the following:
    /// - a u64 representing the register address
    /// - a u64 representing the read mask (i.e. how many bits are read)
    ///   This is necessary due to the way that the generated PACs handles
    ///   generic register sizes.
    pub(crate) static READ_FN: OnceLock<fn(usize,usize)->u64> = OnceLock::new();
    /// Function that will be called when writing to a register using
    /// though the PAC API.
    ///
    /// The function parameters are the following:
    /// - a u64 representing the register address
    /// - a u64 representing the write mask (i.e. how many bits are read)
    ///   This is necessary due to the way that the generated PACs handles
    ///   generic register sizes.
    /// - a u64 representing the value that gets written to the register
    pub (crate) static WRITE_FN: OnceLock<fn(usize,usize,u64)> = OnceLock::new();

    pub (crate) static LDMST: OnceLock<fn(usize,u64)> = OnceLock::new();
}

/// Macro to generate the setters for the thread_local static
/// register access functions.
macro_rules! set_access_fn {
    ($CONST_ID:ident,$fn_id:ident,$fn_literal:literal,$access_fn_type:ty,$doc:literal) => {
        #[doc=$doc]
        pub fn $fn_id(fun: $access_fn_type) -> Result<(), String> {
            $CONST_ID.with(|function| {
                function.set(fun).or_else(|_| {
                    Err(format!(
                        "The thread local {} can only be set once.",
                        $fn_literal
                    ))
                })
            })
        }
    };
}

set_access_fn!(READ_FN, set_read_fn, "read_fn", fn(usize, usize) -> u64, "Set the function that is called when a read to a register happens\n through the PAC API.\n The function is called with the following arguments (in order):\n - a u64 representing the register address\n - a u64 representing the read mask (i.e. how many bits are read)\n This is necessary due to the way that the generated PACs handle\n generic register sizes.\n");
set_access_fn!(WRITE_FN, set_write_fn, "write_fn", fn(usize, usize, u64),"Set the function that is called when a write to a register happens\n through the PAC API.\n The function is called with the following arguments (in order):\n - a u64 representing the register address\n - a u64 representing the write mask (i.e. how many bits are read)\n This is necessary due to the way that the generated PACs handle\n generic register sizes.\n - a u64 representing the value that gets written to the register\n");

// # Why does this exist?
//
// When writing tests, it is useful to be able to read/write registers that
// are normally **read-only/write-only**.
// Especially when simulating registers on non-embedded targets
// one might want to provide a test value to an input buffer register, or
// read a value from a write only resister to check its value.
//
// The two traits: `ReadOnlyWrite` and `WriteOnlyRead` defined in this
// module, provide exactly that functionality to every [`Reg<T, A: Access>`](crate::common::Reg).
//
// By importing this module, all read-only registers become `ReadOnlyWrite`
// and all write-only registers become `WriteOnlyRead`.
// With these new markers the following functions are unlocked:
// - `read_write_only` (like read but for **write-only** registers)
// - `write_read_only` (like write but for **read-only** registers)
// - `init_read_only` (like init but for **read-only** registers)
// - `modify_read_only` (like modify but for **read-only** registers)
// - `modify_write_only` (like modify but for **write-only** registers)
//
// This separates these special register accesses form the ones allowed as by
// the SVD-spec of the target device. The separation is reinforced by keeping
// the trait in a separate module, requiring an explicit import of the module.
// The module should not be imported in productive code, only in test code.
//
// # Alternative Solution
//
// In previous commits, an alternative solution in common.rs as tested:
// ```rust,ignore
// #[cfg(feature = "tracing")]
// impl Read for W {}
// #[cfg(feature = "tracing")]
// impl Write for R {}
// ```
// We found that the usual setup for projects that depend on the generated PAC
// ends up looking like this:
// ```toml
// [dependencies]
// pac_for_chip = { version = "x.x.x" }
// [dev-dependencies]
// pac_for_chip = { version = "x.x.x", features = ["tracing"] }
// ```
//
// Unfortunately, rust-analyzer was not smart enough to **not** suggest
// `read()/write()` for registers that are write-only/read-only when writing
// **non**-test code (i.e. code that would end up in an embedded build).
// `cargo` would not allow you to build in release mode (i.e. for embedded)
// and spit out errors that some registers cannot be read from/written to.
//
// This could cause confusion as to why rust-analyzer suggests these functions
// and cargo failing to build.

/// Allow reading and writing registers that are read-only/write-only.
///
/// Don't ever use this module unless you know what you are doing. This allows
/// reading/writing registers that are write-/read-only respectively.
///
/// Use this module when writing tests where you want to read/write simulated
/// registers to check their contents or simulate an external input to a
/// read-only buffer register.
#[cfg(feature = "tracing")]
pub mod insanely_unsafe {
    use crate::common::sealed::{CastFrom, RegSpec};
    use crate::common::{Access, Read, Reg, Write, R, W};
    use crate::{RegValueT, RegisterValue};

    pub trait WriteOnlyRead: Access {}
    impl WriteOnlyRead for W {}
    pub trait ReadOnlyWrite: Access {}
    impl ReadOnlyWrite for R {}

    impl<T: RegSpec, A: WriteOnlyRead> Reg<T, A> {
        /// Read a **write-only** register.
        ///
        /// # Safety
        /// Reading from a write-only register can cause undefined behavior on target devices.
        /// This function shall only ever be used on non-embedded devices when simulating registers.
        #[inline(always)]
        pub unsafe fn read_write_only(&self) -> RegValueT<T> {
            let val = {
                let mut buf: u64 = 0x0;
                super::READ_FN.with(|rf| {
                    buf = rf.get().unwrap()(self.addr(), std::mem::size_of::<T::DataType>());
                });
                T::DataType::cast_from(buf)
            };
            <RegValueT<_> as RegisterValue<_>>::new(val)
        }
    }

    impl<T: RegSpec, A: ReadOnlyWrite> Reg<T, A> {
        /// Write register value back to **read-only** register.
        ///
        /// # Arguments
        ///
        /// * `reg_value` - A string slice that holds the name of the person
        ///
        /// # Safety
        /// Write operation on a **read-only** register can cause undefined
        /// behavior. This function shall only ever be used on non-embedded targets
        /// (e.g. when simulating registers).
        #[inline(always)]
        pub unsafe fn write_read_only(&self, reg_value: RegValueT<T>) {
            super::WRITE_FN.with(|wf| {
                wf.get().unwrap()(
                    self.addr(),
                    std::mem::size_of::<T::DataType>(),
                    reg_value.data.into(),
                )
            });
        }
    }

    impl<T: Default + RegSpec, A: ReadOnlyWrite> Reg<T, A>
    where
        RegValueT<T>: Default,
    {
        /// Init **read-only** register with value returned by the closure.
        ///
        /// # Arguments
        ///
        /// * `f` - Closure that receive as input a register value initialized with register value at Power On Reset.
        ///
        /// # Safety
        /// This is extremely unsafe and shall only ever be used on non-embedded
        /// devices in order init simulated registers.
        ///
        #[inline(always)]
        /// Write value computed by closure that receive as input the reset value of register
        pub unsafe fn init_read_only(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
            let val = RegValueT::<T>::default();
            let res = f(val);
            self.write_read_only(res);
        }
    }

    impl<T: RegSpec, A: WriteOnlyRead + Write> Reg<T, A> {
        #[inline(always)]
        /// Don't ever use this on embedded targets. Only use for unit tests on
        /// host machines.
        /// Write register with value returned by the closure.
        ///
        /// # Arguments
        ///
        /// * `f` - Closure that receive as input a register value read from register.
        ///
        /// # Safety
        /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
        /// Register is Send and Sync to allow complete freedom. Developer is responsible for proper use with multithreaded tests.
        ///
        pub unsafe fn modify_write_only(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
            let val = self.read_write_only();
            let res = f(val);
            self.write(res);
        }
    }

    impl<T: RegSpec, A: Read + ReadOnlyWrite> Reg<T, A> {
        #[inline(always)]
        /// Write a **read-only** register with value returned by the closure.
        ///
        /// # Arguments
        ///
        /// * `f` - Closure that receive as input a register value read from register.
        ///
        /// # Safety
        /// Write operation on **read-only** registers can cause undefined
        /// behavior on embedded devices. See module level safety warnings for explanation when to use this function.
        /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
        /// Register is Send and Sync to allow complete freedom. Developer is responsible for proper use with multithreaded tests.
        ///
        pub unsafe fn modify_read_only(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
            let val = self.read();
            let res = f(val);
            self.write_read_only(res);
        }
    }
}
