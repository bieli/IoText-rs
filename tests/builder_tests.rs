use iotext_rs::builder::IoTextBuilder;

#[cfg(test)]
mod builder_tests {
    use super::*;
    use iotext_rs::builder::IoTextBuilderError;
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
        let metrics = iotext_data_row
            .get_metrics()
            .clone()
            .unwrap_or_else(Vec::new);
        assert_eq!(metrics.len(), 2);
        assert_eq!(
            metrics.get(0).unwrap(),
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
        let metrics = iotext_data_row
            .get_metrics()
            .clone()
            .unwrap_or_else(Vec::new);
        assert_eq!(metrics.len(), 1);
        assert_eq!(
            metrics.get(0).unwrap(),
            &MetricDataItem {
                name: "temp1".to_string(),
                value: MetricValueType::IntegerItemType(112),
            }
        );
    }

    #[test]
    fn test_valid_add_metric_as_str_with_not_existsing_type() {
        setup();
        let iotext_data_row = IoTextBuilder::new()
            .current_timestamp()
            .device_id("dev02")
            .add_metric_as_str("temp1", "113", "unknown")
            .build(false)
            .unwrap();
        let metrics = iotext_data_row
            .get_metrics()
            .clone()
            .unwrap_or_else(Vec::new);
        assert_eq!(metrics.len(), 1);
        assert_eq!(
            metrics.get(0).unwrap(),
            &MetricDataItem {
                name: "temp1".to_string(),
                value: MetricValueType::TextItemType("".to_string()),
            }
        );
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

        assert!(matches!(
            result,
            Err(IoTextBuilderError::TimestampIsEmpty(_))
        ));
    }

    #[test]
    fn test_invalid_build_missing_device_id() {
        setup();
        let result = IoTextBuilder::new().timestamp(1616161616).build(false);

        assert!(matches!(
            result,
            Err(IoTextBuilderError::DeviceIdIsEmpty(_))
        ));
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
}
