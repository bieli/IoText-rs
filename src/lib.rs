//! `IoText` data protocol: parser, serializer and builder.
//!
//! ```
//! use iotext_rs::{IoTextDataRow, MetricValueType};
//!
//! let row: IoTextDataRow = "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C".parse()?;
//! assert_eq!(row.timestamp, 3900237526042);
//! assert_eq!(row.device_id, "DEV_NAME_002");
//! assert_eq!(row.crc16, Some(0x3E3C));
//! assert_eq!(row.metric("example_metric_name"), Some(&MetricValueType::from(rust_decimal::Decimal::new(1207, 2))));
//! assert_eq!(row.to_string(), "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C");
//!
//! // Malformed input is reported as an error, never a panic.
//! assert!("t|1,d|dev,m|temp".parse::<IoTextDataRow>().is_err());
//! # Ok::<(), iotext_rs::IoTextError>(())
//! ```
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::indexing_slicing
    )
)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;
use log::trace;
use rust_decimal::Decimal;

/// Fluent [`IoTextBuilder`](builder::IoTextBuilder) for composing messages.
pub mod builder;
/// The crate-wide [`IoTextError`].
pub mod error;
pub mod tools;
/// Field validators used by the parser and builder.
pub mod validators;

pub use error::IoTextError;
pub use tools::*;
use validators::Validators;

/// Separates items: `t|1,d|dev`.
pub const SPECIAL_CHAR_DATA_ITEMS_SEP: &str = ",";
/// Separates a metric data type from its value: `i:42`.
pub const SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP: &str = ":";
/// Separates an item type from its content: `t|1`.
pub const SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP: &str = "|";
/// Separates a metric name from its typed value: `name=i:42`.
pub const SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP: &str = "=";

// ---------------------------------------------------------------------------
// Protocol codes
// ---------------------------------------------------------------------------

/// Data type of a metric value: the code between `=` and `:` in `m|name=<code>:value`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum MetricKind {
    /// `i` - signed 64-bit integer.
    Integer,
    /// `b` - boolean, encoded as `0` / `1`.
    Bool,
    /// `d` - arbitrary precision decimal.
    Decimal,
    /// `t` - text (any characters except `,`).
    Text,
}

impl MetricKind {
    /// All kinds, in protocol order.
    pub const ALL: [MetricKind; 4] = [
        MetricKind::Integer,
        MetricKind::Bool,
        MetricKind::Decimal,
        MetricKind::Text,
    ];

    /// The protocol code, e.g. `"i"`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            MetricKind::Integer => "i",
            MetricKind::Bool => "b",
            MetricKind::Decimal => "d",
            MetricKind::Text => "t",
        }
    }

    /// Looks a kind up by its protocol code.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.code() == code)
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for MetricKind {
    type Err = IoTextError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Validators::validate_and_parse_metric_kind_str(code, "")?)
    }
}

/// Item type of a message part: the code before `|`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ItemKind {
    /// `t` - UNIX timestamp in milliseconds.
    Timestamp,
    /// `d` - device id.
    DeviceId,
    /// `m` - metric.
    Metric,
    /// `c` - CRC-16 of everything before it.
    Crc,
}

impl ItemKind {
    /// All kinds, in protocol order.
    pub const ALL: [ItemKind; 4] = [
        ItemKind::Timestamp,
        ItemKind::DeviceId,
        ItemKind::Metric,
        ItemKind::Crc,
    ];

    /// The protocol code, e.g. `"t"`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            ItemKind::Timestamp => "t",
            ItemKind::DeviceId => "d",
            ItemKind::Metric => "m",
            ItemKind::Crc => "c",
        }
    }

    /// Looks a kind up by its protocol code.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.code() == code)
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// Pre-1.0 metric data type codes.
#[deprecated(
    since = "1.0.0",
    note = "use `MetricKind` (e.g. `MetricKind::Integer.code()`)"
)]
#[derive(Debug, Default)]
pub struct MetricDataTypes {}

#[allow(deprecated)]
impl MetricDataTypes {
    /// Code of [`MetricKind::Integer`].
    pub const INTEGER: &'static str = MetricKind::Integer.code();
    /// Code of [`MetricKind::Bool`].
    pub const BOOL: &'static str = MetricKind::Bool.code();
    /// Code of [`MetricKind::Decimal`].
    pub const DECIMAL: &'static str = MetricKind::Decimal.code();
    /// Code of [`MetricKind::Text`].
    pub const TEXT: &'static str = MetricKind::Text.code();
}

