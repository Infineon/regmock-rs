use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std::usize;

use closure::closure;
use pac::common::RegisterValue;
use pac::uart::reg16bitenum::{Bitfield9BitsEnum, Boolenum};
use pac::uart::Reg16BitEnum;

use generic_pac as pac;
#[allow(unused_imports)]
use regmock_rs::utils::access_gen::{read, read_value, write, write_value};
use regmock_rs::utils::{
    RegisterAccess, RegisterAccessBuilder, RegisterAccessType, RegisterMap, Regmock,
};
use regmock_rs::{given, require_seq};
// use slc26v19c_pac_tracing as pac;

mod common;
use common::init_mock;

#[test]
fn test_read() {
    init_mock(None);
    let _ = unsafe { pac::UART[0].reg32bitraw().read() };
    let expected_access = read_value(pac::UART[0].reg32bitraw().addr(), 0);
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.len_full(), 1);
    given!(logs.iter(), require_seq!(vec![&expected_access]));
}

#[test]
fn test_read_write_read() {
    init_mock(None);

    let _input = unsafe { pac::UART[0].reg16bitenum().read() };
    let value = Reg16BitEnum::default().boolenum().set(Boolenum::BOOL_1);
    unsafe { pac::UART[0].reg16bitenum().write(value) };
    let _ = unsafe { pac::UART[0].reg16bitenum().read() };

    let r1 = read_value(pac::UART[0].reg16bitenum().addr(), 0);
    let w1 = write_value(pac::UART[0].reg16bitenum().addr(), 0b1000000000);
    let r2 = read_value(pac::UART[0].reg16bitenum().addr(), 0b1000000000);
    let log = regmock_rs::logs().unwrap();
    assert_eq!(log.len_full(), 3);
    given!(log.iter(), require_seq!(vec![&r1, &w1, &r2]));
    assert!(itertools::equal(&vec![r1, w1, r2], log.iter()));
}

#[test]
fn test_init() {
    init_mock(None);

    unsafe {
        pac::UART[0]
            .reg16bitenum()
            .init(|r| r.bitfield9bitsenum().set(Bitfield9BitsEnum::VAL_256))
    };
    let _ = unsafe { pac::UART[0].reg16bitenum().read() };
    let w = write_value(pac::UART[0].reg16bitenum().addr(), 0x100);

    let r = read_value(pac::UART[0].reg16bitenum().addr(), 0x100);
    let logs = regmock_rs::logs().unwrap();
    given!(logs.iter(), require_seq!(vec![&w, &r]));
    assert!(itertools::equal(&vec![w, r], logs.iter()));
}

#[test]
fn test_write() {
    init_mock(None);

    let value = pac::uart::Reg16BitEnum::default()
        .boolenum()
        .set(Boolenum::BOOL_1);
    unsafe { pac::UART[0].reg16bitenum().write(value) };

    let w = RegisterAccess::new(
        RegisterAccessType::WRITE,
        pac::UART[0].reg16bitenum().addr(),
        2,
        0,
        0x200,
    );
    let logs = regmock_rs::logs().unwrap();
    given!(logs.iter(), require_seq!(vec![&w]));
    assert!(itertools::equal(logs.iter(), &vec![w]));
}

#[test]
fn test_check_access_type_only() {
    init_mock(None);

    let default = Default::default();
    let _ = unsafe { pac::UART[0].reg8bitraw().read() };
    unsafe { pac::UART[0].reg8bitraw().write(default) };

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

    let default = Default::default();

    for _ in 1..10 {
        let _ = unsafe { pac::UART[0].reg8bitraw().read() };
        unsafe { pac::UART[0].reg8bitraw().write(default) };
    }

    let w = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::WRITE))
        .addr(Some(pac::UART[0].reg8bitraw().addr()))
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

#[test]
fn test_access_count() {
    init_mock(None);
    let _ = unsafe { pac::UART[0].reg8bitraw().read() };
    assert_eq!(regmock_rs::logs().unwrap().len_full(), 1);
}

#[test]
fn test_access_count_rle() {
    init_mock(None);
    for _ in 0..10 {
        let _ = unsafe { pac::UART[0].reg8bitraw().read() };
    }
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.iter().count(), 1);
    assert_eq!(logs.len_full(), 10);
}

#[test]
fn test_custom_read_fn() {
    let mut reporter = Regmock::default();
    let cool_fn = |_: &mut RegisterMap, value: u64| -> u64 {
        static mut COUNTER: u64 = 1;
        let mut ret = 0xC0FFEE;
        // operation should be safe as it is behind the lock of the LogEffectMocker anyway
        unsafe {
            if COUNTER < 5 {
                ret = value;
            }
        }
        println!(
            "Cool function was called. Counter={:?} value=0x{:08X} ret=0x{:08X}",
            unsafe { COUNTER },
            value,
            ret
        );
        unsafe { COUNTER += 1 };
        ret
    };
    reporter
        .read_fn
        .insert(pac::UART[0].reg32bitraw().addr(), Box::new(cool_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));

    let value = pac::uart::Reg32BitRaw::default().set_raw(0x123);
    // write cool value to register. This value should only be returned after the fifth read call to the register
    unsafe {
        pac::UART[0].reg32bitraw().write(value);
    }
    for _ in 0..5 {
        let _ = unsafe { pac::UART[0].reg32bitraw().read() };
    }

    let logs = regmock_rs::logs().unwrap();
    logs.iter_full()
        .for_each(|element| println!("{:X?}", element));

    let w = write_value(pac::UART[0].reg32bitraw().addr(), 0x123);
    let r = read_value(pac::UART[0].reg32bitraw().addr(), 0x123);
    let r_final = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::READ))
        .addr(Some(pac::UART[0].reg32bitraw().addr()))
        .after(Some(0xC0FFEE))
        .build()
        .unwrap();
    given!(
        logs.iter_full(),
        require_seq!(vec![&w, &r, &r, &r, &r, &r_final])
    );
    // assert!(itertools::equal(logs.iter(), &vec![w]));
    assert_eq!(logs.len_full(), 6);
}

