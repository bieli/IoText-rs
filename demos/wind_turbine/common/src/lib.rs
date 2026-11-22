//! Message format shared by the wind turbine demo client and dashboard.
//!
//! One turbine state is one IoText line, terminated by `\n`:
//!
//! ```text
//! t|1764000000000,d|wind_turbine_01,m|speed=d:12,m|wind_dir=t:N,m|warnings_cnt=i:0,c|1A2B
//! ```

use std::fmt;
use std::str::FromStr;

pub mod server;
pub mod view;

use iotext_rs::builder::IoTextBuilder;
use iotext_rs::{IoTextDataRow, IoTextError, MetricValueType};
use rust_decimal::Decimal;

/// Device id the client sends and the dashboard displays.
pub const DEVICE_ID: &str = "wind_turbine_01";
/// Default address the dashboard listens on and the client connects to.
pub const DEFAULT_ADDR: &str = "127.0.0.1:8688";
/// Upper bound for the speed the client can set, in m/s.
pub const MAX_SPEED: u32 = 60;

/// Metric names used in the messages.
pub mod metric {
    /// Wind speed in m/s (`d`).
    pub const SPEED: &str = "speed";
    /// Wind direction, one of `N`, `E`, `S`, `W` (`t`).
    pub const WIND_DIR: &str = "wind_dir";
    /// Number of active warnings (`i`).
    pub const WARNINGS: &str = "warnings_cnt";
}

/// Wind direction, in clockwise order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindDir {
    /// North.
    #[default]
    N,
    /// East.
    E,
    /// South.
    S,
    /// West.
    W,
}

impl WindDir {
    const CLOCKWISE: [WindDir; 4] = [WindDir::N, WindDir::E, WindDir::S, WindDir::W];

    fn index(self) -> usize {
        self as usize
    }

    /// Next direction clockwise (N → E → S → W → N).
    #[must_use]
    pub fn clockwise(self) -> Self {
        Self::CLOCKWISE[(self.index() + 1) % 4]
    }

    /// Next direction counter-clockwise (N → W → S → E → N).
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Self::CLOCKWISE[(self.index() + 3) % 4]
    }

    /// Compass rose rotation (degrees, clockwise) that brings this direction under
    /// the marker at the top of the dashboard.
    #[must_use]
    pub fn compass_rotation_deg(self) -> f32 {
        match self {
            WindDir::N => 0.0,
            WindDir::E => 270.0,
            WindDir::S => 180.0,
            WindDir::W => 90.0,
        }
    }

    /// Protocol value, e.g. `"N"`.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            WindDir::N => "N",
            WindDir::E => "E",
            WindDir::S => "S",
            WindDir::W => "W",
        }
    }
}

impl fmt::Display for WindDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for WindDir {
    type Err = DecodeError;

    /// Case-insensitive (`"n"` and `"N"` are both north).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::CLOCKWISE
            .into_iter()
            .find(|dir| dir.code().eq_ignore_ascii_case(s.trim()))
            .ok_or_else(|| DecodeError::UnknownWindDir(s.to_string()))
    }
}

/// Why a received line could not be shown on the dashboard.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DecodeError {
    /// Not a valid IoText message (includes CRC mismatches).
    #[error("invalid IoText message: {0}")]
    Parse(#[from] IoTextError),
    /// Valid message, but from another device.
    #[error("message from another device: '{0}'")]
    OtherDevice(String),
    /// A required metric is missing.
    #[error("missing metric '{0}'")]
    MissingMetric(&'static str),
    /// A metric has an unexpected data type.
    #[error("metric '{metric}' should be {expected}")]
    WrongType {
        /// Metric name.
        metric: &'static str,
        /// Expected IoText data type.
        expected: &'static str,
    },
    /// The wind direction is not one of N, E, S, W.
    #[error("unknown wind direction '{0}' (expected N, E, S or W)")]
    UnknownWindDir(String),
}

/// One turbine state, as sent by the client.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TurbineMetrics {
    /// UNIX timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// Wind speed in m/s.
    pub speed: Decimal,
    /// Wind direction.
    pub wind_dir: WindDir,
    /// Number of active warnings.
    pub warnings: i64,
}

impl TurbineMetrics {
    /// Encodes the state as one IoText line with the current time and a CRC,
    /// terminated by `\n`.
    ///
    /// # Errors
    /// Only fails if the system clock is before the UNIX epoch.
    pub fn to_message(&self) -> Result<String, IoTextError> {
        let row = IoTextBuilder::new()
            .current_timestamp()
            .device_id(DEVICE_ID)
            .add_metric(metric::SPEED, self.speed)
            .add_metric(metric::WIND_DIR, self.wind_dir.code())
            .add_metric(metric::WARNINGS, self.warnings)
            .build(true)?;
        Ok(format!("{row}\n"))
    }
}

impl TryFrom<&IoTextDataRow> for TurbineMetrics {
    type Error = DecodeError;

