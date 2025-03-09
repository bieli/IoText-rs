use std::*;
use std::io::{Read};
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc, Local};

use raylib::prelude::*;
use iotext_rs::*;
use rust_decimal::prelude::*;

use std::collections::HashMap;
use std::vec::Vec;
use std::string::String;

use bevy_reflect::{Reflect, ReflectMut, DynamicStruct, FromReflect, TypeRegistryArc};


// t|3900237526042,d|wind_turbine_01,m|speed=d:12.00,m|wind_dir=t:n,m|temp_out=i:18,m|temp_int_gen=i:35,m|temp_int_gear=i:47,m|gen_voltage_peak=i:235.00,m|gen_current_peak=d:45.50

// openocd -f interface/picoprobe.cfg -f target/rp2040.cfg -c "program app verify reset exit"
const device_id_filter: &str = "wind_turbine_01";


//#[derive(Debug, Reflect, Default)]
//#[reflect(Default)]
#[derive(Debug, Default)]
struct IoTextDataTurbineMetrics {
    timestamp: u64,
    speed: Decimal,
    wind_dir: String,
    //temp_out: Decimal,
    //temp_int_gen: Decimal,
    //temp_int_gear: Decimal,
    //gen_voltage_peak: Decimal,
    //gen_current_peak: Decimal,
    warnings_cnt: i64,
}

#[derive(Debug, Default)]
struct WindDir {
    pos: Vector2,
    ang: f32
}


