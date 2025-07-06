// WHAT I WANT: 
// WHAT IT DOES: 
// TODO: 
// FIXME: 

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct HotSwapConfig {
    pub enabled: bool,
    pub watch_extensions: Vec<String>,
    pub reload_delay_ms: u64,
    pub max_reload_attempts: u32,
    pub preserve_state: bool,
}

#[derive(Debug)]
pub struct HotSwapper {
    config: HotSwapConfig,
    watched_files: HashMap<PathBuf, FileWatchInfo>,
    reload_queue: Vec<PathBuf>,
    state_cache: HashMap<String, String>,
    reload_handlers: Vec<Box<dyn Fn(&Path) -> Result<(), String> + Send + Sync>>,
}

#[derive(Debug, Clone)]
struct FileWatchInfo {
    last_modified: SystemTime,
    checksum: String,
    reload_count: u32,
    last_reload: Option<SystemTime>,
}

impl Default for HotSwapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
                "css".to_string(),
                "html".to_string(),
            ],
            reload_delay_ms: 500,
            max_reload_attempts: 3,
            preserve_state: true,
        }
    }
}

impl Default for HotSwapper {
    fn default() -> Self {
        Self::new(HotSwapConfig::default())
    }
}

impl HotSwapper {
    pub fn new(config: HotSwapConfig) -> Self {
        Self {
            config,
            watched_files: HashMap::new(),
            reload_queue: Vec::new(),
            state_cache: HashMap::new(),
            reload_handlers: Vec::new(),
        }
    }
    
    pub fn watch_file(&mut self, path: PathBuf) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Check if file has a watched extension
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_string();
            if !self.config.watch_extensions.contains(&ext_str) {
                return Ok(());
            }
        }
        
        // Get file metadata
        let metadata = std::fs::metadata(&path)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;
        
        let last_modified = metadata.modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?;
        
        // Calculate checksum
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let checksum = self.calculate_checksum(&content);
        
        let watch_info = FileWatchInfo {
            last_modified,
            checksum,
            reload_count: 0,
            last_reload: None,
        };
        
        self.watched_files.insert(path, watch_info);
        Ok(())
    }
    
    pub fn unwatch_file(&mut self, path: &Path) {
        self.watched_files.remove(path);
    }
    
    pub fn check_for_changes(&mut self) -> Vec<PathBuf> {
        if !self.config.enabled {
            return Vec::new();
        }
        
        let mut changed_files = Vec::new();
        
        for (path, watch_info) in &mut self.watched_files {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > watch_info.last_modified {
                        // File has been modified
                        if let Ok(content) = std::fs::read_to_string(path) {
                            let new_checksum = self.calculate_checksum(&content);
                            
                            if new_checksum != watch_info.checksum {
                                // Content actually changed
                                watch_info.last_modified = modified;
                                watch_info.checksum = new_checksum;
                                changed_files.push(path.clone());
                            }
                        }
                    }
                }
            }
        }
        
        changed_files
    }
    
    pub fn queue_reload(&mut self, path: PathBuf) {
        if !self.reload_queue.contains(&path) {
            self.reload_queue.push(path);
        }
    }
    
    pub fn process_reload_queue(&mut self) -> Vec<ReloadResult> {
        let mut results = Vec::new();
        let paths_to_reload = self.reload_queue.drain(..).collect::<Vec<_>>();
        
        for path in paths_to_reload {
            let result = self.reload_file(&path);
            results.push(ReloadResult {
                path: path.clone(),
                success: result.is_ok(),
                error: result.err(),
                timestamp: SystemTime::now(),
            });
        }
        
        results
    }
    
    fn reload_file(&mut self, path: &Path) -> Result<(), String> {
        // Check reload count
        if let Some(watch_info) = self.watched_files.get_mut(path) {
            if watch_info.reload_count >= self.config.max_reload_attempts {
                return Err("Max reload attempts exceeded".to_string());
            }
            
            // Check reload delay
            if let Some(last_reload) = watch_info.last_reload {
                let elapsed = SystemTime::now().duration_since(last_reload)
                    .map_err(|e| format!("Time error: {}", e))?;
                
                if elapsed.as_millis() < self.config.reload_delay_ms as u128 {
                    return Err("Reload too soon".to_string());
                }
            }
            
            watch_info.reload_count += 1;
            watch_info.last_reload = Some(SystemTime::now());
        }
        
        // Save state if needed
        if self.config.preserve_state {
            self.save_state(path)?;
        }
        
        // Execute reload handlers
        for handler in &self.reload_handlers {
            handler(path)?;
        }
        
        // Restore state if needed
        if self.config.preserve_state {
            self.restore_state(path)?;
        }
        
        Ok(())
    }
    
    fn save_state(&mut self, path: &Path) -> Result<(), String> {
        // This would be implemented based on the specific needs
        // For now, just store a placeholder
        let key = path.to_string_lossy().to_string();
        self.state_cache.insert(key, "state_placeholder".to_string());
        Ok(())
    }
    
    fn restore_state(&self, path: &Path) -> Result<(), String> {
        // This would be implemented based on the specific needs
        let key = path.to_string_lossy().to_string();
        if let Some(_state) = self.state_cache.get(&key) {
            // Restore the state
        }
        Ok(())
    }
    
    fn calculate_checksum(&self, content: &str) -> String {
        // Simple checksum using hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    pub fn add_reload_handler(&mut self, handler: Box<dyn Fn(&Path) -> Result<(), String> + Send + Sync>) {
        self.reload_handlers.push(handler);
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    pub fn get_watched_files(&self) -> Vec<&Path> {
        self.watched_files.keys().map(|p| p.as_path()).collect()
    }
    
    pub fn clear_watched_files(&mut self) {
        self.watched_files.clear();
    }
}

#[derive(Debug, Clone)]
pub struct ReloadResult {
    pub path: PathBuf,
    pub success: bool,
    pub error: Option<String>,
    pub timestamp: SystemTime,
}