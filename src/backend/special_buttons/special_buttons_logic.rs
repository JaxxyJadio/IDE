/// Represents a special button and its action.
#[derive(Debug, Clone)]
pub struct SpecialButton {
    pub label: String,
    pub tooltip: String,
    pub shortcut: Option<String>, // e.g. "Ctrl+Alt+1"
    pub action: SpecialButtonAction,
}

#[derive(Debug, Clone)]
pub enum SpecialButtonAction {
    RunScript(String), // script name
    OpenFile(String),  // file path
    Custom(String),    // custom command or action
}

/// Logic for managing the 5 special shortcut buttons.
pub struct SpecialButtonsLogic {
    pub buttons: Vec<SpecialButton>,
}

impl SpecialButtonsLogic {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    /// Set the configuration for the 5 special buttons.
    pub fn set_buttons(&mut self, buttons: Vec<SpecialButton>) {
        self.buttons = buttons.into_iter().take(5).collect();
    }

    /// Trigger the action for a button by index.
    pub fn trigger(&self, index: usize) -> Option<&SpecialButtonAction> {
        self.buttons.get(index).map(|b| &b.action)
    }
}
