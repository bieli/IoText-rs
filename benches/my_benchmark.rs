//#[macro_use] extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

use iotext_rs::{extract_metric_value_type};
//use criterion::*;

/*
// The function to benchmark
fn foo() {
    // ...
}

fn bench(c: &mut Criterion) {
    c.bench_function("iter", move |b| {
        b.iter(|| foo())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

*/

//use mycrate::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    const METRIC_DATA_TYPE: &str = "i";
    const METRIC_DATA_VALUE: &str = "42";

    c.bench_function("extract_metric_value_type", |b| b.iter(|| extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE)));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
