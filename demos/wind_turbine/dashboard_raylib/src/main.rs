//! Wind turbine dashboard (raylib 6): same behaviour as the macroquad dashboard,
//! drawn with raylib. Listens for IoText messages over TCP.
//!
//! Usage: `cargo run -p wind_turbine_dashboard_raylib [-- <listen address>]`
//! (default: 127.0.0.1:8688). Building needs cmake, a C compiler and the
//! X11/OpenGL development packages.

use std::sync::mpsc::Receiver;
use std::time::Instant;

use raylib::prelude::*;
use wind_turbine_common::server::{NetEvent, spawn_listener};
use wind_turbine_common::view::{DashboardView, Status};
use wind_turbine_common::{DEFAULT_ADDR, DEVICE_ID};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 780;
const COMPASS_CENTER: Vector2 = Vector2 { x: 800.0, y: 250.0 };
const COMPASS_SIZE: f32 = 330.0;
const ROTOR_CENTER: Vector2 = Vector2 { x: 830.0, y: 590.0 };
const ROTOR_RADIUS: f32 = 95.0;
const STATUS_BAR_H: i32 = 40;
const TITLE: &str = "Wind Turbine Remote Metrics - IoText Data Visualization (raylib)";

struct Textures {
    photo: Texture2D,
    compass: Texture2D,
}

fn load_texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    png: &[u8],
) -> Result<Texture2D, String> {
    let image = Image::load_image_from_mem(".png", png).map_err(|e| e.to_string())?;
    let texture = rl
        .load_texture_from_image(thread, &image)
        .map_err(|e| e.to_string())?;
    texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    Ok(texture)
}

/// Text with a dark shadow, readable on the photo.
fn shadow_text(d: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, size: i32) {
    d.draw_text(text, x + 3, y + 3, size, Color::BLACK);
    d.draw_text(text, x, y, size, Color::WHITE);
}

/// raylib only fills triangles given counter-clockwise (as seen on screen).
fn triangle(d: &mut RaylibDrawHandle, a: Vector2, b: Vector2, c: Vector2, color: Color) {
    let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if cross > 0.0 {
        d.draw_triangle(a, c, b, color);
    } else {
        d.draw_triangle(a, b, c, color);
    }
}

