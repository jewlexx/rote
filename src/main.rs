#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::IconData;
use once_cell::sync::Lazy;
use tracing::Level;

mod app;
mod buffer;
mod shortcuts;

static APP_NAME: Lazy<String> = Lazy::new(|| {
    let pkg_name = env!("CARGO_PKG_NAME");
    let hecked = heck::AsUpperCamelCase(pkg_name);

    hecked.to_string()
});

fn load_icon() -> IconData {
    IconData {
        rgba: include_bytes!("../resources/icons/PENCIL.ico").to_vec(),
        width: 256,
        height: 256,
    }
}

fn main() {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
        .with_max_level(Level::DEBUG)
        .init();

    let native_options = eframe::NativeOptions {
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME.as_str(),
        native_options,
        Box::new(|ctx| Box::new(app::Editor::new(ctx))),
    )
    .unwrap();
}
