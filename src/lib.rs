use rust_decimal::prelude::*;
use std::fmt::Display;
use std::*;

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
    //TODO: pub const HEALTH_CHECK: &'static str = "h";
}

#[derive(Debug, PartialEq)]
pub enum MetricValueType {
    IntegerItemType(i64),
    BoolItemType(bool),
    DecimalItemType(Decimal),
    TextItemType(String),
}

impl Default for MetricValueType {
    fn default() -> Self {
        MetricValueType::TextItemType("".to_string())
    }
}

impl Display for MetricValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetricValueType::IntegerItemType(value) => write!(f, "i:{value:?}"),
            MetricValueType::BoolItemType(value) => {
                write!(f, "b:{}", if value.eq(&true) { "1" } else { "0" })
            }
            MetricValueType::DecimalItemType(value) => write!(f, "d:{value:?}"),
            MetricValueType::TextItemType(value) => write!(f, "t:{value}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ItemTypeEnum {
    TimeUnixMilis(u64),
    DeviceId(String),
}

impl Display for ItemTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypeEnum::TimeUnixMilis(value) => write!(f, "t|{:?}", value),
            ItemTypeEnum::DeviceId(value) => write!(f, "d|{}", value),
        }
    }
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        ItemTypeEnum::DeviceId("".to_string())
    }
}

#[derive(Debug, Default)]
pub struct MetricDataItem {
    name: String,
    value: MetricValueType,
}

impl Display for MetricDataItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MetricDataItem { name, value } = self;
        write!(f, "m|{name}={value}")
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Item {
    pub value: ItemTypeEnum,
}

#[derive(Debug, Default)]
pub struct IoTextDataRow {
    pub timestamp: Item,
    pub device_id: Item,
    pub metrics: Option<Vec<MetricDataItem>>,
}

pub trait IoTextData {
    fn extract_metric_value_type(
        &self,
        metric_data_type: &str,
        metric_data_value: &str,
    ) -> MetricValueType;
    fn parse_iotext_str(&self, data_row: &str) -> IoTextDataRow;
    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow) -> String;
}

impl IoTextDataRow {
    // Immutable access.
    pub fn get_timestamp(&self) -> &ItemTypeEnum {
        &self.timestamp.value
    }
    #[must_use]
    pub fn get_device_id(&self) -> &ItemTypeEnum {
        &self.device_id.value
    }

    pub fn get_metrics(&self) -> &Option<Vec<MetricDataItem>> {
        &self.metrics
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
}

impl IoTextData for IoTextDataRow {
    fn extract_metric_value_type(
        &self,
        metric_data_type: &str,
        metric_data_value: &str,
    ) -> MetricValueType {
        match metric_data_type {
            MetricDataTypes::INTEGER => {
                let Ok(value) = metric_data_value.parse::<i64>() else {
                    todo!()
                };
                // println!(
                //    "\t\t\tIntegerItemType: {:?}",
                //    MetricValueType::IntegerItemType(value)
                //);
                MetricValueType::IntegerItemType(value)
            }
            MetricDataTypes::BOOL => {
                let value = match metric_data_value {
                    "1" => true,
                    "0" => false,
                    _ => todo!(),
                };
                //println!(
                //    "\t\t\tBoolItemType: {:?}",
                //   MetricValueType::BoolItemType(value)
                //);
                MetricValueType::BoolItemType(value)
            }
            MetricDataTypes::TEXT => {
                //println!(
                //    "\t\t\tBoolItemType: {:?}",
                //    MetricValueType::TextItemType(metric_data_value.to_string())
                //);
                MetricValueType::TextItemType(metric_data_value.to_string())
            }
            MetricDataTypes::DECIMAL => {
                //println!(
                //    "\t\t\tDecimalItemType: {:?}",
                //    MetricValueType::DecimalItemType(Decimal::from_str(metric_data_value).unwrap())
                //);
                MetricValueType::DecimalItemType(Decimal::from_str(metric_data_value).unwrap())
            }
            _ => MetricValueType::TextItemType(String::new()),
        }
    }

    fn parse_iotext_str(&self, data_row: &str) -> IoTextDataRow {
        let mut iotext_data_row = IoTextDataRow::default();
        let item_parts: Vec<&str> = data_row.split(',').collect();

        for part in item_parts {
            //println!("part: {}", part);
            let item_part: Vec<&str> = part.split('|').collect();
            //println!("item_part: {:?}", item_part);
            let item_type_tmp: &str = item_part[0];
            if item_type_tmp.eq(ItemTypes::METRIC_ITEM) {
                //println!("\tmetric: {}", item_part[1]);
                let metric_parts: Vec<&str> = item_part[1].split('=').collect();
                //println!("\tmetric_parts: {:?}", metric_parts);
                let metric_parts_values: Vec<&str> = metric_parts[1].split(':').collect();
                //println!("\t\tmetric_parts_values: {:?}", metric_parts_values);

                let metrics = iotext_data_row.metrics_mut();
                metrics.get_or_insert(vec![]).push(MetricDataItem {
                    name: metric_parts[0].to_string(),
                    value: self
                        .extract_metric_value_type(metric_parts_values[0], metric_parts_values[1]),
                });
            } else {
                match item_part.first().unwrap().to_owned() {
                    ItemTypes::TIMESTAMP_MILIS => {
                        let Ok(value) = item_part[1].parse::<u64>() else {
                            todo!()
                        };
                        //println!(
                        //    "\t\t\tTIMESTAMP_MILIS: {:?}",
                        //    ItemTypeEnum::TimeUnixMilis(value)
                        //);
                        {
                            let s = iotext_data_row.timestamp_mut();
                            s.value = ItemTypeEnum::TimeUnixMilis(value);
                        }
                    }
                    ItemTypes::DEVICE_ID => {
                        //println!(
                        //    "\t\t\tDEVICE_ID: {:?}",
                        //    ItemTypeEnum::DeviceId(String::from(item_part[1]))
                        //);

                        {
                            let s = iotext_data_row.device_id_mut();
                            s.value = ItemTypeEnum::DeviceId(String::from(item_part[1]));
                        }
                    }
                    ItemTypes::METRIC_ITEM => {
                        //println!("\t\t\tMETRIC_ITEM: {}", String::from(item_part[1]));
                    }
                    _val => {
                        //println!("\t\t\t OTHER: {:?}", val);
                    }
                }
                //println!("\t\tcontext: {}", item_part[1])
            }
        }
        iotext_data_row
    }

    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow) -> String {
        let metrics_as_str: &mut Vec<String> = &mut vec![];

        match &iotext_data_row.get_metrics() {
            Some(metrics) => {
                for metric in metrics {
                    metrics_as_str.push(metric.to_string());
                }
            }
            None => (),
        }

        format!(
            "{},{},{}",
            iotext_data_row.get_timestamp(),
            iotext_data_row.get_device_id(),
            metrics_as_str
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
        .as_str()
        .trim_end_matches(',')
        .to_string()
    }
}
