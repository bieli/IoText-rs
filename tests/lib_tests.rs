use rust_decimal::prelude::*;

use iotext_rs::validators::{DeviceIdError, MetricDataTypeError, MetricNameError};
use iotext_rs::{IoTextDataRow, IoTextError, Item, ItemTypeEnum, MetricValueType};

#[cfg(test)]
mod lib_tests {
    use super::*;

    fn row(ts: u64, device_id: &str) -> IoTextDataRow {
        IoTextDataRow {
            timestamp: Item {
                value: ItemTypeEnum::TimeUnixMilis(ts),
            },
            device_id: Item {
                value: ItemTypeEnum::DeviceId(device_id.to_string()),
            },
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
        assert_eq!(
            result.get_timestamp(),
            &ItemTypeEnum::TimeUnixMilis(3900237526044)
        );
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements() {
        let result: IoTextDataRow = "t|3900237526045,d|device_name_005".parse().unwrap();
        assert_eq!(
            result.get_device_id(),
            &ItemTypeEnum::DeviceId("device_name_005".to_string())
        );
        assert_eq!(result.get_metrics(), &None);
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements_with_crc16() {
        let result: IoTextDataRow = "t|3900237526045,d|device_name_005,c|6F89".parse().unwrap();
        assert_eq!(result.get_crc16().as_ref().unwrap().to_string(), "c|6F89");
    }

    #[test]
    fn test_parse_accepts_lowercase_crc() {
        assert!("t|3900237526045,d|device_name_005,c|6f89"
            .parse::<IoTextDataRow>()
            .is_ok());
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
                received: "0000".into(),
                computed: iotext_rs::Tools::crc16("t|1,d|x", iotext_rs::CRC16_POLY_DEFAULT),
            }
        );
    }

    #[test]
    fn test_parse_error_crc_not_last() {
        let crc = iotext_rs::Tools::crc16("t|1,d|x", iotext_rs::CRC16_POLY_DEFAULT);
        assert_eq!(
            parse_err(&format!("t|1,d|x,c|{crc},m|a=i:1")),
            IoTextError::CrcNotLast
        );
    }

    #[test]
    fn test_parse_error_structure() {
        assert_eq!(parse_err(""), IoTextError::EmptyMessage);
        assert_eq!(parse_err("t|1,d|x,"), IoTextError::MalformedItem("".into()));
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
}
