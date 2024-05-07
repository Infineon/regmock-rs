use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std::usize;

use closure::closure;

use pac::{gpio, RegisterValue, GPIO};
use regmock_rs::utils::access_gen::{read_value, write_value};
use regmock_rs::utils::{
    RegisterAccess, RegisterAccessBuilder, RegisterAccessType, RegisterMap, Regmock,
};
use regmock_rs::{given, require_seq, silent};
use test_pac as pac;

mod common;
use common::init_mock;

#[test]
fn test_read_logging() {
    init_mock(None);

    let read = unsafe { GPIO.we().read() };
    assert_eq!(read.get_raw(), 0);

    let expected_access = read_value(GPIO.we().addr(), 0);
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.len_full(), 1);
    given!(logs.iter(), require_seq!(vec![&expected_access]));
}

#[test]
fn test_init_logging() {
    init_mock(None);

    unsafe { GPIO.we().init(|r| r.gpio8().set(gpio::we::Gpio8::OUT)) };
    let read = unsafe { GPIO.we().read() };
    assert_eq!(read.get_raw(), 0x100);

    let w = write_value(GPIO.we().addr(), 0x100);
    let r = read_value(GPIO.we().addr(), 0x100);
    let logs = regmock_rs::logs().unwrap();
    given!(logs.iter(), require_seq!(vec![&w, &r]));
    assert!(itertools::equal(&vec![w, r], logs.iter()));
}

#[test]
fn test_write_logging() {
    init_mock(None);

    unsafe {
        let current = GPIO.we().read();
        let new = current.gpio4().set(gpio::we::Gpio4::OUT);
        GPIO.we().write(new);
    };
    assert_eq!(
        silent(|| unsafe { GPIO.we().read() }).unwrap().get_raw(),
        0x10
    );

    let r = read_value(GPIO.we().addr(), 0);
    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x10);
    let logs = regmock_rs::logs().unwrap();
    given!(logs.iter(), require_seq!(vec![&r, &w]));
}

#[test]
fn test_read_write_read_logging() {
    init_mock(None);

    let _ = unsafe { GPIO.r#in().read() };
    let output = gpio::We::default().gpio0().set(gpio::we::Gpio0::OUT);
    unsafe { GPIO.we().write(output) };
    let _ = unsafe { GPIO.out().read() };

    let r1 = read_value(GPIO.r#in().addr(), 0);
    let w1 = write_value(GPIO.we().addr(), 0x1);
    let r2 = read_value(GPIO.out().addr(), 0);
    let log = regmock_rs::logs().unwrap();
    given!(log.iter(), require_seq!(vec![&r1, &w1, &r2]));
}

#[test]
fn test_modify_logging() {
    init_mock(None);

    unsafe { GPIO.we().modify(|r| r.gpio4().set(gpio::we::Gpio4::OUT)) };

    let r = read_value(GPIO.we().addr(), 0);
    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x10);
    let logs = regmock_rs::logs().unwrap();
    given!(logs.iter(), require_seq!(vec![&r, &w]));
}

#[test]
fn test_check_access_type_only() {
    init_mock(None);

    let default = Default::default();
    let _ = unsafe { GPIO.we().read() };
    unsafe { GPIO.we().write(default) };

    let w = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::WRITE))
        .build()
        .unwrap();
    let r = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::READ))
        .build()
        .unwrap();

    assert!(itertools::equal(
        &vec![r, w],
        regmock_rs::logs().unwrap().iter()
    ));
}

#[test]
fn test_access_type_sequence() {
    init_mock(None);

    for i in 1..10 {
        let read = unsafe { GPIO.we().read() };
        assert_eq!(read.get_raw(), i - 1);
        unsafe { GPIO.we().write(gpio::We::default().set_raw(i)) };
    }

    let w = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::WRITE))
        .addr(Some(GPIO.we().addr()))
        .build()
        .unwrap();
    let r = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::READ))
        .build()
        .unwrap();
    let accesses: Vec<RegisterAccess> = (1..10)
        .map(|_| vec![r.clone(), w.clone()])
        .flatten()
        .collect();

    assert!(itertools::equal(
        &accesses,
        regmock_rs::logs().unwrap().iter()
    ));
}

// TODO
#[test]
fn test_access_count() {
    init_mock(None);
    let _ = unsafe { GPIO.we().read() };
    assert_eq!(regmock_rs::logs().unwrap().len_full(), 1);
}

// TODO
#[test]
fn test_access_count_rle() {
    init_mock(None);
    for _ in 0..10 {
        let _ = unsafe { GPIO.we().read() };
    }
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.iter().count(), 1);
    assert_eq!(logs.len_full(), 10);
}

