//! Wind turbine dashboard (macroquad): listens for IoText messages over TCP and
//! visualizes the turbine state.
//!
//! Usage: `cargo run -p wind_turbine_dashboard [-- <listen address>]`
//! (default: 127.0.0.1:8688; use 0.0.0.0:8688 to accept other machines).

use std::sync::mpsc::Receiver;
use std::time::Instant;

use macroquad::prelude::*;
use wind_turbine_common::server::{NetEvent, spawn_listener};
use wind_turbine_common::view::{DashboardView, Status};
use wind_turbine_common::{DEFAULT_ADDR, DEVICE_ID};

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 780.0;
const COMPASS_CENTER: Vec2 = Vec2::new(800.0, 250.0);
const COMPASS_SIZE: f32 = 330.0;
const ROTOR_CENTER: Vec2 = Vec2::new(830.0, 590.0);
const ROTOR_RADIUS: f32 = 95.0;
const STATUS_BAR_H: f32 = 40.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Wind Turbine Remote Metrics - IoText Data Visualization".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn texture(bytes: &[u8]) -> Texture2D {
    let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    t.set_filter(FilterMode::Linear);
    t
}

/// Text with a dark shadow, readable on the photo.
fn shadow_text(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x + 3.0, y + 3.0, size, BLACK);
    draw_text(text, x, y, size, color);
}

/// Draws the photo scaled to cover the window (cropping the sides).
fn draw_background(photo: &Texture2D) {
    let scale = (WIDTH / photo.width()).max(HEIGHT / photo.height());
    let size = vec2(photo.width(), photo.height()) * scale;
    let params = DrawTextureParams {
        dest_size: Some(size),
        ..Default::default()
    };
    draw_texture_ex(
        photo,
        (WIDTH - size.x) / 2.0,
        (HEIGHT - size.y) / 2.0,
        WHITE,
        params,
    );
}

fn draw_compass(compass: &Texture2D, rotation_deg: f32) {
    let top_left = COMPASS_CENTER - Vec2::splat(COMPASS_SIZE / 2.0);
    // The image has a thin frame along its edges: crop it away.
    let inset = 6.0;
    let params = DrawTextureParams {
        dest_size: Some(Vec2::splat(COMPASS_SIZE)),
        source: Some(Rect::new(
            inset,
            inset,
            compass.width() - 2.0 * inset,
            compass.height() - 2.0 * inset,
        )),
        rotation: rotation_deg.to_radians(), // clockwise, around the center
        ..Default::default()
    };
    draw_texture_ex(
        compass,
        top_left.x,
        top_left.y,
        Color::new(0.75, 0.85, 1.0, 1.0),
        params,
    );
    // Marker: the direction under it is where the wind comes from.
    let tip_y = COMPASS_CENTER.y - COMPASS_SIZE / 2.0 - 4.0;
    draw_triangle(
        vec2(COMPASS_CENTER.x - 28.0, tip_y - 34.0),
        vec2(COMPASS_CENTER.x + 28.0, tip_y - 34.0),
        vec2(COMPASS_CENTER.x, tip_y),
        BLACK,
    );
}

/// A small three-blade rotor that spins with the reported wind speed.
fn draw_rotor(rotor_deg: f32) {
    draw_poly(
        ROTOR_CENTER.x,
        ROTOR_CENTER.y,
        64,
        ROTOR_RADIUS + 12.0,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.35),
    );
    for blade in 0..3 {
        let a = (rotor_deg + blade as f32 * 120.0).to_radians();
        let dir = vec2(a.sin(), -a.cos());
        let side = vec2(dir.y, -dir.x) * 9.0;
        let tip = ROTOR_CENTER + dir * ROTOR_RADIUS;
        draw_triangle(ROTOR_CENTER + side, ROTOR_CENTER - side, tip, WHITE);
    }
    draw_poly(ROTOR_CENTER.x, ROTOR_CENTER.y, 32, 14.0, 0.0, LIGHTGRAY);
    shadow_text(
        "rotor",
        ROTOR_CENTER.x - 38.0,
        ROTOR_CENTER.y + ROTOR_RADIUS + 42.0,
        32.0,
        WHITE,
    );
}

