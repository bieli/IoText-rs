//! `IoText` data protocol: parser, serializer and builder.
//!
//! ```
//! use iotext_rs::IoTextDataRow;
//!
//! let row: IoTextDataRow = "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C".parse()?;
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

use log::trace;
use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;

pub mod builder;
pub mod error;
pub mod tools;
pub mod validators;

pub use error::IoTextError;
pub use tools::*;
use validators::Validators;

pub const SPECIAL_CHAR_DATA_ITEMS_SEP: &str = ",";
pub const SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP: &str = ":";
pub const SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP: &str = "|";
pub const SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP: &str = "=";

#[derive(Debug, Default)]
pub struct MetricDataTypes {}

impl MetricDataTypes {
    pub const INTEGER: &'static str = "i";
    pub const BOOL: &'static str = "b";
    pub const DECIMAL: &'static str = "d";
    pub const TEXT: &'static str = "t";
}

#[derive(Debug, Default)]
pub struct ItemTypes {}

impl ItemTypes {
    pub const TIMESTAMP_MILIS: &'static str = "t";
    pub const DEVICE_ID: &'static str = "d";
    pub const METRIC_ITEM: &'static str = "m";
    pub const CRC: &'static str = "c";
    //TODO: pub const HEALTH_CHECK: &'static str = "h";
}

#[derive(Debug, PartialEq, Clone)]
pub enum MetricValueType {
    IntegerItemType(i64),
    BoolItemType(bool),
    DecimalItemType(Decimal),
    TextItemType(String),
}

impl Default for MetricValueType {
    fn default() -> Self {
        MetricValueType::TextItemType(String::new())
    }
}

impl MetricValueType {
    /// Parses a metric value from its protocol data type (`i`, `b`, `d`, `t`) and raw value.
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
        let metric_data_type =
            Validators::validate_metric_data_type_str(metric_data_type, metric_name)?;
        let parsed = match metric_data_type {
            MetricDataTypes::INTEGER => value
                .parse::<i64>()
                .map(MetricValueType::IntegerItemType)
                .map_err(|_| IoTextError::InvalidInteger(value.to_string()))?,
            MetricDataTypes::BOOL => match value {
                "1" => MetricValueType::BoolItemType(true),
                "0" => MetricValueType::BoolItemType(false),
                _ => return Err(IoTextError::InvalidBool(value.to_string())),
            },
            MetricDataTypes::DECIMAL => Decimal::from_str(value)
                .map(MetricValueType::DecimalItemType)
                .map_err(|_| IoTextError::InvalidDecimal(value.to_string()))?,
            // validate_metric_data_type_str() guarantees TEXT here.
            _ => MetricValueType::TextItemType(
                Validators::validate_text_value_str(value)?.to_string(),
            ),
        };
        trace!("metric value parsed: {parsed:?}");
        Ok(parsed)
    }
}

impl<'a> TryFrom<(&'a str, &'a str)> for MetricValueType {
    type Error = IoTextError;

    /// `(data_type, value)`, e.g. `("i", "42")`.
    fn try_from((metric_data_type, value): (&'a str, &'a str)) -> Result<Self, Self::Error> {
        Self::from_type_and_value(metric_data_type, value)
    }
}

