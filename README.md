# IoText-rs (IN DEVELOPMENT)
RUST implementation of IoText data protocol - specification: https://github.com/bieli/IoText-data-protocol


# Development STDOUT (parsing IoText data protocol from String)

```bash
$ cargo b
$ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/iotext-rs`
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

```


## Run unit tests

```bash

$ cargo test

    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/main.rs (target/debug/deps/iotext_rs-a68ae8edfd0a7e63)

running 5 tests
test tests::test_extract_metric_value_type_bool_false ... ok
test tests::test_extract_metric_value_type_bool_true ... ok
test tests::test_extract_metric_value_type_integer ... ok
test tests::test_extract_metric_value_type_decimal ... ok
test tests::test_extract_metric_value_type_text ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```
