#![warn(clippy::pedantic)]

use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use egui::{Align2, FontSelection, Style, TextStyle, Vec2, Widget};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    buffer::ContentsBuffer,
    shortcuts::{EditShortcut, FileShortcut, Shortcut, ViewShortcut},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Editor {
    path: Option<PathBuf>,

    /// Zoom range as a percentage (*100)
    ///
    /// Thus 1.1 would be 110%
    zoom: f32,

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
            zoom: 1.0,
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

            if let Some(path) = data.path.clone() {
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

    pub fn file_execute(
        &mut self,
        shortcut: FileShortcut,
        _: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        match shortcut {
            FileShortcut::Open => {
                // TODO: Save final directory
                let file = rfd::FileDialog::new().pick_file();

                if let Some(path) = file {
                    self.open_file(path).unwrap();
                }
            }
            FileShortcut::Save => {
                if let Some(path) = self.path.clone() {
                    self.save_file(&path);
                } else {
                    self.save_as();
                }
            }
            FileShortcut::SaveAs => self.save_as(),
            FileShortcut::Close => self.reset(),
            FileShortcut::Quit => frame.close(),
        }
    }

    pub fn edit_execute(
        &mut self,
        shortcut: EditShortcut,
        _: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
    }

    pub fn view_execute(
        &mut self,
        shortcut: ViewShortcut,
        _: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        match shortcut {
            ViewShortcut::ZoomIn => self.zoom += 0.1,
            ViewShortcut::ZoomOut => self.zoom -= 0.1,
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
        let font = {
            let mut font_id = TextStyle::Body.resolve(&ctx.style());
            let old_size = font_id.size;
            font_id.size = old_size * self.zoom;

            font_id
        };

        let name = if let Some(path) = self.path.as_ref() {
            let stripped_path = path.with_extension("");
            stripped_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        } else {
            crate::DEFAULT_NAME.clone()
        };

        let formatted_name = format!("{}{}", name, if self.contents.edited() { "*" } else { "" });

        frame.set_window_title(&formatted_name);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    for shortcut in FileShortcut::iter() {
                        if shortcut.into_button(ctx).ui(ui).clicked() {
                            self.file_execute(shortcut, ctx, frame);
                        }
                    }
                });

                ui.menu_button("Edit", |ui| {
                    for shortcut in EditShortcut::iter() {
                        if shortcut.into_button(ctx).ui(ui).clicked() {
                            self.edit_execute(shortcut, ctx, frame);
                        }
                    }
                });

                ui.menu_button("View", |ui| {
                    for shortcut in ViewShortcut::iter() {
                        if shortcut.into_button(ctx).ui(ui).clicked() {
                            self.view_execute(shortcut, ctx, frame);
                        }
                    }

                    if ui.button("Reset Zoom").clicked() {
                        self.zoom = 1.0;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let available = ui.available_size();
                ui.set_min_height(available.y);
                ui.set_max_height(available.y);
                ui.add_sized(
                    available,
                    egui::TextEdit::multiline(&mut self.contents).font(font),
                );
            });
        });

        ctx.input_mut(|state| {
            for shortcut in FileShortcut::iter() {
                if state.consume_shortcut(&shortcut.get_details().1) {
                    self.file_execute(shortcut, ctx, frame);
                }
            }

            for shortcut in EditShortcut::iter() {
                if state.consume_shortcut(&shortcut.get_details().1) {
                    self.edit_execute(shortcut, ctx, frame);
                }
            }

            for shortcut in ViewShortcut::iter() {
                if state.consume_shortcut(&shortcut.get_details().1) {
                    self.view_execute(shortcut, ctx, frame);
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
                            }

                            if ui.button("Save").clicked() {
                                self.file_execute(FileShortcut::Save, ctx, frame);
                            }

                            frame.close();
                        });
                    });
                });
        }
    }
}
