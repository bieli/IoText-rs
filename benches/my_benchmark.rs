use criterion::{criterion_group, criterion_main, Criterion};
//use self::criterion::{Bencher, Criterion, Benchmark, PlotConfiguration, AxisScale};
use iotext_rs::parse_iotext_str;

pub fn criterion_benchmark(c: &mut Criterion) {
    const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

    c.bench_function("parse_iotext_str", |b| {
        b.iter(|| parse_iotext_str(MSG_EXAMPLE))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

// Using Criterion::default() for simplicity; normally you'd use the macros.
//let mut criterion = Criterion::default();
//let mut benchmark_group = criterion.benchmark_group("Group name");
//benchmark_group.plot_config(plot_config);
