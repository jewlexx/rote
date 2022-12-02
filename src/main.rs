#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tracing::Level;

#[macro_use]
extern crate tracing;

mod app;

fn main() {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
        .with_max_level(Level::DEBUG)
        .init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Potenad",
        native_options,
        Box::new(|ctx| Box::new(app::EditorApp::new(ctx))),
    );
}
