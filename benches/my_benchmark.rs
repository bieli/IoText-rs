use criterion::{criterion_group, criterion_main, Criterion};
use iotext_rs::IoTextData;
use iotext_rs::IoTextDataRow;

pub fn criterion_benchmark(c: &mut Criterion) {
    let data_obj: IoTextDataRow = IoTextDataRow::default();
    const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

    c.bench_function("parse_iotext_str", |b| {
        b.iter(|| data_obj.parse_iotext_str(MSG_EXAMPLE))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
