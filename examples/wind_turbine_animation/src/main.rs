use std::*;
use std::io::{Read};
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

//extern crate iotext_rs;

use raylib::prelude::*;
use iotext_rs::*;
use rust_decimal::prelude::*;

use std::collections::HashMap;
use std::vec::Vec;
use std::string::String;

use bevy_reflect::{Reflect, ReflectMut, DynamicStruct, FromReflect, TypeRegistryArc};


// t|3900237526042,d|wind_turbine_01,m|speed=d:12.00,m|wind_dir=t:n,m|temp_out=i:18,m|temp_int_gen=i:35,m|temp_int_gear=i:47,m|gen_voltage_peak=i:235.00,m|gen_current_peak=d:45.50

// openocd -f interface/picoprobe.cfg -f target/rp2040.cfg -c "program app verify reset exit"


//#[derive(Debug, Reflect, Default)]
//#[reflect(Default)]
#[derive(Debug, Default)]
struct IoTextDataTurbineMetrics {
    speed: Decimal,
    wind_dir: String,
    //temp_out: Decimal,
    //temp_int_gen: Decimal,
    //temp_int_gear: Decimal,
    //gen_voltage_peak: Decimal,
    //gen_current_peak: Decimal,
    warnings_cnt: i64,
}

/*
impl Default for IoTextDataTurbineMetrics {
    fn default() -> Self {
        IoTextDataTurbineMetrics {
            speed: Decimal::new(0, 0),
            wind_dir: String::new(),
            warnings_cnt: 0,
        }
    }
}
*/

/*
macro_rules! remap_metrics {
    ($metrics:expr, $struct_name:ty) => {{
        let mut output = <$struct_name as Default>::default();
        let mut dynamic_struct = DynamicStruct::default();

        for field in output.reflect_mut().fields_mut() {
            dynamic_struct.insert(field.name().to_string(), field);
        }

        for (key, value) in $metrics.iter() {
            if let Some(mut field) = dynamic_struct.field_mut(key) {
                match (field.type_id(), value) {
                    (id, MetricValueType::DecimalItemType(val)) if id == std::any::TypeId::of::<Decimal>() => {
                        *field.downcast_mut::<Decimal>().unwrap() = *val;
                    }
                    (id, MetricValueType::TextItemType(val)) if id == std::any::TypeId::of::<String>() => {
                        *field.downcast_mut::<String>().unwrap() = val.clone();
                    }
                    (id, MetricValueType::IntegerItemType(val)) if id == std::any::TypeId::of::<i64>() => {
                        *field.downcast_mut::<i64>().unwrap() = *val;
                    }
                    _ => {},
                }
            }
        }

        output
    }};
}
*/


fn plot_metric_and_value(d: &mut RaylibDrawHandle, name: String, value: String, unit: String, pos_no: i32) {
    let oY = pos_no * 20;
    d.draw_text(
        &format!("{} {:.2} {}", name, value, unit),
        23,
        oY + 3,
        40,
        Color::BLACK,
    );

    d.draw_text(
        &format!("{} {:.2} {}", name, value, unit),
        20,
        oY,
        40,
        Color::WHITE,
    );
}

//TODO: add to library

//fn get_metric_by_name(iotext_data_obj: &IoTextDataRow, name: &str) -> Option<&MetricDataItem> {
fn get_metric_by_name<'a>(iotext_data_obj: &'a IoTextDataRow, name: &'a str) -> Option<&'a MetricValueType> {
    if let Some(metrics) = &iotext_data_obj.metrics {
        for metric in metrics {
            if metric.name == name {
                return Some(&metric.value);
            }
        }
    }
    None
}

