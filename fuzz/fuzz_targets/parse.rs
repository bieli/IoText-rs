//! Fuzz the parser with arbitrary bytes.
//!
//! Oracle: the parser never panics, and every accepted message
//! re-serializes to a canonical message that parses back to the same row.
#![no_main]

use iotext_rs::IoTextDataRow;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = core::str::from_utf8(data) else {
        return;
    };
    let Ok(row) = text.parse::<IoTextDataRow>() else {
        return;
    };

    // Anything the parser accepts satisfies the documented row invariants.
    row.validate()
        .unwrap_or_else(|e| panic!("parsed row fails validate(): {e}\ninput: {text:?}"));

    // Canonical form: items reordered to t, d, m..., CRC (if present) recomputed.
    let canonical = row.to_string();
    let reparsed: IoTextDataRow = canonical.parse().unwrap_or_else(|e| {
        panic!("canonical form rejected: {e}\ninput:     {text:?}\ncanonical: {canonical:?}")
    });

    assert_eq!(reparsed.timestamp, row.timestamp, "input: {text:?}");
    assert_eq!(reparsed.device_id, row.device_id, "input: {text:?}");
    assert_eq!(reparsed.metrics, row.metrics, "input: {text:?}");
    // The stored CRC is the *received* one; it differs when items were reordered.
    assert_eq!(
        reparsed.crc16.is_some(),
        row.crc16.is_some(),
        "input: {text:?}"
    );
    assert_eq!(
        reparsed.to_string(),
        canonical,
        "canonical form is not a fixed point"
    );
});
