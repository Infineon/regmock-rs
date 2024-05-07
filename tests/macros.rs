use regmock_rs::given;
use regmock_rs::require_reg;
use regmock_rs::utils::*;

#[test]
fn read_last_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":456 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, read_last));
}

#[test]
#[should_panic]
fn read_last_fail_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, read_last));
}

#[test]
fn not_written_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":456 },
        {"type":"r","addr":456 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, not_written));
}

#[test]
#[should_panic]
fn not_written_fail_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":123 },
        {"type":"w","addr":456 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, not_written));
}

#[test]
fn all_written_before_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":123 },
        {"type":"w","addr":123 },
        {"type":"w","addr":789 },
        {"type":"w","addr":456 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, all_written_before(456)));
}

#[test]
#[should_panic]
fn all_written_before_fail_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"w","addr":123 },
        {"type":"w","addr":123 },
        {"type":"w","addr":789 },
        {"type":"w","addr":456 },
        {"type":"w","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, all_written_before(456)));
}

#[test]
fn all_written_before_no_target_write_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"r","addr":123 },
        {"type":"r","addr":123 },
        {"type":"r","addr":123 },
        {"type":"w","addr":789 },
        {"type":"w","addr":456 },
        {"type":"r","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, all_written_before(456)));
}
#[test]
fn all_written_before_no_other_write_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"w","addr":123 },
        {"type":"w","addr":123 },
        {"type":"r","addr":123 },
        {"type":"w","addr":789 },
        {"type":"r","addr":456 },
        {"type":"r","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(log.iter(), require_reg!(123, all_written_before(456)));
}
#[test]
fn written_sequence_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"w","addr":123,"after": 2730 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"w","addr":123,"after": 2731 },
        {"type":"r","addr":456,"after": 1024 },
        {"type":"w","addr":123,"after": 2732 },
        {"type":"r","addr":123 },
        {"type":"w","addr":123,"after": 2733 },
        {"type":"w","addr":123,"after": 2734 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"r","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(
        log.iter(),
        require_reg!(
            123,
            written_sequence(vec![0xAAAu32, 0xAABu32, 0xAACu32, 0xAADu32, 0xAAEu32])
        )
    );
}
#[test]
#[should_panic]
fn written_sequence_fail_test() {
    let log: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        {"type":"w","addr":123,"after": 2730 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"w","addr":123,"after": 2731 },
        {"type":"r","addr":456,"after": 1024 },
        {"type":"w","addr":123,"after": 2732 },
        {"type":"w","addr":123,"after": 12648430 },
        {"type":"r","addr":123 },
        {"type":"w","addr":123,"after": 2733 },
        {"type":"w","addr":123,"after": 2734 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"w","addr":456,"after": 1024 },
        {"type":"r","addr":123 }
        ]"#,
    )
    .unwrap();
    given!(
        log.iter(),
        require_reg!(
            123,
            written_sequence(vec![0xAAAu32, 0xAABu32, 0xAACu32, 0xAADu32, 0xAAEu32])
        )
    );
}