impl fmt::Display for MetricValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sep = SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP;
        match self {
            MetricValueType::IntegerItemType(value) => {
                write!(f, "{}{sep}{value}", MetricDataTypes::INTEGER)
            }
            MetricValueType::BoolItemType(value) => {
                write!(f, "{}{sep}{}", MetricDataTypes::BOOL, u8::from(*value))
            }
            MetricValueType::DecimalItemType(value) => {
                write!(f, "{}{sep}{value}", MetricDataTypes::DECIMAL)
            }
            MetricValueType::TextItemType(value) => {
                write!(f, "{}{sep}{value}", MetricDataTypes::TEXT)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemTypeEnum {
    TimeUnixMilis(u64),
    DeviceId(String),
    Crc(String),
}

impl fmt::Display for ItemTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sep = SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP;
        match self {
            ItemTypeEnum::TimeUnixMilis(value) => {
                write!(f, "{}{sep}{value}", ItemTypes::TIMESTAMP_MILIS)
            }
            ItemTypeEnum::DeviceId(value) => write!(f, "{}{sep}{value}", ItemTypes::DEVICE_ID),
            ItemTypeEnum::Crc(value) => write!(f, "{}{sep}{value}", ItemTypes::CRC),
        }
    }
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        ItemTypeEnum::DeviceId(String::new())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MetricDataItem {
    pub name: String,
    pub value: MetricValueType,
}

impl fmt::Display for MetricDataItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            ItemTypes::METRIC_ITEM,
            SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
            self.name,
            SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP,
            self.value
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    pub value: ItemTypeEnum,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IoTextDataRow {
    pub timestamp: Item,
    pub device_id: Item,
    pub metrics: Option<Vec<MetricDataItem>>,
    pub crc16: Option<Item>,
}

impl IoTextDataRow {
    // Immutable access.
    #[must_use]
    pub fn get_timestamp(&self) -> &ItemTypeEnum {
        &self.timestamp.value
    }

    #[must_use]
    pub fn get_device_id(&self) -> &ItemTypeEnum {
        &self.device_id.value
    }

    #[must_use]
    pub fn get_metrics(&self) -> &Option<Vec<MetricDataItem>> {
        &self.metrics
    }

    #[must_use]
    pub fn get_crc16(&self) -> &Option<Item> {
        &self.crc16
    }

    // Mutable access.
    pub fn timestamp_mut(&mut self) -> &mut Item {
        &mut self.timestamp
    }

    pub fn device_id_mut(&mut self) -> &mut Item {
        &mut self.device_id
    }

    pub fn metrics_mut(&mut self) -> &mut Option<Vec<MetricDataItem>> {
        &mut self.metrics
    }

    pub fn crc16_mut(&mut self) -> &mut Option<Item> {
        &mut self.crc16
    }

    /// The message without the CRC item - exactly the bytes the CRC is computed over.
    fn body_to_string(&self) -> String {
        use std::fmt::Write;
        let mut body = format!(
            "{}{SPECIAL_CHAR_DATA_ITEMS_SEP}{}",
            self.timestamp, self.device_id
        );
        for metric in self.metrics.iter().flatten() {
            // Writing into a String cannot fail.
            let _ = write!(body, "{SPECIAL_CHAR_DATA_ITEMS_SEP}{metric}");
        }
        body
    }

    /// Computes the CRC-16 item value for the current content of this row.
    #[must_use]
    pub fn compute_crc16(&self) -> String {
        Tools::crc16(&self.body_to_string(), CRC16_POLY_DEFAULT)
    }

    /// Serializes the row; with `add_crc16` a freshly computed `c|XXXX` item is appended.
    #[must_use]
    pub fn to_iotext_string(&self, add_crc16: bool) -> String {
        let mut msg = self.body_to_string();
        if add_crc16 {
            let crc = Tools::crc16(&msg, CRC16_POLY_DEFAULT);
            msg.push_str(SPECIAL_CHAR_DATA_ITEMS_SEP);
            msg.push_str(ItemTypes::CRC);
            msg.push_str(SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP);
            msg.push_str(&crc);
        }
        msg
    }
}

/// Serializes the row; a CRC item is appended (recomputed) when `crc16` is `Some`.
impl fmt::Display for IoTextDataRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        let mut crc16: Option<&str> = None;
        let mut part_start = 0usize;

        for part in data_row.split(SPECIAL_CHAR_DATA_ITEMS_SEP) {
            trace!("part: {part}");
            let this_part_start = part_start;
            part_start += part.len() + SPECIAL_CHAR_DATA_ITEMS_SEP.len();

            if crc16.is_some() {
                return Err(IoTextError::CrcNotLast);
            }

            let (item_type, value) = part
                .split_once(SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP)
                .ok_or_else(|| IoTextError::MalformedItem(part.to_string()))?;

            match item_type {
                ItemTypes::METRIC_ITEM => {
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
                ItemTypes::TIMESTAMP_MILIS => {
                    if timestamp.is_some() {
                        return Err(IoTextError::DuplicatedItem(part.to_string()));
                    }
                    timestamp = Some(Validators::validate_and_parse_timestamp_str(value)?);
                }
                ItemTypes::DEVICE_ID => {
                    if device_id.is_some() {
                        return Err(IoTextError::DuplicatedItem(part.to_string()));
                    }
                    device_id = Some(Validators::validate_device_id_str(value)?);
                }
                ItemTypes::CRC => {
                    let received = Validators::validate_crc_str(value)?;
                    // CRC covers everything before ",c|XXXX". ',' is ASCII, so this is a char boundary.
                    let body = data_row
                        .get(..this_part_start.saturating_sub(SPECIAL_CHAR_DATA_ITEMS_SEP.len()))
                        .unwrap_or_default();
                    let computed = Tools::crc16(body, CRC16_POLY_DEFAULT);
                    if !computed.eq_ignore_ascii_case(received) {
                        return Err(IoTextError::CrcMismatch {
                            received: received.to_string(),
                            computed,
                        });
                    }
                    crc16 = Some(received);
                }
                other => return Err(IoTextError::UnknownItemType(other.to_string())),
            }
        }

        Ok(IoTextDataRow {
            timestamp: Item {
                value: ItemTypeEnum::TimeUnixMilis(timestamp.ok_or(IoTextError::MissingTimestamp)?),
            },
            device_id: Item {
                value: ItemTypeEnum::DeviceId(
                    device_id.ok_or(IoTextError::MissingDeviceId)?.to_string(),
                ),
            },
            metrics: if metrics.is_empty() {
                None
            } else {
                Some(metrics)
            },
            crc16: crc16.map(|crc| Item {
                value: ItemTypeEnum::Crc(crc.to_string()),
            }),
        })
    }
}

/// Pre-0.6 API kept for easier migration; methods now return `Result` instead of panicking.
#[deprecated(
    since = "0.6.0",
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
