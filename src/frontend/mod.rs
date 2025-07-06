// C:\JadioAI\IDE\src\frontend\mod.rs
// Frontend UI module organization for JadioAI IDE

// UI Panel modules
pub mod code_agent_ui;
pub mod code_editor_ui;
pub mod explorer_ui;
pub mod other_window_ui;
pub mod shell_terminal_ui;
pub mod status_bar_ui;
pub mod top_menu_bar_ui;

// Style module
pub mod style;

// Re-export main UI components for easier imports
pub use code_agent_ui::codeagent::CodeAgent;
pub use code_agent_ui::codeagentactivitybar::CodeAgentActivityBar;
pub use code_editor_ui::code_editor::Editor;
pub use explorer_ui::explorer::Explorer;
pub use explorer_ui::exploreractivitybar::ExplorerActivityBar;
pub use shell_terminal_ui::shell_terminal::Terminal;
pub use status_bar_ui::statusbar::StatusBar;
pub use top_menu_bar_ui::dropdownmenu::DropdownMenu;
pub use top_menu_bar_ui::searchbar::SearchBar;
pub use top_menu_bar_ui::specialbuttonsbar::SpecialButtonsBar;