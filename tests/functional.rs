use regmock_rs::utils::{RegisterAccess, RegisterAccessType};
use serde_json;

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
    let mut ra_cmp = RegisterAccess::default();
    ra_cmp.ty = Some(RegisterAccessType::READ);
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