    fn try_from(row: &IoTextDataRow) -> Result<Self, Self::Error> {
        if row.device_id != DEVICE_ID {
            return Err(DecodeError::OtherDevice(row.device_id.clone()));
        }
        let get = |name: &'static str| row.metric(name).ok_or(DecodeError::MissingMetric(name));

        let speed = match get(metric::SPEED)? {
            MetricValueType::DecimalItemType(v) => *v,
            MetricValueType::IntegerItemType(v) => Decimal::from(*v),
            _ => {
                return Err(DecodeError::WrongType {
                    metric: metric::SPEED,
                    expected: "a number (d or i)",
                });
            }
        };
        let wind_dir = match get(metric::WIND_DIR)? {
            MetricValueType::TextItemType(v) => v.parse()?,
            _ => {
                return Err(DecodeError::WrongType {
                    metric: metric::WIND_DIR,
                    expected: "text (t)",
                });
            }
        };
        let warnings = match get(metric::WARNINGS)? {
            MetricValueType::IntegerItemType(v) => *v,
            _ => {
                return Err(DecodeError::WrongType {
                    metric: metric::WARNINGS,
                    expected: "an integer (i)",
                });
            }
        };

        Ok(TurbineMetrics {
            timestamp_ms: row.timestamp,
            speed,
            wind_dir,
            warnings,
        })
    }
}

/// Parses one received line (surrounding whitespace, e.g. telnet's `\r\n`, is ignored).
///
/// # Errors
/// See [`DecodeError`].
pub fn decode_line(line: &str) -> Result<TurbineMetrics, DecodeError> {
    let row: IoTextDataRow = line.trim().parse()?;
    TurbineMetrics::try_from(&row)
}

/// Formats a millisecond UNIX timestamp in local time, e.g. `2026-11-22 15:04:05.123`.
#[must_use]
pub fn format_timestamp(timestamp_ms: u64) -> String {
    i64::try_from(timestamp_ms)
        .ok()
        .and_then(chrono::DateTime::from_timestamp_millis)
        .map_or_else(
            || "-".to_string(),
            |utc| {
                utc.with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M:%S%.3f")
                    .to_string()
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TurbineMetrics {
        TurbineMetrics {
            timestamp_ms: 0,
            speed: Decimal::new(125, 1),
            wind_dir: WindDir::W,
            warnings: 2,
        }
    }

    #[test]
    fn message_roundtrip() {
        let msg = sample().to_message().unwrap();
        assert!(msg.ends_with('\n'));
        assert!(msg.contains(",c|"), "message carries a CRC: {msg}");

        let decoded = decode_line(&msg).unwrap();
        assert!(decoded.timestamp_ms > 0);
        assert_eq!(
            TurbineMetrics {
                timestamp_ms: 0,
                ..decoded
            },
            sample()
        );
    }

    #[test]
    fn accepts_telnet_line_without_crc_and_lowercase_direction() {
        let m =
            decode_line("t|1,d|wind_turbine_01,m|speed=i:7,m|wind_dir=t:e,m|warnings_cnt=i:0\r\n")
                .unwrap();
        assert_eq!(
            (m.speed, m.wind_dir, m.warnings),
            (Decimal::from(7), WindDir::E, 0)
        );
    }

    #[test]
    fn rejects_corrupted_crc() {
        let msg = sample()
            .to_message()
            .unwrap()
            .replacen("wind_dir=t:W", "wind_dir=t:N", 1);
        assert!(matches!(
            decode_line(&msg),
            Err(DecodeError::Parse(IoTextError::CrcMismatch { .. }))
        ));
    }

    #[test]
    fn decode_errors() {
        let base = "t|1,d|wind_turbine_01";
        assert!(matches!(decode_line("garbage"), Err(DecodeError::Parse(_))));
        assert_eq!(
            decode_line("t|1,d|other,m|speed=i:1,m|wind_dir=t:N,m|warnings_cnt=i:0"),
            Err(DecodeError::OtherDevice("other".into()))
        );
        assert_eq!(
            decode_line(&format!("{base},m|wind_dir=t:N,m|warnings_cnt=i:0")),
            Err(DecodeError::MissingMetric(metric::SPEED))
        );
        assert!(matches!(
            decode_line(&format!(
                "{base},m|speed=t:fast,m|wind_dir=t:N,m|warnings_cnt=i:0"
            )),
            Err(DecodeError::WrongType {
                metric: metric::SPEED,
                ..
            })
        ));
        assert_eq!(
            decode_line(&format!(
                "{base},m|speed=i:1,m|wind_dir=t:NE,m|warnings_cnt=i:0"
            )),
            Err(DecodeError::UnknownWindDir("NE".into()))
        );
    }

    #[test]
    fn wind_dir_rotation_and_order() {
        assert_eq!(WindDir::N.clockwise(), WindDir::E);
        assert_eq!(WindDir::W.clockwise(), WindDir::N);
        assert_eq!(WindDir::N.counter_clockwise(), WindDir::W);
        for dir in WindDir::CLOCKWISE {
            assert_eq!(dir.clockwise().counter_clockwise(), dir);
            assert_eq!(dir.code().parse::<WindDir>().unwrap(), dir);
        }
    }

    #[test]
    fn timestamp_formatting() {
        assert_eq!(format_timestamp(u64::MAX), "-");
        assert_eq!(
            format_timestamp(1_000).len(),
            "1970-01-01 00:00:01.000".len()
        );
    }
}
