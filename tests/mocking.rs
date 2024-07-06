use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::usize;

use pac::{gpio, RegisterValue, GPIO};
use regmock_rs::utils::access_gen::{read_value, write_value};
use regmock_rs::utils::{RegisterAccessBuilder, RegisterAccessType, RegisterMap, Regmock};
use regmock_rs::{given, require_seq};
use test_pac as pac;

mod common;
use common::init_mock;

#[test]
fn default_rw_mock() {
    init_mock(None);

    unsafe {
        GPIO.we().init(|r| r.set_raw(0xff));
        assert_eq!(GPIO.we().read().get_raw(), 0xff);

        GPIO.we().init(|r| r.set_raw(0xff));
        assert_eq!(GPIO.we().read().get_raw(), 0xff);
    }
}

#[test]
fn custom_read_fn() {
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

    let logs = regmock_rs::logs();
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
fn custom_write_fn() {
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

    let logs = regmock_rs::logs();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let last_read = RegisterAccessBuilder::default()
        .ty(RegisterAccessType::READ)
        .after(10u64) // the default value of the Ctrl register
        .build()
        .unwrap();
    assert_eq!(logs.len_full(), 18);
    assert_eq!(&last_read, logs.iter().last().unwrap());
}

#[test]
fn custom_advanced_write_fn() {
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

    let logs = regmock_rs::logs();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let r = read_value(GPIO.r#in().addr(), 0);
    let w = write_value(GPIO.out().addr(), 0x200);
    let r2 = RegisterAccessBuilder::default()
        .addr(GPIO.r#in().addr())
        .ty(RegisterAccessType::READ)
        .after(0xC0FFEEu64)
        .build()
        .unwrap();
    assert_eq!(logs.len_full(), 3);
    given!(logs.iter_full(), require_seq!(vec![&r, &w, &r2]));
    assert!(itertools::equal(&vec![r, w, r2], logs.iter()));
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
    let logs = regmock_rs::logs();

    assert_eq!(logs.len_full(), 2);
    assert_eq!(*local_value.lock().unwrap(), 1);
    assert!(other_value.lock().unwrap().eq(&0.5));
}
