//! Collection of data structures and functions that power `regmock_rs`.
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use derive_builder::Builder;
use serde::Deserialize;
use serde_json;

/// Enum representing types of register accesses.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegisterAccessType {
    #[serde(alias = "r")]
    READ,
    #[serde(alias = "w")]
    WRITE,
    #[cfg(feature = "aurix")]
    LDMST,
}

/// Stores information of a specific registers access.
///
/// Members are all optional to allow for easy partial comparison. See the
/// [`PartialEq`](#impl-PartialEq%3CRegisterAccess%3E-for-RegisterAccess)
/// implementation for more details and specific examples.
///
/// See the convenience functions [`access_gen::read`], [`access_gen::read_value`],
/// [`access_gen::write`] and [`access_gen::write_value`] for a shorthand ways to
/// construct `RegisterAccess` structs.
#[derive(Default, Clone, Eq, Builder, Deserialize)]
#[builder(default)]
#[serde(default)]
pub struct RegisterAccess {
    /// Type of the register access.
    #[serde(alias = "type")]
    #[builder(setter(into, strip_option))]
    pub ty: Option<RegisterAccessType>,
    /// Address of accessed register.
    #[builder(setter(into, strip_option))]
    pub addr: Option<usize>,
    /// Length of the access mask in bytes.
    #[builder(setter(into, strip_option))]
    pub len: Option<usize>,
    /// Value of the register before the access.
    #[builder(setter(into, strip_option))]
    pub before: Option<u64>,
    /// Value of the register after the access.
    #[builder(setter(into, strip_option))]
    pub after: Option<u64>,
}

impl Debug for RegisterAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("RegisterAccess");
        if let Some(ty) = &self.ty {
            debug_struct.field("ty", ty);
        }
        if let Some(addr) = &self.addr {
            debug_struct.field("addr", addr);
        }
        if let Some(len) = &self.len {
            debug_struct.field("len", len);
        }
        if let Some(before) = &self.before {
            debug_struct.field("before", before);
        }
        if let Some(after) = &self.after {
            debug_struct.field("after", after);
        }
        debug_struct.finish()
    }
}

/// Compares only part of the struct that are `Some()` on `&self`. As this struct
/// should only be used for writing unit tests, it is useful to provide
/// an implementation of `PartialEq` that checks values that are there.
/// Consequent this means that a `RegisterAccess` struct with all members
/// `None` is equal to every `RegisterAccess` struct.
///
/// # Examples
///
/// ```rust
/// use regmock_rs::utils::*;
///
/// let full  = RegisterAccess::new(RegisterAccessType::READ,
///     0xDEADC0DE,
///     8,
///     0x0,
///     0xC0FFEE,
/// );
/// let mut partial = RegisterAccess::default();
/// partial.ty = Some(RegisterAccessType::READ);
/// assert_eq!(partial, full);
/// ```
impl PartialEq for RegisterAccess {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        if self.ty.is_some() && other.ty.is_some() {
            ret = ret && self.ty.eq(&other.ty);
        }
        if self.addr.is_some() && other.addr.is_some() {
            ret = ret && self.addr.eq(&other.addr);
        }
        if self.len.is_some() && other.len.is_some() {
            ret = ret && self.len.eq(&other.len);
        }
        if self.before.is_some() && other.before.is_some() {
            ret = ret && self.before.eq(&other.before);
        }
        if self.after.is_some() && other.after.is_some() {
            ret = ret && self.after.eq(&other.after);
        }
        ret
    }
}

impl RegisterAccess {
    /// Constructor for `RegisterAccess` that takes all its members as arguments.
    ///
    /// See also the shorthand constructors [`access_gen::read`], [`access_gen::read_value`],
    /// [`access_gen::write`] and [`access_gen::write_value`] for constructing partial
    /// `RegisterAccess` structs.
    pub fn new(ty: RegisterAccessType, addr: usize, len: usize, before: u64, after: u64) -> Self {
        Self {
            ty: Some(ty),
            addr: Some(addr),
            len: Some(len),
            before: Some(before),
            after: Some(after),
        }
    }
    /// Deserialize a sequence of register accesses from a JSON array.
    pub fn seq_from_json(data: &str) -> Vec<RegisterAccess> {
        serde_json::from_str(data).unwrap()
    }
}

