use criterion::{criterion_group, criterion_main, Criterion};
//use self::criterion::{Bencher, Criterion, Benchmark, PlotConfiguration, AxisScale};
use iotext_rs::extract_metric_value_type;

pub fn criterion_benchmark(c: &mut Criterion) {
    const METRIC_DATA_TYPE: &str = "i";
    const METRIC_DATA_VALUE: &str = "42";

    c.bench_function("extract_metric_value_type", |b| {
        b.iter(|| extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

// Using Criterion::default() for simplicity; normally you'd use the macros.
//let mut criterion = Criterion::default();
//let mut benchmark_group = criterion.benchmark_group("Group name");
//benchmark_group.plot_config(plot_config);
