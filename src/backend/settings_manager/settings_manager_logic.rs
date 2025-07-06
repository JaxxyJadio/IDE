// WHAT I WANT: Settings management for the Jadio IDE, including loading, saving, updating, exporting, and importing user/editor/UI/AI/terminal/git settings.
// WHAT IT DOES: Provides a persistent, serializable settings manager with ergonomic update methods for each settings section, defaulting, and file I/O.
// TODO: Add validation, error reporting, and migration for settings schema changes.
// FIXME: Handle partial/corrupt settings files more gracefully.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

/// All IDE settings grouped by category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDESettings {
    /// Editor-related settings (font, tabs, etc.)
    pub editor: EditorSettings,
    /// UI layout and theme settings
    pub ui: UISettings,
    /// AI integration and model settings
    pub ai: AISettings,
    /// Terminal emulator settings
    pub terminal: TerminalSettings,
    /// Git integration settings
    pub git: GitSettings,
}

/// Editor configuration (font, tabs, formatting, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub tab_size: usize,
    pub use_spaces: bool,
    pub word_wrap: bool,
    pub show_line_numbers: bool,
    pub highlight_current_line: bool,
    pub auto_save: bool,
    pub auto_save_delay: u64, // seconds
    pub format_on_save: bool,
    pub trim_whitespace_on_save: bool,
}

/// UI layout and theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    pub theme: Theme,
    pub window_size: (f32, f32),
    pub window_position: Option<(f32, f32)>,
    pub show_explorer: bool,
    pub show_terminal: bool,
    pub show_code_agent: bool,
    pub show_status_bar: bool,
    pub explorer_width: f32,
    pub code_agent_width: f32,
    pub terminal_height: f32,
}

/// AI integration and model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISettings {
    pub provider: AIProvider,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub enable_auto_complete: bool,
    pub enable_code_suggestions: bool,
    pub enable_code_review: bool,
}

/// Terminal emulator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub shell: String,
    pub font_family: String,
    pub font_size: f32,
    pub cursor_style: CursorStyle,
    pub scroll_back_limit: usize,
}

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSettings {
    pub user_name: String,
    pub user_email: String,
    pub auto_fetch: bool,
    pub auto_push: bool,
    pub show_diff_in_editor: bool,
}

/// Supported UI themes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    HighContrast,
    Custom(CustomTheme),
}

/// Custom theme color palette
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomTheme {
    pub name: String,
    pub background: [u8; 3],
    pub foreground: [u8; 3],
    pub accent: [u8; 3],
    pub panel: [u8; 3],
    pub border: [u8; 3],
}

/// Supported AI providers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIProvider {
    Anthropic,
    OpenAI,
    Local,
    Custom(String),
}

/// Terminal cursor style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
}

impl Default for IDESettings {
    /// Returns default settings for all categories.
    fn default() -> Self {
        Self {
            editor: EditorSettings::default(),
            ui: UISettings::default(),
            ai: AISettings::default(),
            terminal: TerminalSettings::default(),
            git: GitSettings::default(),
        }
    }
}

impl Default for EditorSettings {
    /// Returns default editor settings.
    fn default() -> Self {
        Self {
            font_family: "Fira Code".to_string(),
            font_size: 14.0,
            line_height: 1.4,
            tab_size: 4,
            use_spaces: true,
            word_wrap: false,
            show_line_numbers: true,
            highlight_current_line: true,
            auto_save: true,
            auto_save_delay: 5,
            format_on_save: true,
            trim_whitespace_on_save: true,
        }
    }
}

impl Default for UISettings {
    /// Returns default UI settings.
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            window_size: (1200.0, 800.0),
            window_position: None,
            show_explorer: true,
            show_terminal: true,
            show_code_agent: true,
            show_status_bar: true,
            explorer_width: 250.0,
            code_agent_width: 300.0,
            terminal_height: 200.0,
        }
    }
}