fn draw_status(status: Status, text: &str) {
    let color = match status {
        Status::Ok => Color::new(0.10, 0.45, 0.15, 0.9),
        Status::Waiting => Color::new(0.35, 0.35, 0.35, 0.9),
        Status::Error => Color::new(0.70, 0.10, 0.10, 0.9),
    };
    draw_rectangle(0.0, HEIGHT - STATUS_BAR_H, WIDTH, STATUS_BAR_H, color);
    draw_text(
        fit_width(text, 22, WIDTH - 24.0),
        12.0,
        HEIGHT - 13.0,
        22.0,
        WHITE,
    );
}

/// Truncates `text` (with "...") so it fits in `max_width` pixels at `font_size`.
fn fit_width(text: &str, font_size: u16, max_width: f32) -> String {
    let fits = |t: &str| measure_text(t, None, font_size, 1.0).width <= max_width;
    if fits(text) {
        return text.to_string();
    }
    let mut chars: Vec<char> = text.chars().collect();
    while !chars.is_empty() && !fits(&format!("{}...", chars.iter().collect::<String>())) {
        chars.pop();
    }
    format!("{}...", chars.iter().collect::<String>())
}

fn draw_dashboard(view: &DashboardView, photo: &Texture2D, compass: &Texture2D, now: Instant) {
    clear_background(DARKGRAY);
    draw_background(photo);
    for (i, (label, value)) in view.rows().iter().enumerate() {
        let y = 70.0 + i as f32 * 60.0;
        let size = if *label == "time" { 30.0 } else { 40.0 };
        shadow_text(label, 20.0, y, size, WHITE);
        shadow_text(&format!(": {value}"), 190.0, y, size, WHITE);
    }
    shadow_text(
        &format!("device_id: {DEVICE_ID}"),
        20.0,
        HEIGHT - STATUS_BAR_H - 20.0,
        34.0,
        WHITE,
    );
    draw_compass(compass, view.compass_deg);
    draw_rotor(view.rotor_deg);
    let (status, text) = view.status(now);
    draw_status(status, &text);
}

async fn show_fatal(message: &str) {
    loop {
        clear_background(DARKGRAY);
        draw_text(message, 20.0, HEIGHT / 2.0, 26.0, WHITE);
        draw_text(
            "Press Esc to quit.",
            20.0,
            HEIGHT / 2.0 + 40.0,
            26.0,
            LIGHTGRAY,
        );
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        next_frame().await;
    }
}

fn drain(events: &Receiver<NetEvent>, view: &mut DashboardView, now: Instant) {
    while let Ok(event) = events.try_recv() {
        if let NetEvent::Line {
            raw,
            result: Err(e),
        } = &event
        {
            eprintln!("[rejected] {e}: {raw}");
        }
        view.apply(event, now);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    let (bound, events) = match spawn_listener(&addr) {
        Ok(ok) => ok,
        Err(e) => return show_fatal(&format!("Cannot listen on {addr}: {e}")).await,
    };
    println!(
        "Listening for IoText data on {bound} (try: telnet {} {})",
        bound.ip(),
        bound.port()
    );

    let photo = texture(include_bytes!("../../assets/turbine.png"));
    let compass = texture(include_bytes!("../../assets/wind_dir1.png"));
    let mut view = DashboardView::new(bound.to_string());

    while !is_key_pressed(KeyCode::Escape) {
        let now = Instant::now();
        drain(&events, &mut view, now);
        view.tick(get_frame_time(), now);
        draw_dashboard(&view, &photo, &compass, now);
        next_frame().await;
    }
}
