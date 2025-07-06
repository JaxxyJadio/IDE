use std::process::Child;

/// Represents a single open shell terminal.
#[derive(Debug)]
pub struct ShellTerminal {
    pub name: String,
    pub process: Option<Child>,
    pub cwd: String,
}

/// Logic for managing the list of open shell terminals.
pub struct ShellTerminalListLogic {
    pub terminals: Vec<ShellTerminal>,
    pub active_index: Option<usize>,
}

impl ShellTerminalListLogic {
    pub fn new() -> Self {
        Self {
            terminals: Vec::new(),
            active_index: None,
        }
    }

    /// Add a new shell terminal to the list.
    pub fn add_terminal(&mut self, name: String, process: Option<Child>, cwd: String) {
        self.terminals.push(ShellTerminal { name, process, cwd });
        self.active_index = Some(self.terminals.len() - 1);
    }

    /// Remove a terminal by index.
    pub fn remove_terminal(&mut self, index: usize) {
        if index < self.terminals.len() {
            self.terminals.remove(index);
            if let Some(active) = self.active_index {
                if active == index {
                    self.active_index = None;
                } else if active > index {
                    self.active_index = Some(active - 1);
                }
            }
        }
    }

    /// Switch to a terminal by index.
    pub fn switch_terminal(&mut self, index: usize) {
        if index < self.terminals.len() {
            self.active_index = Some(index);
        }
    }

    /// Get the currently active terminal, if any.
    pub fn active_terminal(&self) -> Option<&ShellTerminal> {
        self.active_index.and_then(|i| self.terminals.get(i))
    }
}
