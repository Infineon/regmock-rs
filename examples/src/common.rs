use std::sync::{Arc, Mutex};

use regmock_rs::{init_regmock, utils::Regmock};
use test_pac::tracing::{set_read_fn, set_write_fn};

/// The setup function that we'll use from most tests
///
/// It will do the wiring between the PAC and regmock
/// such that the normal callback functions from regmock
/// are called on reads/writes to registers.
///
/// This way the machinery of regmock to record accesses
/// and mock registers will be enabled.
///
/// We have the ability to pass a custom regmock instance in
/// case we need it to wire it up with a hardware model.
/// In the same vein it's also returned so that we can set
/// up some simple mocking.
pub fn init_mock(regmock: Option<Arc<Mutex<Regmock>>>) -> Arc<Mutex<Regmock>> {
    let regmock = match regmock {
        Some(regmock) => regmock,
        None => Arc::new(Mutex::new(Regmock::with_resolver(
            &test_pac::reg_name::reg_name_from_addr,
        ))),
    };
    // set this regmock up as the thread-local global
    init_regmock(regmock.clone());

    set_read_fn(regmock_rs::read_fn).unwrap();
    set_write_fn(regmock_rs::write_fn).unwrap();

    regmock
}
