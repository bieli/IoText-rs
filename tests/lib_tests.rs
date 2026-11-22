//! Parser / serializer tests.

use rust_decimal::prelude::*;

use iotext_rs::validators::{DeviceIdError, MetricDataTypeError, MetricNameError};
use iotext_rs::{IoTextDataRow, IoTextError, ItemKind, MetricKind, MetricValueType, crc16};

#[cfg(test)]
mod lib_tests {
    use super::*;

    fn row(ts: u64, device_id: &str) -> IoTextDataRow {
        IoTextDataRow {
            timestamp: ts,
            device_id: device_id.to_string(),
            ..Default::default()
        }
    }

    fn parse_err(msg: &str) -> IoTextError {
        msg.parse::<IoTextDataRow>()
            .expect_err(&format!("expected error for {msg:?}"))
    }

    // ---------- metric values ----------

    #[test]
    fn test_extract_metric_value_type_integer() {
        let result = MetricValueType::from_type_and_value("i", "42");
        assert_eq!(result, Ok(MetricValueType::IntegerItemType(42)));
    }

    #[test]
    fn test_extract_metric_value_type_decimal() {
        let result = MetricValueType::from_type_and_value("d", "42.123");
        assert_eq!(
            result,
            Ok(MetricValueType::DecimalItemType(
                Decimal::from_str("42.123").unwrap()
            ))
        );
    }

    #[test]
    fn test_extract_metric_value_type_bool_true() {
        let result = MetricValueType::try_from(("b", "1"));
        assert_eq!(result, Ok(MetricValueType::BoolItemType(true)));
    }

    #[test]
    fn test_extract_metric_value_type_bool_false() {
        let result = MetricValueType::try_from(("b", "0"));
        assert_eq!(result, Ok(MetricValueType::BoolItemType(false)));
    }

    #[test]
    fn test_extract_metric_value_type_text() {
        let result = MetricValueType::from_type_and_value("t", "hello");
        assert_eq!(
            result,
            Ok(MetricValueType::TextItemType("hello".to_string()))
        );
    }

    #[test]
    fn test_extract_metric_value_type_invalid_values() {
        assert_eq!(
            MetricValueType::from_type_and_value("i", "4.2"),
            Err(IoTextError::InvalidInteger("4.2".into()))
        );
        assert_eq!(
            MetricValueType::from_type_and_value("b", "true"),
            Err(IoTextError::InvalidBool("true".into()))
        );
        assert_eq!(
            MetricValueType::from_type_and_value("d", "abc"),
            Err(IoTextError::InvalidDecimal("abc".into()))
        );
        assert!(matches!(
            MetricValueType::from_type_and_value("x", "1"),
            Err(IoTextError::MetricDataType(
                MetricDataTypeError::UnknownMetricDataType(..)
            ))
        ));
    }

    // ---------- dump ----------

    #[test]
    fn test_dump_iotext_to_str_without_measurements_without_crc16() {
        let result = row(3900237526042, "device_name_001").to_iotext_string(false);
        assert_eq!(result, "t|3900237526042,d|device_name_001");
    }

    #[test]
    fn test_dump_iotext_to_str_with_measurements_without_crc16() {
        let expected = "t|3900237526044,d|device_name_002,m|val1i=i:123,m|val2d=d:123.45,m|valb1=b:1,m|valb0=b:0,m|valt=t:abc";
        let iotext_data_row: IoTextDataRow = expected.parse().unwrap();

        assert_eq!(iotext_data_row.to_iotext_string(false), expected);
        assert_eq!(iotext_data_row.to_string(), expected);
    }

    #[test]
    fn test_dump_iotext_to_str_without_measurements_with_crc16() {
        // CRC is computed over exactly the bytes before ",c|" (0.5.x wrongly included a trailing ',').
        let result = row(3900237526042, "DEV_NAME_002").to_iotext_string(true);
        assert_eq!(result, "t|3900237526042,d|DEV_NAME_002,c|A4BA");
    }

    #[test]
    fn test_dump_iotext_to_str_with_measurements_with_crc16() {
        let expected = "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C";
        let iotext_data_row: IoTextDataRow = expected.parse().unwrap();

        assert_eq!(iotext_data_row.to_iotext_string(true), expected);
        assert_eq!(iotext_data_row.to_string(), expected);
    }

