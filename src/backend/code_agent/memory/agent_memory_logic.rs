use std::fs;
use std::path::{Path, PathBuf};

/// AgentMemory manages a temporary folder for storing files when memory is enabled.
pub struct AgentMemory {
    enabled: bool,
    temp_dir: Option<PathBuf>,
}

impl AgentMemory {
    /// Create a new AgentMemory, disabled by default.
    pub fn new() -> Self {
        Self {
            enabled: false,
            temp_dir: None,
        }
    }

    /// Enable memory: create a temp folder if not already present.
    pub fn enable(&mut self) -> std::io::Result<()> {
        if !self.enabled {
            let dir = std::env::temp_dir().join("jadio_agent_memory");
            if !dir.exists() {
                fs::create_dir_all(&dir)?;
            }
            self.temp_dir = Some(dir);
            self.enabled = true;
        }
        Ok(())
    }

    /// Disable memory: remove the temp folder and clear state.
    pub fn disable(&mut self) -> std::io::Result<()> {
        if let Some(ref dir) = self.temp_dir {
            if dir.exists() {
                fs::remove_dir_all(dir)?;
            }
        }
        self.temp_dir = None;
        self.enabled = false;
        Ok(())
    }

    /// Store a file in the temp folder (if enabled).
    pub fn store_file(&self, filename: &str, contents: &[u8]) -> std::io::Result<()> {
        if let (true, Some(ref dir)) = (self.enabled, &self.temp_dir) {
            let file_path = dir.join(filename);
            fs::write(file_path, contents)?;
        }
        Ok(())
    }

    /// List files in the temp folder (if enabled).
    pub fn list_files(&self) -> std::io::Result<Vec<String>> {
        if let (true, Some(ref dir)) = (self.enabled, &self.temp_dir) {
            let mut files = Vec::new();
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(name.to_string());
                    }
                }
            }
            Ok(files)
        } else {
            Ok(Vec::new())
        }
    }

    /// Read a file from the temp folder (if enabled).
    pub fn read_file(&self, filename: &str) -> std::io::Result<Option<Vec<u8>>> {
        if let (true, Some(ref dir)) = (self.enabled, &self.temp_dir) {
            let file_path = dir.join(filename);
            if file_path.exists() {
                let data = fs::read(file_path)?;
                return Ok(Some(data));
            }
        }
        Ok(None)
    }

    /// Check if memory is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
