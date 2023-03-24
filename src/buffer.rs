// TODO: Custom serialize with for contentsbuffer

use egui::TextBuffer;

#[derive(Debug, Clone)]
pub struct ContentsBuffer {
    pub contents: String,

    pub edited: bool,
    pub mutable: bool,
}

impl Default for ContentsBuffer {
    fn default() -> Self {
        Self {
            contents: String::new(),
            edited: false,
            mutable: true,
        }
    }
}

impl ContentsBuffer {
    pub fn get_contents(&self) -> &String {
        &self.contents
    }

    pub fn get_contents_mut(&mut self) -> &mut String {
        &mut self.contents
    }

    pub fn edited(&self) -> bool {
        self.edited
    }

    pub fn set_edited(&mut self, edited: bool) {
        self.edited = edited;
    }
}

impl egui::TextBuffer for ContentsBuffer {
    fn is_mutable(&self) -> bool {
        self.contents.is_mutable()
    }

    fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    fn clear(&mut self) {
        self.set_edited(true);
        self.contents.clear();
    }

    fn take(&mut self) -> String {
        self.set_edited(true);
        self.contents.take()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.set_edited(true);
        self.contents.insert_text(text, char_index)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.set_edited(true);
        self.contents.delete_char_range(char_range);
    }

    fn replace(&mut self, text: &str) {
        self.set_edited(true);
        self.contents.replace(text);
    }
}
