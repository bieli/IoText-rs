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

/// Connection to the dashboard and what was sent last.
struct Client {
    addr: String,
    state: State,
    stream: Option<TcpStream>,
    last_connect_try: Option<Instant>,
    /// `None` means "send now" (state changed or just connected).
    last_sent: Option<Instant>,
    last_message: String,
}

impl Client {
    fn new(addr: String) -> Self {
        Client {
            addr,
            state: State::default(),
            stream: None,
            last_connect_try: None,
            last_sent: None,
            last_message: String::from("-"),
        }
    }

    /// (Re)connects at most once per [`RECONNECT`].
    fn ensure_connected(&mut self) {
        if self.stream.is_some()
            || self
                .last_connect_try
                .is_some_and(|t| t.elapsed() < RECONNECT)
        {
            return;
        }
        self.last_connect_try = Some(Instant::now());
        if let Ok(stream) = TcpStream::connect(&self.addr) {
            self.stream = Some(stream);
            self.last_sent = None; // send the current state right away
        }
    }

    /// Waits up to [`KEY_POLL`] for a key press and applies it.
    fn poll_key(&mut self) -> io::Result<Action> {
        if !event::poll(KEY_POLL)? {
            return Ok(Action::None);
        }
        let action = match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                handle_key(&mut self.state, key.code)
            }
            _ => Action::None,
        };
        if matches!(action, Action::Changed) {
            self.last_sent = None;
        }
        Ok(action)
    }

    /// Sends the state if it changed, the heartbeat is due, or a corrupted message was requested.
    fn send_if_due(&mut self, corrupted: bool) -> io::Result<()> {
        let due = corrupted || self.last_sent.is_none_or(|t| t.elapsed() >= HEARTBEAT);
        let Some(stream) = self.stream.as_mut().filter(|_| due) else {
            return Ok(());
        };
        let msg = self
            .state
            .metrics()
            .to_message()
            .map_err(io::Error::other)?;
        let msg = if corrupted { corrupt(&msg) } else { msg };
        if stream.write_all(msg.as_bytes()).is_err() {
            self.stream = None; // dashboard went away: reconnect
            return Ok(());
        }
        self.last_sent = Some(Instant::now());
        let note = if corrupted { " (broken CRC)" } else { "" };
        self.last_message = format!("sent{note} {}", msg.trim_end());
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    print_help(&addr);
    let _raw = RawModeGuard::enable()?;

    let mut client = Client::new(addr);
    loop {
        client.ensure_connected();
        let action = client.poll_key()?;
        if matches!(action, Action::Quit) {
            break;
        }
        client.send_if_due(matches!(action, Action::SendCorrupted))?;
        status_line(
            &client.state,
            client.stream.is_some(),
            &client.addr,
            &client.last_message,
        )?;
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
