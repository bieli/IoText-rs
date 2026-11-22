//! Fuzz the builder with structured input.
//!
//! Oracle: the builder never panics, and whatever it accepts the parser
//! accepts too, producing an identical row (builder and parser enforce the
//! same rules).
#![no_main]

use arbitrary::Arbitrary;
use iotext_rs::builder::IoTextBuilder;
use iotext_rs::{IoTextDataRow, MetricValueType};
use libfuzzer_sys::fuzz_target;
use rust_decimal::Decimal;

#[derive(Arbitrary, Debug)]
enum Value {
    Int(i64),
    Bool(bool),
    Dec(i64, u8),
    Text(String),
}

#[derive(Arbitrary, Debug)]
struct Input {
    timestamp: u64,
    device_id: String,
    metrics: Vec<(String, Value)>,
    add_crc: bool,
}

fuzz_target!(|input: Input| {
    let mut builder = IoTextBuilder::new()
        .timestamp(input.timestamp)
        .device_id(&input.device_id);
    for (name, value) in &input.metrics {
        let value: MetricValueType = match value {
            Value::Int(v) => (*v).into(),
            Value::Bool(v) => (*v).into(),
            Value::Dec(mantissa, scale) => match Decimal::try_new(*mantissa, u32::from(*scale)) {
                Ok(d) => d.into(),
                Err(_) => return,
            },
            Value::Text(v) => v.as_str().into(),
        };
        builder = builder.add_metric(name, value);
    }

    let Ok(row) = builder.build(input.add_crc) else {
        return;
    };
    let msg = row.to_string();
    let parsed: IoTextDataRow = msg
        .parse()
        .unwrap_or_else(|e| panic!("builder output rejected by parser: {e}\nmsg: {msg:?}"));
    assert_eq!(parsed, row, "msg: {msg:?}");
});