/// Pre-1.0 item type codes.
#[deprecated(
    since = "1.0.0",
    note = "use `ItemKind` (e.g. `ItemKind::Timestamp.code()`)"
)]
#[derive(Debug, Default)]
pub struct ItemTypes {}

#[allow(deprecated)]
impl ItemTypes {
    /// Code of [`ItemKind::Timestamp`].
    pub const TIMESTAMP_MILIS: &'static str = ItemKind::Timestamp.code();
    /// Code of [`ItemKind::DeviceId`].
    pub const DEVICE_ID: &'static str = ItemKind::DeviceId.code();
    /// Code of [`ItemKind::Metric`].
    pub const METRIC_ITEM: &'static str = ItemKind::Metric.code();
    /// Code of [`ItemKind::Crc`].
    pub const CRC: &'static str = ItemKind::Crc.code();
}

// ---------------------------------------------------------------------------
// Metric values
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
/// A typed metric value.
pub enum MetricValueType {
    /// `i:<i64>`
    IntegerItemType(i64),
    /// `b:0` / `b:1`
    BoolItemType(bool),
    /// `d:<decimal>`
    DecimalItemType(Decimal),
    /// `t:<text>` - any characters except `,`.
    TextItemType(String),
}

impl Default for MetricValueType {
    fn default() -> Self {
        MetricValueType::TextItemType(String::new())
    }
}

impl MetricValueType {
    /// The data type of this value.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        match self {
            MetricValueType::IntegerItemType(_) => MetricKind::Integer,
            MetricValueType::BoolItemType(_) => MetricKind::Bool,
            MetricValueType::DecimalItemType(_) => MetricKind::Decimal,
            MetricValueType::TextItemType(_) => MetricKind::Text,
        }
    }

    /// Parses a raw protocol value of the given kind.
    ///
    /// # Errors
    /// Returns [`IoTextError`] when `value` is not valid for `kind`.
    pub fn from_kind_and_value(kind: MetricKind, value: &str) -> Result<Self, IoTextError> {
        let parsed = match kind {
            MetricKind::Integer => value
                .parse::<i64>()
                .map(MetricValueType::IntegerItemType)
                .map_err(|_| IoTextError::InvalidInteger(value.to_string()))?,
            MetricKind::Bool => match value {
                "1" => MetricValueType::BoolItemType(true),
                "0" => MetricValueType::BoolItemType(false),
                _ => return Err(IoTextError::InvalidBool(value.to_string())),
            },
            MetricKind::Decimal => Decimal::from_str(value)
                .map(MetricValueType::DecimalItemType)
                .map_err(|_| IoTextError::InvalidDecimal(value.to_string()))?,
            MetricKind::Text => MetricValueType::TextItemType(
                Validators::validate_text_value_str(value)?.to_string(),
            ),
        };
        trace!("metric value parsed: {parsed:?}");
        Ok(parsed)
    }

    /// Parses a metric value from its protocol data type code (`i`, `b`, `d`, `t`) and raw value.
    ///
    /// # Errors
    /// Returns [`IoTextError`] for an unknown data type or a value that does not match it.
    pub fn from_type_and_value(metric_data_type: &str, value: &str) -> Result<Self, IoTextError> {
        Self::from_type_and_value_named(metric_data_type, value, "")
    }

    fn from_type_and_value_named(
        metric_data_type: &str,
        value: &str,
        metric_name: &str,
    ) -> Result<Self, IoTextError> {
        let kind = Validators::validate_and_parse_metric_kind_str(metric_data_type, metric_name)?;
        Self::from_kind_and_value(kind, value)
    }
}

impl<'a> TryFrom<(&'a str, &'a str)> for MetricValueType {
    type Error = IoTextError;

    /// `(data_type, value)`, e.g. `("i", "42")`.
    fn try_from((metric_data_type, value): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        Self::from_type_and_value(metric_data_type, value)
    }
}

impl From<i64> for MetricValueType {
    fn from(value: i64) -> Self {
        MetricValueType::IntegerItemType(value)
    }
}

impl From<i32> for MetricValueType {
    fn from(value: i32) -> Self {
        MetricValueType::IntegerItemType(i64::from(value))
    }
}

impl From<u32> for MetricValueType {
    fn from(value: u32) -> Self {
        MetricValueType::IntegerItemType(i64::from(value))
    }
}

impl From<bool> for MetricValueType {
    fn from(value: bool) -> Self {
        MetricValueType::BoolItemType(value)
    }
}

