use std::path::PathBuf;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EditorApp {
    file_path: Option<PathBuf>,

    // Do not include entire file contents in state saving
    #[serde(skip)]
    file_contents: String,
}
