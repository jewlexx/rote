use std::{path::PathBuf, time::Duration};

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EditorApp {
    file_path: Option<PathBuf>,

    // Do not include entire file contents in state saving
    #[serde(skip)]
    file_contents: String,
}

impl EditorApp {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = ctx.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl eframe::App for EditorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }

    // TODO: Add config to be able to change this and other options
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}
