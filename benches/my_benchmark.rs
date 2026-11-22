//! Parser throughput benchmarks for 10-200 metrics per message.
#![allow(clippy::similar_names, clippy::missing_panics_doc, missing_docs)]

use criterion::{Criterion, criterion_group, criterion_main};
use iotext_rs::IoTextDataRow;
use std::hint::black_box;
/*
use rand::Rng;

pub fn gen_rnd_str() -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789_";
  const OUTPUT_LENGTH: usize = 30;
  let mut rng = rand::thread_rng();

  let gen_str: String = (0..OUTPUT_LENGTH)
      .map(|_| {
          let idx = rng.gen_range(0..CHARSET.len());
          CHARSET[idx] as char
      })
      .collect();

  gen_str
}

pub fn gen_next_10_metrics() -> String {
  format!("|{gen_rnd_str()}=i:12345678,m|{gen_rnd_str()}=i:15,m|bulb_state2=b:1,m|connector_state2=b:0,m|temp_02=d:341.14,m|temp_022=d:316.4,m|temp_032=d:20.4,m|pwr2=d:32.231,m|current2=d:4.429,m|current_battery2=d:1.548".to_owned())
}
*/

pub fn criterion_benchmark(c: &mut Criterion) {
    const MSG_EXAMPLE_10_METRICS_FIRST: &str = "t|3900237526042,d|device_name_001,m|val_water_001=i:1234,m|val_water_002=i:15,m|bulb_state=b:1,m|connector_state=b:0,m|temp_01=d:34.4,m|temp_02=d:36.4,m|temp_03=d:10.4,m|pwr=d:12.231,m|current=d:1.429,m|current_battery=d:1.548";

    const MSG_EXAMPLE_10_METRICS_2: &str = ",m|val_water_002=i:12345678,m|val_water_0022=i:15,m|bulb_state2=b:1,m|connector_state2=b:0,m|temp_02=d:341.14,m|temp_022=d:316.4,m|temp_032=d:20.4,m|pwr2=d:32.231,m|current2=d:4.429,m|current_battery2=d:1.548";

    const MSG_EXAMPLE_10_METRICS_3: &str = ",m|val_water_003=i:12345678,m|val_water_0023=i:15,m|bulb_state3=b:1,m|connector_state3=b:0,m|temp_03=d:341.14,m|temp_023=d:316.4,m|temp_033=d:20.4,m|pwr3=d:32.231,m|current3=d:4.429,m|current_battery3=d:1.548";

    const MSG_EXAMPLE_10_METRICS_4: &str = ",m|val_water_004=i:12345678,m|val_water_0024=i:15,m|bulb_state4=b:1,m|connector_state4=b:0,m|temp_04=d:341.14,m|temp_024=d:316.4,m|temp_034=d:20.4,m|pwr4=d:32.231,m|current4=d:4.429,m|current_battery4=d:1.548";

    let msg_example_20_metrics: String =
        format!("{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}");

    let msg_example_30_metrics: String = format!(
        "{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}"
    );

    let msg_example_40_metrics: String = format!(
        "{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}"
    );

    let msg_example_50_metrics: String = format!(
        "{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}"
    );

    let msg_example_100_metrics: String = format!(
        "{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}"
    );

    let msg_example_200_metrics: String = format!(
        "{MSG_EXAMPLE_10_METRICS_FIRST}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_2}{MSG_EXAMPLE_10_METRICS_3}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}{MSG_EXAMPLE_10_METRICS_4}"
    );

    // Guard: every fixture must be a valid message, otherwise we benchmark the error path.
    for (name, msg) in [
        ("10", MSG_EXAMPLE_10_METRICS_FIRST),
        ("20", msg_example_20_metrics.as_str()),
        ("30", msg_example_30_metrics.as_str()),
        ("40", msg_example_40_metrics.as_str()),
        ("50", msg_example_50_metrics.as_str()),
        ("100", msg_example_100_metrics.as_str()),
        ("200", msg_example_200_metrics.as_str()),
    ] {
        let row = msg
            .parse::<IoTextDataRow>()
            .unwrap_or_else(|e| panic!("{name}-metrics fixture is invalid: {e}"));
        assert_eq!(row.metrics.len(), name.parse::<usize>().unwrap());
    }

    c.bench_function("parse_iotext_str - 10 metrics", |b| {
        b.iter(|| black_box(MSG_EXAMPLE_10_METRICS_FIRST).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 20 metrics", |b| {
        b.iter(|| black_box(msg_example_20_metrics.as_str()).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 30 metrics", |b| {
        b.iter(|| black_box(msg_example_30_metrics.as_str()).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 40 metrics", |b| {
        b.iter(|| black_box(msg_example_40_metrics.as_str()).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 50 metrics", |b| {
        b.iter(|| black_box(msg_example_50_metrics.as_str()).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 100 metrics", |b| {
        b.iter(|| black_box(msg_example_100_metrics.as_str()).parse::<IoTextDataRow>());
    });

    c.bench_function("parse_iotext_str - 200 metrics", |b| {
        b.iter(|| black_box(msg_example_200_metrics.as_str()).parse::<IoTextDataRow>());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
