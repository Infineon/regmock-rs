use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use closure::closure;

use pac::{gpio, RegisterValue, GPIO};
use regmock_rs::utils::Regmock;
use test_pac as pac;

mod common;
use common::init_mock;

#[test]
fn test_wait_until_polling() -> Result<(), String> {
    let reporter = Arc::new(Mutex::new(Regmock::default()));
    init_mock(Some(reporter.clone()));

    let dut_thread = thread::spawn(closure!(clone reporter, ||{
        init_mock(Some(reporter.clone()));
        loop {
            if unsafe{ GPIO.r#in().read().get_raw() } == 0xC0FFEEu32 {
                eprintln!("Read 0xC0FFEE from GPIO.in register");
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }));

    regmock_rs::wait_until_polled(GPIO.r#in().addr(), 20, Some(Duration::from_millis(1000)))
        .unwrap();
    regmock_rs::silent(|| unsafe {
        // This is a false positive. The trait defined in the module is used.
        #[allow(unused_imports)]
        use pac::tracing::insanely_unsafe;
        GPIO.r#in().write_read_only(gpio::In::new(0xC0FFEE))
    });

    dut_thread
        .join()
        .map_err(|_| "Was not able to join DUT thread.".to_owned())
}

#[test]
fn test_wait_until_polling_fail() -> Result<(), String> {
    let reporter = Arc::new(Mutex::new(Regmock::default()));
    init_mock(Some(reporter.clone()));

    let dut_thread = thread::spawn(closure!(clone reporter, ||{
        init_mock(Some(reporter.clone()));
        loop {
            // we poll the "wrong" register here to make the test fail
            if unsafe{ GPIO.we().read().get_raw() } == 0xC0FFEEu32 {
                eprintln!("Read 0xC0FFEE from GPIO.in register");
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }));

    eprintln!("THREAD[0]: Waiting for barrier");
    let result =
        regmock_rs::wait_until_polled(GPIO.r#in().addr(), 20, Some(Duration::from_millis(1000)));
    eprintln!("THREAD[0]: Reached timeout waiting for dut_thread to poll");

    assert!(result.unwrap_err().contains("Timed out"));

    // set register so that thread will exit gracefully
    regmock_rs::silent(|| unsafe { GPIO.we().write(gpio::We::new(0xC0FFEE)) });

    dut_thread
        .join()
        .map_err(|_| "Was not able to join DUT thread.".to_owned())
}