impl Default for AISettings {
    /// Returns default AI settings.
    fn default() -> Self {
        Self {
            provider: AIProvider::Anthropic,
            api_key: String::new(),
            model: "claude-3-sonnet-20240229".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            enable_auto_complete: true,
            enable_code_suggestions: true,
            enable_code_review: false,
        }
    }
}

impl Default for TerminalSettings {
    /// Returns default terminal settings.
    fn default() -> Self {
        Self {
            shell: if cfg!(windows) {
                "powershell.exe".to_string()
            } else {
                "/bin/bash".to_string()
            },
            font_family: "Consolas".to_string(),
            font_size: 12.0,
            cursor_style: CursorStyle::Block,
            scroll_back_limit: 10000,
        }
    }
}

impl Default for GitSettings {
    /// Returns default git settings.
    fn default() -> Self {
        Self {
            user_name: String::new(),
            user_email: String::new(),
            auto_fetch: false,
            auto_push: false,
            show_diff_in_editor: true,
        }
    }
}

/// Manages loading, saving, and updating IDE settings on disk.
pub struct SettingsManager {
    settings: IDESettings,
    settings_path: PathBuf,
}

impl SettingsManager {
    /// Create a new SettingsManager, loading from disk or creating defaults if missing.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let settings_path = Self::get_settings_path()?;
        let settings = Self::load_settings(&settings_path)?;
        
        Ok(Self {
            settings,
            settings_path,
        })
    }

    /// Get the path to the settings file (platform-specific config dir).
    fn get_settings_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        let jadio_dir = config_dir.join("jadio-ide");
        fs::create_dir_all(&jadio_dir)?;
        
        Ok(jadio_dir.join("settings.json"))
    }

    /// Load settings from the given path, or create defaults if missing.
    fn load_settings(path: &PathBuf) -> Result<IDESettings, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let settings: IDESettings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            // Create default settings file
            let default_settings = IDESettings::default();
            let content = serde_json::to_string_pretty(&default_settings)?;
            fs::write(path, content)?;
            Ok(default_settings)
        }
    }

    /// Save current settings to disk.
    pub fn save_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.settings)?;
        fs::write(&self.settings_path, content)?;
        Ok(())
    }

    /// Get a reference to all settings.
    pub fn get_settings(&self) -> &IDESettings {
        &self.settings
    }

    /// Get a mutable reference to all settings.
    pub fn get_settings_mut(&mut self) -> &mut IDESettings {
        &mut self.settings
    }

    /// Update editor settings using a closure, then save.
    pub fn update_editor_settings<F>(&mut self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut EditorSettings),
    {
        updater(&mut self.settings.editor);
        self.save_settings()
    }

    /// Update UI settings using a closure, then save.
    pub fn update_ui_settings<F>(&mut self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut UISettings),
    {
        updater(&mut self.settings.ui);
        self.save_settings()
    }

    /// Update AI settings using a closure, then save.
    pub fn update_ai_settings<F>(&mut self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut AISettings),
    {
        updater(&mut self.settings.ai);
        self.save_settings()
    }

    /// Update terminal settings using a closure, then save.
    pub fn update_terminal_settings<F>(&mut self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut TerminalSettings),
    {
        updater(&mut self.settings.terminal);
        self.save_settings()
    }

    /// Update git settings using a closure, then save.
    pub fn update_git_settings<F>(&mut self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut GitSettings),
    {
        updater(&mut self.settings.git);
        self.save_settings()
    }

    /// Reset all settings to defaults and save.
    pub fn reset_to_defaults(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.settings = IDESettings::default();
        self.save_settings()
    }

    /// Export current settings to a given path as pretty JSON.
    pub fn export_settings<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.settings)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Import settings from a given path, replacing current settings and saving.
    pub fn import_settings<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        self.settings = serde_json::from_str(&content)?;
        self.save_settings()
    }
}

impl Default for SettingsManager {
    /// Returns a default SettingsManager (in-memory, not persisted).
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            settings: IDESettings::default(),
            settings_path: PathBuf::from("settings.json"),
        })
    }
}