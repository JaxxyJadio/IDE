// C:\JadioAI\IDE\src\mod.rs
// Module organization for JadioAI IDE

// Backend logic modules
pub mod backend;

// UI Panel modules
pub mod codeagent;
pub mod codeagentactivitybar;
pub mod dropdownmenu;
pub mod editor;
pub mod explorer;
pub mod exploreractivitybar;
pub mod searchbar;
pub mod specialbuttonsbar;
pub mod statusbar;
pub mod terminal;

// Re-export for easier imports
pub use codeagent::CodeAgent;
pub use codeagentactivitybar::CodeAgentActivityBar;
pub use dropdownmenu::DropdownMenu;
pub use editor::Editor;
pub use explorer::Explorer;
pub use exploreractivitybar::ExplorerActivityBar;
pub use searchbar::SearchBar;
pub use specialbuttonsbar::SpecialButtonsBar;
pub use statusbar::StatusBar;
pub use terminal::Terminal;