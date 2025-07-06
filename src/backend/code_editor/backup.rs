use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::Local;

/// Handles backing up files for the code editor.
pub struct CodeEditorBackup;

impl CodeEditorBackup {
    /// Backup the given file to the backup folder, appending a timestamp.
    pub fn backup_file<P: AsRef<Path>>(file_path: P, backup_dir: P) -> io::Result<PathBuf> {
        let file_path = file_path.as_ref();
        let backup_dir = backup_dir.as_ref();
        fs::create_dir_all(backup_dir)?;
        let file_name = file_path.file_name().unwrap_or_default();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let backup_file_name = format!("{}_{}", timestamp, file_name.to_string_lossy());
        let backup_path = backup_dir.join(backup_file_name);
        fs::copy(file_path, &backup_path)?;
        Ok(backup_path)
    }
}