/// Short-hand constructors for various [`RegisterAccess`] types. Useful when
/// constructing sequences to match logs against.
pub mod access_gen {
    use super::RegisterAccess;
    use super::RegisterAccessType::{READ, WRITE};

    /// Construct a [`RegisterAccess`] of type [`READ`]
    /// from a specific register.
    pub fn read(address: usize) -> RegisterAccess {
        RegisterAccess {
            ty: Some(READ),
            addr: Some(address),
            len: None,
            before: None,
            after: None,
        }
    }

    /// Construct a [`RegisterAccess`] of type [`READ`]
    /// that from a specific register with a read value.
    pub fn read_value(address: usize, value: u64) -> RegisterAccess {
        RegisterAccess {
            ty: Some(READ),
            addr: Some(address),
            len: None,
            before: None,
            after: Some(value),
        }
    }

    /// Construct a [`RegisterAccess`] of type [`WRITE`]
    /// that from a specific register.
    pub fn write(address: usize) -> RegisterAccess {
        RegisterAccess {
            ty: Some(WRITE),
            addr: Some(address),
            len: None,
            before: None,
            after: None,
        }
    }

    /// Construct a [`RegisterAccess`] of type [`WRITE`]
    /// that from a specific register with a written value.
    pub fn write_value(address: usize, value: u64) -> RegisterAccess {
        RegisterAccess {
            ty: Some(WRITE),
            addr: Some(address),
            len: None,
            before: None,
            after: Some(value),
        }
    }
}

/// List of [`RegisterAccess`]'s where **`READ`** accesses are run-length-encoded.
#[derive(Debug, Clone, Default)]
pub struct RegmockLog {
    /// List of register accesses with run-length-encoded **`READ`** access.
    pub log: Vec<(RegisterAccess, usize)>,
}

impl RegmockLog {
    // Add new log entry to the log. Reads accesses are run-length-encoded.
    pub(crate) fn push_log_entry(&mut self, entry: RegisterAccess) {
        match self.log.last_mut() {
            Some(ref mut last) => {
                if entry
                    .ty
                    .as_ref()
                    .is_some_and(|ty| *ty == RegisterAccessType::READ)
                    && last.0 == entry
                {
                    last.1 += 1;
                } else {
                    self.log.push((entry, 1));
                }
            }
            None => {
                self.log.push((entry, 1));
            }
        }
    }

    /// Check if specified register is currently being polled for at least `count` times.
    pub(crate) fn is_being_polled(&self, addr: usize, count: usize) -> bool {
        self.log.last().is_some_and(|last| {
            last.0.addr.as_ref().is_some_and(|a| *a == addr)
                && last
                    .0
                    .ty
                    .as_ref()
                    .is_some_and(|ty| ty == &RegisterAccessType::READ)
                && last.1 > count
        })
    }

    /// Get a [`IterRegmockLogNoPolling`] iterator without polling **READ**'s.
    pub fn iter(&self) -> IterRegmockLogNoPolling<'_> {
        IterRegmockLogNoPolling {
            inner: self,
            pos: 0,
        }
    }

    /// Get a [`IterRegmockLogDecoded`] iterator with **all** recorded accesses.
    pub fn iter_full(&self) -> IterRegmockLogDecoded<'_> {
        IterRegmockLogDecoded {
            inner: self,
            pos: 0,
            counter: 0,
        }
    }

    /// Count number of **all** recorded register accesses in the log.
    pub fn len_full(&self) -> usize {
        self.iter_full().count()
    }
}

/// Iterator over [`RegisterAccess`] values without sequences of duplicate
/// **`READ`** entries (i.e. skips register polling accesses).
pub struct IterRegmockLogNoPolling<'a> {
    inner: &'a RegmockLog,
    pos: usize,
}

impl<'a> Iterator for IterRegmockLogNoPolling<'a> {
    type Item = &'a RegisterAccess;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.log.len() {
            None
        } else {
            self.pos += 1;
            match self.inner.log.get(self.pos - 1) {
                Some(e) => Some(&e.0),
                None => None,
            }
        }
    }
}

