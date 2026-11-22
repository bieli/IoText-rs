# Changelog

All notable changes to this project are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Fixed
- A decimal negative zero (e.g. `d:-1E-40`) serialized as `-0.000...` but parsed back as
  positive zero, so the message text changed on a parse/serialize round-trip. Found by `cargo fuzz`.

### Changed
- CRC-16 uses slice-by-8 tables (built at compile time): ~4.6x faster, parsing ~15% faster.

### Added
- `cargo fuzz` targets (`fuzz/`), rewritten benchmarks, more doctests.

## [1.3.0] - 2026-11-22

> Contains source-breaking changes to the data model compared with 1.0.0;
> see "Migrating from 1.0.0 to 1.3.0" in the README.
>
> Tags `1.1.0` and `1.2.0` point at the 1.0.0 code and were never published to crates.io.

### Changed
- `IoTextDataRow` is a flat struct: `timestamp: u64`, `device_id: String`,
  `metrics: Vec<MetricDataItem>`, `crc16: Option<u16>`.
- `IoTextError::CrcMismatch` carries `u16` values.
- Edition 2024; minimum supported Rust version is 1.85.
- The demo binary `iotext_rs_main` moved to `examples/basic_usage.rs`.
- Dependencies reduced to `log`, `rust_decimal` and `thiserror`
  (all `default-features = false`).

### Added
- `MetricKind` and `ItemKind` enums for protocol codes.
- `From<i64 | i32 | u32 | bool | Decimal | String | &str>` for `MetricValueType`;
  `IoTextBuilder::add_metric` accepts plain Rust values; `add_metric_of_kind`.
- `IoTextDataRow::metric(name)`, `IoTextDataRow::validate()`, `MetricValueType::kind()`.
- `const fn crc16(&[u8]) -> u16` (compile-time lookup table), `crc16_with_poly`, `crc16_to_hex`.
- `no_std` + `alloc` support (`default-features = false`); `std` feature (default).
- Optional `serde` feature for the data model.
- Documentation for every public item.

### Deprecated
- `MetricDataTypes`, `ItemTypes` (use `MetricKind` / `ItemKind`).
- `Tools::crc16` (use `crc16`).

### Removed
- `Item`, `ItemTypeEnum` and the `get_*` / `*_mut` accessors of `IoTextDataRow`.

## [1.0.0] - 2026-11-22

### Changed
- Parsing and building never panic; all failures are returned as `IoTextError`.
- `impl FromStr` / `Display` for `IoTextDataRow`; `to_iotext_string(add_crc16)`.
- Builder setters record the first error, returned by `build()`.
- Unknown metric types, unknown item types, duplicated `t`/`d` items and
  missing `t`/`d` items are errors.

### Fixed
- Malformed input (e.g. `m|a`, `m|a=d:abc`) panicked.
- CRC was parsed but never verified; it is now checked and must be the last item.
- CRC of a message without metrics covered a trailing `,`.
- Text values containing `,` were silently truncated; separators are rejected in identifiers.

### Deprecated
- `IoTextData` trait, `IoTextBuilderError`.

[Unreleased]: https://github.com/bieli/IoText-rs/compare/1.3.0...HEAD
[1.3.0]: https://github.com/bieli/IoText-rs/compare/1.0.0...1.3.0
[1.0.0]: https://github.com/bieli/IoText-rs/compare/0.5.0...1.0.0
