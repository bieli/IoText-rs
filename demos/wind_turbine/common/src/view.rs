//! Renderer-independent dashboard state: what to show and how it animates.
//! The macroquad and raylib dashboards only draw this.

use std::time::{Duration, Instant};

use rust_decimal::prelude::ToPrimitive;

use crate::server::NetEvent;
use crate::{TurbineMetrics, format_timestamp};

/// Rotor revolutions per second for each m/s of wind speed.
const ROTOR_RPS_PER_MS: f32 = 0.05;
/// How fast the compass turns towards a new wind direction (degrees per second).
const COMPASS_DEG_PER_SEC: f32 = 270.0;
/// The last update is shown as stale (and the rotor stops) after this long.
pub const STALE_AFTER: Duration = Duration::from_secs(3);

/// Severity of the status line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Data is flowing.
    Ok,
    /// No data yet, or no update for [`STALE_AFTER`].
    Waiting,
    /// The last received line was rejected.
    Error,
}

/// Dashboard state.
#[derive(Debug)]
pub struct DashboardView {
    /// Last successfully decoded turbine state.
    pub metrics: Option<TurbineMetrics>,
    last_update: Option<Instant>,
    last_error: Option<(String, Instant)>,
    /// Number of accepted messages.
    pub accepted: u64,
    /// Number of rejected lines.
    pub rejected: u64,
    /// Number of connected clients.
    pub clients: usize,
    /// Rotor angle in degrees, `0..360`.
    pub rotor_deg: f32,
    /// Current (animated) compass rotation in degrees, `0..360`.
    pub compass_deg: f32,
    listen_addr: String,
}

impl DashboardView {
    /// New view for a dashboard listening on `listen_addr`.
    #[must_use]
    pub fn new(listen_addr: impl Into<String>) -> Self {
        DashboardView {
            metrics: None,
            last_update: None,
            last_error: None,
            accepted: 0,
            rejected: 0,
            clients: 0,
            rotor_deg: 0.0,
            compass_deg: 0.0,
            listen_addr: listen_addr.into(),
        }
    }

    /// Applies an event from the listener.
    pub fn apply(&mut self, event: NetEvent, now: Instant) {
        match event {
            NetEvent::Connected(_) => self.clients += 1,
            NetEvent::Disconnected(_) => self.clients = self.clients.saturating_sub(1),
            NetEvent::Line { result: Ok(m), .. } => {
                // A rejection stays visible for STALE_AFTER even if valid data keeps coming.
                self.metrics = Some(m);
                self.last_update = Some(now);
                self.accepted += 1;
            }
            NetEvent::Line {
                raw,
                result: Err(e),
            } => self.reject(format!("{e} <- {}", shorten(&raw)), now),
            NetEvent::Invalid(reason) => self.reject(reason, now),
        }
    }

    fn reject(&mut self, message: String, now: Instant) {
        self.last_error = Some((message, now));
        self.rejected += 1;
    }

    /// Advances the animations by `dt` seconds.
    pub fn tick(&mut self, dt: f32, now: Instant) {
        if !self.is_stale(now) {
            let speed = self.speed_f32();
            self.rotor_deg =
                (self.rotor_deg + speed * ROTOR_RPS_PER_MS * 360.0 * dt).rem_euclid(360.0);
        }
        let target = self
            .metrics
            .as_ref()
            .map_or(0.0, |m| m.wind_dir.compass_rotation_deg());
        // Shortest way round, at a limited angular speed.
        let diff = (target - self.compass_deg + 540.0).rem_euclid(360.0) - 180.0;
        let step = COMPASS_DEG_PER_SEC * dt;
        self.compass_deg = if diff.abs() <= step {
            target
        } else {
            self.compass_deg + step * diff.signum()
        }
        .rem_euclid(360.0);
    }

    fn is_stale(&self, now: Instant) -> bool {
        self.last_update
            .is_none_or(|t| now.duration_since(t) > STALE_AFTER)
    }

    fn speed_f32(&self) -> f32 {
        self.metrics
            .as_ref()
            .and_then(|m| m.speed.to_f32())
            .unwrap_or(0.0)
            .max(0.0)
    }

    /// Rows of the metrics table: (label, value).
    #[must_use]
    pub fn rows(&self) -> Vec<(&'static str, String)> {
        let dash = || "-".to_string();
        let m = self.metrics.as_ref();
        vec![
            (
                "time",
                m.map_or_else(dash, |m| format_timestamp(m.timestamp_ms)),
            ),
            ("speed", m.map_or_else(dash, |m| format!("{} m/s", m.speed))),
            ("wind_dir", m.map_or_else(dash, |m| m.wind_dir.to_string())),
            ("warnings", m.map_or_else(dash, |m| m.warnings.to_string())),
        ]
    }

