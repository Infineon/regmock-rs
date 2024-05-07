use pac::tracing::{set_read_fn, set_write_fn};
use regmock_rs::init_regmock;
use regmock_rs::utils::Regmock;
use std::sync::{Arc, Mutex};
use test_pac as pac;

/// Initialize mock related things.
/// - register a `Regmock` object as the thread_local mock object.
/// - register the read_fn and write_fn with the `pac::tracing`
pub fn init_mock(regmock: Option<Arc<Mutex<Regmock>>>) -> Arc<Mutex<Regmock>> {
    let regmock = regmock.unwrap_or(Arc::new(Mutex::new(Regmock::default())));
    init_regmock(regmock.clone());
    set_read_fn(regmock_rs::read_fn).unwrap();
    set_write_fn(regmock_rs::write_fn).unwrap();
    regmock
}
