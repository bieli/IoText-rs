use crate::validators::Validators;
use crate::{IoTextDataRow, IoTextError, Item, ItemTypeEnum, MetricDataItem, MetricValueType};
use log::trace;
use std::time::{SystemTime, UNIX_EPOCH};

/// Kept for source compatibility; the builder now reports [`IoTextError`].
#[deprecated(since = "1.0.0", note = "use `iotext_rs::IoTextError`")]
pub type IoTextBuilderError = IoTextError;

/// Builds an [`IoTextDataRow`]. Setter methods never panic: the first validation
/// error is remembered and returned by [`IoTextBuilder::build`].
///
/// ```
/// use iotext_rs::{builder::IoTextBuilder, MetricValueType};
///
/// let row = IoTextBuilder::new()
///     .timestamp(3900237526042)
///     .device_id("DEV_NAME_002")
///     .add_metric("count", MetricValueType::IntegerItemType(7))
///     .add_metric_as_str("temp", "12.07", "d")
///     .build(true)?;
/// assert_eq!(row.to_string(), "t|3900237526042,d|DEV_NAME_002,m|count=i:7,m|temp=d:12.07,c|CC8C");
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
    pub fn new() -> Self {
        Self::default()
    }

    fn record(&mut self, err: IoTextError) {
        trace!("IoTextBuilder - recorded error: {err}");
        self.error.get_or_insert(err);
    }

    #[must_use]
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        trace!("IoTextBuilder::timestamp() - timestamp: {timestamp}");
        self.timestamp = Some(timestamp);
        self
    }

    #[must_use]
    pub fn current_timestamp(mut self) -> Self {
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
    pub fn device_id(mut self, device_id: &str) -> Self {
        trace!("IoTextBuilder::device_id() - device_id: {device_id}");
        match Validators::validate_device_id_str(device_id) {
            Ok(value) => self.device_id = Some(value.to_string()),
            Err(err) => self.record(err.into()),
        }
        self
    }

    #[must_use]
    pub fn add_metric(mut self, name: &str, metric_data_type_value: MetricValueType) -> Self {
        trace!("IoTextBuilder::add_metric - name: {name}, value: {metric_data_type_value}");
        let validated = Validators::validate_metric_name_str(name)
            .map_err(IoTextError::from)
            .and_then(|name| {
                if let MetricValueType::TextItemType(text) = &metric_data_type_value {
                    Validators::validate_text_value_str(text)?;
                }
                Ok(name)
            });
        match validated {
            Ok(name) => self.metrics.push(MetricDataItem {
                name: name.to_string(),
                value: metric_data_type_value,
            }),
            Err(err) => self.record(err),
        }
        self
    }

    #[must_use]
    pub fn add_metric_as_str(mut self, name: &str, value: &str, metric_data_type: &str) -> Self {
        trace!(
            "IoTextBuilder::add_metric_as_str - name: {name}, value: {value}, metric_data_type: {metric_data_type}"
        );
        let parsed = Validators::validate_metric_name_str(name)
            .map_err(IoTextError::from)
            .and_then(|name| {
                Validators::validate_metric_data_type_str(metric_data_type, name)?;
                Ok((
                    name,
                    MetricValueType::from_type_and_value(metric_data_type, value)?,
                ))
            });
        match parsed {
            Ok((name, value)) => self.metrics.push(MetricDataItem {
                name: name.to_string(),
                value,
            }),
            Err(err) => self.record(err),
        }
        self
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
            timestamp: Item {
                value: ItemTypeEnum::TimeUnixMilis(
                    self.timestamp.ok_or(IoTextError::MissingTimestamp)?,
                ),
            },
            device_id: Item {
                value: ItemTypeEnum::DeviceId(self.device_id.ok_or(IoTextError::MissingDeviceId)?),
            },
            metrics: if self.metrics.is_empty() {
                None
            } else {
                Some(self.metrics)
            },
            crc16: None,
        };

        if add_crc {
            row.crc16 = Some(Item {
                value: ItemTypeEnum::Crc(row.compute_crc16()),
            });
        }
        Ok(row)
    }
}
