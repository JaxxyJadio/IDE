// File: src/backend/mod.rs
// Replace the entire content with:

pub mod file_system {
    include!("file_system/file_system_logic.rs");
}

pub mod project_manager {
    include!("project_manager/project_manager_logic.rs");
}

pub mod settings_manager {
    include!("settings_manager/settings_manager_logic.rs");
}

pub mod terminal_handler {
    include!("shell_terminal/shell_terminal_logic.rs");
}

// Re-exports
pub use file_system::*;
pub use project_manager::*;
pub use settings_manager::*;
pub use terminal_handler::*;