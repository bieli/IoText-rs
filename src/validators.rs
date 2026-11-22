/// Field validators; each returns the validated (or parsed) value or a typed error.
pub struct Validators;

use crate::{
    MetricKind, SPECIAL_CHAR_DATA_ITEMS_SEP, SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP, SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP,
};
use alloc::string::{String, ToString};
use thiserror::Error;

/// Minimum device id length in bytes.
pub const DEVICE_ID_MIN_LEN: usize = 1;
/// Maximum device id length in bytes.
pub const DEVICE_ID_MAX_LEN: usize = 128;
/// Length of the CRC item value (hex digits).
pub const CRC_CHECKSUM_LEN: usize = 4;
/// Minimum metric name length in bytes.
pub const METRIC_NAME_MIN_LEN: usize = DEVICE_ID_MIN_LEN;
/// Maximum metric name length in bytes.
pub const METRIC_NAME_MAX_LEN: usize = DEVICE_ID_MAX_LEN;

/// Protocol separators that are never allowed inside identifiers (device id, metric name).
pub const SPECIAL_CHARS: [&str; 4] = [
    SPECIAL_CHAR_DATA_ITEMS_SEP,
    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP,
    SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
    SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP,
];

fn contains_special_char(value: &str) -> bool {
    SPECIAL_CHARS.iter().any(|sep| value.contains(sep))
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid `t|...` value.
pub enum TimestampError {
    #[error("Invalid timestamp: {0} (expected only digits, no any other characters)")]
    /// Contains characters other than ASCII digits.
    InvalidTimestamp(String),

    #[error("Timestamp out of bounds {0} ({min} <= expected < {max})", min = u64::MIN, max = u64::MAX)]
    /// Does not fit in `u64`.
    TimestampOutOfBounds(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid `d|...` value.
pub enum DeviceIdError {
    #[error("DeviceId is empty: {0} (expected minimum one ascii character)")]
    /// Empty or whitespace only.
    DeviceIdIsEmpty(String),

    #[error("DeviceId allow only ASCII characters: {0} (unexpected UTF characters)")]
    /// Contains non-ASCII characters.
    DeviceIdExpectedOnlyAsciiCharacters(String),

    #[error("DeviceId is too long {0} ({min} <= expected < {max})", min = DEVICE_ID_MIN_LEN, max = DEVICE_ID_MAX_LEN)]
    /// Longer than [`DEVICE_ID_MAX_LEN`].
    DeviceIdTooLong(String),

    #[error("DeviceId contains protocol separator: {0} (unexpected characters: , : | =)")]
    /// Contains one of `, : | =`.
    DeviceIdContainsSpecialChar(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid `c|...` value.
pub enum CrcError {
    #[error("Crc wrong size: {0} (expected exactly four HEX ASCII [0-9A-F] characters)")]
    /// Not exactly [`CRC_CHECKSUM_LEN`] bytes.
    CrcIsWrongSize(String),

    #[error(
        "Crc allow only four HEX ASCII [0-9A-F] characters: {0} (unexpected characters i.e. UTF)"
    )]
    /// Contains non-hex characters.
    CrcExpectedOnlyAsciiCharacters(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid metric name.
pub enum MetricNameError {
    #[error("MetricName is empty: {0} (expected minimum one ascii character)")]
    /// Empty or whitespace only.
    MetricNameIsEmpty(String),

    #[error("MetricName allow only ASCII characters: {0} (unexpected UTF characters)")]
    /// Contains non-ASCII characters.
    MetricNameExpectedOnlyAsciiCharacters(String),

    #[error("MetricName is too long {0} ({min} <= expected < {max})", min = METRIC_NAME_MIN_LEN, max = METRIC_NAME_MAX_LEN)]
    /// Longer than [`METRIC_NAME_MAX_LEN`].
    MetricNameTooLong(String),

    #[error("MetricName contains protocol separator: {0} (unexpected characters: , : | =)")]
    /// Contains one of `, : | =`.
    MetricNameContainsSpecialChar(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid metric data type code.
pub enum MetricDataTypeError {
    #[error(
        "MetricDataType is empty or wrong: {0} (expected minimum one ascii and not UTF character)"
    )]
    /// Empty or whitespace only.
    MetricDataTypeIsEmpty(String),

    #[error(
        "MetricDataType is unknown: '{0}' for metric name '{1}' (non-exists definition for this type)"
    )]
    /// Not a known code; payload is `(code, metric name)`.
    UnknownMetricDataType(String, String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Invalid text metric value.
pub enum TextValueError {
    #[error("Text metric value contains items separator ',': {0}")]
    /// Contains the items separator `,`.
    TextValueContainsItemsSeparator(String),
}

impl Validators {
    /// # Errors
    /// Returns [`TimestampError`] when the value is not a decimal `u64`.
    pub fn validate_and_parse_timestamp_str(timestamp_str: &str) -> Result<u64, TimestampError> {
        if !timestamp_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(TimestampError::InvalidTimestamp(timestamp_str.to_string()));
        }

        timestamp_str
            .parse::<u64>()
            .map_err(|_| TimestampError::TimestampOutOfBounds(timestamp_str.to_string()))
    }

    /// # Errors
    /// Returns [`DeviceIdError`] when the device id is empty, non-ASCII, too long
    /// or contains a protocol separator.
    pub fn validate_device_id_str(device_id_str: &str) -> Result<&str, DeviceIdError> {
        if device_id_str.trim().is_empty() {
            return Err(DeviceIdError::DeviceIdIsEmpty(device_id_str.to_string()));
        }

        if !device_id_str.is_ascii() {
            return Err(DeviceIdError::DeviceIdExpectedOnlyAsciiCharacters(
                device_id_str.to_string(),
            ));
        }

        if device_id_str.len() < DEVICE_ID_MIN_LEN || device_id_str.len() > DEVICE_ID_MAX_LEN {
            return Err(DeviceIdError::DeviceIdTooLong(device_id_str.to_string()));
        }

        if contains_special_char(device_id_str) {
            return Err(DeviceIdError::DeviceIdContainsSpecialChar(
                device_id_str.to_string(),
            ));
        }

        Ok(device_id_str)
    }

    /// # Errors
    /// Returns [`CrcError`] when the value is not exactly four hex digits.
    pub fn validate_crc_str(crc_str: &str) -> Result<&str, CrcError> {
        if crc_str.len() != CRC_CHECKSUM_LEN {
            return Err(CrcError::CrcIsWrongSize(crc_str.to_string()));
        }

        if !crc_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(CrcError::CrcExpectedOnlyAsciiCharacters(
                crc_str.to_string(),
            ));
        }

        Ok(crc_str)
    }

    /// # Errors
    /// Returns [`CrcError`] when the value is not exactly four hex digits.
    pub fn validate_and_parse_crc_str(crc_str: &str) -> Result<u16, CrcError> {
        let crc_str = Self::validate_crc_str(crc_str)?;
        u16::from_str_radix(crc_str, 16)
            .map_err(|_| CrcError::CrcExpectedOnlyAsciiCharacters(crc_str.to_string()))
    }

    /// # Errors
    /// Returns [`MetricNameError`] when the name is empty, non-ASCII, too long
    /// or contains a protocol separator.
    pub fn validate_metric_name_str(metric_name_str: &str) -> Result<&str, MetricNameError> {
        if metric_name_str.trim().is_empty() {
            return Err(MetricNameError::MetricNameIsEmpty(
                metric_name_str.to_string(),
            ));
        }

        if !metric_name_str.is_ascii() {
            return Err(MetricNameError::MetricNameExpectedOnlyAsciiCharacters(
                metric_name_str.to_string(),
            ));
        }

        if metric_name_str.len() < METRIC_NAME_MIN_LEN
            || metric_name_str.len() > METRIC_NAME_MAX_LEN
        {
            return Err(MetricNameError::MetricNameTooLong(
                metric_name_str.to_string(),
            ));
        }

        if contains_special_char(metric_name_str) {
            return Err(MetricNameError::MetricNameContainsSpecialChar(
                metric_name_str.to_string(),
            ));
        }

        Ok(metric_name_str)
    }

    /// # Errors
    /// Returns [`MetricDataTypeError`] when the type is empty or not one of `i`, `b`, `d`, `t`.
    pub fn validate_metric_data_type_str<'a>(
        metric_data_type_str: &'a str,
        metric_name: &str,
    ) -> Result<&'a str, MetricDataTypeError> {
        Self::validate_and_parse_metric_kind_str(metric_data_type_str, metric_name)
            .map(|_| metric_data_type_str)
    }

    /// # Errors
    /// Returns [`MetricDataTypeError`] when the type is empty or not one of `i`, `b`, `d`, `t`.
    pub fn validate_and_parse_metric_kind_str(
        metric_data_type_str: &str,
        metric_name: &str,
    ) -> Result<MetricKind, MetricDataTypeError> {
        if metric_data_type_str.trim().is_empty() {
            return Err(MetricDataTypeError::MetricDataTypeIsEmpty(
                metric_data_type_str.to_string(),
            ));
        }

        MetricKind::from_code(metric_data_type_str).ok_or_else(|| {
            MetricDataTypeError::UnknownMetricDataType(
                metric_data_type_str.to_string(),
                metric_name.to_string(),
            )
        })
    }

    /// Text values may contain any character except the items separator `,`.
    ///
    /// # Errors
    /// Returns [`TextValueError`] when the value contains `,`.
    pub fn validate_text_value_str(text_value_str: &str) -> Result<&str, TextValueError> {
        if text_value_str.contains(SPECIAL_CHAR_DATA_ITEMS_SEP) {
            return Err(TextValueError::TextValueContainsItemsSeparator(
                text_value_str.to_string(),
            ));
        }
        Ok(text_value_str)
    }
}
