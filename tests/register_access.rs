use regmock_rs::utils::{RegisterAccess, RegisterAccessBuilder, RegisterAccessType};

// Some simple tests regarding the JSON deserialization.

static READ_ACCESS: &str =
    r#" { "type": "read", "addr": 1234, "len": 4, "before": 1, "after": 1 } "#;
static WRITE_ACCESS: &str =
    r#" { "type": "write", "addr": 1234, "len": 4, "before": 1, "after": 1 } "#;

#[test]
fn test_from_json_str() {
    let _ra: RegisterAccess = serde_json::from_str(READ_ACCESS).unwrap();
    let _wa: RegisterAccess = serde_json::from_str(WRITE_ACCESS).unwrap();
}

#[test]
fn test_incomplete_data_test() {
    let ra: RegisterAccess = serde_json::from_str(r#"{"type":"r"}"#).unwrap();
    let ra_cmp = RegisterAccess {
        ty: Some(RegisterAccessType::READ),
        ..Default::default()
    };
    assert_eq!(ra_cmp, ra);
}

#[test]
fn test_get_sequence() {
    let _ra: Vec<RegisterAccess> = serde_json::from_str(
        r#"[
        { "type": "write", "addr": 1234, "len": 4, "before": 1, "after": 1 },
        { "type": "w", "addr": 1234, "len": 4, "before": 1, "after": 1 },
        { "type": "read", "addr": 1234, "len": 4, "before": 1, "after": 1 },
        { "type": "r", "addr": 1234, "len": 4, "before": 1, "after": 1 },
        { "type": "write", "len": 4, "before": 1, "after": 1 },
        { "type": "write", "addr": 1234, "before": 1, "after": 1 },
        { "type": "write", "addr": 1234, "len": 4,  "after": 1 },
        { "type": "write", "addr": 1234, "len": 4, "before": 1  }
        ]
        "#,
    )
    .unwrap();
}

#[test]
fn debug_formatting() {
    let w = RegisterAccessBuilder::default()
        .ty(RegisterAccessType::WRITE)
        .addr(0xDEADC0DEusize)
        .len(8usize)
        .before(0xC0FFEEu64)
        .after(0xDEAD10CCu64)
        .build()
        .unwrap();

    println!("{:?}", w);
    println!("{:x?}", w);
    println!("{:X?}", w);
}

macro_rules! symmetric_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
        assert_eq!($right, $left);
    };
}

macro_rules! symmetric_assert_ne {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right);
        assert_ne!($right, $left);
    };
}

#[test]
fn partial_equality() {
    let access = RegisterAccess::new(RegisterAccessType::READ, 0x1234, 4, 0, 1);
    let mut builder = RegisterAccessBuilder::default();
    builder
        .ty(RegisterAccessType::READ)
        .addr(0x1234usize)
        .len(4usize)
        .before(0u64)
        .after(1u64);

    symmetric_assert_eq!(access, access);

    symmetric_assert_eq!(
        access,
        RegisterAccessBuilder::default()
            .ty(RegisterAccessType::READ)
            .build()
            .unwrap()
    );
    symmetric_assert_ne!(
        access,
        builder
            .clone()
            .ty(RegisterAccessType::WRITE)
            .build()
            .unwrap()
    );

    symmetric_assert_eq!(
        access,
        RegisterAccessBuilder::default()
            .addr(0x1234usize)
            .build()
            .unwrap()
    );
    symmetric_assert_ne!(access, builder.clone().addr(0x4321usize).build().unwrap());

    symmetric_assert_eq!(
        access,
        RegisterAccessBuilder::default()
            .len(4usize)
            .build()
            .unwrap()
    );
    symmetric_assert_ne!(access, builder.clone().len(8usize).build().unwrap());

    symmetric_assert_eq!(
        access,
        RegisterAccessBuilder::default()
            .before(0u64)
            .build()
            .unwrap()
    );
    symmetric_assert_ne!(access, builder.clone().before(1u64).build().unwrap());

    symmetric_assert_eq!(
        access,
        RegisterAccessBuilder::default()
            .after(1u64)
            .build()
            .unwrap()
    );
    symmetric_assert_ne!(access, builder.clone().after(0u64).build().unwrap());
}
