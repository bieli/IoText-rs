use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::*;

pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

#[derive(Debug, Default)]
pub struct MetricDataTypes {}

impl MetricDataTypes {
    pub const INTEGER: &str = "i";
    pub const BOOL: &str = "b";
    pub const DECIMAL: &str = "d";
    pub const TEXT: &str = "t";
}

#[derive(Debug, Default)]
pub struct ItemTypes {}

impl ItemTypes {
    pub const TIMESTAMP_MILIS: &str = "t";
    pub const DEVICE_ID: &str = "d";
    pub const METRIC_ITEM: &str = "m";
    //TODO: pub const HEALTH_CHECK: &str = "h";
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

use std::fmt::Display;

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
#[derive(Debug, Default, PartialEq)]
pub struct Item {
    pub value: ItemTypeEnum,
}

#[derive(Debug, Default)]
pub struct IotextDataRow {
    pub timestamp: Item,
    pub device_id: Item,
    pub metrics: Option<Vec<MetricDataItem>>,
}

impl IotextDataRow {
    // Immutable access.
    fn get_timestamp(&self) -> &ItemTypeEnum {
        &self.timestamp.value
    }
    fn get_device_id(&self) -> String {
        self.device_id.value.to_string()
    }
    fn get_metrics(&self) -> &Option<Vec<MetricDataItem>> {
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

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn extract_metric_value_type(
    metric_data_type: &str,
    metric_data_value: &str,
) -> MetricValueType {
    match metric_data_type {
        MetricDataTypes::INTEGER => {
            let value = match metric_data_value.parse::<i64>() {
                Ok(number) => number,
                Err(_) => todo!(),
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
        _ => MetricValueType::TextItemType("".to_string()),
    }
}

pub fn parse_iotext_str(data_row: &str) -> IotextDataRow {
    let mut iotext_data_row = IotextDataRow::default();
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
                value: extract_metric_value_type(metric_parts_values[0], metric_parts_values[1]),
            });
        } else {
            match item_part[0] {
                ItemTypes::TIMESTAMP_MILIS => {
                    let value = match item_part[1].parse::<u64>() {
                        Ok(number) => number,
                        Err(_) => todo!(),
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
                val => {
                    //println!("\t\t\t OTHER: {:?}", val);
                }
            }
            //println!("\t\tcontext: {}", item_part[1])
        }
    }
    iotext_data_row
}

pub fn dump_iotext_to_str(iotext_data_row: &IotextDataRow) -> String {
    // TODO: add dump for metrics values
    format!(
        "{},{}",
        iotext_data_row.get_timestamp(),
        iotext_data_row.get_device_id()
    )
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_metric_value_type_integer() {
        const METRIC_DATA_TYPE: &str = "i";
        const METRIC_DATA_VALUE: &str = "42";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::IntegerItemType(42));
    }

    #[test]
    fn test_extract_metric_value_type_decimal() {
        const METRIC_DATA_TYPE: &str = "d";
        const METRIC_DATA_VALUE: &str = "42.123";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::DecimalItemType(Decimal::from_str(METRIC_DATA_VALUE).unwrap())
        );
    }

    #[test]
    fn test_extract_metric_value_type_bool_true() {
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "1";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(true));
    }

    #[test]
    fn test_extract_metric_value_type_bool_false() {
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "0";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(false));
    }

    #[test]
    fn test_extract_metric_value_type_text() {
        const METRIC_DATA_TYPE: &str = "t";
        const METRIC_DATA_VALUE: &str = "hello";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::TextItemType(METRIC_DATA_VALUE.to_string())
        );
    }

    #[test]
    fn test_dump_iotext_to_str_without_measurements() {
        let expected: String = "t|3900237526042,d|device_name_001".to_string();
        let mut iotext_data_row = IotextDataRow::default();

        let s = iotext_data_row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(3900237526042);

        let s = iotext_data_row.device_id_mut();
        s.value = ItemTypeEnum::DeviceId(String::from("device_name_001".to_string()));

        let result: String = dump_iotext_to_str(&iotext_data_row);

        assert_eq!(result, expected);
    }
}