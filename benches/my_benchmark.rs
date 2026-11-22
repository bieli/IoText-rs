//! Benchmarks: parse, serialize and build messages with 10-200 metrics, and CRC-16.
//!
//! Fixtures are generated with the builder (mixed i/b/d/t metrics, CRC included), so they
//! are always valid messages and the parser is measured on its success path.
#![allow(missing_docs, clippy::missing_panics_doc)]

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use iotext_rs::builder::IoTextBuilder;
use iotext_rs::{IoTextDataRow, crc16, crc16_with_poly};
use rust_decimal::Decimal;

const METRIC_COUNTS: [usize; 5] = [10, 25, 50, 100, 200];

fn builder_with_metrics(n: usize) -> IoTextBuilder {
    let mut builder = IoTextBuilder::new()
        .timestamp(3900237526042)
        .device_id("device_name_001");
    for i in 0..n {
        let name = format!("metric_{i:03}");
        let i64_value = i64::try_from(i).unwrap();
        builder = match i % 4 {
            0 => builder.add_metric(&name, i64_value * 1_000),
            1 => builder.add_metric(&name, i % 2 == 0),
            2 => builder.add_metric(&name, Decimal::new(i64_value * 1_234, 3)),
            _ => builder.add_metric(&name, "in_progress"),
        };
    }
    builder
}

fn message(n: usize) -> String {
    let msg = builder_with_metrics(n).build(true).unwrap().to_string();
    // Guard: a fixture that does not parse would only benchmark the error path.
    assert_eq!(msg.parse::<IoTextDataRow>().unwrap().metrics.len(), n);
    msg
}

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    for n in METRIC_COUNTS {
        let msg = message(n);
        group.throughput(Throughput::Bytes(msg.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &msg, |b, msg| {
            b.iter(|| black_box(msg.as_str()).parse::<IoTextDataRow>().unwrap());
        });
    }
    group.finish();
}

fn bench_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");
    for n in METRIC_COUNTS {
        let msg = message(n);
        let row: IoTextDataRow = msg.parse().unwrap();
        group.throughput(Throughput::Bytes(msg.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &row, |b, row| {
            b.iter(|| black_box(row).to_string());
        });
    }
    group.finish();
}

fn bench_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("build");
    for n in METRIC_COUNTS {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| builder_with_metrics(black_box(n)).build(true).unwrap());
        });
    }
    group.finish();
}

fn bench_crc16(c: &mut Criterion) {
    let data = message(50).into_bytes();
    let mut group = c.benchmark_group("crc16");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("slice_by_8", |b| b.iter(|| crc16(black_box(&data))));
    group.bench_function("bitwise", |b| {
        b.iter(|| crc16_with_poly(black_box(&data), 0xA001));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_parse,
    bench_serialize,
    bench_build,
    bench_crc16
);
criterion_main!(benches);
