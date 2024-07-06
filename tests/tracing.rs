use pac::{gpio, RegisterValue, GPIO};
use regmock_rs::utils::{RegisterAccess, RegisterAccessType};
use regmock_rs::{given, require_seq, silent};
use test_pac as pac;

mod common;
use common::init_mock;

#[test]
fn read_traced_correctly() {
    init_mock(None);

    let read = unsafe { GPIO.we().read() };
    assert_eq!(read.get_raw(), 0);

    let logs = regmock_rs::logs();
    assert_eq!(logs.len_full(), 1);

    let expected_access = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    assert_eq!(logs.log, vec![(expected_access.clone(), 1)]);
    given!(logs.iter(), require_seq!(vec![&expected_access])); // TODO?
}

#[test]
fn init_traced_correctly() {
    init_mock(None);

    unsafe { GPIO.we().init(|r| r.gpio8().set(gpio::we::Gpio8::OUT)) };
    let read = unsafe { GPIO.we().read() };
    assert_eq!(read.get_raw(), 0x100);

    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x100);
    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0x100, 0x100);
    let logs = regmock_rs::logs();

    assert_eq!(logs.log, vec![(w.clone(), 1), (r.clone(), 1)]);

    given!(logs.iter_full(), require_seq!(vec![&w, &r])); // TODO ?
    assert!(itertools::equal(&vec![w, r], logs.iter())); // TODO ?
}

#[test]
fn write_traced_correctly() {
    init_mock(None);

    unsafe {
        let current = GPIO.we().read();
        let new = current.gpio4().set(gpio::we::Gpio4::OUT);
        GPIO.we().write(new);
    };
    assert_eq!(silent(|| unsafe { GPIO.we().read() }).get_raw(), 0x10);

    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x10);

    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r.clone(), 1), (w.clone(), 1)]);

    given!(logs.iter_full(), require_seq!(vec![&r, &w])); // TODO ?
}

#[test]
fn read_write_read_traced_correctly() {
    init_mock(None);

    unsafe {
        let _ = GPIO.we().read();
        let output = gpio::We::default().gpio0().set(gpio::we::Gpio0::OUT);
        GPIO.we().write(output);
        let _ = GPIO.we().read();
    };

    let r1 = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let w1 = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x1);
    let r2 = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0x1, 0x1);
    let logs = regmock_rs::logs();
    assert_eq!(
        logs.log,
        vec![(r1.clone(), 1), (w1.clone(), 1), (r2.clone(), 1)]
    );

    given!(logs.iter_full(), require_seq!(vec![&r1, &w1, &r2])); // TODO ?
}

#[test]
fn modify_traced_correctly() {
    init_mock(None);

    unsafe { GPIO.we().modify(|r| r.gpio4().set(gpio::we::Gpio4::OUT)) };

    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x10);
    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r.clone(), 1), (w.clone(), 1)]);

    given!(logs.iter_full(), require_seq!(vec![&r, &w])); // TODO ?
}

#[test]
fn reads_rle_encoded() {
    init_mock(None);
    // check write does not get combined with read
    for _ in 0..5 {
        let _ = unsafe { GPIO.we().read() };
    }

    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r.clone(), 5)]);

    assert_eq!(logs.iter().count(), 1); // TODO ?
    assert_eq!(logs.len_full(), 5);
}

#[test]
fn read_and_write_not_combined_in_rle() {
    init_mock(None);
    // check write does not get combined with read
    for _ in 0..10 {
        let _ = unsafe { GPIO.we().read() };
    }
    unsafe { GPIO.we().init(|r| r.gpio0().set(gpio::we::Gpio0::OUT)) };

    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let w = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.we().addr(), 4, 0, 0x1);
    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r.clone(), 10), (w.clone(), 1)]);

    assert_eq!(logs.iter().count(), 2); // TODO ?
    assert_eq!(logs.len_full(), 11);
}

// TODO modify

#[test]
fn reads_of_different_register_not_combined_in_rle() {
    init_mock(None);
    // check that different register reads don't get combined
    for _ in 0..3 {
        let _ = unsafe { GPIO.out().read() };
    }
    for _ in 0..4 {
        let _ = unsafe { GPIO.we().read() };
    }

    let r = RegisterAccess::new(RegisterAccessType::READ, GPIO.out().addr(), 4, 0, 0);
    let w = RegisterAccess::new(RegisterAccessType::READ, GPIO.we().addr(), 4, 0, 0);
    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r.clone(), 3), (w.clone(), 4)]);

    assert_eq!(logs.iter().count(), 2); // TODO ?
    assert_eq!(logs.len_full(), 7);
}

#[test]
fn writes_not_combined_in_rle() {
    init_mock(None);
    // check that register writes with different values don't get combined
    for i in 0..3 {
        println!("i: {}", i);
        unsafe { GPIO.out().init(|r| r.gpio0().set(true)) };
    }

    let state0 = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.out().addr(), 4, 0, 1);
    let state1 = RegisterAccess::new(RegisterAccessType::WRITE, GPIO.out().addr(), 4, 1, 1);
    let logs = regmock_rs::logs();

    assert_eq!(
        logs.log,
        vec![
            (state0.clone(), 1),
            (state1.clone(), 1),
            (state1.clone(), 1)
        ]
    );

    assert_eq!(logs.iter().count(), 3); // TODO ?
    assert_eq!(logs.len_full(), 3);
}

#[test]
fn reads_of_different_values_not_combined_in_rle() {
    let regmock = init_mock(None);
    // check that register reads with different values don't get combined
    {
        let mut regmock = regmock.lock().unwrap();
        regmock.read_fn.insert(
            GPIO.out().addr(),
            Box::new(|state, val| -> u64 {
                *state.get_mut(&GPIO.out().addr()).unwrap() = val + 1;
                val + 1
            }),
        );
    }
    for _ in 0..4 {
        let _ = unsafe { GPIO.out().read() };
    }

    let r0 = RegisterAccess::new(RegisterAccessType::READ, GPIO.out().addr(), 4, 0, 1);
    let r1 = RegisterAccess::new(RegisterAccessType::READ, GPIO.out().addr(), 4, 1, 2);
    let r2 = RegisterAccess::new(RegisterAccessType::READ, GPIO.out().addr(), 4, 2, 3);
    let r3 = RegisterAccess::new(RegisterAccessType::READ, GPIO.out().addr(), 4, 3, 4);
    let logs = regmock_rs::logs();
    assert_eq!(logs.log, vec![(r0, 1), (r1, 1), (r2, 1), (r3, 1)]);

    assert_eq!(logs.iter().count(), 4);
    assert_eq!(logs.len_full(), 4);
}

#[test]
fn silent_access_not_traced() {
    init_mock(None);

    silent(|| unsafe { GPIO.r#in().read() });
    let _ = unsafe { GPIO.r#in().read() };

    let logs = regmock_rs::logs();
    assert_eq!(logs.len_full(), 1);
}
