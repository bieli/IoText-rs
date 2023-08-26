use criterion::{criterion_group, criterion_main, Criterion};
//use self::criterion::{Bencher, Criterion, Benchmark, PlotConfiguration, AxisScale};
use iotext_rs::{parse_iotext_str, IotextDataRow};

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

/*
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
}

fn main() {
    let data = r#"
        {
            "name": "Alice",
            "age": 30,
            "is_student": false
        }
    "#;

    let person: Person = serde_json::from_str(data).unwrap();

    println!("{:?}", person);
}
*/

/*
pub fn criterion_benchmark_iotext(c: &mut Criterion) {
    const MSG_EXAMPLE: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

    c.bench_function("parse_iotext_str", |b| {
        b.iter(|| parse_iotext_str(MSG_EXAMPLE))
    });
}
*/

pub fn criterion_benchmark_json(c: &mut Criterion) {
    const MSG_EXAMPLE_JSON: &str = "[{\"t\":3900237526042},{\"d\":\"device_name_001\"},{\"m\":[{\"val_water_001\":{\"i\":1234}},{\"val_water_002\":{\"i\":15}},{\"bulb_state\":{\"b\":1}},{\"connector_state\":{\"b\":0}},{\"temp_01\":{\"d\":\"34.4\"}},{\"temp_02\":{\"d\":\"36.4\"}},{\"temp_03\":{\"d\":\"10.4\"}},{\"pwr\":{\"d\":\"12.231\"}},{\"current\":{\"d\":\"1.429\"}},{\"current_battery\": {\"d\":\"1.548\"}}]}]";

    c.bench_function("parse_iotext_str_as_json", |b| {
        b.iter(|| serde_json::<>::from_str::<IotextDataRow>(MSG_EXAMPLE_JSON).unwrap())
    });
}


//criterion_group!(benches, criterion_benchmark_iotext);
criterion_group!(benches, criterion_benchmark_json);
criterion_main!(benches);

//let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

// Using Criterion::default() for simplicity; normally you'd use the macros.
//let mut criterion = Criterion::default();
//let mut benchmark_group = criterion.benchmark_group("Group name");
//benchmark_group.plot_config(plot_config);
