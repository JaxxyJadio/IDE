use std::path::PathBuf;
use crate::backend::code_editor::saving::CodeEditorSaver;
use crate::backend::code_editor::syntax_highlighting::{SyntaxHighlighter, HighlightedToken};

/// Represents the state of a code editor tab.
#[derive(Debug, Clone)]
pub struct EditorTab {
    pub file_path: Option<PathBuf>,
    pub content: String,
    pub is_dirty: bool,
    pub highlighted: Vec<Vec<HighlightedToken>>,
}

/// Main code editor logic, managing tabs, saving, and highlighting.
pub struct CodeEditorLogic {
    pub tabs: Vec<EditorTab>,
    pub current_tab: usize,
    pub highlighter: SyntaxHighlighter,
}

impl CodeEditorLogic {
    /// Create a new code editor logic instance.
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            current_tab: 0,
            highlighter: SyntaxHighlighter::new(),
        }
    }

    /// Open a file in a new tab.
    pub fn open_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(&path)?;
        let highlighted = content.lines().map(|l| self.highlighter.highlight_line(l)).collect();
        self.tabs.push(EditorTab {
            file_path: Some(path),
            content,
            is_dirty: false,
            highlighted,
        });
        self.current_tab = self.tabs.len() - 1;
        Ok(())
    }

    /// Create a new empty tab.
    pub fn new_tab(&mut self) {
        self.tabs.push(EditorTab {
            file_path: None,
            content: String::new(),
            is_dirty: false,
            highlighted: Vec::new(),
        });
        self.current_tab = self.tabs.len() - 1;
    }

    /// Edit the content of the current tab.
    pub fn edit_current(&mut self, new_content: String) {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            tab.content = new_content.clone();
            tab.is_dirty = true;
            tab.highlighted = new_content.lines().map(|l| self.highlighter.highlight_line(l)).collect();
        }
    }

    /// Save the current tab (if it has a file path).
    pub fn save_current(&mut self) -> std::io::Result<bool> {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            if let Some(ref path) = tab.file_path {
                let changed = CodeEditorSaver::save_if_changed(path, &tab.content)?;
                if changed {
                    tab.is_dirty = false;
                }
                return Ok(changed);
            }
        }
        Ok(false)
    }

    /// Get the content of the current tab.
    pub fn current_content(&self) -> Option<&str> {
        self.tabs.get(self.current_tab).map(|t| t.content.as_str())
    }

    /// Get highlighted lines for the current tab.
    pub fn current_highlighted(&self) -> Option<&[Vec<HighlightedToken>]> {
        self.tabs.get(self.current_tab).map(|t| t.highlighted.as_slice())
    }
}
