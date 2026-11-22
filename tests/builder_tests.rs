use iotext_rs::builder::IoTextBuilder;

#[cfg(test)]
mod builder_tests {
    use super::*;
    use iotext_rs::validators::{DeviceIdError, MetricDataTypeError, TextValueError};
    use iotext_rs::IoTextError;
    use iotext_rs::MetricValueType;
    use iotext_rs::{ItemTypeEnum, MetricDataItem};

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            // Any setup code, e.g., initialize logger
        });
    }

    #[test]
    fn test_valid_timestamp() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .timestamp(1616161616)
            .device_id("device123")
            .build(false)
            .unwrap();
        assert_eq!(
            iotext_data_row.timestamp.value,
            ItemTypeEnum::TimeUnixMilis(1616161616)
        );
    }

    #[test]
    fn test_valid_current_timestamp() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .current_timestamp()
            .device_id("device123")
            .build(false)
            .unwrap();
        assert!(matches!(
            iotext_data_row.timestamp.value,
            ItemTypeEnum::TimeUnixMilis(_)
        ));
    }

    #[test]
    fn test_valid_device_id() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .current_timestamp()
            .device_id("device123")
            .build(false)
            .unwrap();
        assert_eq!(
            iotext_data_row.device_id.value,
            ItemTypeEnum::DeviceId("device123".to_string())
        );
    }

    #[test]
    fn test_valid_add_metric() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .current_timestamp()
            .device_id("device123")
            .add_metric("temp1", MetricValueType::TextItemType("12".to_string()))
            .add_metric("temp2", MetricValueType::IntegerItemType(23))
            .build(false)
            .unwrap();
        let metrics = iotext_data_row.get_metrics().clone().unwrap_or_default();
        assert_eq!(metrics.len(), 2);
        assert_eq!(
            metrics.first().unwrap(),
            &MetricDataItem {
                name: "temp1".to_string(),
                value: MetricValueType::TextItemType("12".to_string()),
            }
        );
        assert_eq!(
            metrics.get(1).unwrap(),
            &MetricDataItem {
                name: "temp2".to_string(),
                value: MetricValueType::IntegerItemType(23),
            }
        );
    }

    #[test]
    fn test_valid_add_metric_as_str() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .current_timestamp()
            .device_id("dev01")
            .add_metric_as_str("temp1", "112", "i")
            .build(false)
            .unwrap();
        let metrics = iotext_data_row.get_metrics().clone().unwrap_or_default();
        assert_eq!(metrics.len(), 1);
        assert_eq!(
            metrics.first().unwrap(),
            &MetricDataItem {
                name: "temp1".to_string(),
                value: MetricValueType::IntegerItemType(112),
            }
        );
    }

    #[test]
    fn test_invalid_add_metric_as_str_with_not_existsing_type() {
        setup();
        // 0.5.x silently stored an empty TEXT value here.
        let result = IoTextBuilder::new()
            .current_timestamp()
            .device_id("dev02")
            .add_metric_as_str("temp1", "113", "unknown")
            .build(false);
        assert_eq!(
            result,
            Err(IoTextError::MetricDataType(
                MetricDataTypeError::UnknownMetricDataType("unknown".into(), "temp1".into())
            ))
        );
    }

    #[test]
    fn test_invalid_add_metric_as_str_with_wrong_value() {
        setup();
        let result = IoTextBuilder::new()
            .timestamp(1)
            .device_id("dev02")
            .add_metric_as_str("temp1", "abc", "i")
            .build(false);
        assert_eq!(result, Err(IoTextError::InvalidInteger("abc".into())));
    }

    #[test]
    fn test_invalid_device_id_returns_error_instead_of_panic() {
        setup();
        let result = IoTextBuilder::new().timestamp(1).device_id("").build(false);
        assert!(matches!(
            result,
            Err(IoTextError::DeviceId(DeviceIdError::DeviceIdIsEmpty(_)))
        ));
    }

    #[test]
    fn test_invalid_metric_name_returns_error_instead_of_panic() {
        setup();
        let result = IoTextBuilder::new()
            .timestamp(1)
            .device_id("dev")
            .add_metric("a=b", MetricValueType::IntegerItemType(1))
            .build(false);
        assert!(matches!(result, Err(IoTextError::MetricName(_))));
    }

    #[test]
    fn test_invalid_text_value_with_items_separator() {
        setup();
        let result = IoTextBuilder::new()
            .timestamp(1)
            .device_id("dev")
            .add_metric("txt", MetricValueType::TextItemType("a,b".into()))
            .build(false);
        assert_eq!(
            result,
            Err(IoTextError::TextValue(
                TextValueError::TextValueContainsItemsSeparator("a,b".into())
            ))
        );
    }

    #[test]
    fn test_first_error_wins() {
        setup();
        let result = IoTextBuilder::new()
            .device_id("")
            .add_metric_as_str("x", "1", "zzz")
            .build(false);
        assert!(matches!(result, Err(IoTextError::DeviceId(_))));
    }

    #[test]
    fn test_valid_build_success_without_crc() {
        setup();
        let result = IoTextBuilder::new()
            .timestamp(1616161616)
            .device_id("device123")
            .build(false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_build_missing_timestamp() {
        setup();
        let result = IoTextBuilder::new().device_id("device123").build(false);

        assert!(matches!(result, Err(IoTextError::MissingTimestamp)));
    }

    #[test]
    fn test_invalid_build_missing_device_id() {
        setup();
        let result = IoTextBuilder::new().timestamp(1616161616).build(false);

        assert!(matches!(result, Err(IoTextError::MissingDeviceId)));
    }

    #[test]
    fn test_valid_build_with_crc() {
        setup();
        let result = IoTextBuilder::new()
            .timestamp(1616161616)
            .device_id("device123")
            .build(true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_with_crc_matches_parser() {
        setup();
        let row = IoTextBuilder::new()
            .timestamp(3900237526042)
            .device_id("DEV_NAME_002")
            .add_metric_as_str("example_metric_name", "12.07", "d")
            .build(true)
            .unwrap();
        let msg = row.to_string();
        assert_eq!(
            msg,
            "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C"
        );
        assert_eq!(msg.parse::<iotext_rs::IoTextDataRow>().unwrap(), row);
    }
}