    /// Status line and its severity.
    #[must_use]
    pub fn status(&self, now: Instant) -> (Status, String) {
        let counts = format!(
            "ok {} / rejected {} / clients {}",
            self.accepted, self.rejected, self.clients
        );
        if let Some((err, at)) = &self.last_error {
            if now.duration_since(*at) < STALE_AFTER {
                return (Status::Error, format!("REJECTED: {err}   [{counts}]"));
            }
        }
        match self.last_update {
            None => (
                Status::Waiting,
                format!(
                    "waiting for IoText data on {}   [{counts}]",
                    self.listen_addr
                ),
            ),
            Some(t) if self.is_stale(now) => (
                Status::Waiting,
                format!(
                    "no update for {} s   [{counts}]",
                    now.duration_since(t).as_secs()
                ),
            ),
            Some(_) => (Status::Ok, format!("receiving   [{counts}]")),
        }
    }
}

fn shorten(s: &str) -> String {
    const MAX: usize = 60;
    if s.chars().count() <= MAX {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(MAX).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecodeError, WindDir};
    use rust_decimal::Decimal;

    fn ok(speed: i64, dir: WindDir) -> NetEvent {
        let m = TurbineMetrics {
            speed: Decimal::from(speed),
            wind_dir: dir,
            ..Default::default()
        };
        NetEvent::Line {
            raw: String::new(),
            result: Ok(m),
        }
    }

    #[test]
    fn counts_and_status() {
        let t0 = Instant::now();
        let mut v = DashboardView::new("127.0.0.1:8688");
        assert_eq!(v.status(t0).0, Status::Waiting);

        v.apply(ok(10, WindDir::E), t0);
        assert_eq!(v.status(t0).0, Status::Ok);
        assert_eq!(v.rows()[1], ("speed", "10 m/s".to_string()));

        v.apply(
            NetEvent::Line {
                raw: "x".into(),
                result: Err(DecodeError::MissingMetric("speed")),
            },
            t0,
        );
        assert_eq!(v.status(t0).0, Status::Error);
        assert_eq!((v.accepted, v.rejected), (1, 1));
        assert_eq!(
            v.metrics.as_ref().unwrap().wind_dir,
            WindDir::E,
            "last good state is kept"
        );

        // A valid message right after the rejection does not hide it.
        v.apply(ok(11, WindDir::E), t0 + Duration::from_secs(1));
        assert_eq!(v.status(t0 + Duration::from_secs(1)).0, Status::Error);

        // Error expired, data still fresh (last update at t0 + 1 s).
        let after_error = t0 + STALE_AFTER + Duration::from_millis(500);
        assert_eq!(v.status(after_error).0, Status::Ok);

        let stale = t0 + Duration::from_secs(1) + STALE_AFTER + Duration::from_secs(1);
        assert_eq!(v.status(stale).0, Status::Waiting);
    }

    #[test]
    fn rotor_spins_with_speed_and_stops_when_stale() {
        let t0 = Instant::now();
        let mut v = DashboardView::new("");
        v.apply(ok(20, WindDir::N), t0);
        v.tick(0.1, t0);
        assert!((v.rotor_deg - 20.0 * ROTOR_RPS_PER_MS * 360.0 * 0.1).abs() < 1e-3);

        let before = v.rotor_deg;
        v.tick(0.1, t0 + STALE_AFTER + Duration::from_secs(1));
        assert!((v.rotor_deg - before).abs() < f32::EPSILON);
    }

    #[test]
    fn compass_turns_the_short_way_and_settles() {
        let t0 = Instant::now();
        let mut v = DashboardView::new("");
        v.apply(ok(1, WindDir::W), t0); // target 90
        v.tick(0.1, t0);
        assert!(v.compass_deg > 0.0 && v.compass_deg < 90.0);
        for _ in 0..20 {
            v.tick(0.1, t0);
        }
        assert!((v.compass_deg - 90.0).abs() < f32::EPSILON);

        v.apply(ok(1, WindDir::E), t0); // target 270: from 90 either way is 180
        for _ in 0..20 {
            v.tick(0.1, t0);
        }
        assert!((v.compass_deg - 270.0).abs() < f32::EPSILON);
    }
}