#[test]
fn test_custom_read_fn() {
    // Mock the we register such that after 4 reads the value 0xC0FFEE is returned.
    let cool_fn = |_: &mut RegisterMap, value: u64| -> u64 {
        static COUNTER: Mutex<u32> = Mutex::new(0);
        let mut ret = 0xC0FFEE;
        let mut counter = COUNTER.lock().unwrap();
        if *counter < 4 {
            ret = value;
        }
        println!(
            "Cool function was called. Counter={:?} value=0x{:08X} ret=0x{:08X}",
            *counter, value, ret
        );
        *counter += 1;
        ret
    };

    let mut reporter = Regmock::default();
    reporter.read_fn.insert(GPIO.we().addr(), Box::new(cool_fn));

    init_mock(Some(Arc::new(Mutex::new(reporter))));

    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0);
    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0);
    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0);
    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0);
    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0xC0FFEE);
    assert_eq!(unsafe { GPIO.we().read().get_raw() }, 0xC0FFEE);

    let logs = regmock_rs::logs().unwrap();
    logs.iter_full()
        .for_each(|element| println!("{:X?}", element));

    let r_0 = read_value(GPIO.we().addr(), 0);
    let r_coffee = read_value(GPIO.we().addr(), 0xC0FFEE);
    given!(
        logs.iter_full(),
        require_seq!(vec![&r_0, &r_0, &r_0, &r_0, &r_coffee, &r_coffee])
    );
    assert_eq!(logs.len_full(), 6);
}

#[test]
fn test_custom_write_fn() {
    // This function causes writes that are always one bigger than the value
    // that was actually written into the register.
    let cool_fn = |_: &mut HashMap<usize, u64>, _, val: u64| -> u64 { val + 1 };

    let mut reporter = Regmock::default();
    reporter
        .write_fn
        .insert(GPIO.we().addr(), Box::new(cool_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));

    for i in 1..10 {
        unsafe {
            GPIO.we().write(gpio::We::new(i));
            assert_eq!(GPIO.we().read().get_raw(), i + 1);
        }
    }

    let logs = regmock_rs::logs().unwrap();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let last_read = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::READ))
        .after(Some(10)) // the default value of the Ctrl register
        .build()
        .unwrap();
    assert_eq!(logs.len_full(), 18);
    assert_eq!(&last_read, logs.iter().last().unwrap());
}

