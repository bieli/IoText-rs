use std::*;

use iotext_rs::*;
#[warn(unused_imports)]
use rust_decimal::Decimal;
#[warn(unused_imports)]
use std::str::FromStr;

fn main() {
    let iotext_data_row = parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row);

    let str_from_iotext_data: String = dump_iotext_to_str(&iotext_data_row);
    println!("str_from_iotext_data: {}", str_from_iotext_data);

    println!(
        "dump_iotext_to_json_str: {:?}",
        dump_iotext_to_json_str(&iotext_data_row)
    )
}

/*
STDOUT:

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

str_from_iotext_data: t|3900237526042,d|device_name_001



*/
