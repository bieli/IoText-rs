use crate::validators::Validators;
use crate::{IoTextDataRow, IoTextError, MetricDataItem, MetricKind, MetricValueType};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::trace;

/// Kept for source compatibility; the builder now reports [`IoTextError`].
#[deprecated(since = "1.0.0", note = "use `iotext_rs::IoTextError`")]
pub type IoTextBuilderError = IoTextError;

/// Builds an [`IoTextDataRow`]. Setter methods never panic: the first validation
/// error is remembered and returned by [`IoTextBuilder::build`].
///
/// ```
/// use iotext_rs::builder::IoTextBuilder;
/// use rust_decimal::Decimal;
///
/// let row = IoTextBuilder::new()
///     .timestamp(3900237526042)
///     .device_id("DEV_NAME_002")
///     .add_metric("count", 7)
///     .add_metric("temp", Decimal::new(1207, 2))
///     .add_metric("ok", true)
///     .add_metric("status", "in_progress")
///     .build(true)?;
/// assert_eq!(
///     row.to_string(),
///     "t|3900237526042,d|DEV_NAME_002,m|count=i:7,m|temp=d:12.07,m|ok=b:1,m|status=t:in_progress,c|8E25"
/// );
///
/// assert!(IoTextBuilder::new().timestamp(1).device_id("bad,id").build(false).is_err());
/// # Ok::<(), iotext_rs::IoTextError>(())
/// ```
#[derive(Debug, Default)]
pub struct IoTextBuilder {
    timestamp: Option<u64>,
    device_id: Option<String>,
    metrics: Vec<MetricDataItem>,
    error: Option<IoTextError>,
}

impl IoTextBuilder {
    #[must_use]
    /// Creates an empty builder; `timestamp` and `device_id` must be set before [`build`](Self::build).
    pub fn new() -> Self {
        Self::default()
    }

    fn record(&mut self, err: IoTextError) {
        trace!("IoTextBuilder - recorded error: {err}");
        self.error.get_or_insert(err);
    }

    fn push_metric(&mut self, name: &str, value: MetricValueType) {
        let validated = Validators::validate_metric_name_str(name)
            .map_err(IoTextError::from)
            .and_then(|name| {
                if let MetricValueType::TextItemType(text) = &value {
                    Validators::validate_text_value_str(text)?;
                }
                Ok(name)
            });
        match validated {
            Ok(name) => self.metrics.push(MetricDataItem {
                name: name.to_string(),
                value,
            }),
            Err(err) => self.record(err),
        }
    }

    #[must_use]
    /// Sets the UNIX timestamp in milliseconds.
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        trace!("IoTextBuilder::timestamp() - timestamp: {timestamp}");
        self.timestamp = Some(timestamp);
        self
    }

    /// Sets the timestamp to the current system time (requires the `std` feature).
    #[cfg(feature = "std")]
    #[must_use]
    pub fn current_timestamp(mut self) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .and_then(|d| u64::try_from(d.as_millis()).ok());
        trace!("IoTextBuilder::current_timestamp() - {now:?}");
        match now {
            Some(ts) => self.timestamp = Some(ts),
            None => self.record(IoTextError::InvalidSystemTime),
        }
        self
    }

    #[must_use]
    /// Sets the device id; an invalid id is reported by [`build`](Self::build).
    pub fn device_id(mut self, device_id: &str) -> Self {
        trace!("IoTextBuilder::device_id() - device_id: {device_id}");
        match Validators::validate_device_id_str(device_id) {
            Ok(value) => self.device_id = Some(value.to_string()),
            Err(err) => self.record(err.into()),
        }
        self
    }

    /// Adds a metric; accepts `i64`/`i32`/`u32`, `bool`, `Decimal`, `String`/`&str`
    /// or a [`MetricValueType`].
    #[must_use]
    pub fn add_metric(mut self, name: &str, value: impl Into<MetricValueType>) -> Self {
        let value = value.into();
        trace!("IoTextBuilder::add_metric - name: {name}, value: {value}");
        self.push_metric(name, value);
        self
    }

    /// Adds a metric from a raw protocol value of the given kind, e.g. `("temp", MetricKind::Decimal, "12.07")`.
    #[must_use]
    pub fn add_metric_of_kind(mut self, name: &str, kind: MetricKind, value: &str) -> Self {
        trace!("IoTextBuilder::add_metric_of_kind - name: {name}, kind: {kind}, value: {value}");
        match MetricValueType::from_kind_and_value(kind, value) {
            Ok(value) => self.push_metric(name, value),
            Err(err) => self.record(err),
        }
        self
    }

    /// Adds a metric from a raw protocol value and data type code (`i`, `b`, `d`, `t`).
    #[must_use]
    pub fn add_metric_as_str(mut self, name: &str, value: &str, metric_data_type: &str) -> Self {
        trace!(
            "IoTextBuilder::add_metric_as_str - name: {name}, value: {value}, metric_data_type: {metric_data_type}"
        );
        match Validators::validate_and_parse_metric_kind_str(metric_data_type, name) {
            Ok(kind) => self.add_metric_of_kind(name, kind, value),
            Err(err) => {
                self.record(err.into());
                self
            }
        }
    }

    /// # Errors
    /// Returns the first error recorded by a setter, or
    /// [`IoTextError::MissingTimestamp`] / [`IoTextError::MissingDeviceId`].
    pub fn build(self, add_crc: bool) -> Result<IoTextDataRow, IoTextError> {
        trace!("IoTextBuilder::build - add_crc: {add_crc}");
        if let Some(err) = self.error {
            return Err(err);
        }

        let mut row = IoTextDataRow {
            timestamp: self.timestamp.ok_or(IoTextError::MissingTimestamp)?,
            device_id: self.device_id.ok_or(IoTextError::MissingDeviceId)?,
            metrics: self.metrics,
            crc16: None,
        };
        if add_crc {
            row.crc16 = Some(row.compute_crc16());
        }
        Ok(row)
    }
}
