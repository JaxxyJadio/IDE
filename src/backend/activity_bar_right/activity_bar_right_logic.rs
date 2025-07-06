use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CodeAgentActivityItem {
    Assistant,
    CodeAnalysis,
    Suggestions,
    CodeMetrics,
    CodeSearch,
}

pub struct ActivityBarRightLogic {
    active_item: Option<CodeAgentActivityItem>,
    item_states: HashMap<CodeAgentActivityItem, bool>,
    ai_enabled: bool,
}

impl Default for ActivityBarRightLogic {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityBarRightLogic {
    pub fn new() -> Self {
        let mut item_states = HashMap::new();
        item_states.insert(CodeAgentActivityItem::Assistant, true);
        item_states.insert(CodeAgentActivityItem::CodeAnalysis, false);
        item_states.insert(CodeAgentActivityItem::Suggestions, false);
        item_states.insert(CodeAgentActivityItem::CodeMetrics, false);
        item_states.insert(CodeAgentActivityItem::CodeSearch, false);
        
        Self {
            active_item: Some(CodeAgentActivityItem::Assistant),
            item_states,
            ai_enabled: true,
        }
    }
    
    pub fn set_active(&mut self, item: CodeAgentActivityItem) {
        if !self.ai_enabled {
            return;
        }
        
        // Deactivate previous
        if let Some(ref current) = self.active_item {
            self.item_states.insert(current.clone(), false);
        }
        
        // Activate new
        self.item_states.insert(item.clone(), true);
        self.active_item = Some(item);
    }
    
    pub fn get_active(&self) -> Option<&CodeAgentActivityItem> {
        self.active_item.as_ref()
    }
    
    pub fn is_active(&self, item: &CodeAgentActivityItem) -> bool {
        self.item_states.get(item).copied().unwrap_or(false)
    }
    
    pub fn set_ai_enabled(&mut self, enabled: bool) {
        self.ai_enabled = enabled;
        if !enabled {
            self.active_item = None;
            for (_, state) in self.item_states.iter_mut() {
                *state = false;
            }
        }
    }
    
    pub fn is_ai_enabled(&self) -> bool {
        self.ai_enabled
    }
}