/// Iterator over [`RegisterAccess`] values including **all** sequences of duplicate
/// log entries (i.e. decodes the RLE encoded `log` of [`RegmockLog`])
///
/// # Note
/// This iterator skips entries in [`RegmockLog`] that have a run-length of 0.
pub struct IterRegmockLogDecoded<'a> {
    inner: &'a RegmockLog,
    pos: usize,
    counter: usize,
}

impl<'a> Iterator for IterRegmockLogDecoded<'a> {
    type Item = &'a RegisterAccess;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.log.len() {
            None
        } else {
            match self.inner.log.get(self.pos) {
                Some(current) => {
                    if self.counter >= current.1 {
                        self.counter = 0;
                        self.pos += 1;
                        self.next()
                    } else {
                        self.counter += 1;
                        Some(&current.0)
                    }
                }
                None => None,
            }
        }
    }
}

/// Type used to mock registers. Export to make typing in tests more readable.
/// Used by [`Regmock`]
pub type RegisterMap = HashMap<usize, u64>;
/// Type of read-access callback functions that can be registered in [`Regmock::read_fn`].
pub type ReadFunction = Box<dyn FnMut(&mut RegisterMap, u64) -> u64 + Send>;
/// Type of write-access callback functions that can be registered in [`Regmock::write_fn`].
pub type WriteFunction = Box<dyn FnMut(&mut RegisterMap, u64, u64) -> u64 + Send>;

/// Mock and record register accesses of embedded devices.
/// # Regmock
///
/// [`Regmock`] is the heart of `regmock-rs` and serves both as mock for
/// registers of embedded devices, as well as a recorder access logs.
///
/// In addition to mocking and logging register accesses, it allows registering
/// callbacks with specific registers, that get executed when a specific
/// type of access happens to a register. See [`read_fn`](#structfield.read_fn) and [`write_fn`](#structfield.write_fn).
///
/// # Examples
///
/// The code below creates a new [`Regmock`] struct and sets it as the
/// `thread_local` mock object.
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use regmock_rs::{init_regmock, utils::Regmock};
/// init_regmock(Arc::new(Mutex::new(Regmock::default())));
/// ```
pub struct Regmock {
    /// List of [`RegisterAccess`] that happen through the PAC library.
    ///
    /// Stores tuples of [`RegisterAccess`] structs and counter for run-length-encoding.
    /// To circumvent the logging, either use [`crate::silent()`].
    pub log: RegmockLog,

    /// Register mocks.
    ///
    /// ## Note:
    /// Registers are initialized to 0x0 and not the chip specific initial
    /// register value. Developers shall use an unrecorded call to `pac::REGISTER::init()`
    /// or write the desired initial value directly into [`register_mocks`](#register_mocks)
    /// object to have correct initial register values if absolutely needed.
    pub register_mocks: RegisterMap,

    /// A map from register addresses to [`ReadFunction`] that gets called
    /// every time a specific register is *read* from through the PAC.
    ///
    /// # Closure
    /// When reading from a specific register though the PAC and a
    /// [`ReadFunction`] exists in [`super::read_fn`], the closure gets called,
    /// and the result is treated as the value that was read from
    /// the register.
    ///
    /// If no [`ReadFunction`] exists for a specific address, the read
    /// is passed though to the mocked register in [`register_mock`](crate::utils::Regmock#structfield.register_mock)
    ///
    /// ## Arguments
    /// When a function is called, it gets a mutable reference to the
    /// [`register_mocks`](#register_mocks) member and
    /// (this will probably be removed) the value that was read from the mocked
    /// register.
    /// The value returned from this function is treated as the value that
    /// was read from the register is passed back to the PAC.
    ///
    /// # Examples
    ///
    /// A function that modify the `pac::PERIPHERAL.register_x()` register, when a
    /// read of `pac::PERIPHERAL.register_y()` happens.
    ///
    /// ```rust,ignore
    /// let mut mock = regmock_rs::utils::Regmock::default();
    /// let callback = |registers: &mut HashMap<usize, u64>, val: u64| -> u64 {
    ///     registers.insert(pac::PERIPHERAL.register_x().addr(), 0xDEADC0DE);
    ///     val
    /// };
    /// mock
    ///     .read_fn
    ///     .insert(pac::PERIPHERAL.register_y().addr(), Box::new(callback));
    /// ```
    ///
    /// # Limitations
    ///
    /// Due to lifetime reasons, it is currently not possible to capture
    /// the environment by reference. Only `move` captures are allowed.
    ///
    /// To have references to local values in the callbacks, store the
    /// local value in a `Rc<T>` and move a cloned value of that `Rc<T>` into
    /// the closure.
    ///
    /// ```rust,ignore
    /// let mut mock = regmock_rs::utils::Regmock::default();
    /// let local_value = std::rc::Rc::new(std::cell::RefCell::new(0));
    /// let local_value_clone = local_value.clone();
    /// let callback = |_: &mut HashMap<usize, u64>, val| -> u64 {
    ///     *local_value_clone.borrow_mut() +=1;
    ///     val
    /// };
    /// mock.read_fn
    ///     .insert(pac::PERIPHERAL.register_y().addr(), Box::new(callback));
    /// let _ = unsafe { pac::PERIPHERAL.register_y().read() };
    /// assert_eq!(local_value.borrow(), 1);
    /// ```
    ///
    /// See crate level examples and tests for more.
    pub read_fn: HashMap<usize, ReadFunction>,

