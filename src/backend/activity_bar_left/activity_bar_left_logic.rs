// WHAT I WANT: The activity bar to the left of the file explorer. logic is all contained here, and UI is handled in the frontend.
// WHAT IT DOES: 
// TODO: 
// FIXME: 

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityBarItem {
    Explorer,
    Search,
    SourceControl,
    Debug,
    Extensions,
    Settings,
}

pub struct ActivityBarLeftLogic {
    active_item: Option<ActivityBarItem>,
    item_states: HashMap<ActivityBarItem, bool>,
    callbacks: HashMap<ActivityBarItem, Box<dyn Fn() + Send + Sync>>,
}

impl Default for ActivityBarLeftLogic {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityBarLeftLogic {
    pub fn new() -> Self {
        let mut item_states = HashMap::new();
        item_states.insert(ActivityBarItem::Explorer, true);
        item_states.insert(ActivityBarItem::Search, false);
        item_states.insert(ActivityBarItem::SourceControl, false);
        item_states.insert(ActivityBarItem::Debug, false);
        item_states.insert(ActivityBarItem::Extensions, false);
        item_states.insert(ActivityBarItem::Settings, false);
        
        Self {
            active_item: Some(ActivityBarItem::Explorer),
            item_states,
            callbacks: HashMap::new(),
        }
    }
    
    pub fn set_active(&mut self, item: ActivityBarItem) {
        // Deactivate previous
        if let Some(ref current) = self.active_item {
            self.item_states.insert(current.clone(), false);
        }
        
        // Activate new
        self.item_states.insert(item.clone(), true);
        self.active_item = Some(item.clone());
        
        // Trigger callback if exists
        if let Some(callback) = self.callbacks.get(&item) {
            callback();
        }
    }
    
    pub fn get_active(&self) -> Option<&ActivityBarItem> {
        self.active_item.as_ref()
    }
    
    pub fn is_active(&self, item: &ActivityBarItem) -> bool {
        self.item_states.get(item).copied().unwrap_or(false)
    }
    
    pub fn register_callback(&mut self, item: ActivityBarItem, callback: Box<dyn Fn() + Send + Sync>) {
        self.callbacks.insert(item, callback);
    }
}