//! `serde` feature tests (run with `cargo test --features serde`).
#![cfg(feature = "serde")]

use iotext_rs::IoTextDataRow;

const BODY: &str =
    "t|3900237526042,d|DEV_NAME_002,m|n=i:-7,m|ok=b:1,m|temp=d:12.070,m|s=t:in_progress";

#[test]
fn test_row_json_roundtrip() {
    let msg = BODY
        .parse::<IoTextDataRow>()
        .unwrap()
        .to_iotext_string(true);
    let row: IoTextDataRow = msg.parse().unwrap();

    let json = serde_json::to_string(&row).unwrap();
    let back: IoTextDataRow = serde_json::from_str(&json).unwrap();

    assert_eq!(back, row);
    assert_eq!(back.to_string(), msg);
}

#[test]
fn test_json_shape_is_stable() {
    let row: IoTextDataRow = "t|1,d|dev,m|temp=d:12.070,m|ok=b:1".parse().unwrap();
    let json = serde_json::to_value(&row).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "timestamp": 1,
            "device_id": "dev",
            "metrics": [
                { "name": "temp", "value": { "DecimalItemType": "12.070" } },
                { "name": "ok", "value": { "BoolItemType": true } }
            ],
            "crc16": null
        })
    );
}
