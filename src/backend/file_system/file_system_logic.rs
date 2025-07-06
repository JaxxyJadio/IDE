use std::path::{Path, PathBuf};
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

pub struct FileSystem {
    current_workspace: Option<PathBuf>,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            current_workspace: None,
        }
    }

    pub fn set_workspace<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        if path.exists() && path.is_dir() {
            self.current_workspace = Some(path);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"))
        }
    }

    pub fn get_workspace(&self) -> Option<&PathBuf> {
        self.current_workspace.as_ref()
    }

    pub fn list_directory<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            
            let file_entry = FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path.clone(),
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: metadata.modified().ok(),
            };
            
            entries.push(file_entry);
        }

        // Sort: directories first, then files, both alphabetically
        entries.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok(entries)
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> io::Result<()> {
        fs::write(path, content)
    }

    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if path.as_ref().is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }

    pub fn rename_file<P: AsRef<Path>>(&self, from: P, to: P) -> io::Result<()> {
        fs::rename(from, to)
    }

    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    pub fn get_file_extension<P: AsRef<Path>>(&self, path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    pub fn is_text_file<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(ext) = self.get_file_extension(&path) {
            matches!(ext.as_str(), 
                "txt" | "rs" | "py" | "js" | "ts" | "html" | "css" | "json" | 
                "yaml" | "yml" | "toml" | "md" | "xml" | "c" | "cpp" | "h" | 
                "hpp" | "java" | "php" | "rb" | "go" | "swift" | "kt" | "scala"
            )
        } else {
            false
        }
    }

    pub fn get_relative_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if let Some(workspace) = &self.current_workspace {
            path.as_ref().strip_prefix(workspace).ok().map(|p| p.to_path_buf())
        } else {
            None
        }
    }
}file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::File::create(path)?;
        Ok(())
    }

    pub fn create_