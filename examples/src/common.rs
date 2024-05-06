use std::sync::{Arc, Mutex};

use regmock_rs::{init_regmock, utils::Regmock};
use test_pac::tracing::{set_read_fn, set_write_fn};

pub fn init_mock(regmock: Option<Arc<Mutex<Regmock>>>) {
    match regmock {
        Some(regmock) => init_regmock(regmock),
        None => init_regmock(Arc::new(Mutex::new(Regmock::default()))),
    }
    set_read_fn(regmock_rs::read_fn).unwrap();
    set_write_fn(regmock_rs::write_fn).unwrap();
}
