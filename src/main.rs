use std::*;

// COPY -> PASTE to https://play.rust-lang.org/ for check parser prototype

//use std::intrinsics::type_name::*;
//use std::num::ParseIntError;
//use rust_decimal::Decimal;
use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;

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
    DeviceId(String)
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        ItemTypeEnum::DeviceId("".to_string())
    }
}



/*
//TODO: use in dedicated function
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
    timestamp: &'a Item<'a>,
    device_id: &'a Item<'a>,
    metrics: Option<Vec<&'a MetricDataItem<'a>>>,
}

impl<'a> IotextDataRow<'_> {
    fn set_timestamp(&mut self, timestamp: &'a Item) -> () {
        self.timestamp = timestamp
    }
    fn set_device_id(&mut self, device_id: &'a Item) -> () {
        self.device_id = device_id
    }
    fn add_metric(&mut self, metric: &'a MetricDataItem) -> () {
        self.metrics.as_ref().unwrap().push(metric)
    }
}
*/

#[derive(Debug, Default)]
struct MetricDataItem {
    //data_type: Box<ItemTypeEnum>,
    //data_type: ItemTypeEnum,
    name: String,
    value: MetricValueType,
}
#[derive(Debug, Default)]
struct Item {
    //kind: ItemTypes,
    value: ItemTypeEnum,
    //name: String,
    //metric: Option<MetricDataItem>,
}

#[derive(Debug, Default)]
struct IotextDataRow {
    timestamp: Item,
    device_id: Item,
    metrics: Option<Vec<MetricDataItem>>,
}

impl IotextDataRow {
    // Immutable access.
    fn get_timestamp(&self) -> &ItemTypeEnum {
        &self.timestamp.value
    }
    fn get_device_id(&self) -> &ItemTypeEnum {
        &self.device_id.value
    }
    fn get_metrics(&self) -> &Option<Vec<MetricDataItem>> {
        &self.metrics
    }

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



// use libc::*;
use core::ffi::c_ulonglong;

#[cfg(target_pointer_width = "64")]
pub type BnUlong = c_ulonglong;
#[cfg(target_pointer_width = "32")]
pub type BnUlong = c_uint;


use core::fmt::Error;

#[derive(Debug, Clone)]
pub struct ErrorParser(Vec<Error>);

/*
struct Aaa {
    name: &str,
}

impl Aaa {
    fn set_name(&self, &mut str: name) -> () {
        self.name = name
    }
}

sss = "aaaaaaa";
Aaa::set_name(sss);
*/

/*
#[derive(Default)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn name(&self) -> &str use std::*;

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
/*
//TODO: use in dedicated function
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
    timestamp: &'a Item<'a>,
    device_id: &'a Item<'a>,
    metrics: Option<Vec<&'a MetricDataItem<'a>>>,
}

impl<'a> IotextDataRow<'_> {
    fn set_timestamp(&mut self, timestamp: &'a Item) -> () {
        self.timestamp = timestamp
    }
    fn set_device_id(&mut self, device_id: &'a Item) -> () {
        self.device_id = device_id
    }
    fn add_metric(&mut self, metric: &'a MetricDataItem) -> () {
        self.metrics.as_ref().unwrap().push(metric)
    }
}
*/

// use libc::*;
use core::ffi::c_ulonglong;

#[cfg(target_pointer_width = "64")]
pub type BnUlong = c_ulonglong;
#[cfg(target_pointer_width = "32")]
pub type BnUlong = c_uint;


use core::fmt::Error;

#[derive(Debug, Clone)]
pub struct ErrorParser(Vec<Error>);

/*
struct Aaa {
    name: &str,
}

impl Aaa {
    fn set_name(&self, &mut str: name) -> () {
        self.name = name
    }
}

sss = "aaaaaaa";
Aaa::set_name(sss);
*/

/*
#[derive(Default)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u8 {
        self.age
    }
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

#[derive(Default)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Immutable access.
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }

    // Mutable access.
    fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }
    fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }
}{
        &self.name
    }
    pub fn age(&self) -> u8 {
        self.age
    }
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

#[derive(Default)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Immutable access.
    fn get_first_name(&self) -> &str {
        self.first_name.as_str()
    }
    fn get_last_name(&self) -> &str {
        self.last_name.as_str()
    }

    // Mutable access.
    fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }
    fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }
}

fn main() {
    let mut iotext_data_row = IotextDataRow::default();

    println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row.timestamp);
    println!("iotext_data_row.device_id_mut: {:#?}", iotext_data_row.device_id);
    

    //*iotext_data_row.timestamp_mut() = String::from("123123123");
    //*iotext_data_row.device_id_mut() = "device_name_001".to_string();

    //println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row.timestamp);
    //println!("iotext_data_row.device_id_mut: {:#?}", iotext_data_row.device_id);
    
    // Can't do this efficiently with getter/setter!
    {
        let s = iotext_data_row.timestamp_mut();
        //TODO: s.kind = ItemTypes::TIMESTAMP_MILIS;
        s.value = ItemTypeEnum::TimeUnixMilis(123123123);
        //s.name = "123123123123".to_string();
    }

    {
        let s = iotext_data_row.device_id_mut();
        s.value = ItemTypeEnum::DeviceId("DEVICE_ID_123".to_string());
        //s.name.truncate(2);
        //s.name.push('w');
    }


    {
        let metrics = iotext_data_row.metrics_mut();
        //m.metrics.unwrap().push(Some(vec![]))
        metrics.get_or_insert(vec![]).push(
            MetricDataItem {
                //data_type: ItemTypeEnum::Metric(),
                name: "METRIC_NAME_01".to_string(),
                value: MetricValueType::IntegerItemType(42),
            }
        );
    }

    println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row);

    //    parse_iotext_str(MSG_EXAMPLE);

    //let mut my_person = Person::default();
    //println!("my_person: {:?}", my_person)
}
