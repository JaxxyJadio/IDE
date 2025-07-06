/// Represents the available terminal menu tabs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalMenuTab {
    Problems,
    Output,
    Debug,
    Ports,
    Terminal,
}

/// Logic for managing the terminal menu state.
pub struct ShellTerminalMenuLogic {
    pub active_tab: TerminalMenuTab,
}

impl ShellTerminalMenuLogic {
    pub fn new() -> Self {
        Self {
            active_tab: TerminalMenuTab::Terminal,
        }
    }

    /// Switch to a different tab.
    pub fn switch_tab(&mut self, tab: TerminalMenuTab) {
        self.active_tab = tab;
    }

    /// Get the current active tab.
    pub fn current_tab(&self) -> &TerminalMenuTab {
        &self.active_tab
    }
}
