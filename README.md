# IoText-rs

![CI status](https://github.com/bieli/IoText-rs/actions/workflows/test.yaml/badge.svg)


`IoText` is an IoT and IIoT text-based data protocol, simpler than JSON for time-series data from measurements.

This project was written in `RUST language` and it is an implementation of IoText data protocol.

IoText data protocol specification is here: https://github.com/bieli/IoText-data-protocol


# Development STDOUT (parsing IoText data protocol from String)

### Input example IoText data row
```bash
t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress
```

### Parsing results (data from below example IoText data row)
```bash
$ cargo b
$ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/iotext-rs`
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

## Run benchmark tests

```bash

$ cargo bench

...

parse_iotext_str        time:   [1.9652 µs 2.0015 µs 2.0388 µs]                              
Found 19 outliers among 100 measurements (19.00%)
  8 (8.00%) high mild
  11 (11.00%) high severe

```
