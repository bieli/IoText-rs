//! TCP listener for the dashboards: one IoText message per `\n`-terminated line,
//! any number of clients (the demo client, or `telnet 127.0.0.1 8688`).

use std::io::{self, BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::{DecodeError, TurbineMetrics, decode_line};

/// Longer lines are rejected (protects the dashboard from unbounded input).
pub const MAX_LINE_LEN: usize = 4096;

/// What the listener reports to the dashboard.
#[derive(Debug)]
pub enum NetEvent {
    /// A client connected.
    Connected(SocketAddr),
    /// A client disconnected.
    Disconnected(SocketAddr),
    /// A line was received and decoded (or failed to decode).
    Line {
        /// The received line, without the line terminator.
        raw: String,
        /// Decoding result.
        result: Result<TurbineMetrics, DecodeError>,
    },
    /// A line was too long or not valid UTF-8.
    Invalid(String),
}

/// Binds `addr` and handles clients on background threads.
///
/// Returns the bound address (useful with port 0) and the event receiver.
///
/// # Errors
/// Fails if the address cannot be bound.
pub fn spawn_listener(addr: &str) -> io::Result<(SocketAddr, Receiver<NetEvent>)> {
    let listener = TcpListener::bind(addr)?;
    let local = listener.local_addr()?;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let tx = tx.clone();
            thread::spawn(move || handle_client(stream, &tx));
        }
    });
    Ok((local, rx))
}

fn handle_client(stream: TcpStream, tx: &Sender<NetEvent>) {
    let Ok(peer) = stream.peer_addr() else { return };
    if tx.send(NetEvent::Connected(peer)).is_err() {
        return;
    }
    let mut reader = BufReader::new(stream);
    let mut buf = Vec::new();
    loop {
        buf.clear();
        // Read at most MAX_LINE_LEN + 1 bytes of the next line.
        let limit = (MAX_LINE_LEN + 1) as u64;
        let n = match reader.by_ref().take(limit).read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        let event = if buf.last() != Some(&b'\n') && n > MAX_LINE_LEN {
            skip_rest_of_line(&mut reader);
            NetEvent::Invalid(format!("line longer than {MAX_LINE_LEN} bytes"))
        } else {
            match std::str::from_utf8(&buf) {
                Ok(text) => {
                    let raw = text.trim_end_matches(['\r', '\n']).to_string();
                    if raw.trim().is_empty() {
                        continue;
                    }
                    let result = decode_line(&raw);
                    NetEvent::Line { raw, result }
                }
                Err(_) => NetEvent::Invalid("line is not valid UTF-8".into()),
            }
        };
        if tx.send(event).is_err() {
            return;
        }
    }
    let _ = tx.send(NetEvent::Disconnected(peer));
}

fn skip_rest_of_line(reader: &mut BufReader<TcpStream>) {
    let mut sink = Vec::new();
    loop {
        sink.clear();
        match reader
            .by_ref()
            .take(MAX_LINE_LEN as u64)
            .read_until(b'\n', &mut sink)
        {
            Ok(0) | Err(_) => return,
            Ok(_) if sink.last() == Some(&b'\n') => return,
            Ok(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TurbineMetrics, WindDir};
    use std::io::Write;
    use std::time::Duration;

    fn next(rx: &Receiver<NetEvent>) -> NetEvent {
        rx.recv_timeout(Duration::from_secs(5)).expect("event")
    }

    #[test]
    fn receives_framed_lines_from_a_client() {
        let (addr, rx) = spawn_listener("127.0.0.1:0").unwrap();
        let mut client = TcpStream::connect(addr).unwrap();
        assert!(matches!(next(&rx), NetEvent::Connected(_)));

        let state = TurbineMetrics {
            wind_dir: WindDir::S,
            ..Default::default()
        };
        // Two messages in one write must arrive as two events.
        let two = format!(
            "{}{}",
            state.to_message().unwrap(),
            state.to_message().unwrap()
        );
        client.write_all(two.as_bytes()).unwrap();
        for _ in 0..2 {
            match next(&rx) {
                NetEvent::Line { result: Ok(m), .. } => assert_eq!(m.wind_dir, WindDir::S),
                other => panic!("unexpected {other:?}"),
            }
        }

        // telnet-style line, blank line (ignored), garbage, too-long line.
        client
            .write_all(
                b"t|1,d|wind_turbine_01,m|speed=i:3,m|wind_dir=t:n,m|warnings_cnt=i:1\r\n\r\n",
            )
            .unwrap();
        assert!(matches!(next(&rx), NetEvent::Line { result: Ok(_), .. }));
        client.write_all(b"hello\n").unwrap();
        assert!(matches!(next(&rx), NetEvent::Line { result: Err(_), .. }));
        client.write_all(&vec![b'a'; MAX_LINE_LEN * 3]).unwrap();
        client.write_all(b"\n").unwrap();
        assert!(matches!(next(&rx), NetEvent::Invalid(_)));

        drop(client);
        assert!(matches!(next(&rx), NetEvent::Disconnected(_)));
    }
}
