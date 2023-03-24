use egui::{Button, Context, Key, KeyboardShortcut, Modifiers};
use strum::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Shortcut {
    Open,
    Save,
    Close,
    Quit,
}

impl Shortcut {
    pub fn get_details(&self) -> (&'static str, KeyboardShortcut) {
        match self {
            Shortcut::Open => ("Open", KeyboardShortcut::new(Modifiers::CTRL, Key::O)),
            Shortcut::Save => ("Save", KeyboardShortcut::new(Modifiers::CTRL, Key::S)),
            Shortcut::Close => ("Close", KeyboardShortcut::new(Modifiers::CTRL, Key::W)),
            Shortcut::Quit => ("Quit", KeyboardShortcut::new(Modifiers::CTRL, Key::Q)),
        }
    }

    pub fn into_button(self, ctx: &Context) -> Button {
        let (title, shortcut) = self.get_details();
        Button::new(title).shortcut_text(ctx.format_shortcut(&shortcut))
    }
}
