//! NOTE: not under feature flagged serde module to ensure tests are run with
//! `cargo test`

use crate::{tests::mock_from_str, *};

fn mock_number() -> PaddedNumber<1, 4> {
    mock_from_str("0123")
}

fn mock_json_str() -> &'static str {
    "\"0123\""
}

#[test]
fn str_deserialization() {
    let deserialized_number = serde_json::from_str(mock_json_str()).unwrap();
    assert_eq!(mock_number(), deserialized_number);
}

#[test]
fn str_serialization() {
    let actual_json = serde_json::to_string(&mock_number()).unwrap();
    assert_eq!(mock_json_str(), actual_json);
}
