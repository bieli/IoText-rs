use std::*;

use iotext_rs::*;

pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6";

fn main() {
    let data_obj = IoTextDataRow::default();
    let iotext_data_row = data_obj.parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row: {:#?}", iotext_data_row);

    let str_from_iotext_data: String = data_obj.dump_iotext_to_str(&iotext_data_row, true);
    println!("str_from_iotext_data: {}", str_from_iotext_data)
}

/*
STDOUT:

iotext_data_row: IoTextDataRow {
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
                name: "val_water_001",
                value: IntegerItemType(
                    1234,
                ),
            },
            MetricDataItem {
                name: "val_water_002",
                value: IntegerItemType(
                    15,
                ),
            },
            MetricDataItem {
                name: "bulb_state",
                value: BoolItemType(
                    true,
                ),
            },
            MetricDataItem {
                name: "connector_state",
                value: BoolItemType(
                    false,
                ),
            },
            MetricDataItem {
                name: "temp_01",
                value: DecimalItemType(
                    34.4,
                ),
            },
            MetricDataItem {
                name: "temp_02",
                value: DecimalItemType(
                    36.4,
                ),
            },
            MetricDataItem {
                name: "temp_03",
                value: DecimalItemType(
                    10.4,
                ),
            },
            MetricDataItem {
                name: "pwr",
                value: DecimalItemType(
                    12.231,
                ),
            },
            MetricDataItem {
                name: "current",
                value: DecimalItemType(
                    1.429,
                ),
            },
            MetricDataItem {
                name: "current_battery",
                value: DecimalItemType(
                    1.548,
                ),
            },
            MetricDataItem {
                name: "status_txt",
                value: TextItemType(
                    "in_progress",
                ),
            },
        ],
    ),
    crc16: Some(
        Item {
            value: Crc(
                "ECF6",
            ),
        },
    ),
}
str_from_iotext_data: t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6
*/
