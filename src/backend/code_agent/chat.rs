use std::collections::VecDeque;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: MessageMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Default)]
pub struct MessageMetadata {
    pub file_context: Option<String>,
    pub line_range: Option<(usize, usize)>,
    pub language: Option<String>,
    pub tokens_used: Option<u32>,
    pub processing_time: Option<f64>,
}

#[derive(Debug)]
pub struct ChatSession {
    pub id: String,
    pub messages: VecDeque<ChatMessage>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub max_messages: usize,
}

#[derive(Debug)]
pub struct ChatManager {
    sessions: Vec<ChatSession>,
    active_session: Option<usize>,
    system_prompts: Vec<String>,
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatManager {
    pub fn new() -> Self {
        let mut manager = Self {
            sessions: Vec::new(),
            active_session: None,
            system_prompts: Vec::new(),
        };
        
        // Create default session
        manager.create_session();
        manager
    }
    
    pub fn create_session(&mut self) -> &mut ChatSession {
        let session = ChatSession {
            id: uuid::Uuid::new_v4().to_string(),
            messages: VecDeque::new(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            max_messages: 1000,
        };
        
        self.sessions.push(session);
        self.active_session = Some(self.sessions.len() - 1);
        
        // Add system prompt if available
        if let Some(system_prompt) = self.system_prompts.first() {
            self.add_system_message(system_prompt.clone());
        }
        
        &mut self.sessions[self.sessions.len() - 1]
    }
    
    pub fn get_active_session(&mut self) -> Option<&mut ChatSession> {
        if let Some(index) = self.active_session {
            self.sessions.get_mut(index)
        } else {
            None
        }
    }
    
    pub fn add_user_message(&mut self, content: String) -> Option<&ChatMessage> {
        self.add_message(MessageRole::User, content)
    }
    
    pub fn add_assistant_message(&mut self, content: String) -> Option<&ChatMessage> {
        self.add_message(MessageRole::Assistant, content)
    }
    
    pub fn add_system_message(&mut self, content: String) -> Option<&ChatMessage> {
        self.add_message(MessageRole::System, content)
    }
    
    fn add_message(&mut self, role: MessageRole, content: String) -> Option<&ChatMessage> {
        if let Some(session) = self.get_active_session() {
            let message = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                role,
                content,
                timestamp: Utc::now(),
                metadata: MessageMetadata::default(),
            };
            
            session.messages.push_back(message);
            session.last_activity = Utc::now();
            
            // Trim old messages if exceeding limit
            while session.messages.len() > session.max_messages {
                session.messages.pop_front();
            }
            
            session.messages.back()
        } else {
            None
        }
    }
    
    pub fn get_conversation_context(&self, max_messages: usize) -> Vec<ChatMessage> {
        if let Some(index) = self.active_session {
            if let Some(session) = self.sessions.get(index) {
                let start = session.messages.len().saturating_sub(max_messages);
                session.messages.iter().skip(start).cloned().collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
    
    pub fn clear_current_session(&mut self) {
        if let Some(session) = self.get_active_session() {
            session.messages.clear();
            session.last_activity = Utc::now();
            
            // Re-add system prompt
            if let Some(system_prompt) = self.system_prompts.first() {
                self.add_system_message(system_prompt.clone());
            }
        }
    }
    
    pub fn set_system_prompt(&mut self, prompt: String) {
        self.system_prompts.clear();
        self.system_prompts.push(prompt);
    }
    
    pub fn export_session(&self, session_index: usize) -> Option<String> {
        if let Some(session) = self.sessions.get(session_index) {
            let mut export = String::new();
            export.push_str(&format!("Chat Session: {}\n", session.id));
            export.push_str(&format!("Created: {}\n", session.created_at));
            export.push_str(&format!("Last Activity: {}\n\n", session.last_activity));
            
            for message in &session.messages {
                export.push_str(&format!("[{}] {}: {}\n\n", 
                    message.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    match message.role {
                        MessageRole::User => "User",
                        MessageRole::Assistant => "Assistant",
                        MessageRole::System => "System",
                    },
                    message.content
                ));
            }
            
            Some(export)
        } else {
            None
        }
    }
    
    pub fn search_messages(&self, query: &str) -> Vec<&ChatMessage> {
        let mut results = Vec::new();
        
        for session in &self.sessions {
            for message in &session.messages {
                if message.content.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(message);
                }
            }
        }
        
        results
    }
}

// Mock UUID generation since we don't have the uuid crate
mod uuid {
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> Self {
            Self
        }
        
        pub fn to_string(&self) -> String {
            format!("{:x}-{:x}-{:x}-{:x}", 
                rand::random::<u32>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u32>()
            )
        }
    }
}

// Mock random
mod rand {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn random<T: From<u64>>() -> T {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        T::from(nanos)
    }
}