    /// A map of register addresses to [`WriteFunction`] that get called
    /// every time a specific register is *written* to through the PAC.
    ///
    /// # Closure
    /// When writing to a specific register though the PAC and a
    /// [`WriteFunction`] exists for that register, the [`WriteFunction`] gets
    /// called.
    ///
    /// If no [`WriteFunction`] exists for a specific address, the write
    /// is passed though to the mock.
    ///
    /// ## Arguments
    /// When a function is called, it gets a mutable reference to the
    /// [`register_mocks`](#register_mocks) member and
    /// (this will probably be removed) the value that was read from the mocked
    /// register.
    /// The value returned from this function is treated as the value that
    /// was read from the register is passed back to the PAC.
    ///
    /// The key is the physical address of the register. The associated
    /// value is a [`WriteFunction`].
    ///
    /// When a function is called, it gets passed
    /// - a mutable reference to the [`register_mocks`](#register_mocks) member
    /// - the value of the register before the write.
    /// - the value that should be written to the register.
    ///
    /// The value returned from this function is treated as the value that
    /// is written to the register.
    ///
    /// ## Examples
    ///
    /// A function that modifies the `pac::PERIPHERAL::register_x()` register,
    /// when a write to `pac::PERIPHERAL::register_y()` happens.
    ///
    /// ```rust,ignore
    /// let a = |registers: &mut HashMap<usize, u64>, before: u64, val: u64| -> u64 {
    ///     registers.insert(GPIO0.r#in().addr(), 0xDEADC0DE);
    ///     val
    /// };
    /// ```
    ///
    /// # Limitations
    ///
    /// See [`Regmock::read_fn`] limitations section.
    pub write_fn: HashMap<usize, WriteFunction>,

    /// Controls if the register accesses get logged.
    /// Defaults to `true`.
    pub log_enabled: bool,

    /// Controls if the register callback functions get executed. Used
    /// by the `no_log!` macro to bypass the logging and callbacks.
    /// Defaults to `true`.
    pub callback_enabled: bool,

    /// Function to resolve the address of a register to its name
    ///
    /// Consider using [`Regmock::get_reg_name`] which provides a simpler interface
    pub name_resolver: Option<Box<dyn Fn(u64) -> Option<&'static &'static str> + Send>>,
}

impl Debug for Regmock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Regmock")
            .field("log", &self.log)
            .field("read_fn", &"TODO: HashMap<usize, ReadFunction>")
            .field("write_fn", &"TODO: HashMap<usize, WriteFunction>")
            .field("log_enabled", &self.log_enabled)
            .field("callback_enabled", &self.callback_enabled)
            .finish()
    }
}

impl Default for Regmock {
    /// Construct a default [`Regmock`] struct.
    fn default() -> Self {
        Self {
            log: Default::default(),
            register_mocks: Default::default(),
            read_fn: Default::default(),
            write_fn: Default::default(),
            log_enabled: true,
            callback_enabled: true,
            name_resolver: None,
        }
    }
}

