#![warn(clippy::pedantic)]

use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use egui::{Align2, Vec2, Widget};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{buffer::ContentsBuffer, shortcuts::Shortcut};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Editor {
    path: Option<PathBuf>,

    #[serde(skip)]
    trying_to_close: bool,

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
            trying_to_close: false,
            contents: ContentsBuffer::default(),
            channel: std::sync::mpsc::channel(),
        }
    }
}

impl Editor {
    pub fn new(ctx: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = ctx.storage {
            let mut data: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            let path = {
                if let Some(ref path) = crate::ARGS.path {
                    Some(path)
                } else if let Some(ref path) = data.path {
                    Some(path)
                } else {
                    None
                }
            }
            .cloned();

            if let Some(path) = path {
                if data.open_file(path).is_err() {
                    data.path = None;
                }
            }

            data
        } else {
            Self::default()
        }
    }

    pub fn reset(&mut self) {
        self.contents = ContentsBuffer::default();
        self.path = None;
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

    pub fn save_file(&mut self, path: &PathBuf) {
        use std::{fs::File, io::Write};

        let mut file = File::create(path).unwrap();
        file.write_all(self.contents.get_contents().as_bytes())
            .unwrap();

        self.contents.set_edited(false);
    }

    pub fn save_as(&mut self) {
        let file = rfd::FileDialog::new().pick_file();

        if let Some(path) = file {
            self.save_file(&path);
            self.path = Some(path);
        }
    }

    pub fn execute(&mut self, shortcut: Shortcut, _: &egui::Context, frame: &mut eframe::Frame) {
        match shortcut {
            Shortcut::Open => {
                // TODO: Save final directory
                let file = rfd::FileDialog::new().pick_file();

                if let Some(path) = file {
                    self.open_file(path).unwrap();
                }
            }
            Shortcut::Save => {
                if let Some(path) = self.path.clone() {
                    self.save_file(&path);
                } else {
                    self.save_as();
                }
            }
            Shortcut::SaveAs => self.save_as(),
            Shortcut::Close => self.reset(),
            Shortcut::Quit => frame.close(),
        }
    }
}

impl eframe::App for Editor {
    fn on_close_event(&mut self) -> bool {
        self.trying_to_close = true;
        !self.contents.edited()
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // TODO: Add config to be able to change this and other options
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let name = if let Some(path) = self.path.as_ref() {
            let stripped_path = path.with_extension("");
            stripped_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        } else {
            crate::APP_NAME.clone()
        };

        let formatted_name = format!("{}{}", name, if self.contents.edited() { "*" } else { "" });

        frame.set_window_title(&formatted_name);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    for shortcut in Shortcut::iter() {
                        if shortcut.into_button(ctx).ui(ui).clicked() {
                            self.execute(shortcut, ctx, frame);
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let available = ui.available_size();
                ui.set_min_height(available.y);
                ui.set_max_height(available.y);
                ui.add_sized(available, egui::TextEdit::multiline(&mut self.contents));
            });
        });

        ctx.input_mut(|state| {
            for shortcut in Shortcut::iter() {
                if state.consume_shortcut(&shortcut.get_details().1) {
                    self.execute(shortcut, ctx, frame);
                }
            }
        });

        if self.trying_to_close {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("You will lose all changes.");

                        ui.horizontal(|ui| {
                            if ui.button("Don't Save").clicked() {
                                self.contents.set_edited(false);
                                frame.close();
                            }

                            if ui.button("Save").clicked() {
                                self.execute(Shortcut::Save, ctx, frame);
                                frame.close();
                            };

                            if ui.button("Cancel").clicked() {
                                self.trying_to_close = false;
                            }
                        });
                    });
                });
        }
    }
}
