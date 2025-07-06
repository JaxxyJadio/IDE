// WHAT I WANT: 
// WHAT IT DOES: 
// TODO: 
// FIXME: 

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub timestamp: DateTime<Utc>,
    pub line_changes: Vec<LineChange>,
    pub author: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Renamed(PathBuf), // Old path
    Moved(PathBuf),   // Old path
}

#[derive(Debug, Clone)]
pub struct LineChange {
    pub line_number: usize,
    pub change_kind: LineChangeKind,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineChangeKind {
    Added,
    Deleted,
    Modified,
}

#[derive(Debug)]
pub struct FileChangeTracker {
    changes: HashMap<PathBuf, VecDeque<FileChange>>,
    recent_changes: VecDeque<FileChange>,
    max_changes_per_file: usize,
    max_recent_changes: usize,
    auto_save_enabled: bool,
    change_listeners: Vec<Box<dyn Fn(&FileChange) + Send + Sync>>,
}

impl Default for FileChangeTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl FileChangeTracker {
    pub fn new() -> Self {
        Self {
            changes: HashMap::new(),
            recent_changes: VecDeque::new(),
            max_changes_per_file: 100,
            max_recent_changes: 500,
            auto_save_enabled: true,
            change_listeners: Vec::new(),
        }
    }
    
    pub fn track_file_created(&mut self, path: PathBuf, content: &str) {
        let change = FileChange {
            path: path.clone(),
            change_type: ChangeType::Created,
            timestamp: Utc::now(),
            line_changes: content.lines().enumerate().map(|(i, line)| {
                LineChange {
                    line_number: i + 1,
                    change_kind: LineChangeKind::Added,
                    old_content: None,
                    new_content: Some(line.to_string()),
                }
            }).collect(),
            author: "User".to_string(),
            description: Some("File created".to_string()),
        };
        
        self.add_change(change);
    }
    
    pub fn track_file_modified(&mut self, path: PathBuf, old_content: &str, new_content: &str) {
        let line_changes = self.compute_line_changes(old_content, new_content);
        
        let change = FileChange {
            path: path.clone(),
            change_type: ChangeType::Modified,
            timestamp: Utc::now(),
            line_changes,
            author: "User".to_string(),
            description: None,
        };
        
        self.add_change(change);
    }
    
    pub fn track_file_deleted(&mut self, path: PathBuf, content: &str) {
        let change = FileChange {
            path: path.clone(),
            change_type: ChangeType::Deleted,
            timestamp: Utc::now(),
            line_changes: content.lines().enumerate().map(|(i, line)| {
                LineChange {
                    line_number: i + 1,
                    change_kind: LineChangeKind::Deleted,
                    old_content: Some(line.to_string()),
                    new_content: None,
                }
            }).collect(),
            author: "User".to_string(),
            description: Some("File deleted".to_string()),
        };
        
        self.add_change(change);
    }
    
    pub fn track_file_renamed(&mut self, old_path: PathBuf, new_path: PathBuf) {
        let change = FileChange {
            path: new_path.clone(),
            change_type: ChangeType::Renamed(old_path.clone()),
            timestamp: Utc::now(),
            line_changes: Vec::new(),
            author: "User".to_string(),
            description: Some(format!("Renamed from {:?}", old_path)),
        };
        
        self.add_change(change);
    }
    
    fn add_change(&mut self, change: FileChange) {
        // Notify listeners
        for listener in &self.change_listeners {
            listener(&change);
        }
        
        // Add to file-specific history
        let file_changes = self.changes.entry(change.path.clone()).or_insert_with(VecDeque::new);
        file_changes.push_back(change.clone());
        
        // Trim old changes for this file
        while file_changes.len() > self.max_changes_per_file {
            file_changes.pop_front();
        }
        
        // Add to recent changes
        self.recent_changes.push_back(change);
        
        // Trim old recent changes
        while self.recent_changes.len() > self.max_recent_changes {
            self.recent_changes.pop_front();
        }
    }
    
    fn compute_line_changes(&self, old_content: &str, new_content: &str) -> Vec<LineChange> {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();
        let mut changes = Vec::new();
        
        // Simple line-by-line comparison (could be improved with proper diff algorithm)
        let max_lines = old_lines.len().max(new_lines.len());
        
        for i in 0..max_lines {
            match (old_lines.get(i), new_lines.get(i)) {
                (Some(old), Some(new)) if old != new => {
                    changes.push(LineChange {
                        line_number: i + 1,
                        change_kind: LineChangeKind::Modified,
                        old_content: Some(old.to_string()),
                        new_content: Some(new.to_string()),
                    });
                }
                (Some(old), None) => {
                    changes.push(LineChange {
                        line_number: i + 1,
                        change_kind: LineChangeKind::Deleted,
                        old_content: Some(old.to_string()),
                        new_content: None,
                    });
                }
                (None, Some(new)) => {
                    changes.push(LineChange {
                        line_number: i + 1,
                        change_kind: LineChangeKind::Added,
                        old_content: None,
                        new_content: Some(new.to_string()),
                    });
                }
                _ => {} // Lines are the same, no change
            }
        }
        
        changes
    }
    
    pub fn get_file_history(&self, path: &PathBuf) -> Option<&VecDeque<FileChange>> {
        self.changes.get(path)
    }
    
    pub fn get_recent_changes(&self, count: usize) -> Vec<&FileChange> {
        self.recent_changes.iter().rev().take(count).collect()
    }
    
    pub fn get_changes_since(&self, since: DateTime<Utc>) -> Vec<&FileChange> {
        self.recent_changes
            .iter()
            .filter(|change| change.timestamp > since)
            .collect()
    }
    
    pub fn undo_last_change(&mut self, path: &PathBuf) -> Option<FileChange> {
        if let Some(file_changes) = self.changes.get_mut(path) {
            file_changes.pop_back()
        } else {
            None
        }
    }
    
    pub fn clear_file_history(&mut self, path: &PathBuf) {
        self.changes.remove(path);
    }
    
    pub fn clear_all_history(&mut self) {
        self.changes.clear();
        self.recent_changes.clear();
    }
    
    pub fn add_change_listener(&mut self, listener: Box<dyn Fn(&FileChange) + Send + Sync>) {
        self.change_listeners.push(listener);
    }
    
    pub fn get_statistics(&self) -> ChangeStatistics {
        let total_changes = self.recent_changes.len();
        let files_changed = self.changes.len();
        
        let mut lines_added = 0;
        let mut lines_deleted = 0;
        let mut lines_modified = 0;
        
        for change in &self.recent_changes {
            for line_change in &change.line_changes {
                match line_change.change_kind {
                    LineChangeKind::Added => lines_added += 1,
                    LineChangeKind::Deleted => lines_deleted += 1,
                    LineChangeKind::Modified => lines_modified += 1,
                }
            }
        }
        
        ChangeStatistics {
            total_changes,
            files_changed,
            lines_added,
            lines_deleted,
            lines_modified,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangeStatistics {
    pub total_changes: usize,
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub lines_modified: usize,
}