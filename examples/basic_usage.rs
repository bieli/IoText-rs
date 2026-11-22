//! Parsing, serializing and building `IoText` messages.
//!
//! Run with `cargo run --example basic_usage`.

use env_logger::Builder;
use iotext_rs::builder::IoTextBuilder;
use iotext_rs::{IoTextDataRow, IoTextError};
use log::LevelFilter;
use rust_decimal::Decimal;

/// Example message with 11 metrics and a CRC.
pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6";

fn main() -> Result<(), IoTextError> {
    Builder::new().filter(None, LevelFilter::Error).init();

    // ------------------
    // ENCODE / DECODE EXAMPLE
    // ------------------

    let iotext_data_row: IoTextDataRow = MSG_EXAMPLE.parse()?;
    println!("iotext_data_row: {iotext_data_row:#?}");

    let str_from_iotext_data: String = iotext_data_row.to_string();
    println!("str_from_iotext_data: {str_from_iotext_data}");

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
        .add_metric("connector_state", false)
        .add_metric("temp_01", Decimal::new(344, 1))
        .add_metric("temp_02", Decimal::new(364, 1))
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric("current_battery", Decimal::new(1548, 3))
        .add_metric("status_txt", "in_progress")
        .build(false)?;

    println!(
        "iotext_data_row as STR without CRC: {:?}",
        iotext_data_row_from_builder_without_crc.to_string()
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
        .add_metric("connector_state", false)
        .add_metric("temp_01", Decimal::new(344, 1))
        .add_metric("temp_02", Decimal::new(364, 1))
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric("current_battery", Decimal::new(1548, 3))
        .add_metric("status_txt", "in_progress")
        .build(true)?;

    println!(
        "iotext_data_row as STR with CRC: {:?}",
        iotext_data_row_from_builder_with_crc.to_string()
    );

    Ok(())
}

/*
STDOUT:

iotext_data_row: IoTextDataRow {
    timestamp: 3900237526042,
    device_id: "device_name_001",
    metrics: [
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
    crc16: Some(
        60662,
    ),
}
str_from_iotext_data: t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6
iotext_data_row as STR without CRC: "t|1795334198780,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress"
iotext_data_row as STR with CRC: "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6"
*/
