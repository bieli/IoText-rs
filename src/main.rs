use std::*;

// COPY -> PASTE to https://play.rust-lang.org/ for check parser prototype

//use std::intrinsics::type_name::*;
//use std::num::ParseIntError;
//use rust_decimal::Decimal;
use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;

struct MetricDataTypes {}

impl MetricDataTypes {
    const INTEGER: &str = "i";
    const BOOL: &str = "b";
    const DECIMAL: &str = "d";
    const TEXT: &str = "t";
}

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

#[derive(Debug)]
enum ItemTypeEnum {
    TimeUnixMilis(u64),
    DeviceId(String),
}

//TODO: use in dedicated function
/*
struct MetricDataItem<'a> {
    data_type: ItemTypeEnum,
    name: &'a str,
    value: MetricValueType,
}

struct Item<'a> {
    kind: ItemTypes,
    name: &'a str,
    metric: Option<MetricDataItem<'a>>,
}

struct IotextDataRow<'a> {
    timestamp: Item<'a>,
    device_id: Item<'a>,
    metrics: Option<Vec<MetricDataItem<'a>>>,
}
*/

const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_level1=i:42,m|light_on=b:1,m|bulb_on=b:0,m|msg_machine_01=t:hello,m|wind_speed=d:1234.5678";

//fn parse_iotext_str<E>(data_row: &str) -> Result<IotextDataRow, E> {
fn parse_iotext_str(data_row: &str) -> () {
    let item_parts: Vec<&str> = data_row.split(',').collect();

    for part in item_parts {
        println!("part: {}", part);
        let item_part: Vec<&str> = part.split('|').collect();
        println!("item_part: {:?}", item_part);
        let item_type_tmp: &str = item_part[0];
        let item_type_metric: &str = "m";
        if item_type_tmp.eq(item_type_metric) {
            println!("\tmetric: {}", item_part[1]);
            let metric_parts: Vec<&str> = item_part[1].split('=').collect();
            println!("\tmetric_parts: {:?}", metric_parts);
            //let metric_name: String = item_parts[0].to_string();
            //println!("metric_name: {}", metric_name);
            let metric_parts_values: Vec<&str> = metric_parts[1].split(':').collect();
            println!("\t\tmetric_parts_values: {:?}", metric_parts_values);
            match metric_parts_values[0] {
                MetricDataTypes::INTEGER => {
                    let value = match metric_parts_values[1].parse::<i64>() {
                        Ok(number) => number,
                        Err(_) => todo!(),
                    };
                    println!(
                        "\t\t\tIntegerItemType: {:?}",
                        MetricValueType::IntegerItemType(value)
                    )
                }
                MetricDataTypes::BOOL => {
                    let value = match metric_parts_values[1] {
                        "1" => true,
                        "0" => false,
                        _ => todo!(),
                    };
                    println!(
                        "\t\t\tBoolItemType: {:?}",
                        MetricValueType::BoolItemType(value)
                    )
                }
                MetricDataTypes::TEXT => {
                    println!(
                        "\t\t\tBoolItemType: {:?}",
                        MetricValueType::TextItemType(metric_parts_values[1].to_string())
                    )
                }
                MetricDataTypes::DECIMAL => {
                    println!(
                        "\t\t\tDecimalItemType: {:?}",
                        MetricValueType::DecimalItemType(
                            Decimal::from_str(metric_parts_values[1]).unwrap()
                        )
                    )
                }

                _ => println!("\t\t\tother"),
            }
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
                    )
                }
                ItemTypes::DEVICE_ID => {
                    println!(
                        "\t\t\tDEVICE_ID: {:?}",
                        ItemTypeEnum::DeviceId(String::from(item_part[1]))
                    )
                }
                ItemTypes::METRIC_ITEM => {
                    println!("\t\t\tMETRIC_ITEM: {}", String::from(item_part[1]))
                }
                val => {
                    println!("\t\t\t OTHER: {:?}", val);
                    //print_type_of(val)
                }
            }
            println!("\t\tcontext: {}", item_part[1])
        }
        /*match item_part.split('=').collect() {
            [item, details] => {
                println!("{:?}", item);
                println!("{:?}", details);
            }
            _ => println!("rest"),
        }*/
    }
    //assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);
    //println!("{:?}", v);
    //MetricValueType::IntegerItemType(0)

    //Ok(IotextDataRow { timestamp: Item{}, device_id: Item{}, metrics: Null })
}

fn main() {
    parse_iotext_str(MSG_EXAMPLE)
}
