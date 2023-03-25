#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::IconData;
use once_cell::sync::Lazy;
use tracing::Level;

#[macro_use]
extern crate tracing;

mod app;
mod buffer;
mod diff;
mod shortcuts;

static APP_NAME: Lazy<String> = Lazy::new(|| {
    let pkg_name = env!("CARGO_PKG_NAME");
    let hecked = heck::AsUpperCamelCase(pkg_name);

    hecked.to_string()
});

fn load_icon() -> Option<IconData> {
    match image::load_from_memory(include_bytes!("../resources/icons/PENCIL.png")) {
        Ok(image) => {
            trace!("Loading icon");
            let rgba = image.into_rgba8();
            let (width, height) = rgba.dimensions();
            let bytes = rgba.into_raw();

            Some(IconData {
                rgba: bytes,
                width,
                height,
            })
        }
        Err(err) => {
            error!("Invalid image: {}", err);
            None
        }
    }
}

fn main() {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
        .with_max_level(Level::DEBUG)
        .init();

    let native_options = eframe::NativeOptions {
        icon_data: load_icon(),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME.as_str(),
        native_options,
        Box::new(|ctx| Box::new(app::Editor::new(ctx))),
    )
    .unwrap();
}
