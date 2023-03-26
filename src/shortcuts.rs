use egui::{Button, Context, Key, KeyboardShortcut, Modifiers};
use strum::EnumIter;

// TODO: Add shortcuts without keyboard shortcuts

pub trait Shortcut {
    fn get_details(&self) -> (&'static str, KeyboardShortcut);

    fn into_button(self, ctx: &Context) -> Button
    where
        Self: Sized,
    {
        let (title, ref shortcut) = self.get_details();

        Button::new(title).shortcut_text(ctx.format_shortcut(shortcut))
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum FileShortcut {
    Open,
    Save,
    SaveAs,
    Close,
    Quit,
}

impl Shortcut for FileShortcut {
    fn get_details(&self) -> (&'static str, KeyboardShortcut) {
        match self {
            FileShortcut::Open => ("Open", KeyboardShortcut::new(Modifiers::CTRL, Key::O)),
            FileShortcut::Save => ("Save", KeyboardShortcut::new(Modifiers::CTRL, Key::S)),
            FileShortcut::SaveAs => (
                "Save As",
                KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::S),
            ),
            FileShortcut::Close => ("Close", KeyboardShortcut::new(Modifiers::CTRL, Key::W)),
            FileShortcut::Quit => ("Quit", KeyboardShortcut::new(Modifiers::CTRL, Key::Q)),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum EditShortcut {
    Undo,
    Redo,
    Copy,
    Paste,
}

impl Shortcut for EditShortcut {
    fn get_details(&self) -> (&'static str, KeyboardShortcut) {
        match self {
            EditShortcut::Undo => ("Undo", KeyboardShortcut::new(Modifiers::CTRL, Key::Z)),
            EditShortcut::Redo => ("Redo", KeyboardShortcut::new(Modifiers::CTRL, Key::Y)),
            EditShortcut::Copy => ("Copy", KeyboardShortcut::new(Modifiers::CTRL, Key::C)),
            EditShortcut::Paste => ("Paste", KeyboardShortcut::new(Modifiers::CTRL, Key::V)),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ViewShortcut {
    ZoomIn,
    ZoomOut,
}

impl Shortcut for ViewShortcut {
    fn get_details(&self) -> (&'static str, KeyboardShortcut) {
        match self {
            ViewShortcut::ZoomIn => (
                "Zoom In",
                KeyboardShortcut::new(Modifiers::CTRL, Key::PlusEquals),
            ),
            ViewShortcut::ZoomOut => (
                "Zoom Out",
                KeyboardShortcut::new(Modifiers::CTRL, Key::Minus),
            ),
        }
    }
}
