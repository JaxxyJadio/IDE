/// Represents an item in the status bar.
#[derive(Debug, Clone)]
pub struct StatusBarItem {
    pub label: String,
    pub value: String,
    pub tooltip: Option<String>,
    pub clickable: bool,
    pub on_click: Option<StatusBarAction>,
}

#[derive(Debug, Clone)]
pub enum StatusBarAction {
    OpenSettings,
    OpenGit,
    Custom(String),
}

/// Logic for managing the status bar at the bottom of the IDE.
pub struct StatusBarLogic {
    pub items: Vec<StatusBarItem>,
}

impl StatusBarLogic {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Set the status bar items (e.g., git branch, settings, etc).
    pub fn set_items(&mut self, items: Vec<StatusBarItem>) {
        self.items = items;
    }

    /// Update a status bar item by label.
    pub fn update_item(&mut self, label: &str, value: String) {
        if let Some(item) = self.items.iter_mut().find(|i| i.label == label) {
            item.value = value;
        }
    }

    /// Handle a click on a status bar item.
    pub fn handle_click(&self, label: &str) -> Option<&StatusBarAction> {
        self.items.iter().find(|i| i.label == label).and_then(|i| i.on_click.as_ref())
    }
}
