use parking_lot::Mutex;

#[derive(Debug)]
pub enum Diff {
    Add(String, usize),
    Remove(String, usize),
}

impl Diff {
    pub fn invert(self) -> Self {
        match self {
            Diff::Add(a, b) => Diff::Remove(a, b),
            Diff::Remove(a, b) => Diff::Add(a, b),
        }
    }
}

pub static DIFF_BUFFER: DiffBuffer = DiffBuffer::new();

#[derive(Debug)]
pub struct DiffBuffer {
    diffs: Mutex<Vec<Diff>>,
}

impl DiffBuffer {
    pub const fn new() -> Self {
        Self {
            diffs: Mutex::new(Vec::new()),
        }
    }

    pub fn size(&self) -> usize {
        self.diffs.lock().len()
    }

    pub fn add(&self, text: String, position: usize) {
        self.add_change(Diff::Add(text, position))
    }

    pub fn delete(&self, text: String, position: usize) {
        self.add_change(Diff::Remove(text, position))
    }

    pub fn add_change(&self, diff: Diff) {
        debug!(
            "Added {:#?}\nBuffer is now {} diffs long",
            diff,
            self.size()
        );
        self.diffs.lock().push(diff);
    }
}

// pub fn add_change(old: &str, new: &str) {
//     DIFF_BUFFER.lock().push(Changeset::new(old, new, " "));
// }

// pub fn apply_change(current: &str) {
//     let latest_diff = DIFF_BUFFER.lock().pop();

//     if let Some(diff) = latest_diff {
//         dbg!(diff.diffs);
//         // diff.apply(current)
//     }

//     // todo!()
// }