fn get_metrics_by_names<'a>(iotext_data_obj: &'a IoTextDataRow, names: &[&str]) -> Vec<&'a MetricDataItem> {
    let mut found_metrics = Vec::new();
    if let Some(metrics) = &iotext_data_obj.metrics {
        for metric in metrics {
            if names.contains(&metric.name.as_str()) {
                found_metrics.push(metric);
            }
        }
    }
    found_metrics
}
/*
fn get_metrics_by_names_as_hashmap1<'a, MetricDataValue>(iotext_data_obj: &'a IoTextDataRow<MetricDataValue>, names: &[&str]) -> HashMap<String, &'a MetricDataItem<MetricDataValue>> {
    iotext_data_obj.metrics.as_ref()
        .map_or(HashMap::new(), |metrics| {
            metrics.iter()
                .filter(|metric| names.contains(&metric.name.as_str()))
                .map(|metric| (metric.name.clone(), metric))
                .collect()
        })
}


fn get_metrics_values_by_names_as_hashmap_values<'a, MetricDataValue>(iotext_data_obj: &'a IoTextDataRow<MetricDataValue>, names: &[&str]) -> HashMap<String, &'a MetricDataValue> {
    iotext_data_obj.metrics.as_ref()
        .map_or(HashMap::new(), |metrics| {
            metrics.iter()
                .filter(|metric| names.contains(&metric.name.as_str()))
                .map(|metric| (metric.name.clone(), &metric.value))
                .collect()
        })
}

fn get_hashmap_for_metrics_by_names<'a, MetricDataValue>(iotext_data_obj: &'a IoTextDataRow, names: &[&str]) -> HashMap<String, MetricDataValue> {
    let mut found_metrics = HashMap::new();
    if let Some(metrics) = &iotext_data_obj.metrics {
        for metric in metrics {
            if names.contains(&metric.name.as_str()) {
                found_metrics.insert(metric.name.clone(), metric.value);
            }
        }
    }
    found_metrics
}
*/
//TODO: from IoText protocol remove panic!!!!!
fn metrics_decoder(iotext_data_obj: &IoTextDataRow, recived_iotext_data_row: String) -> Result<IoTextDataTurbineMetrics, ()> {
    let iotext_data_row = iotext_data_obj.parse_iotext_str(&recived_iotext_data_row.trim());
    println!("[DECODED] iotext_data_row: {:#?}", iotext_data_row);

    //TODO: Ok(iotext_data_mapper!(IoTextDataTurbineMetrics, iotext_data_row))
    let Some(speed_metric_data_item) = get_metric_by_name(&iotext_data_row, "speed") else { todo!() };
    
    println!("speed_metric_data_item.value.clone(): {}", speed_metric_data_item.clone());
    let speed: Decimal;
    match speed_metric_data_item.clone() {
        MetricValueType::DecimalItemType(val) => speed = val,
        _ => todo!(),
    }

    let Some(wind_dir_metric_data_item) = get_metric_by_name(&iotext_data_row, "wind_dir") else { todo!() };
    
    let wind_dir: String;
    match wind_dir_metric_data_item.clone() {
        MetricValueType::TextItemType(val) => wind_dir = val,
        _ => todo!(),
    }

    let Some(warnings_cnt_metric_data_item) = get_metric_by_name(&iotext_data_row, "warnings_cnt") else { todo!() };

    let warnings_cnt: i64;
    match warnings_cnt_metric_data_item.clone() {
        MetricValueType::IntegerItemType(val) => warnings_cnt = val,
        _ => todo!(),
    }
/*
    let names = ["speed", "wind_dir"];
    let filtered_metrics = get_hashmap_for_metrics_by_names(&iotext_data_row, &names);

    println!("filtered_metrics['speed']: {}", filtered_metrics["speed"]);
    let speed: Decimal;
    match filtered_metrics["speed"] {
        MetricValueType::DecimalItemType(val) => speed = *val,
        _ => todo!(),
    }

    println!("filtered_metrics['wind_dir']: {}", filtered_metrics["wind_dir"]);
    let wind_dir: String;
    match filtered_metrics["wind_dir"] {
        MetricValueType::TextItemType(val) => wind_dir = val.clone(),
        _ => todo!(),
    }
*/
    
    Ok(IoTextDataTurbineMetrics {
      speed: speed,
      wind_dir: wind_dir,
      //temp_out: Decimal::from_str("0.0").unwrap(),
      //temp_int_gen: 0.0,
      //temp_int_gear: 0.0,
      //gen_voltage_peak: Decimal::from_str("0.0").unwrap(),
      //gen_current_peak: Decimal::from_str("0.0").unwrap(),
      warnings_cnt: warnings_cnt,
    })
}

