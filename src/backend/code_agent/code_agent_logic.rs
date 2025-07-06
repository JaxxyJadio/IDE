use super::{agent::CodeAgent, chat::ChatManager, autoprompt::AutoPromptEngine, context::ContextManager};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CodeAgentSystem {
    agent: Arc<Mutex<CodeAgent>>,
    chat_manager: Arc<Mutex<ChatManager>>,
    prompt_engine: Arc<Mutex<AutoPromptEngine>>,
    context_manager: Arc<Mutex<ContextManager>>,
    runtime: Option<tokio::runtime::Runtime>,
}

impl Default for CodeAgentSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeAgentSystem {
    pub fn new() -> Self {
        let config = super::agent::AgentConfig::default();
        
        Self {
            agent: Arc::new(Mutex::new(CodeAgent::new(config))),
            chat_manager: Arc::new(Mutex::new(ChatManager::new())),
            prompt_engine: Arc::new(Mutex::new(AutoPromptEngine::new())),
            context_manager: Arc::new(Mutex::new(ContextManager::new())),
            runtime: tokio::runtime::Runtime::new().ok(),
        }
    }
    
    pub fn process_user_message(&mut self, message: String) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(runtime) = &self.runtime {
            let agent = self.agent.clone();
            let chat_manager = self.chat_manager.clone();
            
            runtime.block_on(async move {
                // Add user message to chat
                let mut chat = chat_manager.lock().await;
                chat.add_user_message(message.clone());
                drop(chat);
                
                // Process with agent
                let agent = agent.lock().await;
                let response = agent.process_message(message).await?;
                drop(agent);
                
                // Add assistant response to chat
                let mut chat = chat_manager.lock().await;
                chat.add_assistant_message(response.clone());
                
                Ok(response)
            })
        } else {
            Err("Tokio runtime not available".into())
        }
    }
    
    pub fn analyze_current_code(&mut self, code: String, language: String) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(runtime) = &self.runtime {
            let agent = self.agent.clone();
            
            runtime.block_on(async move {
                let agent = agent.lock().await;
                agent.analyze_code(code, language).await
            })
        } else {
            Err("Tokio runtime not available".into())
        }
    }
    
    pub fn get_code_suggestions(&mut self, code: String, cursor_pos: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if let Some(runtime) = &self.runtime {
            let agent = self.agent.clone();
            
            runtime.block_on(async move {
                let agent = agent.lock().await;
                agent.suggest_completion(code, cursor_pos).await
            })
        } else {
            Err("Tokio runtime not available".into())
        }
    }
    
    pub fn update_context(&mut self, file: Option<String>, project: Option<String>) {
        if let Some(runtime) = &self.runtime {
            let agent = self.agent.clone();
            let context_manager = self.context_manager.clone();
            
            runtime.block_on(async move {
                // Update agent context
                let agent = agent.lock().await;
                agent.update_context(file.clone(), project.clone()).await;
                drop(agent);
                
                // Update context manager
                let mut ctx = context_manager.lock().await;
                if let Some(f) = file {
                    ctx.set_current_file(f);
                }
                if let Some(p) = project {
                    ctx.set_current_project(p);
                }
            });
        }
    }
    
    pub fn get_chat_history(&self) -> Vec<String> {
        if let Some(runtime) = &self.runtime {
            let chat_manager = self.chat_manager.clone();
            
            runtime.block_on(async move {
                let chat = chat_manager.lock().await;
                chat.get_conversation_context(50)
                    .iter()
                    .map(|msg| format!("{}: {}", 
                        match msg.role {
                            super::chat::MessageRole::User => "You",
                            super::chat::MessageRole::Assistant => "AI",
                            super::chat::MessageRole::System => "System",
                        },
                        msg.content
                    ))
                    .collect()
            })
        } else {
            Vec::new()
        }
    }
    
    pub fn clear_chat(&mut self) {
        if let Some(runtime) = &self.runtime {
            let chat_manager = self.chat_manager.clone();
            
            runtime.block_on(async move {
                let mut chat = chat_manager.lock().await;
                chat.clear_current_session();
            });
        }
    }
    
    pub fn get_prompt_suggestions(&self, context: &str) -> Vec<String> {
        if let Some(runtime) = &self.runtime {
            let prompt_engine = self.prompt_engine.clone();
            
            runtime.block_on(async move {
                let engine = prompt_engine.lock().await;
                engine.suggest_prompt(context)
            })
        } else {
            Vec::new()
        }
    }
    
    pub fn execute_quick_action(&