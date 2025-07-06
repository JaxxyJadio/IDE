use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Handles saving code editor content to disk.
pub struct CodeEditorSaver;

impl CodeEditorSaver {
    /// Save the given content to the specified file path.
    pub fn save_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    /// Save only if content has changed (compared to file on disk).
    pub fn save_if_changed<P: AsRef<Path>>(path: P, content: &str) -> io::Result<bool> {
        if let Ok(existing) = fs::read_to_string(&path) {
            if existing == content {
                return Ok(false); // No change
            }
        }
        Self::save_to_file(path, content)?;
        Ok(true)
    }
}
