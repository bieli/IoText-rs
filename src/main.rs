use std::*;

use env_logger::Builder;
use iotext_rs::builder::*;
use iotext_rs::*;
use log::LevelFilter;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6";

fn main() {
    Builder::new().filter(None, LevelFilter::Error).init();

    // ------------------
    // ENCODE / DECODE EXAMPLE
    // ------------------

    let data_obj = IoTextDataRow::default();
    let iotext_data_row = data_obj.parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row: {:#?}", iotext_data_row);

    let str_from_iotext_data: String = data_obj.dump_iotext_to_str(&iotext_data_row, true);
    println!("str_from_iotext_data: {}", str_from_iotext_data);

    // ------------------
    // BUILDER EXAMPLE WITHOUT CRC
    // ------------------

    let iotext_data_row_from_builder_without_crc = IoTextBuilder::new()
        .current_timestamp()
        // .timestamp(3900237526042)
        .device_id("device_name_001")
        .add_metric_as_str("val_water_001", "1234", "i")
        .add_metric_as_str("val_water_002", "15", "i")
        .add_metric_as_str("bulb_state", "1", "b")
        .add_metric("connector_state", MetricValueType::BoolItemType(false))
        .add_metric(
            "temp_01",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(34.4).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric(
            "temp_02",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(36.4).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric(
            "current_battery",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(1.548).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric(
            "status_txt",
            MetricValueType::TextItemType("in_progress".to_string()),
        )
        .build(false)
        .unwrap();

    println!(
        "iotext_data_row as STR without CRC: {:?}",
        data_obj.dump_iotext_to_str(
            &iotext_data_row_from_builder_without_crc,
            !iotext_data_row_from_builder_without_crc.crc16.is_none()
        )
    );

    // ------------------
    // BUILDER EXAMPLE WITH CRC
    // ------------------

    let iotext_data_row_from_builder_with_crc = IoTextBuilder::new()
        .timestamp(3900237526042)
        .device_id("device_name_001")
        .add_metric_as_str("val_water_001", "1234", "i")
        .add_metric_as_str("val_water_002", "15", "i")
        .add_metric_as_str("bulb_state", "1", "b")
        .add_metric("connector_state", MetricValueType::BoolItemType(false))
        .add_metric(
            "temp_01",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(34.4).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric(
            "temp_02",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(36.4).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric(
            "current_battery",
            MetricValueType::DecimalItemType(
                Decimal::from_f64(1.548).expect("Could not convert value to decimal!"),
            ),
        )
        .add_metric(
            "status_txt",
            MetricValueType::TextItemType("in_progress".to_string()),
        )
        .build(true)
        .unwrap();

    println!(
        "iotext_data_row as STR with CRC: {:?}",
        data_obj.dump_iotext_to_str(
            &iotext_data_row_from_builder_with_crc,
            !iotext_data_row_from_builder_with_crc.crc16.is_none()
        )
    );
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
iotext_data_row as STR without CRC: "t|1740851375155,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress"
iotext_data_row as STR with CRC: "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6"
*/
