use criterion::{criterion_group, criterion_main, Criterion};
use iotext_rs::{parse_iotext_str, IotextDataRow};

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

pub fn criterion_benchmark_iotext(c: &mut Criterion) {
    const MSG_EXAMPLE_IOTEXT: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

    const MSG_EXAMPLE_JSON: &str = "{\"timestamp\":{\"value\":{\"TimeUnixMilis\":3900237526042}},\"device_id\":{\"value\":{\"DeviceId\":\"device_name_001\"}},\"metrics\":[{\"name\":\"val_water_001\",\"value\":{\"IntegerItemType\":1234}},{\"name\":\"val_water_002\",\"value\":{\"IntegerItemType\":15}},{\"name\":\"bulb_state\",\"value\":{\"BoolItemType\":true}},{\"name\":\"connector_state\",\"value\":{\"BoolItemType\":false}},{\"name\":\"temp_01\",\"value\":{\"DecimalItemType\":\"34.4\"}},{\"name\":\"temp_02\",\"value\":{\"DecimalItemType\":\"36.4\"}},{\"name\":\"temp_03\",\"value\":{\"DecimalItemType\":\"10.4\"}},{\"name\":\"pwr\",\"value\":{\"DecimalItemType\":\"12.231\"}},{\"name\":\"current\",\"value\":{\"DecimalItemType\":\"1.429\"}},{\"name\":\"current_battery\",\"value\":{\"DecimalItemType\":\"1.548\"}}]}";

    c.bench_function("parse_iotext_str", |b| {
        b.iter(|| parse_iotext_str(MSG_EXAMPLE_IOTEXT))
    });

    c.bench_function("parse_iotext_str_as_json", |b| {
        b.iter(|| serde_json::from_str::<IotextDataRow>(MSG_EXAMPLE_JSON).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark_iotext);
criterion_main!(benches);