/// Draws the photo scaled to cover the window (cropping the sides).
fn draw_background(d: &mut RaylibDrawHandle, photo: &Texture2D) {
    let (w, h) = (photo.width as f32, photo.height as f32);
    let scale = (WIDTH as f32 / w).max(HEIGHT as f32 / h);
    let dest = Rectangle::new(
        (WIDTH as f32 - w * scale) / 2.0,
        (HEIGHT as f32 - h * scale) / 2.0,
        w * scale,
        h * scale,
    );
    d.draw_texture_pro(
        photo,
        Rectangle::new(0.0, 0.0, w, h),
        dest,
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

fn draw_compass(d: &mut RaylibDrawHandle, compass: &Texture2D, rotation_deg: f32) {
    // The image has a thin frame along its edges: crop it away.
    let inset = 6.0;
    let source = Rectangle::new(
        inset,
        inset,
        compass.width as f32 - 2.0 * inset,
        compass.height as f32 - 2.0 * inset,
    );
    let dest = Rectangle::new(
        COMPASS_CENTER.x,
        COMPASS_CENTER.y,
        COMPASS_SIZE,
        COMPASS_SIZE,
    );
    let origin = Vector2::new(COMPASS_SIZE / 2.0, COMPASS_SIZE / 2.0); // rotate around the center
    d.draw_texture_pro(
        compass,
        source,
        dest,
        origin,
        rotation_deg,
        Color::new(191, 217, 255, 255),
    );
    // Marker: the direction under it is where the wind comes from.
    let tip_y = COMPASS_CENTER.y - COMPASS_SIZE / 2.0 - 4.0;
    triangle(
        d,
        Vector2::new(COMPASS_CENTER.x - 28.0, tip_y - 34.0),
        Vector2::new(COMPASS_CENTER.x + 28.0, tip_y - 34.0),
        Vector2::new(COMPASS_CENTER.x, tip_y),
        Color::BLACK,
    );
}

/// A small three-blade rotor that spins with the reported wind speed.
fn draw_rotor(d: &mut RaylibDrawHandle, rotor_deg: f32) {
    d.draw_poly(
        ROTOR_CENTER,
        64,
        ROTOR_RADIUS + 12.0,
        0.0,
        Color::new(0, 0, 0, 90),
    );
    for blade in 0..3 {
        let a = (rotor_deg + blade as f32 * 120.0).to_radians();
        let dir = Vector2::new(a.sin(), -a.cos());
        let side = Vector2::new(dir.y, -dir.x) * 9.0;
        triangle(
            d,
            ROTOR_CENTER + side,
            ROTOR_CENTER - side,
            ROTOR_CENTER + dir * ROTOR_RADIUS,
            Color::WHITE,
        );
    }
    d.draw_poly(ROTOR_CENTER, 32, 14.0, 0.0, Color::LIGHTGRAY);
    shadow_text(
        d,
        "rotor",
        ROTOR_CENTER.x as i32 - 38,
        (ROTOR_CENTER.y + ROTOR_RADIUS) as i32 + 18,
        32,
    );
}

/// Truncates `text` (with "...") so it fits in `max_width` pixels at `font_size`.
fn fit_width(rl: &RaylibHandle, text: &str, font_size: i32, max_width: i32) -> String {
    let fits = |t: &str| rl.measure_text(t, font_size) <= max_width;
    if fits(text) {
        return text.to_string();
    }
    let mut chars: Vec<char> = text.chars().collect();
    while !chars.is_empty() && !fits(&format!("{}...", chars.iter().collect::<String>())) {
        chars.pop();
    }
    format!("{}...", chars.iter().collect::<String>())
}

fn draw_dashboard(
    d: &mut RaylibDrawHandle,
    view: &DashboardView,
    tex: &Textures,
    status: Status,
    status_text: &str,
) {
    d.clear_background(Color::DARKGRAY);
    draw_background(d, &tex.photo);
    for (y, (label, value)) in (40..).step_by(60).zip(view.rows()) {
        let size = if label == "time" { 30 } else { 40 };
        shadow_text(d, label, 20, y, size);
        shadow_text(d, &format!(": {value}"), 235, y, size);
    }
    shadow_text(
        d,
        &format!("device_id: {DEVICE_ID}"),
        20,
        HEIGHT - STATUS_BAR_H - 48,
        34,
    );
    draw_compass(d, &tex.compass, view.compass_deg);
    draw_rotor(d, view.rotor_deg);

    let color = match status {
        Status::Ok => Color::new(26, 115, 38, 230),
        Status::Waiting => Color::new(89, 89, 89, 230),
        Status::Error => Color::new(179, 26, 26, 230),
    };
    d.draw_rectangle(0, HEIGHT - STATUS_BAR_H, WIDTH, STATUS_BAR_H, color);
    d.draw_text(
        status_text,
        12,
        HEIGHT - STATUS_BAR_H + 10,
        20,
        Color::WHITE,
    );
}

fn show_fatal(rl: &mut RaylibHandle, thread: &RaylibThread, message: &str) {
    eprintln!("{message}");
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::DARKGRAY);
        d.draw_text(message, 20, HEIGHT / 2, 24, Color::WHITE);
        d.draw_text(
            "Press Esc to quit.",
            20,
            HEIGHT / 2 + 40,
            24,
            Color::LIGHTGRAY,
        );
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

fn main() {
    set_trace_log(TraceLogLevel::LOG_WARNING);
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title(TITLE).build();
    rl.set_target_fps(60);

    let (bound, events) = match spawn_listener(&addr) {
        Ok(ok) => ok,
        Err(e) => return show_fatal(&mut rl, &thread, &format!("Cannot listen on {addr}: {e}")),
    };
    println!(
        "Listening for IoText data on {bound} (try: telnet {} {})",
        bound.ip(),
        bound.port()
    );

    let textures = match (
        load_texture(&mut rl, &thread, include_bytes!("../../assets/turbine.png")),
        load_texture(
            &mut rl,
            &thread,
            include_bytes!("../../assets/wind_dir1.png"),
        ),
    ) {
        (Ok(photo), Ok(compass)) => Textures { photo, compass },
        (Err(e), _) | (_, Err(e)) => {
            return show_fatal(&mut rl, &thread, &format!("Cannot load images: {e}"));
        }
    };
    let mut view = DashboardView::new(bound.to_string());

    while !rl.window_should_close() {
        let now = Instant::now();
        drain(&events, &mut view, now);
        view.tick(rl.get_frame_time(), now);
        let (status, text) = view.status(now);
        let text = fit_width(&rl, &text, 20, WIDTH - 24);
        let mut d = rl.begin_drawing(&thread);
        draw_dashboard(&mut d, &view, &textures, status, &text);
    }
}