impl From<Decimal> for MetricValueType {
    fn from(value: Decimal) -> Self {
        MetricValueType::DecimalItemType(value)
    }
}

impl From<String> for MetricValueType {
    fn from(value: String) -> Self {
        MetricValueType::TextItemType(value)
    }
}

impl From<&str> for MetricValueType {
    fn from(value: &str) -> Self {
        MetricValueType::TextItemType(value.to_string())
    }
}

impl fmt::Display for MetricValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.kind();
        let sep = SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP;
        match self {
            MetricValueType::IntegerItemType(value) => write!(f, "{kind}{sep}{value}"),
            MetricValueType::BoolItemType(value) => write!(f, "{kind}{sep}{}", u8::from(*value)),
            MetricValueType::DecimalItemType(value) => write!(f, "{kind}{sep}{value}"),
            MetricValueType::TextItemType(value) => write!(f, "{kind}{sep}{value}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// One `m|<name>=<type>:<value>` item.
pub struct MetricDataItem {
    /// Metric name (ASCII, no protocol separators).
    pub name: String,
    /// Typed metric value.
    pub value: MetricValueType,
}

impl fmt::Display for MetricDataItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP}{}{SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP}{}",
            ItemKind::Metric,
            self.name,
            self.value
        )
    }
}

// ---------------------------------------------------------------------------
// Message row
// ---------------------------------------------------------------------------

/// One `IoText` message.
///
/// Rows returned by [`str::parse`] and [`builder::IoTextBuilder::build`] are always valid.
/// When constructing one by hand, use [`IoTextDataRow::validate`] before sending it.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IoTextDataRow {
    /// UNIX timestamp in milliseconds (`t|...`).
    pub timestamp: u64,
    /// Device id (`d|...`).
    pub device_id: String,
    /// Metrics (`m|...`), in message order.
    pub metrics: Vec<MetricDataItem>,
    /// CRC-16 (`c|XXXX`). When `Some`, [`Display`](fmt::Display) appends a freshly computed CRC.
    pub crc16: Option<u16>,
}

impl IoTextDataRow {
    /// Value of the first metric with the given name.
    #[must_use]
    pub fn metric(&self, name: &str) -> Option<&MetricValueType> {
        self.metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| &metric.value)
    }

    /// Checks the invariants that [`str::parse`] and the builder guarantee.
    ///
    /// # Errors
    /// Returns the first invalid field as [`IoTextError`].
    pub fn validate(&self) -> Result<(), IoTextError> {
        Validators::validate_device_id_str(&self.device_id)?;
        for metric in &self.metrics {
            Validators::validate_metric_name_str(&metric.name)?;
            if let MetricValueType::TextItemType(text) = &metric.value {
                Validators::validate_text_value_str(text)?;
            }
        }
        Ok(())
    }

    /// The message without the CRC item - exactly the bytes the CRC is computed over.
    fn body_to_string(&self) -> String {
        use core::fmt::Write;
        let mut body = format!(
            "{}{SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP}{}{SPECIAL_CHAR_DATA_ITEMS_SEP}{}{SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP}{}",
            ItemKind::Timestamp,
            self.timestamp,
            ItemKind::DeviceId,
            self.device_id
        );
        for metric in &self.metrics {
            // Writing into a String cannot fail.
            let _ = write!(body, "{SPECIAL_CHAR_DATA_ITEMS_SEP}{metric}");
        }
        body
    }

    /// Computes the CRC-16 for the current content of this row.
    #[must_use]
    pub fn compute_crc16(&self) -> u16 {
        crc16(self.body_to_string().as_bytes())
    }

    /// Serializes the row; with `add_crc16` a freshly computed `c|XXXX` item is appended.
    #[must_use]
    pub fn to_iotext_string(&self, add_crc16: bool) -> String {
        let mut msg = self.body_to_string();
        if add_crc16 {
            let crc = crc16(msg.as_bytes());
            msg.push_str(SPECIAL_CHAR_DATA_ITEMS_SEP);
            msg.push_str(ItemKind::Crc.code());
            msg.push_str(SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP);
            msg.push_str(&crc16_to_hex(crc));
        }
        msg
    }
}

/// Serializes the row; a CRC item is appended (recomputed) when `crc16` is `Some`.
impl fmt::Display for IoTextDataRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_iotext_string(self.crc16.is_some()))
    }
}

/// Parses an `IoText` message. A CRC item, if present, must be last and must match.
impl FromStr for IoTextDataRow {
    type Err = IoTextError;

