#![warn(clippy::pedantic)]

use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::buffer::ContentsBuffer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Editor {
    path: Option<PathBuf>,

    // Do not include entire file contents in state saving
    #[serde(skip)]
    contents: ContentsBuffer,

    #[serde(skip)]
    channel: (Sender<PathBuf>, Receiver<PathBuf>),
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            path: None,
            contents: ContentsBuffer::default(),
            channel: std::sync::mpsc::channel(),
        }
    }
}

impl Editor {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = ctx.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn open_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        use std::{fs::File, io::Read};

        let path = path.as_ref();
        let mut file = File::open(path)?;

        file.read_to_string(self.contents.get_contents_mut())
            .expect("read to contents buffer");

        self.path = Some(path.to_path_buf());

        Ok(())
    }
}

impl eframe::App for Editor {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // TODO: Add config to be able to change this and other options
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        debug!("Updating...");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // TODO: Save final directory
                        let file = rfd::FileDialog::new().pick_file();

                        if let Some(path) = file {
                            self.open_file(path).unwrap();
                        }
                    }

                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_height(100.0);
            ui.set_max_height(100.0);
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.contents),
            );
        });
    }
}
