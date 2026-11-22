//! Wind turbine client: change the turbine state with the keyboard; every change (and a
//! heartbeat once per second) is sent to the dashboard as one IoText line with a CRC.
//!
//! Usage: `cargo run -p wind_turbine_client [-- <dashboard address>]`
//! (default address: 127.0.0.1:8688).

use std::io::{self, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute, queue, style};
use rust_decimal::Decimal;
use wind_turbine_common::{DEFAULT_ADDR, MAX_SPEED, TurbineMetrics, WindDir};

const HEARTBEAT: Duration = Duration::from_secs(1);
const RECONNECT: Duration = Duration::from_secs(1);
const KEY_POLL: Duration = Duration::from_millis(100);

/// Puts the terminal into raw mode and restores it on drop, also on errors and panics.
struct RawModeGuard;

impl RawModeGuard {
    fn enable() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), cursor::Hide)?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show);
        let _ = terminal::disable_raw_mode();
        println!();
    }
}

/// Turbine state controlled from the keyboard.
#[derive(Debug, Default)]
struct State {
    speed: u32,
    wind_dir: WindDir,
    warnings: u32,
}

impl State {
    fn metrics(&self) -> TurbineMetrics {
        TurbineMetrics {
            timestamp_ms: 0, // set by `to_message`
            speed: Decimal::from(self.speed),
            wind_dir: self.wind_dir,
            warnings: i64::from(self.warnings),
        }
    }
}

enum Action {
    Changed,
    SendCorrupted,
    Quit,
    None,
}

fn handle_key(state: &mut State, code: KeyCode) -> Action {
    match code {
        KeyCode::Up | KeyCode::Char('S') => state.speed = (state.speed + 1).min(MAX_SPEED),
        KeyCode::Down | KeyCode::Char('s') => state.speed = state.speed.saturating_sub(1),
        KeyCode::Right | KeyCode::Char('W') => state.wind_dir = state.wind_dir.clockwise(),
        KeyCode::Left | KeyCode::Char('w') => state.wind_dir = state.wind_dir.counter_clockwise(),
        KeyCode::Char('E') => state.warnings = state.warnings.saturating_add(1),
        KeyCode::Char('e') => state.warnings = state.warnings.saturating_sub(1),
        KeyCode::Char('x' | 'X') => return Action::SendCorrupted,
        KeyCode::Char('q' | 'Q') | KeyCode::Esc => return Action::Quit,
        _ => return Action::None,
    }
    Action::Changed
}

/// Flips one character of the message body so the dashboard's CRC check rejects it.
fn corrupt(msg: &str) -> String {
    msg.replacen("speed=d:", "speed=d:9", 1)
}

fn status_line(state: &State, connected: bool, addr: &str, last: &str) -> io::Result<()> {
    let link = if connected {
        format!("connected to {addr}")
    } else {
        format!("waiting for dashboard at {addr}...")
    };
    let mut out = io::stdout();
    queue!(
        out,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
        style::Print(format!(
            "speed {:>2} m/s | wind {} | warnings {} | {link} | {last}",
            state.speed, state.wind_dir, state.warnings
        ))
    )?;
    out.flush()
}

fn print_help(addr: &str) {
    // Printed before raw mode is enabled, so plain newlines work.
    println!("IoText wind turbine client -> dashboard at {addr}");
    println!("  Up/Down or S/s  speed +/-      Right/Left or W/w  wind direction");
    println!("  E/e             warnings +/-   x  send a message with a broken CRC");
    println!("  q / Esc         quit");
    println!();
}

fn main() -> io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    print_help(&addr);
    let _raw = RawModeGuard::enable()?;

    let mut state = State::default();
    let mut stream: Option<TcpStream> = None;
    let mut last_connect_try: Option<Instant> = None;
    let mut last_sent: Option<Instant> = None;
    let mut last = String::from("-");

    loop {
        if stream.is_none() && last_connect_try.is_none_or(|t| t.elapsed() >= RECONNECT) {
            last_connect_try = Some(Instant::now());
            if let Ok(s) = TcpStream::connect(&addr) {
                stream = Some(s);
                last_sent = None; // send the current state right away
            }
        }

        let mut send_corrupted = false;
        if event::poll(KEY_POLL)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match handle_key(&mut state, key.code) {
                        Action::Quit => break,
                        Action::Changed => last_sent = None,
                        Action::SendCorrupted => send_corrupted = true,
                        Action::None => {}
                    }
                }
            }
        }

        if let Some(s) = stream.as_mut() {
            if send_corrupted || last_sent.is_none_or(|t| t.elapsed() >= HEARTBEAT) {
                let msg = state.metrics().to_message().map_err(io::Error::other)?;
                let msg = if send_corrupted { corrupt(&msg) } else { msg };
                if s.write_all(msg.as_bytes()).is_ok() {
                    last_sent = Some(Instant::now());
                    last = format!(
                        "sent{} {}",
                        if send_corrupted { " (broken CRC)" } else { "" },
                        msg.trim_end()
                    );
                } else {
                    stream = None; // dashboard went away: reconnect
                }
            }
        }

        status_line(&state, stream.is_some(), &addr, &last)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wind_turbine_common::{DecodeError, decode_line};

    #[test]
    fn keys_clamp_speed_and_warnings() {
        let mut s = State::default();
        handle_key(&mut s, KeyCode::Down);
        handle_key(&mut s, KeyCode::Char('e'));
        assert_eq!((s.speed, s.warnings), (0, 0));
        for _ in 0..100 {
            handle_key(&mut s, KeyCode::Up);
        }
        assert_eq!(s.speed, MAX_SPEED);
    }

    #[test]
    fn right_arrow_turns_clockwise() {
        let mut s = State::default();
        handle_key(&mut s, KeyCode::Right);
        assert_eq!(s.wind_dir, WindDir::E);
        handle_key(&mut s, KeyCode::Left);
        handle_key(&mut s, KeyCode::Left);
        assert_eq!(s.wind_dir, WindDir::W);
    }

    #[test]
    fn corrupted_message_fails_crc_check() {
        let msg = State::default().metrics().to_message().unwrap();
        assert!(decode_line(&msg).is_ok());
        assert!(matches!(
            decode_line(&corrupt(&msg)),
            Err(DecodeError::Parse(_))
        ));
    }
}
