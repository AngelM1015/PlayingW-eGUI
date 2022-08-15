
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        icon_data: None,
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        initial_window_pos: None,
        initial_window_size: Option::from(egui::Vec2::new(1800 as f32, 800 as f32)),
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
    };

    eframe::run_native(
        "OS",
        native_options,
        Box::new(|cc| Box::new(os::MyApp::new(cc))),
    );
}