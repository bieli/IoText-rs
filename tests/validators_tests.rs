#[cfg(test)]
mod validators_tests {
    // #[cfg(test)]
    // use pretty_assertions::assert_eq;

    use iotext_rs::validators::Validators;
    use iotext_rs::validators::{
        CrcError, MetricDataTypeError, MetricNameError, METRIC_NAME_MAX_LEN,
    };
    use iotext_rs::validators::{DeviceIdError, TimestampError, DEVICE_ID_MAX_LEN};
    use iotext_rs::MetricDataTypes;

    #[test]
    fn test_valid_timestamp() {
        let timestamp_str = "1627843245678";
        let result = Validators::validate_and_parse_timestamp_str(timestamp_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1627843245678);
    }

    #[test]
    fn test_invalid_timestamp_non_digit() {
        let timestamp_str = "1627abc5678";
        let result = Validators::validate_and_parse_timestamp_str(timestamp_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimestampError::InvalidTimestamp(s) => assert_eq!(s, timestamp_str),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_out_of_bounds_timestamp() {
        let timestamp_str = "18446744073709551616"; // u64::MAX + 1
        let result = Validators::validate_and_parse_timestamp_str(timestamp_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimestampError::TimestampOutOfBounds(s) => assert_eq!(s, timestamp_str),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_valid_minimum_valid_timestamp() {
        let timestamp_str = "0";
        let result = Validators::validate_and_parse_timestamp_str(timestamp_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_valid_maximum_valid_timestamp() {
        let timestamp_str = "18446744073709551615"; // u64::MAX
        let result = Validators::validate_and_parse_timestamp_str(timestamp_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 18446744073709551615);
    }

    #[test]
    fn test_invalid_device_id_is_empty() {
        let device_id = "";
        let result = Validators::validate_device_id_str(device_id);
        assert!(result.is_err());
        match result.err().unwrap() {
            DeviceIdError::DeviceIdIsEmpty(s) => assert_eq!(s, device_id.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_device_id_contains_non_ascii() {
        let device_id = "device_id_with_非_ascii";
        let result = Validators::validate_device_id_str(device_id);
        assert!(result.is_err());
        match result.err().unwrap() {
            DeviceIdError::DeviceIdExpectedOnlyAsciiCharacters(s) => {
                assert_eq!(s, device_id.to_string())
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_device_id_too_long() {
        let device_id = "a".repeat(DEVICE_ID_MAX_LEN + 1);
        let result = Validators::validate_device_id_str(&device_id);
        assert!(result.is_err());
        match result.err().unwrap() {
            DeviceIdError::DeviceIdTooLong(s) => assert_eq!(s, device_id),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_device_id_all_empty_chars() {
        let device_id = " ".repeat(DEVICE_ID_MAX_LEN);
        let result = Validators::validate_device_id_str(&device_id);
        assert!(result.is_err());
        match result.err().unwrap() {
            DeviceIdError::DeviceIdIsEmpty(s) => assert_eq!(s, device_id),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_valid_device_id() {
        let device_id = "valid_device_id";
        let result = Validators::validate_device_id_str(device_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), device_id);
    }

    #[test]
    fn test_invalid_device_id_too_short() {
        let device_id = "";
        let result = Validators::validate_device_id_str(device_id);
        assert!(result.is_err());
        match result.err().unwrap() {
            DeviceIdError::DeviceIdIsEmpty(s) => assert_eq!(s, device_id.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_crc_wrong_size() {
        let crc_str = "12345";
        let result = Validators::validate_crc_str(crc_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            CrcError::CrcIsWrongSize(s) => assert_eq!(s, crc_str.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_crc_contains_non_ascii() {
        let crc_str = "12GZ";
        let result = Validators::validate_crc_str(crc_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            CrcError::CrcExpectedOnlyAsciiCharacters(s) => assert_eq!(s, crc_str.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_valid_crc() {
        let crc_str = "1A2F";
        let result = Validators::validate_crc_str(crc_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), crc_str);
    }

    #[test]
    fn test_invalid_crc_too_short() {
        let crc_str = "1A";
        let result = Validators::validate_crc_str(crc_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            CrcError::CrcIsWrongSize(s) => assert_eq!(s, crc_str.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_crc_utf_char() {
        let crc_str = "ン";
        let result = Validators::validate_crc_str(crc_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            CrcError::CrcIsWrongSize(s) => assert_eq!(s, crc_str.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_name_is_empty() {
        let metric_name = "";
        let result = Validators::validate_metric_name_str(metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricNameError::MetricNameIsEmpty(s) => assert_eq!(s, metric_name.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_name_contains_non_ascii() {
        let metric_name = "metric_name_with_非_ascii";
        let result = Validators::validate_metric_name_str(metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricNameError::MetricNameExpectedOnlyAsciiCharacters(s) => {
                assert_eq!(s, metric_name.to_string())
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_name_too_long() {
        let metric_name = "a".repeat(METRIC_NAME_MAX_LEN + 1);
        let result = Validators::validate_metric_name_str(&metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricNameError::MetricNameTooLong(s) => assert_eq!(s, metric_name),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_valid_metric_name() {
        let metric_name = "valid_metric_name";
        let result = Validators::validate_metric_name_str(metric_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metric_name);
    }

    #[test]
    fn test_invalid_metric_name_too_short() {
        let metric_name = "";
        let result = Validators::validate_metric_name_str(metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricNameError::MetricNameIsEmpty(s) => assert_eq!(s, metric_name.to_string()),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_name_all_empty_chars() {
        let metric_name = " ".repeat(METRIC_NAME_MAX_LEN);
        let result = Validators::validate_metric_name_str(&metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricNameError::MetricNameIsEmpty(s) => assert_eq!(s, metric_name),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_data_type_is_empty() {
        let metric_data_type = "";
        let metric_name = "test_metric888";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricDataTypeError::MetricDataTypeIsEmpty(s) => {
                assert_eq!(s, metric_data_type.to_string())
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_valid_metric_data_type_known_integer() {
        let metric_data_type = MetricDataTypes::INTEGER;
        let metric_name = "test_metric666";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metric_data_type);
    }

    #[test]
    fn test_valid_metric_data_type_known_bool() {
        let metric_data_type = MetricDataTypes::BOOL;
        let metric_name = "test_metric555";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metric_data_type);
    }

    #[test]
    fn test_valid_metric_data_type_known_text() {
        let metric_data_type = MetricDataTypes::TEXT;
        let metric_name = "test_metric444";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metric_data_type);
    }

    #[test]
    fn test_valid_metric_data_type_known_decimal() {
        let metric_data_type = MetricDataTypes::DECIMAL;
        let metric_name = "test_metric333";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metric_data_type);
    }

    #[test]
    fn test_invalid_metric_data_type_unknown() {
        let metric_data_type = "UNKNOWN_TYPE";
        let metric_name = "test_metric222";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricDataTypeError::UnknownMetricDataType(data_type, name) => {
                assert_eq!(data_type, metric_data_type.to_string());
                assert_eq!(name, metric_name.to_string());
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_metric_data_type_contains_non_ascii() {
        let metric_data_type = "非";
        let metric_name = "test_metric111";
        let result = Validators::validate_metric_data_type_str(metric_data_type, metric_name);
        assert!(result.is_err());
        match result.err().unwrap() {
            MetricDataTypeError::UnknownMetricDataType(data_type, name) => {
                assert_eq!(data_type, metric_data_type.to_string());
                assert_eq!(name, metric_name.to_string());
            }
            _ => panic!("Unexpected error type"),
        }
    }
}