fn timestamp_to_datetime_string(timestamp: u64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp((timestamp / 1000) as i64, ((timestamp % 1000) * 1_000_000) as u32);
    let utc_datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    let local_datetime: DateTime<Local> = utc_datetime.with_timezone(&Local);
    local_datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

/*
fn timestamp_to_datetime_string<Utc: chrono::TimeZone<Offset = Utc>>(timestamp: u64) -> String where Utc: std::fmt::Display {
    let naive_datetime = NaiveDateTime::from_timestamp((timestamp / 1000).try_into().unwrap(), ((timestamp % 1000) * 1_000_000) as u32);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

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
        &format!("{}", name),
        23,
        oY + 3,
        40,
        Color::BLACK,
    );

    d.draw_text(
        &format!("{}", name),
        20,
        oY,
        40,
        Color::WHITE,
    );
    
    d.draw_text(
        &format!(": {} {}", value, unit),
        213,
        oY + 3,
        40,
        Color::BLACK,
    );

    d.draw_text(
        &format!(": {} {}", value, unit),
        210,
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
fn metrics_decoder(device_id_filter_var: &str, iotext_data_obj: &IoTextDataRow, recived_iotext_data_row: String) -> Result<IoTextDataTurbineMetrics, &'static str> {
    let iotext_data_row = iotext_data_obj.parse_iotext_str(&recived_iotext_data_row.trim());
    println!("[DECODED] iotext_data_row: {:#?}", iotext_data_row);

    let device_id_item = &iotext_data_row.device_id.value;
    let mut device_id: String = String::new();

    println!("device_id_item: {}", device_id_item);
    let speed: Decimal;
    match device_id_item {
        ItemTypeEnum::DeviceId(val) => device_id = val.clone().to_string(),
        _ => todo!(),
    }
        
    if device_id == device_id_filter_var.to_string() {
      println!("device_id: {}", device_id);
      let mut speed: Decimal = Decimal::new(0, 0);
      let mut wind_dir: String = String::new();
      let mut warnings_cnt: i64 = 0;
      let mut timestamp: u64 = 0;

      let timestamp_item = &iotext_data_row.timestamp.value;

      println!("timestamp_item: {}", timestamp_item);
      let speed: Decimal;
      match timestamp_item {
          ItemTypeEnum::TimeUnixMilis(val) => timestamp = *val,
          _ => todo!(),
      }
      
      println!("timestamp: {}", timestamp);
    
      //TODO: Ok(iotext_data_mapper!(IoTextDataTurbineMetrics, iotext_data_row))
      let Some(speed_metric_data_item) = get_metric_by_name(&iotext_data_row, "speed") else { todo!() };
      
      println!("speed_metric_data_item.value.clone(): {}", speed_metric_data_item.clone());
      match speed_metric_data_item.clone() {
          MetricValueType::DecimalItemType(val) => speed = val,
          _ => todo!(),
      }

      let Some(wind_dir_metric_data_item) = get_metric_by_name(&iotext_data_row, "wind_dir") else { todo!() };
      
      match wind_dir_metric_data_item.clone() {
          MetricValueType::TextItemType(val) => wind_dir = val,
          _ => todo!(),
      }

      let Some(warnings_cnt_metric_data_item) = get_metric_by_name(&iotext_data_row, "warnings_cnt") else { todo!() };

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
    return Ok(IoTextDataTurbineMetrics {
      timestamp: timestamp,
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
  Err("device_id not matched!")
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
    let wind_dir_texture = rl.load_texture(&thread, "wind_dir1.png").unwrap();
    let mut angle = 0.0;

    let mut turbine_metrics = IoTextDataTurbineMetrics {
      timestamp: 0,
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
            match metrics_decoder(device_id_filter, &iotext_data_obj, data.to_string()) {
              Ok(metrics) => turbine_metrics = metrics,
              Err(err_msg) => println!("[ERROR] [DECODED MSG] - error msg: {}", err_msg),
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

        let mut wind_dir_angles_map: HashMap<String, f32> = HashMap::new();
        wind_dir_angles_map.insert("N".to_string(), 0.0);
        wind_dir_angles_map.insert("W".to_string(), 90.0);
        wind_dir_angles_map.insert("S".to_string(), 180.0);
        wind_dir_angles_map.insert("E".to_string(), 270.0);

        let mut wind_dir_tmp = turbine_metrics.wind_dir.to_uppercase();
        if wind_dir_tmp == "" || wind_dir_tmp == "*" {
          wind_dir_tmp = "W".to_string();
        }

        //d.draw_texture_ex(
        //    &wind_dir_texture,
        //    wind_dir_hm[wind_dir_tmp].pos,
        //    wind_dir_hm[wind_dir_tmp].ang,
        //    1.0,
        //    Color::BLUE,
        //);

        let xx = 785.0;
        let rotation_angle = wind_dir_angles_map[&wind_dir_tmp];

        // Calculate the rotation point (center of the texture)
        let center = Vector2::new(wind_dir_texture.width as f32 / 2.0, wind_dir_texture.height as f32 / 2.0);

        // Calculate the destination rectangle (positioning the texture in the middle of the screen)
        let dest_rect = Rectangle::new(
            xx,
            240.0,
            wind_dir_texture.width as f32,
            wind_dir_texture.height as f32,
        );
        
        let wind_dir_texture_source_rect = Rectangle::new(0.0, 0.0, wind_dir_texture.width as f32, wind_dir_texture.height as f32);
        
        d.draw_texture_pro(
            &wind_dir_texture,
            wind_dir_texture_source_rect,
            dest_rect,
            center,
            rotation_angle,
            Color::BLUE,
        );


        d.draw_triangle(
            Vector2::new(xx + 50.0, 20.0),
            Vector2::new(xx - 50.0, 20.0),
            Vector2::new(xx, 50.0),
            Color::BLACK,
        );
        

        plot_metric_and_value(&mut d, "TS".to_string(), format!("{}", timestamp_to_datetime_string(turbine_metrics.timestamp)), "".to_string(), 1);        
        plot_metric_and_value(&mut d, "speed".to_string(), format!("{}", turbine_metrics.speed), "m/s".to_string(), 4);
        plot_metric_and_value(&mut d, "wind_dir".to_string(), format!("{}", turbine_metrics.wind_dir.to_uppercase()), "".to_string(), 7);
        //plot_metric_and_value(&mut d, "temp_out  : ".to_string(), format!("{}", turbine_metrics.speed), "°C".to_string(), 7);
        plot_metric_and_value(&mut d, "warns".to_string(), format!("{}", turbine_metrics.warnings_cnt), "".to_string(), 10);

        plot_metric_and_value(&mut d, "device_id".to_string(), format!("{}", device_id_filter), "".to_string(), 36);

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

