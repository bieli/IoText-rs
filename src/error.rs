use crate::validators::{
    CrcError, DeviceIdError, MetricDataTypeError, MetricNameError, TextValueError, TimestampError,
};
use alloc::string::String;
use thiserror::Error;

/// Every error the `IoText` parser, serializer and builder can return.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IoTextError {
    #[error("IoText message is empty")]
    /// The input string is empty.
    EmptyMessage,

    #[error("Malformed item: '{0}' (expected '<item_type>|<value>')")]
    /// An item has no `|` separator (payload: the item text).
    MalformedItem(String),

    #[error("Malformed metric: '{0}' (expected 'm|<name>=<data_type>:<value>')")]
    /// A metric item is not `m|<name>=<type>:<value>` (payload: the item text).
    MalformedMetric(String),

    #[error("Unknown item type: '{0}' (expected one of: t, d, m, c)")]
    /// The item type code is not one of `t`, `d`, `m`, `c` (payload: the code).
    UnknownItemType(String),

    #[error("Duplicated item: '{0}' (expected only once in message)")]
    /// A `t` or `d` item occurs more than once (payload: the duplicate item).
    DuplicatedItem(String),

    #[error("Timestamp item 't|...' is required")]
    /// No `t|...` item.
    MissingTimestamp,

    #[error("DeviceId item 'd|...' is required")]
    /// No `d|...` item.
    MissingDeviceId,

    #[error("CRC item 'c|...' must be the last item in message")]
    /// Items follow the `c|...` item.
    CrcNotLast,

    #[error("CRC mismatch: message contains '{received:04X}', computed '{computed:04X}'")]
    /// The CRC in the message does not match the computed one.
    CrcMismatch {
        /// CRC found in the message.
        received: u16,
        /// CRC computed over the message body.
        computed: u16,
    },

    #[error("Invalid INTEGER (i64) metric value: '{0}'")]
    /// An `i` value is not a valid `i64` (payload: the raw value).
    InvalidInteger(String),

    #[error("Invalid BOOL metric value: '{0}' (expected '0' or '1')")]
    /// A `b` value is not `0` or `1` (payload: the raw value).
    InvalidBool(String),

    #[error("Invalid DECIMAL metric value: '{0}'")]
    /// A `d` value is not a valid decimal (payload: the raw value).
    InvalidDecimal(String),

    #[error("System clock is before UNIX epoch or out of u64 milliseconds range")]
    /// The system clock cannot be represented as `u64` UNIX milliseconds.
    InvalidSystemTime,

    #[error(transparent)]
    /// Invalid timestamp value.
    Timestamp(#[from] TimestampError),

    #[error(transparent)]
    /// Invalid device id.
    DeviceId(#[from] DeviceIdError),

    #[error(transparent)]
    /// Invalid CRC item value.
    Crc(#[from] CrcError),

    #[error(transparent)]
    /// Invalid metric name.
    MetricName(#[from] MetricNameError),

    #[error(transparent)]
    /// Invalid metric data type code.
    MetricDataType(#[from] MetricDataTypeError),

    #[error(transparent)]
    /// Invalid text metric value.
    TextValue(#[from] TextValueError),
}
