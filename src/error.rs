use crate::validators::{
    CrcError, DeviceIdError, MetricDataTypeError, MetricNameError, TextValueError, TimestampError,
};
use thiserror::Error;

/// Every error the `IoText` parser, serializer and builder can return.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IoTextError {
    #[error("IoText message is empty")]
    EmptyMessage,

    #[error("Malformed item: '{0}' (expected '<item_type>|<value>')")]
    MalformedItem(String),

    #[error("Malformed metric: '{0}' (expected 'm|<name>=<data_type>:<value>')")]
    MalformedMetric(String),

    #[error("Unknown item type: '{0}' (expected one of: t, d, m, c)")]
    UnknownItemType(String),

    #[error("Duplicated item: '{0}' (expected only once in message)")]
    DuplicatedItem(String),

    #[error("Timestamp item 't|...' is required")]
    MissingTimestamp,

    #[error("DeviceId item 'd|...' is required")]
    MissingDeviceId,

    #[error("CRC item 'c|...' must be the last item in message")]
    CrcNotLast,

    #[error("CRC mismatch: message contains '{received}', computed '{computed}'")]
    CrcMismatch { received: String, computed: String },

    #[error("Invalid INTEGER (i64) metric value: '{0}'")]
    InvalidInteger(String),

    #[error("Invalid BOOL metric value: '{0}' (expected '0' or '1')")]
    InvalidBool(String),

    #[error("Invalid DECIMAL metric value: '{0}'")]
    InvalidDecimal(String),

    #[error("System clock is before UNIX epoch or out of u64 milliseconds range")]
    InvalidSystemTime,

    #[error(transparent)]
    Timestamp(#[from] TimestampError),

    #[error(transparent)]
    DeviceId(#[from] DeviceIdError),

    #[error(transparent)]
    Crc(#[from] CrcError),

    #[error(transparent)]
    MetricName(#[from] MetricNameError),

    #[error(transparent)]
    MetricDataType(#[from] MetricDataTypeError),

    #[error(transparent)]
    TextValue(#[from] TextValueError),
}
