use rust_decimal::prelude::*;
use std::fmt::Display;
use std::*;

pub mod tools;
pub use tools::CRC16_POLY_DEFAULT;
pub use tools::*;

pub mod validators;

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
            MetricValueType::IntegerItemType(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value:?}",
                    MetricDataTypes::INTEGER,
                    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP
                )
            ),
            MetricValueType::BoolItemType(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{}",
                    MetricDataTypes::BOOL,
                    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP,
                    if value.eq(&true) { "1" } else { "0" }
                )
            ),
            MetricValueType::DecimalItemType(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value:?}",
                    MetricDataTypes::DECIMAL,
                    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP
                )
            ),
            MetricValueType::TextItemType(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value}",
                    MetricDataTypes::TEXT,
                    SPECIAL_CHAR_DATA_TYPES_AND_VALUES_SEP
                )
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ItemTypeEnum {
    TimeUnixMilis(u64),
    DeviceId(String),
    Crc(String),
}

impl Display for ItemTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypeEnum::TimeUnixMilis(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value:?}",
                    ItemTypes::TIMESTAMP_MILIS,
                    SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP
                )
            ),
            ItemTypeEnum::DeviceId(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value}",
                    ItemTypes::DEVICE_ID,
                    SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP
                )
            ),
            ItemTypeEnum::Crc(value) => write!(
                f,
                "{}",
                format!(
                    "{}{}{value}",
                    ItemTypes::CRC,
                    SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP
                )
            ),
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
        write!(
            f,
            "{}",
            format!(
                "{}{}{name}{}{value}",
                ItemTypes::METRIC_ITEM,
                SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
                SPECIAL_CHAR_SPLIT_DATA_ITEM_SEP
            )
        )
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Item {
    pub value: ItemTypeEnum,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

#[derive(Debug, Default)]
pub struct IoTextDataRow {
    pub timestamp: Item,
    pub device_id: Item,
    pub metrics: Option<Vec<MetricDataItem>>,
    pub crc16: Option<Item>,
}

pub trait IoTextData {
    fn extract_metric_value_type(
        &self,
        metric_data_type: &str,
        metric_data_value: &str,
    ) -> MetricValueType;
    fn parse_iotext_str(&self, data_row: &str) -> IoTextDataRow;
    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow, add_crc16: bool) -> String;
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
                    panic!("Error with parsing INTEGER (i64) metric data value: '{metric_data_value}'!");
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
                    _ => panic!(
                        "Error with parsing BOOL metric data value: '{metric_data_value}' (expected '0' or '1')!"
                    )
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
            //TODO: validate this concept - maybe we nee to have Error + panic here with unknown metric type
            _ => MetricValueType::TextItemType(String::new()),
        }
    }

    fn parse_iotext_str(&self, data_row: &str) -> IoTextDataRow {
        let mut iotext_data_row: IoTextDataRow = IoTextDataRow::default();
        let item_parts: Vec<&str> = data_row.split(',').collect();

        for part in item_parts {
            //println!("part: {}", part);
            let item_part: Vec<&str> = part.split('|').collect();
            //println!("item_part: {:?}", item_part);
            let item_type_tmp: &str = item_part[0];
            if item_type_tmp.eq(ItemTypes::METRIC_ITEM) {
                // println!("\tmetric: {}", item_part[1]);
                let metric_parts: Vec<&str> = item_part[1].split('=').collect();
                // println!("\tmetric_parts: {:?}", metric_parts);
                let metric_parts_values: Vec<&str> = metric_parts[1].split(':').collect();
                // println!("\t\tmetric_parts_values: {:?}", metric_parts_values);

                let parsed_metric_name =
                    match validators::Validators::validate_metric_name_str(metric_parts[0]) {
                        Ok(value) => value,
                        Err(err) => panic!("{}", err),
                    };
                let metric_data_type = metric_parts_values[0];
                let metric_data_value = metric_parts_values[1];

                let parsed_metric_data_type =
                    match validators::Validators::validate_metric_data_type_str(
                        metric_data_type,
                        parsed_metric_name,
                    ) {
                        Ok(value) => value,
                        Err(err) => panic!("{}", err),
                    };

                let metrics = iotext_data_row.metrics_mut();
                metrics.get_or_insert(vec![]).push(MetricDataItem {
                    name: parsed_metric_name.to_string(),
                    value: self
                        .extract_metric_value_type(parsed_metric_data_type, metric_data_value),
                });
            } else {
                match item_part.first().unwrap().to_owned() {
                    ItemTypes::TIMESTAMP_MILIS => {
                        let parsed_value =
                            match validators::Validators::validate_and_parse_timestamp_str(
                                item_part[1],
                            ) {
                                Ok(value) => value,
                                Err(err) => panic!("{}", err),
                            };
                        //println!(
                        //    "\t\t\tTIMESTAMP_MILIS: {:?}",
                        //    ItemTypeEnum::TimeUnixMilis(value)
                        //);
                        {
                            let s = iotext_data_row.timestamp_mut();
                            s.value = ItemTypeEnum::TimeUnixMilis(parsed_value);
                        }
                    }
                    ItemTypes::DEVICE_ID => {
                        // match Validators::validate_excluded_special_chars(item_part[1]) {
                        //     Err(err) => panic!("{}", err),
                        //     _ => {}
                        // }

                        let parsed_value =
                            match validators::Validators::validate_device_id_str(item_part[1]) {
                                Ok(value) => value,
                                Err(err) => panic!("{}", err),
                            };

                        //println!(
                        //    "\t\t\tDEVICE_ID: {:?}",
                        //    ItemTypeEnum::DeviceId(String::from(item_part[1]))
                        //);

                        {
                            let s = iotext_data_row.device_id_mut();
                            s.value = ItemTypeEnum::DeviceId(String::from(parsed_value));
                        }
                    }
                    ItemTypes::METRIC_ITEM => {
                        //println!("\t\t\tMETRIC_ITEM: {}", String::from(item_part[1]));
                    }
                    ItemTypes::CRC => {
                        let parsed_value =
                            match validators::Validators::validate_crc_str(item_part[1]) {
                                Ok(value) => value,
                                Err(err) => panic!("{}", err),
                            };
                        // println!("\t\t\tCRC: {}", String::from(item_part[1]));

                        //let &mut crc16_mut = iotext_data_row.crc16_mut();

                        // let &mut optional_crc16_mut = iotext_data_row.crc16_mut();
                        // optional_crc16_mut.unwrap().value = ItemTypeEnum::Crc(
                        //             item_part[1].to_string()
                        // )

                        //let optional_crc16_mut = iotext_data_row.crc16_mut();
                        //optional_crc16_mut.as_mut().unwrap().value = ItemTypeEnum::Crc(
                        //    item_part[1].to_string()
                        //)

                        // let optional_crc16_mut = iotext_data_row.crc16_mut();
                        // match optional_crc16_mut.as_mut() {
                        //     Some(item) => item.value =
                        //         ItemTypeEnum::Crc(item_part[1].to_string()),
                        //     None => {}
                        // }

                        // let optional_crc16_mut = iotext_data_row.crc16_mut();
                        // optional_crc16_mut.as_mut().unwrap().value =
                        // Item {
                        //     value: ItemTypeEnum::Crc(item_part[1].to_string())
                        // }.value

                        let optional_crc16_mut = iotext_data_row.crc16_mut();
                        optional_crc16_mut.get_or_insert_with(Item::default).value =
                            ItemTypeEnum::Crc(parsed_value.to_string());
                    }
                    _val => {
                        //println!("\t\t\t OTHER: metric_parts{:?}", val);
                    }
                }
                //println!("\t\tcontext: {}", item_part[1])
            }
        }
        iotext_data_row
    }

    fn dump_iotext_to_str(&self, iotext_data_row: &IoTextDataRow, add_crc16: bool) -> String {
        let metrics_as_str: &mut Vec<String> = &mut vec![];
        match &iotext_data_row.get_metrics() {
            Some(metrics) => {
                for metric in metrics {
                    metrics_as_str.push(metric.to_string());
                }
            }
            None => (),
        }

        let msg_val: String = format!(
            "{}{}{}{}{}",
            iotext_data_row.get_timestamp(),
            SPECIAL_CHAR_DATA_ITEMS_SEP,
            iotext_data_row.get_device_id(),
            SPECIAL_CHAR_DATA_ITEMS_SEP,
            metrics_as_str
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(SPECIAL_CHAR_DATA_ITEMS_SEP)
        );

        let crc16_element: String = if add_crc16 == true {
            format!(
                "{}{}{}{}",
                SPECIAL_CHAR_DATA_ITEMS_SEP,
                ItemTypes::CRC,
                SPECIAL_CHAR_DATA_TYPES_AND_CMD_CTX_SEP,
                //format!("{}", msg_val.as_str())
                Tools::crc16(msg_val.as_str(), CRC16_POLY_DEFAULT)
            )
        } else {
            "".to_string()
        };

        // let crc16_element: String = if add_crc16 {
        //     match &iotext_data_row.get_crc16() {
        //         Some(_) => {
        //             let crc16 = Tools::crc16("ABCD", CRC16_POLY_DEFAULT);
        //             //format!(",{}|{}", ItemTypes::CRC, val.to_string())
        //             format!(",{}|{}", ItemTypes::CRC, crc16)
        //         },
        //         None => "".to_string(),
        //     }
        // } else {
        //     "".to_string()
        // };

        msg_val
            .trim_end_matches(SPECIAL_CHAR_DATA_ITEMS_SEP)
            .to_string()
            + &crc16_element
    }
}