impl Regmock {
    pub fn with_resolver<T>(resolver: &'static T) -> Self
    where
        T: Fn(u64) -> Option<&'static &'static str> + Send + Sync,
    {
        Self {
            log: Default::default(),
            register_mocks: Default::default(),
            read_fn: Default::default(),
            write_fn: Default::default(),
            log_enabled: true,
            callback_enabled: true,
            name_resolver: Some(Box::new(resolver)),
        }
    }

    fn get_reg_value(&mut self, addr: usize) -> u64 {
        let register_mocks = &mut self.register_mocks;
        if let std::collections::hash_map::Entry::Vacant(e) = register_mocks.entry(addr) {
            e.insert(0u64);
            0u64
        } else {
            *register_mocks.get(&addr).unwrap()
        }
    }
    /// Execute the register specific `read_fn` callback/closure thing if there
    /// exists one for the current register.
    fn exec_read_fn(&mut self, addr: usize, before: u64) -> u64 {
        if !self.read_fn.contains_key(&addr) || !self.callback_enabled {
            before
        } else {
            let cb = self.read_fn.get_mut(&addr).unwrap();
            cb(&mut self.register_mocks, before)
        }
    }

    /// Execute the register specific `read_fn` callback/closure thing if there
    /// exists one for the current register.
    fn exec_write_fn(&mut self, addr: usize, before: u64, val: u64) -> u64 {
        if !self.write_fn.contains_key(&addr) || !self.callback_enabled {
            val
        } else {
            let cb = self.write_fn.get_mut(&addr).unwrap();
            cb(&mut self.register_mocks, before, val)
        }
    }

    /// Construct a default [`Regmock`].
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    /// Get a copy of the recorded register accesses.
    pub fn get_logs(&self) -> RegmockLog {
        self.log.clone()
    }

    pub fn get_reg_name(&self, addr: usize) -> Option<&'static str> {
        self.name_resolver
            .as_ref()
            .and_then(|r| r(addr as u64))
            .copied()
    }
}

impl Regmock {
    /// Read from the mocked register and return the read value.
    ///
    /// This function can be registered as the function to call when reading
    /// from a register with `feature=tracing` enabled.
    /// # Examples
    ///
    /// ```rust,ignore
    /// pac::tracing::set_read_fn(regmock_rs::read_fn).unwrap();
    /// ```
    /// To register the function with the PAC library. Consult the documentation
    /// of your specific PAC for more information.
    pub fn read_volatile(&mut self, addr: usize, len: usize) -> u64 {
        let before = self.get_reg_value(addr);
        let after = self.exec_read_fn(addr, before);

        if self.log_enabled {
            self.log.push_log_entry(RegisterAccess::new(
                RegisterAccessType::READ,
                addr,
                len,
                before,
                after,
            ));
        }
        after
    }

    /// Write value to a mocked register.
    ///
    /// This function can be registered in the used `PAC`, as the function to
    /// call when writing to a register with `feature=tracing` enabled.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// pac::tracing::set_write_fn(regmock_rs::write_fn).unwrap();
    /// ```
    /// To register the function with the PAC library. Consult the documentation
    /// of your specific PAC for more information.
    pub fn write_volatile(&mut self, addr: usize, len: usize, val: u64) {
        let before = self.get_reg_value(addr);
        let after = self.exec_write_fn(addr, before, val);

        if self.log_enabled {
            self.log.push_log_entry(RegisterAccess::new(
                RegisterAccessType::WRITE,
                addr,
                len,
                before,
                after,
            ));
        }
        self.register_mocks.insert(addr, after);
    }

    #[cfg(feature = "aurix")]
    /// Basic impl of the LMST tracing interace for the Aurix chip.
    /// TODO: maybe add a dedicated field for callbacks for lmst register
    /// accesses.
    pub fn load_modify_store(&mut self, addr: usize, len: usize, val: u64) {
        let before = self.get_reg_value(addr);
        let after = self.exec_write_fn(addr, before, val);

        if !self.log_enabled {
            self.log.push_log_entry(RegisterAccess::new(
                RegisterAccessType::WRITE,
                addr,
                len,
                before,
                after,
            ));
        }
        self.register_mocks.insert(addr, after);
    }
}
