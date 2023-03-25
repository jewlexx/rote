#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use once_cell::sync::Lazy;
use tracing::Level;

mod app;
mod buffer;
mod shortcuts;

static DEFAULT_NAME: Lazy<String> = Lazy::new(|| {
    let pkg_name = env!("CARGO_PKG_NAME");
    let hecked = heck::AsUpperCamelCase(pkg_name);

    hecked.to_string()
});

fn main() {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
        .with_max_level(Level::DEBUG)
        .init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        DEFAULT_NAME.as_str(),
        native_options,
        Box::new(|ctx| Box::new(app::Editor::new(ctx))),
    )
    .unwrap();
}
