use std::*;

use rust_decimal::prelude::*;

#[derive(Debug, Default)]
struct MetricDataTypes {}

impl MetricDataTypes {
    const INTEGER: &str = "i";
    const BOOL: &str = "b";
    const DECIMAL: &str = "d";
    const TEXT: &str = "t";
}

#[derive(Debug, Default)]
struct ItemTypes {}

impl ItemTypes {
    const TIMESTAMP_MILIS: &str = "t";
    const DEVICE_ID: &str = "d";
    const METRIC_ITEM: &str = "m";
    //TODO: const HEALTH_CHECK: &str = "h";
}

#[derive(Debug)]
enum MetricValueType {
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

#[derive(Debug)]
enum ItemTypeEnum {
    TimeUnixMilis(u64),
    DeviceId(String),
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        ItemTypeEnum::DeviceId("".to_string())
    }
}

#[derive(Debug, Default)]
struct MetricDataItem {
    name: String,
    value: MetricValueType,
}
#[derive(Debug, Default)]
struct Item {
    value: ItemTypeEnum,
}

#[derive(Debug, Default)]
struct IotextDataRow {
    timestamp: Item,
    device_id: Item,
    metrics: Option<Vec<MetricDataItem>>,
}

impl IotextDataRow {
    // Immutable access.
    /*
    fn get_timestamp(&self) -> &ItemTypeEnum {
        &self.timestamp.value
    }
    fn get_device_id(&self) -> &ItemTypeEnum {
        &self.device_id.value
    }
    fn get_metrics(&self) -> &Option<Vec<MetricDataItem>> {
        &self.metrics
    }
    */

    // Mutable access.
    fn timestamp_mut(&mut self) -> &mut Item {
        &mut self.timestamp
    }
    fn device_id_mut(&mut self) -> &mut Item {
        &mut self.device_id
    }
    fn metrics_mut(&mut self) -> &mut Option<Vec<MetricDataItem>> {
        &mut self.metrics
    }
}

const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_level1=i:42,m|light_on=b:1,m|bulb_on=b:0,m|msg_machine_01=t:hello,m|wind_speed=d:1234.5678";

fn extract_metric_value_type(metric_data_type: &str, metric_data_value: &str) -> MetricValueType {
    match metric_data_type {
        MetricDataTypes::INTEGER => {
            let value = match metric_data_value.parse::<i64>() {
                Ok(number) => number,
                Err(_) => todo!(),
            };
            println!(
                "\t\t\tIntegerItemType: {:?}",
                MetricValueType::IntegerItemType(value)
            );
            MetricValueType::IntegerItemType(value)
        }
        MetricDataTypes::BOOL => {
            let value = match metric_data_value {
                "1" => true,
                "0" => false,
                _ => todo!(),
            };
            println!(
                "\t\t\tBoolItemType: {:?}",
                MetricValueType::BoolItemType(value)
            );
            MetricValueType::BoolItemType(value)
        }
        MetricDataTypes::TEXT => {
            println!(
                "\t\t\tBoolItemType: {:?}",
                MetricValueType::TextItemType(metric_data_value.to_string())
            );
            MetricValueType::TextItemType(metric_data_value.to_string())
        }
        MetricDataTypes::DECIMAL => {
            println!(
                "\t\t\tDecimalItemType: {:?}",
                MetricValueType::DecimalItemType(Decimal::from_str(metric_data_value).unwrap())
            );
            MetricValueType::DecimalItemType(Decimal::from_str(metric_data_value).unwrap())
        }
        _ => MetricValueType::TextItemType("".to_string()),
    }
}

