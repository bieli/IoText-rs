# IoText-rs

![CI status](https://github.com/bieli/IoText-rs/actions/workflows/test.yaml/badge.svg)
![github_tag](https://img.shields.io/github/v/tag/bieli/IoText-rs)
[![Crates.io](https://img.shields.io/crates/v/iotext_rs.svg)](https://crates.io/crates/iotext_rs)

`IoText` is an IoT and IIoT text-based data protocol, simpler than JSON for time-series data from measurements.

This project was written in `RUST language` and it is an implementation of [IoText data protocol](https://github.com/bieli/IoText-data-protocol).

IoText data protocol specification is here: https://github.com/bieli/IoText-data-protocol


## Development details


### Example library code usage (parsing String with IoText row and preparing String from IoText MSG_EXAMPLE row)

```rust
use std::*;

use env_logger::Builder;
use log::LevelFilter;

use iotext_rs::*;

pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6";

fn main() {
    Builder::new().filter(None, LevelFilter::Error).init();

    let data_obj = IoTextDataRow::default();
    let iotext_data_row = data_obj.parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row: {:#?}", iotext_data_row);

    let str_from_iotext_data: String = data_obj.dump_iotext_to_str(&iotext_data_row, true);
    println!("str_from_iotext_data: {}", str_from_iotext_data)
}
```

## Benchmark synthetics tests - guaranty of linear inc. parsing time depends IoText metrics count

Table with tests results:

| IoText message size (bytes) | IoText msg. metrics count  | avg. parsing time (µs) |
| --- | --- | --- |
| 206 | 10 | 2.5 |
| 432 | 20 | 4.5 |
| 638 | 30 | 6.5 |
| 844 | 40 | 8.5 |
| 1050 | 50 | 10.5 |
| 2080 | 100 | 20.5 |
| 4160 | 200 | 41.5 |

**Based on the below data in avg. 1 second time, we can parse avg. 24000 IoText data messages (with 200 metrics per msg.) with this RUST implementation.**
**It is quite an impressive result and of course, expected !!!**

Tests were prepared on a machine with Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz 6-cores processor and 32GB RAM. With `load average: 1,44, 2,49, 3,51` (info from `upime` cmd) and approx. 80% RAM used. Linux kernel 5.15 and distribution Linux Ubuntu 20.04.6 LTS.

More detailed statistics from `cargo bench` running you can see in bottom section `Example cargo bench outputs`.

### Input example IoText data row
```bash
t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6
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
    crc16: Some(
        Item {
            value: Crc(
                "ECF6",
            ),
        },
    ),
}
str_from_iotext_data: t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6
```

### Example cargo bench outputs
```bash

$ cargo bench

parse_iotext_str - 10 metrics                                                                             
                        time:   [1.9704 µs 1.9910 µs 2.0142 µs]
                        change: [-1.0093% +2.9404% +7.3783%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) high mild
  4 (4.00%) high severe

parse_iotext_str - 20 metrics                                                                             
                        time:   [3.8728 µs 3.9334 µs 4.0187 µs]
                        change: [-1.9420% +0.3434% +2.5942%] (p = 0.78 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

parse_iotext_str - 30 metrics                                                                             
                        time:   [5.9316 µs 6.0091 µs 6.0924 µs]
                        change: [+2.2407% +3.9826% +5.6299%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

parse_iotext_str - 40 metrics                                                                             
                        time:   [8.4750 µs 8.6151 µs 8.7691 µs]
                        change: [-2.1735% -0.1968% +1.6887%] (p = 0.85 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

parse_iotext_str - 50 metrics                                                                             
                        time:   [10.914 µs 11.076 µs 11.264 µs]
                        change: [+3.2785% +5.5452% +7.6130%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

parse_iotext_str - 100 metrics                                                                             
                        time:   [20.456 µs 20.593 µs 20.764 µs]
                        change: [-1.5163% +0.1975% +1.7861%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

parse_iotext_str - 200 metrics                                                                             
                        time:   [40.849 µs 41.396 µs 42.023 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```


## Run unit tests

```bash

$ make test

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

