//! Property-based tests for the IoText parser / serializer.
//!
//! Invariants checked here:
//! 1. Parsing never panics, whatever the input.
//! 2. Any valid row survives `to_string()` -> `parse()` unchanged (with and without CRC).
//! 3. A CRC-protected message with any single byte changed is never accepted.

use iotext_rs::{IoTextDataRow, IoTextError, Item, ItemTypeEnum, MetricDataItem, MetricValueType};
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;
use rust_decimal::Decimal;

// ---------- generators ----------

fn identifier() -> impl Strategy<Value = String> {
    "[A-Za-z0-9_.-]{1,32}"
}

fn metric_value() -> impl Strategy<Value = MetricValueType> {
    prop_oneof![
        any::<i64>().prop_map(MetricValueType::IntegerItemType),
        any::<bool>().prop_map(MetricValueType::BoolItemType),
        (any::<i64>(), 0u32..=18).prop_map(|(mantissa, scale)| MetricValueType::DecimalItemType(
            Decimal::new(mantissa, scale)
        )),
        // Any text except the item separator ',' (other separators are allowed in values).
        "[^,]{0,24}".prop_map(MetricValueType::TextItemType),
    ]
}

fn metric() -> impl Strategy<Value = MetricDataItem> {
    (identifier(), metric_value()).prop_map(|(name, value)| MetricDataItem { name, value })
}

fn valid_row() -> impl Strategy<Value = IoTextDataRow> {
    (
        any::<u64>(),
        identifier(),
        proptest::collection::vec(metric(), 0..12),
    )
        .prop_map(|(ts, device_id, metrics)| IoTextDataRow {
            timestamp: Item {
                value: ItemTypeEnum::TimeUnixMilis(ts),
            },
            device_id: Item {
                value: ItemTypeEnum::DeviceId(device_id),
            },
            metrics: if metrics.is_empty() {
                None
            } else {
                Some(metrics)
            },
            crc16: None,
        })
}

/// Strings built from the protocol alphabet, so the fuzzer reaches deep parser paths.
fn protocol_like() -> impl Strategy<Value = String> {
    "[tdmcibx|,=:0-9A-Fa-f._ -]{0,80}"
}

// ---------- properties ----------

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2_000,
        // Shrunk failing cases are saved to tests/proptests/regressions/ and replayed first.
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn parse_never_panics_on_arbitrary_input(s in any::<String>()) {
        let _ = s.parse::<IoTextDataRow>();
    }

    #[test]
    fn parse_never_panics_on_protocol_like_input(s in protocol_like()) {
        let _ = s.parse::<IoTextDataRow>();
    }

    #[test]
    fn parse_never_panics_on_mutated_valid_message(
        row in valid_row(),
        with_crc in any::<bool>(),
        cut in any::<prop::sample::Index>(),
        insert in prop::sample::select(vec![',', '|', '=', ':', 'm', 'c', 'x', ' ']),
    ) {
        let msg = row.to_iotext_string(with_crc);
        let pos = cut.index(msg.len() + 1);
        if msg.is_char_boundary(pos) {
            let mut mutated = msg.clone();
            mutated.insert(pos, insert);
            let _ = mutated.parse::<IoTextDataRow>();
            let _ = msg[..pos].parse::<IoTextDataRow>();
        }
    }

    #[test]
    fn roundtrip_without_crc(row in valid_row()) {
        let msg = row.to_string();
        let parsed: IoTextDataRow = msg.parse().map_err(|e| TestCaseError::fail(format!("{e}: {msg}")))?;
        prop_assert_eq!(&parsed, &row);
        prop_assert_eq!(parsed.to_string(), msg);
    }

    #[test]
    fn roundtrip_with_crc(row in valid_row()) {
        let msg = row.to_iotext_string(true);
        let parsed: IoTextDataRow = msg.parse().map_err(|e| TestCaseError::fail(format!("{e}: {msg}")))?;
        prop_assert!(parsed.crc16.is_some());
        prop_assert_eq!(&parsed.timestamp, &row.timestamp);
        prop_assert_eq!(&parsed.device_id, &row.device_id);
        prop_assert_eq!(&parsed.metrics, &row.metrics);
        prop_assert_eq!(parsed.to_string(), msg);
    }

    #[test]
    fn crc_detects_any_single_byte_change(
        row in valid_row(),
        idx in any::<prop::sample::Index>(),
        replacement in 0x21u8..0x7f,
    ) {
        let msg = row.to_iotext_string(true);
        // Corrupt only the body (CRC-16 detects every burst error of <= 16 bits).
        let body_len = msg.rfind(",c|").unwrap();
        // Pick a single-byte (ASCII) position, so the replacement keeps the string valid UTF-8.
        let ascii_positions: Vec<usize> =
            (0..body_len).filter(|&i| msg.as_bytes()[i].is_ascii()).collect();
        let pos = ascii_positions[idx.index(ascii_positions.len())];
        let replacement = if msg.as_bytes()[pos] == replacement { replacement ^ 1 } else { replacement };

        let mut corrupted = msg[..pos].to_string();
        corrupted.push(replacement as char);
        corrupted.push_str(&msg[pos + 1..]);

        prop_assert!(corrupted.parse::<IoTextDataRow>().is_err(), "accepted corrupted: {}", corrupted);
    }

    #[test]
    fn wrong_crc_is_rejected_with_crc_mismatch(row in valid_row(), bogus in 0u16..=u16::MAX) {
        let good = row.to_iotext_string(true);
        let body = &good[..good.rfind(",c|").unwrap()];
        let bogus = format!("{bogus:04X}");
        prop_assume!(!good.ends_with(&bogus));
        let msg = format!("{body},c|{bogus}");
        let is_mismatch = matches!(msg.parse::<IoTextDataRow>(), Err(IoTextError::CrcMismatch { .. }));
        prop_assert!(is_mismatch);
    }
}