    fn from_str(data_row: &str) -> Result<Self, Self::Err> {
        if data_row.is_empty() {
            return Err(IoTextError::EmptyMessage);
        }

        let mut timestamp: Option<u64> = None;
        let mut device_id: Option<&str> = None;
        let mut metrics: Vec<MetricDataItem> = Vec::new();
        let mut crc: Option<u16> = None;
        let mut part_start = 0usize;

        for part in data_row.split(SPECIAL_CHAR_DATA_ITEMS_SEP) {
            trace!("part: {part}");
            let this_part_start = part_start;
            part_start += part.len() + SPECIAL_CHAR_DATA_ITEMS_SEP.len();

            if crc.is_some() {
                return Err(IoTextError::CrcNotLast);
            }

            let (item_code, value) = part
                .split_once(SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP)
                .ok_or_else(|| IoTextError::MalformedItem(part.to_string()))?;
            let item_kind = ItemKind::from_code(item_code)
                .ok_or_else(|| IoTextError::UnknownItemType(item_code.to_string()))?;

            match item_kind {
                ItemKind::Metric => {
                    let malformed = || IoTextError::MalformedMetric(part.to_string());
                    let (name, typed_value) = value
                        .split_once(SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP)
                        .ok_or_else(malformed)?;
                    let (data_type, raw_value) = typed_value
                        .split_once(SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP)
                        .ok_or_else(malformed)?;
                    let name = Validators::validate_metric_name_str(name)?;
                    metrics.push(MetricDataItem {
                        name: name.to_string(),
                        value: MetricValueType::from_type_and_value_named(
                            data_type, raw_value, name,
                        )?,
                    });
                }
                ItemKind::Timestamp => {
                    if timestamp.is_some() {
                        return Err(IoTextError::DuplicatedItem(part.to_string()));
                    }
                    timestamp = Some(Validators::validate_and_parse_timestamp_str(value)?);
                }
                ItemKind::DeviceId => {
                    if device_id.is_some() {
                        return Err(IoTextError::DuplicatedItem(part.to_string()));
                    }
                    device_id = Some(Validators::validate_device_id_str(value)?);
                }
                ItemKind::Crc => {
                    let received = Validators::validate_and_parse_crc_str(value)?;
                    // CRC covers everything before ",c|XXXX". ',' is ASCII, so this is a char boundary.
                    let body = data_row
                        .get(..this_part_start.saturating_sub(SPECIAL_CHAR_DATA_ITEMS_SEP.len()))
                        .unwrap_or_default();
                    let computed = crc16(body.as_bytes());
                    if computed != received {
                        return Err(IoTextError::CrcMismatch { received, computed });
                    }
                    crc = Some(received);
                }
            }
        }

        Ok(IoTextDataRow {
            timestamp: timestamp.ok_or(IoTextError::MissingTimestamp)?,
            device_id: device_id.ok_or(IoTextError::MissingDeviceId)?.to_string(),
            metrics,
            crc16: crc,
        })
    }
}

/// Pre-1.0 API kept for easier migration; methods now return `Result` instead of panicking.
#[deprecated(
    since = "1.0.0",
    note = "use `str::parse::<IoTextDataRow>()`, `IoTextDataRow::to_iotext_string()` and `MetricValueType::from_type_and_value()`"
)]
pub trait IoTextData {
    /// # Errors
    /// See [`MetricValueType::from_type_and_value`].
    fn extract_metric_value_type(
        &self,
        metric_data_type: &str,
        metric_data_value: &str,
    ) -> Result<MetricValueType, IoTextError>;

    /// # Errors
    /// See [`IoTextDataRow::from_str`].
    fn parse_iotext_str(&self, data_row: &str) -> Result<IoTextDataRow, IoTextError>;

    /// See [`IoTextDataRow::to_iotext_string`].
    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow, add_crc16: bool) -> String;
}

#[allow(deprecated)]
impl IoTextData for IoTextDataRow {
    fn extract_metric_value_type(
        &self,
        metric_data_type: &str,
        metric_data_value: &str,
    ) -> Result<MetricValueType, IoTextError> {
        MetricValueType::from_type_and_value(metric_data_type, metric_data_value)
    }

    fn parse_iotext_str(&self, data_row: &str) -> Result<IoTextDataRow, IoTextError> {
        data_row.parse()
    }

    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow, add_crc16: bool) -> String {
        iotext_data_row.to_iotext_string(add_crc16)
    }
}
