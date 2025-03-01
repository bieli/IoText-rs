use crate::{
    validators, IoTextData, IoTextDataRow, Item, ItemTypeEnum, ItemTypes, MetricDataItem,
    MetricValueType, SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
};
use chrono::Utc;
use log::{debug, error, trace};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoTextBuilderError {
    #[error("Timestamp is empty: {0} (expected u64 non-zero value)")]
    TimestampIsEmpty(Item),

    #[error("DeviceId is empty: {0} (expected minimum one ASCII character)")]
    DeviceIdIsEmpty(String),
}

#[derive(Debug, Default)]
pub struct IoTextBuilder {
    iotext_data_row: IoTextDataRow,
}

impl IoTextBuilder {
    pub fn new() -> Self {
        let mut row = IoTextDataRow::default();

        let s = row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(0);

        IoTextBuilder {
            iotext_data_row: row,
        }
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        trace!(
            "IoTextBuilder::timestamp() - timestamp: {:?}",
            ItemTypeEnum::TimeUnixMilis(timestamp)
        );
        let s = self.iotext_data_row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(timestamp);
        self
    }

    pub fn current_timestamp(mut self) -> Self {
        let current_timestamp = Utc::now().timestamp_millis() as u64;
        trace!(
            "IoTextBuilder::timestamp() - current_timestamp: {:?}",
            ItemTypeEnum::TimeUnixMilis(current_timestamp)
        );
        let s = self.iotext_data_row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(current_timestamp);
        self
    }

    pub fn device_id(mut self, device_id: &str) -> Self {
        trace!("IoTextBuilder::device_id() - device_id: {}", device_id);

        let parsed_value = match validators::Validators::validate_device_id_str(device_id) {
            Ok(value) => value,
            Err(err) => {
                let err_msg = format!("IoTextBuilder::device_id() - err: {err}");
                error!("{}", err_msg);
                panic!("{}", err_msg)
            }
        };

        debug!(
            "IoTextBuilder::device_id() - device_id: {:?}",
            ItemTypeEnum::DeviceId(String::from(parsed_value))
        );

        let s = self.iotext_data_row.device_id_mut();
        s.value = ItemTypeEnum::DeviceId(String::from(parsed_value));
        self
    }

    pub fn add_metric(mut self, name: &str, metric_data_type_value: MetricValueType) -> Self {
        trace!(
            "IoTextBuilder::add_metric - name: {}, metric_data_type_value: {}",
            name,
            metric_data_type_value
        );

        let parsed_metric_name = match validators::Validators::validate_metric_name_str(name) {
            Ok(value) => value,
            Err(err) => {
                error!("{err}");
                panic!("{}", err)
            }
        };

        let metrics = self.iotext_data_row.metrics_mut();
        metrics.get_or_insert(vec![]).push(MetricDataItem {
            name: parsed_metric_name.to_string(),
            value: metric_data_type_value,
        });
        self
    }

    pub fn add_metric_as_str(mut self, name: &str, value: &str, metric_data_type: &str) -> Self {
        trace!(
            "IoTextBuilder::add_metric_as_str - name: {}, value: {}, metric_data_type: {}",
            name,
            value,
            metric_data_type
        );

        let parsed_metric_name = match validators::Validators::validate_metric_name_str(name) {
            Ok(value) => value,
            Err(err) => {
                error!("{err}");
                panic!("{}", err)
            }
        };

        // Extract the metric value before modifying metrics
        let metric_value = self
            .iotext_data_row
            .extract_metric_value_type(metric_data_type, value);

        let metrics = self.iotext_data_row.metrics_mut();
        metrics.get_or_insert(vec![]).push(MetricDataItem {
            name: parsed_metric_name.to_string(),
            value: metric_value,
        });
        self
    }

    // Build the IoText data structure
    pub fn build(self, add_crc: bool) -> Result<IoTextDataRow, IoTextBuilderError> {
        trace!("IoTextBuilder::build - add_crc: {:?}", add_crc);

        trace!("IoTextBuilder::build - validating timestamp...");

        let default_timestamp_value =
            ItemTypes::TIMESTAMP_MILIS.to_string() + SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP + "0";
        if self
            .iotext_data_row
            .timestamp
            .value
            .to_string()
            .eq(default_timestamp_value.as_str())
        {
            let err_msg = "IoTextBuilder::build() - 'timestamp' is required!";
            error!("{err_msg}");
            return Err(IoTextBuilderError::TimestampIsEmpty(
                self.iotext_data_row.timestamp,
            ));
        }

        trace!("IoTextBuilder::build - validating device_id...");

        let default_device_id_value =
            ItemTypes::DEVICE_ID.to_string() + SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP;
        if self
            .iotext_data_row
            .device_id
            .value
            .to_string()
            .eq(default_device_id_value.as_str())
        {
            let err_msg = "IoTextBuilder::build() - 'device_id' is required!";
            error!("{err_msg}");
            return Err(IoTextBuilderError::DeviceIdIsEmpty(
                self.iotext_data_row.device_id.value.to_string(),
            ));
        }

        if add_crc {
            trace!("IoTextBuilder::build - processing add_crc...");

            //TODO: need to add more time & compute optimal solution here
            let data_obj = IoTextDataRow::default();
            let str_from_iotext_data: String =
                data_obj.dump_iotext_to_str(&self.iotext_data_row, add_crc);

            trace!(
                "IoTextBuilder::build - add_crc prepared: {:?}",
                str_from_iotext_data
            );

            return Ok(data_obj.parse_iotext_str(&str_from_iotext_data));
        }

        Ok(self.iotext_data_row)
    }
}
