pub struct Validators;

use std::*;
use thiserror::Error;
use crate::{MetricDataTypes};

pub const DEVICE_ID_MIN_LEN: usize = 1;
pub const DEVICE_ID_MAX_LEN: usize = 128;
pub const CRC_CHECKSUM_LEN: usize = 4;
pub const METRIC_NAME_MIN_LEN: usize = DEVICE_ID_MIN_LEN;
pub const METRIC_NAME_MAX_LEN: usize = DEVICE_ID_MAX_LEN;




#[derive(Error, Debug)]
pub enum TimestampError {
    #[error("Invalid timestamp: {0} (expected only digits, no any other characters)")]
    InvalidTimestamp(String),

    #[error("Timestamp out of bounds {0} ({min} <= expected < {max})", min = u64::MIN, max = u64::MAX)]
    TimestampOutOfBounds(String),
}

#[derive(Error, Debug)]
pub enum DeviceIdError {
    #[error("DeviceId is empty: {0} (expected minimum one ascii character)")]
    DeviceIdIsEmpty(String),

    #[error("DeviceId allow only ASCII characters: {0} (unexpected UTF characters)")]
    DeviceIdExpectedOnlyAsciiCharacters(String),

    #[error("DeviceId is too long {0} ({min} <= expected < {max})", min = DEVICE_ID_MIN_LEN, max = DEVICE_ID_MAX_LEN)]
    DeviceIdTooLong(String),
}

#[derive(Error, Debug)]
pub enum CrcError {
    #[error("Crc wrong size: {0} (expected exactly four HEX ASCII [0-9A-F] characters)")]
    CrcIsWrongSize(String),

    #[error("Crc allow only four HEX ASCII [0-9A-F] characters: {0} (unexpected characters i.e. UTF)")]
    CrcExpectedOnlyAsciiCharacters(String),
}

#[derive(Error, Debug)]
pub enum MetricNameError {
    #[error("MetricName is empty: {0} (expected minimum one ascii character)")]
    MetricNameIsEmpty(String),

    #[error("MetricName allow only ASCII characters: {0} (unexpected UTF characters)")]
    MetricNameExpectedOnlyAsciiCharacters(String),

    #[error("MetricName is too long {0} ({min} <= expected < {max})", min = METRIC_NAME_MIN_LEN, max = METRIC_NAME_MAX_LEN)]
    MetricNameTooLong(String),
}

#[derive(Error, Debug)]
pub enum MetricDataTypeError {
    #[error("MetricDataType is empty: {0} (expected minimum one ascii and not UTF character)")]
    MetricDataTypeIsEmpty(String),

    #[error("MetricDataType is unknown: '{0}' for metric name '{1}' (non-exists definition for this type)")]
    UnknownMetricDataType(String, String),
}

impl Validators {
    pub fn validate_and_parse_timestamp_str(timestamp_str: &str) -> Result<u64, TimestampError> {
        if !timestamp_str.chars().all(|c| c.is_digit(10)) {
            return Err(TimestampError::InvalidTimestamp(timestamp_str.to_string()));
        }

        let Ok(timestamp_value) = timestamp_str.parse::<u64>() else {
            return Err(TimestampError::TimestampOutOfBounds(
                timestamp_str.to_string(),
            ));
        };

        Ok(timestamp_value)
    }

    pub fn validate_device_id_str(device_id_str: &str) -> Result<&str, DeviceIdError> {
        if device_id_str.trim().is_empty() {
            return Err(DeviceIdError::DeviceIdIsEmpty(device_id_str.to_string()));
        }

        if !device_id_str.chars().all(|c| c.is_ascii()) {
            return Err(DeviceIdError::DeviceIdExpectedOnlyAsciiCharacters(device_id_str.to_string()));
        }

        if device_id_str.len() < DEVICE_ID_MIN_LEN || device_id_str.len() > DEVICE_ID_MAX_LEN {
            return Err(DeviceIdError::DeviceIdTooLong(
                device_id_str.to_string(),
            ));
        };

        Ok(device_id_str)
    }

    pub fn validate_crc_str(crc_str: &str) -> Result<&str, CrcError> {
        if crc_str.len() != CRC_CHECKSUM_LEN {
            return Err(CrcError::CrcIsWrongSize(crc_str.to_string()));
        }

        if !crc_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(CrcError::CrcExpectedOnlyAsciiCharacters(crc_str.to_string()));
        }

        Ok(crc_str)
    }

    pub fn validate_metric_name_str(metric_name_str: &str) -> Result<&str, MetricNameError> {
        if metric_name_str.trim().is_empty() {
            return Err(MetricNameError::MetricNameIsEmpty(metric_name_str.to_string()));
        }

        if !metric_name_str.chars().all(|c| c.is_ascii()) {
            return Err(MetricNameError::MetricNameExpectedOnlyAsciiCharacters(metric_name_str.to_string()));
        }

        if metric_name_str.len() < DEVICE_ID_MIN_LEN || metric_name_str.len() > DEVICE_ID_MAX_LEN {
            return Err(MetricNameError::MetricNameTooLong(
                metric_name_str.to_string(),
            ));
        };

        Ok(metric_name_str)
    }

//    pub fn validate_metric_data_type_str(metric_data_type_str: &str, metric_name: &str) -> Result<&str, MetricDataTypeError> {
    pub fn validate_metric_data_type_str<'a>(metric_data_type_str: &'a str, metric_name: &'a str) -> Result<&'a str, MetricDataTypeError> {
        if metric_data_type_str.trim().is_empty()
           || !metric_data_type_str.chars().all(|c| c.is_ascii())
        {
            return Err(MetricDataTypeError::MetricDataTypeIsEmpty(metric_data_type_str.to_string()));
        }

        match metric_data_type_str {
            MetricDataTypes::INTEGER | MetricDataTypes::BOOL
            | MetricDataTypes::TEXT | MetricDataTypes::DECIMAL => Ok(metric_data_type_str),
            _ => Err(MetricDataTypeError::UnknownMetricDataType(metric_data_type_str.to_string(), metric_name.to_string())),
        }
    }
}
