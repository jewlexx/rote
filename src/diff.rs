use difference::Changeset;
use parking_lot::Mutex;

pub static DIFF_BUFFER: Mutex<Vec<Changeset>> = Mutex::new(Vec::new());

pub fn add_change(old: &str, new: &str) {
    DIFF_BUFFER.lock().push(Changeset::new(old, new, " "));
}

pub fn apply_change(current: &str) {
    let latest_diff = DIFF_BUFFER.lock().pop();

    if let Some(diff) = latest_diff {
        dbg!(diff.diffs);
        // diff.apply(current)
    }

    // todo!()
}