#[test]
fn test_custom_advanced_write_fn() {
    // This function causes a value to be written to the
    // GPIO.r#in() register, when the register that this function is
    // registered with is written to.
    let advanced_fn = |register_mocks: &mut HashMap<usize, u64>, _, val: u64| -> u64 {
        register_mocks.insert(GPIO.r#in().addr(), 0xC0FFEE);
        println!("Added 0xC0FFEE to UART[0].reg32bitraw register");
        val
    };
    let mut reporter = Regmock::default();
    reporter
        .write_fn
        .insert(GPIO.out().addr(), Box::new(advanced_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));

    let (before, after) = unsafe {
        let before = GPIO.r#in().read();
        GPIO.out().write(gpio::Out::new(0x200));
        let after = GPIO.r#in().read();
        (before.get_raw(), after.get_raw())
    };
    assert_eq!(before, 0);
    assert_eq!(after, 0xC0FFEE);

    let logs = regmock_rs::logs().unwrap();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let r = read_value(GPIO.r#in().addr(), 0);
    let w = write_value(GPIO.out().addr(), 0x200);
    let r2 = RegisterAccessBuilder::default()
        .addr(Some(GPIO.r#in().addr()))
        .ty(Some(RegisterAccessType::READ))
        .after(Some(0xC0FFEE))
        .build()
        .unwrap();
    assert_eq!(logs.len_full(), 3);
    given!(logs.iter_full(), require_seq!(vec![&r, &w, &r2]));
    assert!(itertools::equal(&vec![r, w, r2], logs.iter()));
}

#[test]
fn test_debug_formatting() {
    let w = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::WRITE))
        .addr(Some(0xDEADC0DE))
        .len(Some(8))
        .before(Some(0xC0FFEE))
        .after(Some(0xDEAD10CC))
        .build()
        .unwrap();
    println!("{:?}", w);
    println!("{:x?}", w);
    println!("{:X?}", w);
}

#[test]
fn test_silent_access() {
    init_mock(None);
    silent(|| unsafe { GPIO.r#in().read() }).unwrap();
    let _ = unsafe { GPIO.r#in().read() };
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.len_full(), 1);
}

#[test]
fn test_boxed_callback_fnmut() {
    // Limitation: due to lifetimes, it is currently not possible to
    // capture by reference. Only moved values are allowed.
    // Unfortunately Rust does not have a native way of telling a closure that
    // it should capture its environment by clone (the way CPP does it). See
    // [rust-lang/rfcs/issues/2407](https://github.com/rust-lang/rfcs/issues/2407)
    // for more info.
    //
    // Solution for now: use `Arc` and pass move a cloned value into the closure.
    let local_value = Arc::new(Mutex::new(0));
    let local_value_clone = local_value.clone();

    let some_fn = move |_: &mut RegisterMap, val: u64| -> u64 {
        let mut lv_guard = local_value_clone.lock().unwrap();
        *lv_guard += 1;
        *lv_guard + val
    };
    let mut reporter = Regmock::default();
    reporter.read_fn.insert(GPIO.we().addr(), Box::new(some_fn));

    // If you want to use the callback functions this way, [closure](https://crates.io/crates/closure)
    // would be a nice solution. This way you can explicitly specify which
    // local values to clone, move, reference.
    // It should then look something like this:

    let other_value = Arc::new(Mutex::new(0.0));
    let other_fn = closure::closure!(clone other_value, |_: &mut RegisterMap, val: u64| -> u64{
        *other_value.lock().unwrap() +=0.5;
        val
    });

    reporter
        .read_fn
        .insert(GPIO.r#in().addr(), Box::new(other_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));
    unsafe {
        let _ = GPIO.we().read();
        let _ = GPIO.r#in().read();
    }
    let logs = regmock_rs::logs().unwrap();

    assert_eq!(logs.len_full(), 2);
    assert_eq!(*local_value.lock().unwrap(), 1);
    assert!(other_value.lock().unwrap().eq(&0.5));
}

#[test]
fn test_wait_until_polling() -> Result<(), String> {
    let reporter = Arc::new(Mutex::new(Regmock::default()));
    init_mock(Some(reporter.clone()));

    let dut_thread = thread::spawn(closure!(clone reporter, ||{
        init_mock(Some(reporter.clone()));
        thread::sleep(Duration::from_millis(200));
        loop {
            // TODO open bug for RegValue not being comparable
            if unsafe{ GPIO.r#in().read().get_raw() } == gpio::In::new(0xC0FFEE).get_raw() {
                eprintln!("Read 0xC0FFEE from UART[0].reg32bitraw register");
                break;
            }
                thread::sleep(Duration::from_millis(10));
        }

    }));
    let _ =
        regmock_rs::wait_until_polled(GPIO.r#in().addr(), 20, Some(Duration::from_millis(1000)));
    regmock_rs::silent(|| unsafe {
        // This is a false positive. The trait defined in the module is used.
        #[allow(unused_imports)]
        use pac::tracing::insanely_unsafe;
        GPIO.r#in().write_read_only(gpio::In::new(0xC0FFEE))
    })?;

    dut_thread
        .join()
        .map_err(|_| "Was not able to join DUT thread.".to_owned())
}

#[test]
fn test_wait_until_polling_fail() -> () {
    let reporter = Arc::new(Mutex::new(Regmock::default()));
    init_mock(Some(reporter.clone()));

    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let dut_thread = thread::spawn(closure!(
            clone reporter, clone pair, ||{
        eprintln!("THREAD[1]: Waiting for barrier");
        let (lock, cvar) = &*pair;
        init_mock(Some(reporter.clone()));
        let _guard = cvar.wait_while(lock.lock().unwrap(), |timeout| {
            eprintln!("THREAD[1]: Waiting... timeout: {:?}, inverse:{:?}", *timeout, !*timeout);
            !*timeout
        }).unwrap();
        eprintln!("THREAD[1] Finished work.");
    }));

    eprintln!("THREAD[0]: Waiting for barrier");
    let result =
        regmock_rs::wait_until_polled(GPIO.r#in().addr(), 20, Some(Duration::from_millis(1000)));
    eprintln!("THREAD[0]: Reached timeout waiting for dut_thread to poll");
    let (lock, cvar) = &*pair;
    let mut timeout_waiting = lock.lock().unwrap();
    *timeout_waiting = true;
    cvar.notify_one();
    drop(timeout_waiting);

    thread::sleep(Duration::from_secs(2));
    cvar.notify_one();
    if let Ok(()) = result {
        panic!("Main thread did not time out, waiting for dut_thread to poll UART[0].r#in()");
    }

    dut_thread
        .join()
        .map_err(|_| "Was not able to join DUT thread.".to_owned())
        .unwrap();
}
