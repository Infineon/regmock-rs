#![doc = include_str!("../README.md")]

use std::sync::{Arc, Mutex, OnceLock};

pub mod matchers;
pub mod utils;
use crate::utils::Regmock;

thread_local! {
    /// Global Regmock object used by `read_fn`,`write_fn` and `ldmst_fn`
    /// to mock registers and chip behavior.
    pub(crate) static  MOCK: ThreadLocalRegmock= const {OnceLock::new()};
}

type ThreadLocalRegmock = OnceLock<Arc<Mutex<Regmock>>>;

/// Errors generated when handling the `thread_local` locked [`Regmock`] object.
#[derive(Debug, Clone)]
pub enum MockError {
    /// [`init_regmock`] was not called
    MockNotInitialized,
    /// could not acquire lock to [`Regmock`] object.
    LockError,
}

impl From<MockError> for String {
    fn from(value: MockError) -> Self {
        format!("failed due to: {:?}", value)
    }
}

/// Execute function against `thread_local` [`Regmock`] object.
pub(crate) fn with_mock<F, R>(f: F) -> Result<R, MockError>
where
    F: FnOnce(&mut Regmock) -> R,
{
    MOCK.with(|mock| -> Result<R, MockError> {
        let mut mock = mock
            .get()
            .ok_or(MockError::MockNotInitialized)?
            .lock()
            .map_err(|_| MockError::LockError)?;
        Ok((f)(&mut mock))
    })
}

/// Initialize the thread_local regmock object.
///
/// # Panics
///
/// This function will `panic!()` when not being able to initialize the `thread_local`
/// [`Regmock`] object.
pub fn init_regmock(mock: Arc<Mutex<Regmock>>) {
    MOCK.with(|m| match m.set(mock) {
        Ok(_) => {}
        Err(e) => panic!("Failed to initialize thread_local Regmock with: {:?}", e),
    })
}

/// Disable logging and execution of callbacks during the closure `f`.
///
/// # Panics
///
/// This function calls `panic!()` when the `thread_local` [`Regmock`] object
/// cannot be accessed.
///
/// # Examples
///
/// To perform a write to a register without logging, pass the write as closure to
/// this function.
///
/// ```rust,ignore
/// regmock_rs::silent(|| {
///     let _ = pac::REGISTER.bitfield().read();
/// }).unwrap();
/// ```
///
/// This function can be called recursively and will restore state.
/// ```rust,ignore
/// regmock_rs::silent(|| unsafe {
///     let val = pac::REGISTER.bitfield().read(); // no log, no callback
///     regmock_rs::logging(true);
///     let val2 = pac::REGISTER.bitfield().read(); // logged, no callback
///     regmock_rs::silent(||{
///         pac::REGISTER.bitfield().write(val); // no log, no callback
///     }).unwrap();
///     pac::REGISTER.bitfield().write(val2); // logged, no callback
/// }).unwrap();
/// ```
pub fn silent<T>(f: impl FnOnce() -> T) -> Result<T, MockError> {
    let mut state = (true, true);
    with_mock(|regmock| {
        state = (regmock.log_enabled, regmock.callback_enabled);
        regmock.log_enabled = false;
        regmock.callback_enabled = false;
    })?;
    let ret = f();
    with_mock(|regmock| {
        (regmock.log_enabled, regmock.callback_enabled) = state;
    })?;
    Ok(ret)
}

/// Enable/disable logging of register accesses in the `thread_local` MOCK object.
pub fn logging(state: bool) -> Result<(), MockError> {
    with_mock(|mock| {
        mock.log_enabled = state;
    })
}

/// Block until specific register is being polled or timeout.
///
/// `count` specifies the number of consecutive reads to a register that should
/// be considered polling.
pub fn wait_until_polled(
    addr: usize,
    count: usize,
    timeout: Option<std::time::Duration>,
) -> Result<(), String> {
    let start = std::time::Instant::now();
    loop {
        match with_mock(|mock| mock.log.is_being_polled(addr, count)) {
            Ok(polled) => {
                if polled {
                    break Ok(());
                } else if timeout.is_some_and(|to| to < start.elapsed()) {
                    break Err(format!(
                        "Timed out waiting for 0x{:08X} to be polled.",
                        addr
                    ));
                } else {
                    std::thread::yield_now();
                }
            }
            Err(e) => {
                break Err(format!(
                    "Failed to aquire thread_local mock due to: {:?}",
                    e
                ))
            }
        };
    }
}

/// Enable/disable the execution of callbacks in the `thread_local` MOCK object.
pub fn callbacks(state: bool) -> Result<(), MockError> {
    with_mock(|mock| {
        mock.callback_enabled = state;
    })
}

/// Get the [`utils::RegmockLog`] form the `thread_local` MOCK object.
pub fn logs() -> Result<utils::RegmockLog, MockError> {
    with_mock(|mock| mock.get_logs())
}

/// Perform a read from the mocked registers.
/// Register this function as the `READ_FN` in the `pacgen` PAC.
///
/// # Panics
///
/// This function calls `panic!()` if the `thead_local`, [`Regmock`] object
/// cannot be accessed.
pub fn read_fn(reg: usize, len: usize) -> u64 {
    with_mock(|mock| mock.read_volatile(reg, len)).unwrap_or_else(|e| {
        panic!(
            "Cound not `read_volatile(0x{:08X}, {:?})` due to: {:?}",
            reg, len, e
        )
    })
}

/// Perform a write from the mocked registers.
/// Register this function as the `WRITE_FN` in the `pacgen` PAC.
///
/// # Panics
///
/// This function calls `panic!()` if the `thead_local`, [`Regmock`] object
/// cannot be accessed.
pub fn write_fn(reg: usize, len: usize, value: u64) {
    with_mock(|mock| mock.write_volatile(reg, len, value)).unwrap_or_else(|e| {
        panic!(
            "Cound not `write_volatile(reg: 0x{:08X}, len: {:?}, value: 0x{:08X})` due to: {:?}",
            reg, len, value, e
        )
    })
}

/// Perform a write from the mocked registers.
/// Register this function as the `WRITE_FN` in the `pacgen` PAC.
///
/// # Panics
///
/// This function calls `panic!()` if the `thead_local`, [`Regmock`] object
/// cannot be accessed.
#[cfg(feature = "aurix")]
pub fn ldmst_fn(reg: usize, len: usize, value: u64) {
    with_mock(|mock| mock.load_modify_store(reg, len, value)).unwrap_or_else(|e| {
        panic!(
            "Cound not `load_modify_store(reg: 0x{:08X}, value: 0x{:08X})` due to: {:?}",
            reg, len, e
        )
    })
}
