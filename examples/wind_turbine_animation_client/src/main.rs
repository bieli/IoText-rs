use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{ExecutableCommand};
use std::io::stdout;


use rust_decimal::prelude::*;
use iotext_rs::*;
use iotext_rs::Item;
use iotext_rs::ItemTypeEnum;

use chrono::Utc;


const device_id: &str = "wind_turbine_01";


fn get_unix_timestamp_with_milliseconds() -> u64 {
    let now = Utc::now();
    now.timestamp_millis().try_into().unwrap()
}


#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnableMouseCapture)?;

    let addr = "127.0.0.1:8688";

    let mut stream = TcpStream::connect(addr).await?;

    let mut speed: i32 = 0;
    let wind_dirs = vec!["N", "E", "S", "W"];
    let mut wind_dir_index = 0;
    let mut warnings_cnt: i32 = 0;

    async fn send_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
        stream.write_all(message.as_bytes()).await?;
        Ok(())
    }

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('S') | KeyCode::Up => {
                        speed += 1;
                        //println!("Speed increased to {}\n", speed);
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        speed -= 1;
                        //println!("Speed decreased to {}\n", speed);
                    }
                    KeyCode::Char('W') | KeyCode::Left => {
                        wind_dir_index = (wind_dir_index + 1) % wind_dirs.len();
                        //println!("Wind direction changed to {}\n", wind_dirs[wind_dir_index]);
                    }
                    KeyCode::Char('w') | KeyCode::Right => {
                        wind_dir_index = (wind_dir_index + wind_dirs.len() - 1) % wind_dirs.len();
                        //println!("Wind direction changed to {}\n", wind_dirs[wind_dir_index]);
                    }
                    KeyCode::Char('E') => {
                        warnings_cnt += 1;
                        //println!("Warnings count increased to {}\n", warnings_cnt);
                    }
                    KeyCode::Char('e') => {
                        warnings_cnt -= 1;
                        //println!("Warnings count decreased to {}\n", warnings_cnt);
                    }
                     KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        //println!("Exiting...\n\n");
                        break;
                    }
                    _ => {}
                }

                // Send the current state to the server
                //let msg = format!(
                //    "Speed: {}, Wind Direction: {}, Warnings Count: {}\n",
                //    speed, wind_dirs[wind_dir_index], warnings_cnt
                //);

                let mut metrics:Vec<MetricDataItem> = Default::default();
                let _ = metrics.push(
                  MetricDataItem{
                    name: "speed".to_string(),
                    value: MetricValueType::DecimalItemType(Decimal::new(speed.into(), 0)),
                });

                metrics.push(
                  MetricDataItem{
                    name: "wind_dir".to_string(),
                    value: MetricValueType::TextItemType(wind_dirs[wind_dir_index].to_string()),
                });

                metrics.push(
                  MetricDataItem{
                    name: "warnings_cnt".to_string(),
                    value: MetricValueType::IntegerItemType(warnings_cnt.into()),
                });

                let iotext_data_row = IoTextDataRow {
                    timestamp: Item {
                        value: ItemTypeEnum::TimeUnixMilis(get_unix_timestamp_with_milliseconds()),
                    },
                    device_id: Item {
                        value: ItemTypeEnum::DeviceId(device_id.to_string()),
                    },
                    metrics: Some(metrics),
                    crc16: None,
                };

                let iotext_data_msg = iotext_data_row.dump_iotext_to_str(&iotext_data_row, false);
                println!("\n{}\n", iotext_data_msg);

                send_message(&mut stream, &iotext_data_msg).await?;
            }
        }

        sleep(Duration::from_millis(100)).await;
    }

    disable_raw_mode()?;
    stdout.execute(DisableMouseCapture)?;
    Ok(())
}