fn main() {
    raylib::set_trace_log(TraceLogLevel::LOG_NONE);

    let iotext_data_obj = IoTextDataRow::default();


/*
    let mut metrics: HashMap<String, MetricValueType> = HashMap::new();
    metrics.insert("speed".to_string(), MetricValueType::DecimalItemType(Decimal::new(100, 1)));
    metrics.insert("wind_dir".to_string(), MetricValueType::TextItemType("NNE".to_string()));
    metrics.insert("warnings_cnt".to_string(), MetricValueType::IntegerItemType(3));

    let turbine_metrics: IoTextDataTurbineMetrics = remap_metrics!(metrics, IoTextDataTurbineMetrics);

    println!("{:?}", turbine_metrics);
*/  


    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    let tcp_port = 8688;
    thread::spawn(move || {
        let listener = TcpListener::bind(format!("0.0.0.0:{tcp_port}")).unwrap();
        println!("Server TCP listining on port: {} ... (use simple connection like 'telnet 0.0.0.0 8688' and put IoText data + Enter)", tcp_port);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let sender = sender.clone();
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        break;
                    }
                    
                    let iot_ext_data: &str;
                    
                    match std::str::from_utf8(&buffer[..n]) {
                        Ok(s) => {
                            println!("[TCP_BUFFER] - data: '{}'", s);
                            iot_ext_data = s;
                            sender.send(iot_ext_data.to_string()).unwrap();
                        }
                        Err(e) => {
                            println!("[TCP_BUFFER] [ERROR] Failed to convert to UTF-8: '{}'", e);
                        }
                    }
                }
            });
        }
    });

    let (mut rl, thread) = raylib::init()
        .size(1000, 780)
        .title("Wind Turbine Remote Metrics - IoText Data Visualization")
        .build();

    let turbine_texture = rl.load_texture(&thread, "turbine.png").unwrap();
    let mut angle = 0.0;

    let mut turbine_metrics = IoTextDataTurbineMetrics {
      speed: Decimal::from_str("0.0").unwrap(),
      wind_dir: '*'.to_string(),
      //temp_out: Decimal::from_str("0.0").unwrap(),
      //temp_int_gen: 0.0,
      //temp_int_gear: 0.0,
      //gen_voltage_peak: Decimal::from_str("0.0").unwrap(),
      //gen_current_peak: Decimal::from_str("0.0").unwrap(),
      warnings_cnt: 0,
    };

    while !rl.window_should_close() {
        if let Ok(data) = receiver.try_recv() {
            println!("[RECIVED] data: '{:?}'", data);
            match metrics_decoder(&iotext_data_obj, data.to_string()) {
              Ok(metrics) => turbine_metrics = metrics,
              _ => todo!("[ERROR] [DECODED MSG] - error msg: ..."),
            }
            
            // Use the received data to update the display
        }

        // Check if ESC key is pressed
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
            break;
        }

        // Update the angle for the turbine animation
        angle += 10.0;
        if angle > 360.0 {
            angle = 0.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        // Draw wind turbine
        d.draw_texture_ex(
            &turbine_texture,
            Vector2::new(0.0, 0.0),
            0.0,
            1.0,
            Color::WHITE,
        );

        plot_metric_and_value(&mut d, "speed     : ".to_string(), format!("{}", turbine_metrics.speed), "m/s".to_string(), 1);
        plot_metric_and_value(&mut d, "wind_dir  : ".to_string(), format!("{}", turbine_metrics.wind_dir), "".to_string(), 4);
        //plot_metric_and_value(&mut d, "temp_out  : ".to_string(), format!("{}", turbine_metrics.speed), "°C".to_string(), 7);
        plot_metric_and_value(&mut d, "warns     : ".to_string(), format!("{}", turbine_metrics.warnings_cnt), "".to_string(), 7);

        /*
        d.draw_texture_ex(
            &turbine_texture,
            Vector2::new(400.0, 300.0),
            angle,
            1.0,
            Color::WHITE,
        );
        */

    }
}