fn parse_iotext_str(data_row: &str) -> IotextDataRow {
    let mut iotext_data_row = IotextDataRow::default();
    let item_parts: Vec<&str> = data_row.split(',').collect();

    for part in item_parts {
        println!("part: {}", part);
        let item_part: Vec<&str> = part.split('|').collect();
        println!("item_part: {:?}", item_part);
        let item_type_tmp: &str = item_part[0];
        if item_type_tmp.eq(ItemTypes::METRIC_ITEM) {
            println!("\tmetric: {}", item_part[1]);
            let metric_parts: Vec<&str> = item_part[1].split('=').collect();
            println!("\tmetric_parts: {:?}", metric_parts);
            let metric_parts_values: Vec<&str> = metric_parts[1].split(':').collect();
            println!("\t\tmetric_parts_values: {:?}", metric_parts_values);

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
                    println!(
                        "\t\t\tTIMESTAMP_MILIS: {:?}",
                        ItemTypeEnum::TimeUnixMilis(value)
                    );

                    {
                        let s = iotext_data_row.timestamp_mut();
                        s.value = ItemTypeEnum::TimeUnixMilis(value);
                    }
                }
                ItemTypes::DEVICE_ID => {
                    println!(
                        "\t\t\tDEVICE_ID: {:?}",
                        ItemTypeEnum::DeviceId(String::from(item_part[1]))
                    );

                    {
                        let s = iotext_data_row.device_id_mut();
                        s.value = ItemTypeEnum::DeviceId(String::from(item_part[1]));
                    }
                }
                ItemTypes::METRIC_ITEM => {
                    println!("\t\t\tMETRIC_ITEM: {}", String::from(item_part[1]));
                }
                val => {
                    println!("\t\t\t OTHER: {:?}", val);
                }
            }
            println!("\t\tcontext: {}", item_part[1])
        }
    }
    iotext_data_row
}

fn main() {
    let iotext_data_row = parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row)
}

/*
STDOUT:

part: t|3900237526042
item_part: ["t", "3900237526042"]
                        TIMESTAMP_MILIS: TimeUnixMilis(3900237526042)
                context: 3900237526042
part: d|device_name_001
item_part: ["d", "device_name_001"]
                        DEVICE_ID: DeviceId("device_name_001")
                context: device_name_001
part: m|val_water_level1=i:42
item_part: ["m", "val_water_level1=i:42"]
        metric: val_water_level1=i:42
        metric_parts: ["val_water_level1", "i:42"]
                metric_parts_values: ["i", "42"]
                        IntegerItemType: IntegerItemType(42)
part: m|light_on=b:1
item_part: ["m", "light_on=b:1"]
        metric: light_on=b:1
        metric_parts: ["light_on", "b:1"]
                metric_parts_values: ["b", "1"]
                        BoolItemType: BoolItemType(true)
part: m|bulb_on=b:0
item_part: ["m", "bulb_on=b:0"]
        metric: bulb_on=b:0
        metric_parts: ["bulb_on", "b:0"]
                metric_parts_values: ["b", "0"]
                        BoolItemType: BoolItemType(false)
part: m|msg_machine_01=t:hello
item_part: ["m", "msg_machine_01=t:hello"]
        metric: msg_machine_01=t:hello
        metric_parts: ["msg_machine_01", "t:hello"]
                metric_parts_values: ["t", "hello"]
                        BoolItemType: TextItemType("hello")
part: m|wind_speed=d:1234.5678
item_part: ["m", "wind_speed=d:1234.5678"]
        metric: wind_speed=d:1234.5678
        metric_parts: ["wind_speed", "d:1234.5678"]
                metric_parts_values: ["d", "1234.5678"]
                        DecimalItemType: DecimalItemType(1234.5678)
iotext_data_row.timestamp_mut: IotextDataRow {
    timestamp: Item {
        value: TimeUnixMilis(
            3900237526042,
        ),
    },
    device_id: Item {
        value: DeviceId(
            "device_name_001",
        ),
    },
    metrics: Some(
        [
            MetricDataItem {
                name: "val_water_level1",
                value: IntegerItemType(
                    42,
                ),
            },
            MetricDataItem {
                name: "light_on",
                value: BoolItemType(
                    true,
                ),
            },
            MetricDataItem {
                name: "bulb_on",
                value: BoolItemType(
                    false,
                ),
            },
            MetricDataItem {
                name: "msg_machine_01",
                value: TextItemType(
                    "hello",
                ),
            },
            MetricDataItem {
                name: "wind_speed",
                value: DecimalItemType(
                    1234.5678,
                ),
            },
        ],
    ),
}



*/