#[test]
fn test_custom_write_fn() {
    let mut reporter = Regmock::default();
    // This function causes writes that are always one bigger than the value
    // that was actually written into the register.
    let cool_fn = |_: &mut HashMap<usize, u64>, _, val: u64| -> u64 { val + 1 };
    reporter
        .write_fn
        .insert(pac::UART[0].reg32bitraw().addr(), Box::new(cool_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));

    let value = pac::uart::Reg32BitRaw::default().set_raw(0x123);
    unsafe {
        pac::UART[0].reg32bitraw().write(value);
    }
    let _ = unsafe { pac::UART[0].reg32bitraw().read() };

    let logs = regmock_rs::logs().unwrap();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let r = RegisterAccessBuilder::default()
        .ty(Some(RegisterAccessType::READ))
        .after(Some(0x124)) // the default value of the Ctrl register
        .build()
        .unwrap();
    assert_eq!(logs.len_full(), 2);
    assert_eq!(&r, logs.iter().last().unwrap());
}

#[test]
fn test_custom_advanced_write_fn() {
    let mut reporter = Regmock::default();
    // This function causes a value to be written to the
    // UART[0]::reg32bitraw() register, when the register that this function is
    // registered with is read from. Not sure how useful this is but it works.
    let advanced_fn = |register_mocks: &mut HashMap<usize, u64>, _, val: u64| -> u64 {
        register_mocks.insert(pac::UART[0].reg32bitraw().addr(), 0xC0FFEE);
        println!("Added 0xC0FFEE to UART[0].reg32bitraw register");
        val
    };
    reporter
        .write_fn
        .insert(pac::UART[0].reg16bitenum().addr(), Box::new(advanced_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));

    let value = pac::uart::Reg16BitEnum::default()
        .boolenum()
        .set(Boolenum::BOOL_1);
    let _ = unsafe { pac::UART[0].reg32bitraw().read() };
    unsafe {
        pac::UART[0].reg16bitenum().write(value);
    }
    let _ = unsafe { pac::UART[0].reg32bitraw().read() };

    let logs = regmock_rs::logs().unwrap();
    logs.iter().for_each(|element| println!("{:X?}", element));
    let r = read_value(pac::UART[0].reg32bitraw().addr(), 0);
    let w = write_value(pac::UART[0].reg16bitenum().addr(), 0x200);
    let r2 = RegisterAccessBuilder::default()
        .addr(Some(pac::UART[0].reg32bitraw().addr()))
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
    regmock_rs::silent(|| unsafe { pac::UART[0].reg8bitraw().read() }).unwrap();
    let _ = unsafe { pac::UART[0].reg8bitraw().read() };
    let logs = regmock_rs::logs().unwrap();
    assert_eq!(logs.len_full(), 1);
}

#[test]
fn test_boxed_callback_fnmut() {
    let mut reporter = Regmock::default();
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
    reporter
        .read_fn
        .insert(pac::UART[0].reg8bitraw().addr(), Box::new(some_fn));

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
        .insert(pac::UART[0].reg16bitraw().addr(), Box::new(other_fn));
    init_mock(Some(Arc::new(Mutex::new(reporter))));
    unsafe {
        let _ = pac::UART[0].reg16bitraw().read();
        let _ = pac::UART[0].reg8bitraw().read();
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
        loop{
            if unsafe{ pac::UART[0].reg32bitraw().read() } == pac::uart::Reg32BitRaw::new(0xC0FFEE){
                eprintln!("Read 0xC0FFEE from UART[0].reg32bitraw register");
                break;
            }else{
                thread::sleep(Duration::from_millis(10));
            }
        }

    }));
    let _ = regmock_rs::wait_until_polled(
        pac::UART[0].reg32bitraw().addr(),
        20,
        Some(Duration::from_millis(1000)),
    );
    regmock_rs::silent(|| unsafe {
        #[allow(unused_imports)]
        // This is a false positive. The trait defined in the module is used.
        use pac::tracing::insanely_unsafe;
        pac::UART[0]
            .reg32bitraw()
            .write(pac::uart::Reg32BitRaw::new(0xC0FFEE))
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
    let result = regmock_rs::wait_until_polled(
        pac::UART[0].reg32bitraw().addr(),
        20,
        Some(Duration::from_millis(1000)),
    );
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
