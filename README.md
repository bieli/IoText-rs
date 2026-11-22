# IoText-rs

![CI status](https://github.com/bieli/IoText-rs/actions/workflows/test.yaml/badge.svg)
![github_tag](https://img.shields.io/github/v/tag/bieli/IoText-rs)
[![Crates.io](https://img.shields.io/crates/v/iotext_rs.svg)](https://crates.io/crates/iotext_rs)
[![docs.rs](https://img.shields.io/docsrs/iotext_rs)](https://docs.rs/iotext_rs)
![MSRV](https://img.shields.io/badge/MSRV-1.85-blue)
![no_std](https://img.shields.io/badge/no__std-alloc-green)

`IoText` is an IoT and IIoT text-based data protocol, simpler than JSON for time-series data from measurements.

This project was written in `RUST language` and it is an implementation of [IoText data protocol](https://github.com/bieli/IoText-data-protocol).

IoText data protocol specification is here: https://github.com/bieli/IoText-data-protocol


## Installation

```toml
[dependencies]
iotext_rs = "1"
```

### Cargo features

| Feature | Default | Description |
|---|---|---|
| `std` | yes | `std::error::Error` for all errors and `IoTextBuilder::current_timestamp()`. |
| `serde` | no | `Serialize` / `Deserialize` for `IoTextDataRow`, `MetricDataItem`, `MetricValueType`, `MetricKind`, `ItemKind` (`Decimal` as a string). |

### `no_std` (embedded)

The crate is `#![no_std]` and only needs `alloc`. Disable default features:

```toml
[dependencies]
iotext_rs = { version = "1", default-features = false }            # + features = ["serde"] if needed
```

Without `std`, set timestamps explicitly with `IoTextBuilder::timestamp()` (for example from an RTC).
The build is verified for `thumbv6m-none-eabi` (Cortex-M0) and `x86_64-unknown-none`.

### Minimum supported Rust version

The MSRV is **1.85** (edition 2024). Raising it is treated as a minor (not patch) release.

## Development details

```bash
cargo test --all-features                       # unit, property, serde and doc tests
cargo run --example basic_usage                 # the example below
make check-nostd                                # no_std check (thumbv6m-none-eabi)
cargo bench
```


### Example library code usage (parsing String with IoText row and preparing String from IoText MSG_EXAMPLE row)

```rust
use env_logger::Builder;
use log::LevelFilter;

use iotext_rs::*;

pub const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6";

fn main() -> Result<(), IoTextError> {
    Builder::new().filter(None, LevelFilter::Error).init();

    // Parsing never panics - malformed input, wrong CRC etc. are returned as `IoTextError`.
    let iotext_data_row: IoTextDataRow = MSG_EXAMPLE.parse()?;
    println!("iotext_data_row: {iotext_data_row:#?}");

    // `Display` appends a freshly computed CRC when `crc16` is `Some`;
    // use `to_iotext_string(add_crc16)` to choose explicitly.
    let str_from_iotext_data: String = iotext_data_row.to_string();
    println!("str_from_iotext_data: {str_from_iotext_data}");
    Ok(())
}
```

### Error handling

```rust
use iotext_rs::{IoTextDataRow, IoTextError};

match "t|1,d|dev,c|0000".parse::<IoTextDataRow>() {
    Err(IoTextError::CrcMismatch { received, computed }) => eprintln!("bad CRC {received:04X} != {computed:04X}"),
    Err(other) => eprintln!("invalid message: {other}"),
    Ok(row) => println!("{row}"),
}
```

### Migrating from 0.5.x to 1.0.0

| 0.5.x | 1.0.0 |
|---|---|
| `IoTextDataRow::default().parse_iotext_str(s)` (panics) | `s.parse::<IoTextDataRow>()?` |
| `data_obj.dump_iotext_to_str(&row, add_crc)` | `row.to_iotext_string(add_crc)` or `row.to_string()` |
| `data_obj.extract_metric_value_type(t, v)` | `MetricValueType::from_type_and_value(t, v)?` |
| `IoTextBuilderError::TimestampIsEmpty / DeviceIdIsEmpty` | `IoTextError::MissingTimestamp / MissingDeviceId` |
| builder setters panic on invalid input | first error is returned by `build()` |
| unknown metric type stored as empty TEXT | `IoTextError::MetricDataType(..)` |
| CRC item parsed but not verified | CRC verified; must be the last item |
| CRC of message without metrics covered a trailing `,` | CRC covers exactly the bytes before `,c\|` |
| separators allowed in device id / metric name | `, : \| =` rejected; `,` rejected in TEXT values |
| `row.timestamp.value == ItemTypeEnum::TimeUnixMilis(ts)` | `row.timestamp == ts` (`u64`) |
| `row.device_id.value == ItemTypeEnum::DeviceId(id)` | `row.device_id == id` (`String`) |
| `row.metrics: Option<Vec<MetricDataItem>>` | `row.metrics: Vec<MetricDataItem>` (empty = no metrics) |
| `row.crc16: Option<Item>` (`Crc(String)`) | `row.crc16: Option<u16>` |
| `get_timestamp()` / `timestamp_mut()` / ... getters | public fields; `row.metric("name")` lookup |
| `Item`, `ItemTypeEnum` | removed |
| `MetricDataTypes::INTEGER`, `ItemTypes::CRC` | `MetricKind::Integer`, `ItemKind::Crc` (`.code()` gives `"i"`, `"c"`) |
| `Tools::crc16(s, CRC16_POLY_DEFAULT)` → `String` | `crc16(s.as_bytes())` → `u16` (`const fn`, table-driven); `crc16_to_hex()` |
| `.add_metric("t", MetricValueType::DecimalItemType(d))` | `.add_metric("t", d)` - accepts `i64`, `i32`, `u32`, `bool`, `Decimal`, `&str`, `String` |
| `.add_metric_as_str("t", "1.5", "d")` | still works; also `.add_metric_of_kind("t", MetricKind::Decimal, "1.5")` |
| `IoTextError::CrcMismatch { received: String, .. }` | `received` / `computed` are `u16` |
| `iotext_rs_main` binary | `cargo run --example basic_usage` |
| `rust_decimal_macros`, `chrono`, `env_logger`, `serde` pulled in always | only `log`, `rust_decimal`, `thiserror`; `serde` is an opt-in feature |
| edition 2021, no MSRV | edition 2024, MSRV 1.85, `no_std` + `alloc` with `default-features = false` |

The old `IoTextData` trait, `Tools::crc16`, `MetricDataTypes` and `ItemTypes` are still available (deprecated).
`MetricValueType`, `MetricKind`, `ItemKind` and all error enums are `#[non_exhaustive]`, so new protocol types can be added in minor releases.

### Example library code usage without CRC (BUILDER pattern usage with IoTextBuilder)

```rust

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
        .add_metric(
            "temp_01",
            Decimal::new(344, 1),
        )
        .add_metric(
            "temp_02",
            Decimal::new(364, 1),
        )
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric(
            "current_battery",
            Decimal::new(1548, 3),
        )
        .add_metric(
            "status_txt",
            "in_progress",
        )
        .build(false)?;

    println!(
        "iotext_data_row as STR without CRC: {:?}",
        iotext_data_row_from_builder_without_crc.to_string()
    );
```

### Example library code usage with CRC (BUILDER pattern usage with IoTextBuilder)

```rust
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
        .add_metric(
            "temp_01",
            Decimal::new(344, 1),
        )
        .add_metric(
            "temp_02",
            Decimal::new(364, 1),
        )
        .add_metric_as_str("temp_03", "10.4", "d")
        .add_metric_as_str("pwr", "12.231", "d")
        .add_metric_as_str("current", "1.429", "d")
        .add_metric(
            "current_battery",
            Decimal::new(1548, 3),
        )
        .add_metric(
            "status_txt",
            "in_progress",
        )
        .build(true)?;

    println!(
        "iotext_data_row as STR with CRC: {:?}",
        iotext_data_row_from_builder_with_crc.to_string()
    );
```

## Benchmarks

`cargo bench` measures parsing, serializing and building messages with 10-200 mixed
(`i`/`b`/`d`/`t`) metrics plus a CRC, and the CRC-16 itself. Fixtures are generated with the
builder and checked before measuring, so every benchmark runs the success path.

| Benchmark | Mean time | Throughput |
| --- | --- | --- |
| `parse/10` | 1.7465 µs | 133.24 MiB/s |
| `parse/25` | 4.8827 µs | 112.31 MiB/s |
| `parse/50` | 9.4493 µs | 112.43 MiB/s |
| `parse/100` | 17.437 µs | 120.98 MiB/s |
| `parse/200` | 32.042 µs | 132.06 MiB/s |
| `serialize/10` | 1.5457 µs | 150.55 MiB/s |
| `serialize/25` | 3.4696 µs | 158.05 MiB/s |
| `serialize/50` | 6.7160 µs | 158.19 MiB/s |
| `serialize/100` | 14.327 µs | 147.24 MiB/s |
| `serialize/200` | 25.759 µs | 164.27 MiB/s |
| `build/10` | 3.3579 µs | 2.9781 Melem/s |
| `build/25` | 12.758 µs | 1.9596 Melem/s |
| `build/50` | 15.684 µs | 3.1880 Melem/s |
| `build/100` | 35.762 µs | 2.7963 Melem/s |
| `build/200` | 61.551 µs | 3.2494 Melem/s |
| `crc16/slice_by_8` | 533.95 ns | 1.9430 GiB/s |
| `crc16/bitwise` | 2.4678 µs | 430.49 MiB/s |

Parsing scales linearly with the number of metrics (about 0.17 µs per metric, ~30 000
messages/s with 200 metrics each on one core). The slice-by-8 CRC is ~4.6x faster than the
bitwise reference; it made parsing ~15% faster than the previous lookup-table version.

Measured with criterion 0.8 on an Intel Core i7-8850H @ 2.60GHz (Linux 5.15, Rust 1.100,
load average ~4.5, so treat the numbers as indicative).

> Benchmark numbers published for versions before 1.0.0 were measured on malformed fixtures
> (only the first 10 metrics were valid), so they are not comparable.

### Input example IoText data row
```bash
t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548,m|status_txt=t:in_progress,c|ECF6
```

### Parsing results (data from below example IoText data row)
```bash
$ cargo run --example basic_usage
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/basic_usage`
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
```

## Testing

```bash
make test                     # unit, property (proptest), serde and doc tests, all features
PROPTEST_CASES=50000 cargo test --release --test lib_proptest   # longer property-test run
```

### Fuzzing

Two [`cargo fuzz`](https://github.com/rust-fuzz/cargo-fuzz) targets live in `fuzz/` (nightly toolchain):

* `parse` - arbitrary bytes into the parser; every accepted message must re-serialize to a canonical
  message that parses back to the same row.
* `builder` - structured random input into `IoTextBuilder`; everything the builder accepts must be
  accepted by the parser as an identical row.

```bash
cargo install cargo-fuzz
make fuzz                     # 60 s per target, with the protocol dictionary
make fuzz-regressions         # replay inputs that crashed before (fuzz/regressions/)
```