    // ---------- parse ----------

    #[test]
    fn test_parse_iotext_str_for_timestamp_without_measurements() {
        let result: IoTextDataRow = "t|3900237526044,d|device_name_004".parse().unwrap();
        assert_eq!(result.timestamp, 3900237526044);
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements() {
        let result: IoTextDataRow = "t|3900237526045,d|device_name_005".parse().unwrap();
        assert_eq!(result.device_id, "device_name_005");
        assert_eq!(result.metrics, [] as [iotext_rs::MetricDataItem; 0]);
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements_with_crc16() {
        let result: IoTextDataRow = "t|3900237526045,d|device_name_005,c|6F89".parse().unwrap();
        assert_eq!(result.crc16, Some(0x6F89));
    }

    #[test]
    fn test_parse_accepts_lowercase_crc() {
        assert!(
            "t|3900237526045,d|device_name_005,c|6f89"
                .parse::<IoTextDataRow>()
                .is_ok()
        );
    }

    #[test]
    fn test_parse_metric_order_and_text_with_inner_separators() {
        let result: IoTextDataRow = "d|dev,m|a=t:x|y=z:w,t|1".parse().unwrap();
        assert_eq!(result.to_string(), "t|1,d|dev,m|a=t:x|y=z:w");
    }

    // ---------- parse errors (each of these panicked in 0.5.x or was silently accepted) ----------

    #[test]
    fn test_parse_error_metric_without_value() {
        assert_eq!(
            parse_err("t|1,d|x,m|a"),
            IoTextError::MalformedMetric("m|a".into())
        );
        assert_eq!(
            parse_err("t|1,d|x,m|a=i"),
            IoTextError::MalformedMetric("m|a=i".into())
        );
    }

    #[test]
    fn test_parse_error_invalid_decimal() {
        assert_eq!(
            parse_err("t|1,d|x,m|a=d:abc"),
            IoTextError::InvalidDecimal("abc".into())
        );
    }

    #[test]
    fn test_parse_error_comma_in_text_is_not_truncated_silently() {
        assert_eq!(
            parse_err("t|1,d|x,m|a=t:hello,world"),
            IoTextError::MalformedItem("world".into())
        );
    }

    #[test]
    fn test_parse_error_crc_mismatch() {
        assert_eq!(
            parse_err("t|1,d|x,c|0000"),
            IoTextError::CrcMismatch {
                received: 0x0000,
                computed: crc16(b"t|1,d|x"),
            }
        );
    }

    #[test]
    fn test_parse_error_crc_not_last() {
        let crc = crc16(b"t|1,d|x");
        assert_eq!(
            parse_err(&format!("t|1,d|x,c|{crc:04X},m|a=i:1")),
            IoTextError::CrcNotLast
        );
    }

    #[test]
    fn test_parse_error_structure() {
        assert_eq!(parse_err(""), IoTextError::EmptyMessage);
        assert_eq!(
            parse_err("t|1,d|x,"),
            IoTextError::MalformedItem(String::new())
        );
        assert_eq!(
            parse_err("garbage"),
            IoTextError::MalformedItem("garbage".into())
        );
        assert_eq!(
            parse_err("t|1,d|x,h|1"),
            IoTextError::UnknownItemType("h".into())
        );
        assert_eq!(parse_err("d|x"), IoTextError::MissingTimestamp);
        assert_eq!(parse_err("t|1"), IoTextError::MissingDeviceId);
        assert_eq!(
            parse_err("t|1,t|2,d|x"),
            IoTextError::DuplicatedItem("t|2".into())
        );
        assert_eq!(
            parse_err("t|1,d|x,d|y"),
            IoTextError::DuplicatedItem("d|y".into())
        );
    }

    #[test]
    fn test_parse_error_from_validators() {
        assert!(matches!(parse_err("t|abc,d|x"), IoTextError::Timestamp(_)));
        assert!(matches!(
            parse_err("t|1,d|a|b"),
            IoTextError::DeviceId(DeviceIdError::DeviceIdContainsSpecialChar(_))
        ));
        assert!(matches!(
            parse_err("t|1,d|x,m|=i:1"),
            IoTextError::MetricName(MetricNameError::MetricNameIsEmpty(_))
        ));
        assert!(matches!(parse_err("t|1,d|x,c|XYZ1"), IoTextError::Crc(_)));
    }

    // ---------- deprecated trait still works ----------

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_trait_api_returns_result() {
        use iotext_rs::IoTextData;
        let data_obj = IoTextDataRow::default();
        let parsed = data_obj.parse_iotext_str("t|1,d|x,m|a=i:5").unwrap();
        assert_eq!(
            data_obj.dump_iotext_to_str(&parsed, false),
            "t|1,d|x,m|a=i:5"
        );
        assert!(data_obj.parse_iotext_str("t|1,d|x,m|a").is_err());
    }

    // ---------- protocol code enums ----------

    #[test]
    fn test_metric_kind_codes_roundtrip() {
        for kind in MetricKind::ALL {
            assert_eq!(MetricKind::from_code(kind.code()), Some(kind));
            assert_eq!(kind.code().parse::<MetricKind>(), Ok(kind));
        }
        assert_eq!(MetricKind::from_code("x"), None);
        assert!("".parse::<MetricKind>().is_err());
        assert_eq!(MetricValueType::from(7).kind(), MetricKind::Integer);
    }

    #[test]
    fn test_item_kind_codes_roundtrip() {
        for kind in ItemKind::ALL {
            assert_eq!(ItemKind::from_code(kind.code()), Some(kind));
            assert_eq!(kind.to_string(), kind.code());
        }
        assert_eq!(ItemKind::from_code("h"), None);
    }

    #[test]
    fn test_validate_hand_built_row() {
        let mut r = row(1, "dev");
        assert_eq!(r.validate(), Ok(()));
        r.device_id = "bad,id".into();
        assert!(matches!(r.validate(), Err(IoTextError::DeviceId(_))));
        let mut r = row(1, "dev");
        r.metrics.push(iotext_rs::MetricDataItem {
            name: "t".into(),
            value: "a,b".into(),
        });
        assert!(matches!(r.validate(), Err(IoTextError::TextValue(_))));
    }

    // ---------- regressions found by cargo fuzz ----------

    #[test]
    fn test_fuzz_regression_negative_decimal_zero_is_canonical() {
        // Rounds to a *negative* zero inside rust_decimal.
        let row: IoTextDataRow = "t|1,d|x,m|a=d:-.000000000000000000000001278965377E-6"
            .parse()
            .unwrap();
        let canonical = row.to_string();
        assert_eq!(canonical, "t|1,d|x,m|a=d:0.0000000000000000000000000000");
        assert_eq!(
            canonical.parse::<IoTextDataRow>().unwrap().to_string(),
            canonical
        );
    }

    #[test]
    fn test_hand_built_negative_zero_serializes_without_sign() {
        let value = MetricValueType::DecimalItemType(Decimal::from_parts(0, 0, 0, true, 2));
        assert_eq!(value.to_string(), "d:0.00");
    }

    // ---------- defaults and deprecated 0.5.x API ----------

    #[test]
    fn test_defaults() {
        assert_eq!(
            MetricValueType::default(),
            MetricValueType::TextItemType(String::new())
        );
        assert_eq!(IoTextDataRow::default().to_string(), "t|0,d|");
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_code_constants_match_enums() {
        use iotext_rs::{ItemTypes, MetricDataTypes};
        assert_eq!(MetricDataTypes::INTEGER, MetricKind::Integer.code());
        assert_eq!(MetricDataTypes::BOOL, MetricKind::Bool.code());
        assert_eq!(MetricDataTypes::DECIMAL, MetricKind::Decimal.code());
        assert_eq!(MetricDataTypes::TEXT, MetricKind::Text.code());
        assert_eq!(ItemTypes::TIMESTAMP_MILIS, ItemKind::Timestamp.code());
        assert_eq!(ItemTypes::DEVICE_ID, ItemKind::DeviceId.code());
        assert_eq!(ItemTypes::METRIC_ITEM, ItemKind::Metric.code());
        assert_eq!(ItemTypes::CRC, ItemKind::Crc.code());
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_extract_metric_value_type() {
        use iotext_rs::IoTextData;
        let row = IoTextDataRow::default();
        assert_eq!(
            row.extract_metric_value_type("i", "42"),
            Ok(MetricValueType::IntegerItemType(42))
        );
        assert!(row.extract_metric_value_type("i", "x").is_err());
    }
}
