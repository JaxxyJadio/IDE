use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub provider: ModelProvider,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub model_path: Option<PathBuf>,
    pub parameters: ModelParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    Anthropic,
    OpenAI,
    Local,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub stop_sequences: Vec<String>,
}

#[derive(Debug)]
pub struct ModelLoader {
    models: HashMap<String, ModelConfig>,
    active_model: Option<String>,
    model_cache: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 4096,
            top_p: 0.9,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: Vec::new(),
        }
    }
}

impl Default for ModelLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelLoader {
    pub fn new() -> Self {
        let mut loader = Self {
            models: HashMap::new(),
            active_model: None,
            model_cache: HashMap::new(),
        };
        
        // Load default models
        loader.load_default_models();
        loader
    }
    
    fn load_default_models(&mut self) {
        // Anthropic models
        self.register_model(ModelConfig {
            name: "claude-3-opus".to_string(),
            provider: ModelProvider::Anthropic,
            api_endpoint: Some("https://api.anthropic.com/v1/messages".to_string()),
            api_key: None,
            model_path: None,
            parameters: ModelParameters {
                temperature: 0.7,
                max_tokens: 4096,
                ..Default::default()
            },
        });
        
        self.register_model(ModelConfig {
            name: "claude-3-sonnet".to_string(),
            provider: ModelProvider::Anthropic,
            api_endpoint: Some("https://api.anthropic.com/v1/messages".to_string()),
            api_key: None,
            model_path: None,
            parameters: ModelParameters {
                temperature: 0.7,
                max_tokens: 4096,
                ..Default::default()
            },
        });
        
        // OpenAI models
        self.register_model(ModelConfig {
            name: "gpt-4".to_string(),
            provider: ModelProvider::OpenAI,
            api_endpoint: Some("https://api.openai.com/v1/chat/completions".to_string()),
            api_key: None,
            model_path: None,
            parameters: ModelParameters {
                temperature: 0.7,
                max_tokens: 4096,
                ..Default::default()
            },
        });
        
        self.register_model(ModelConfig {
            name: "gpt-3.5-turbo".to_string(),
            provider: ModelProvider::OpenAI,
            api_endpoint: Some("https://api.openai.com/v1/chat/completions".to_string()),
            api_key: None,
            model_path: None,
            parameters: ModelParameters {
                temperature: 0.7,
                max_tokens: 4096,
                ..Default::default()
            },
        });
    }
    
    pub fn register_model(&mut self, config: ModelConfig) {
        self.models.insert(config.name.clone(), config);
    }
    
    pub fn load_model(&mut self, name: &str) -> Result<(), String> {
        if !self.models.contains_key(name) {
            return Err(format!("Model '{}' not found", name));
        }
        
        // Check if model needs API key
        if let Some(config) = self.models.get(name) {
            match &config.provider {
                ModelProvider::Anthropic | ModelProvider::OpenAI => {
                    if config.api_key.is_none() {
                        return Err(format!("API key required for model '{}'", name));
                    }
                }
                ModelProvider::Local => {
                    if config.model_path.is_none() {
                        return Err(format!("Model path required for local model '{}'", name));
                    }
                }
                _ => {}
            }
        }
        
        self.active_model = Some(name.to_string());
        Ok(())
    }
    
    pub fn get_active_model(&self) -> Option<&ModelConfig> {
        self.active_model.as_ref()
            .and_then(|name| self.models.get(name))
    }
    
    pub fn set_model_api_key(&mut self, model_name: &str, api_key: String) -> Result<(), String> {
        if let Some(config) = self.models.get_mut(model_name) {
            config.api_key = Some(api_key);
            Ok(())
        } else {
            Err(format!("Model '{}' not found", model_name))
        }
    }
    
    pub fn set_model_parameters(&mut self, model_name: &str, params: ModelParameters) -> Result<(), String> {
        if let Some(config) = self.models.get_mut(model_name) {
            config.parameters = params;
            Ok(())
        } else {
            Err(format!("Model '{}' not found", model_name))
        }
    }
    
    pub fn list_models(&self) -> Vec<&ModelConfig> {
        self.models.values().collect()
    }
    
    pub fn list_models_by_provider(&self, provider: &ModelProvider) -> Vec<&ModelConfig> {
        self.models.values()
            .filter(|config| match (&config.provider, provider) {
                (ModelProvider::Anthropic, ModelProvider::Anthropic) => true,
                (ModelProvider::OpenAI, ModelProvider::OpenAI) => true,
                (ModelProvider::Local, ModelProvider::Local) => true,
                (ModelProvider::Custom(a), ModelProvider::Custom(b)) => a == b,
                _ => false,
            })
            .collect()
    }
    
    pub fn validate_model_config(&self, config: &ModelConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if config.name.is_empty() {
            errors.push("Model name cannot be empty".to_string());
        }
        
        match &config.provider {
            ModelProvider::Anthropic | ModelProvider::OpenAI => {
                if config.api_endpoint.is_none() {
                    errors.push("API endpoint required for cloud models".to_string());
                }
            }
            ModelProvider::Local => {
                if config.model_path.is_none() {
                    errors.push("Model path required for local models".to_string());
                } else if let Some(path) = &config.model_path {
                    if !path.exists() {
                        errors.push(format!("Model file not found: {:?}", path));
                    }
                }
            }
            _ => {}
        }
        
        if config.parameters.temperature < 0.0 || config.parameters.temperature > 2.0 {
            errors.push("Temperature must be between 0.0 and 2.0".to_string());
        }
        
        if config.parameters.max_tokens == 0 {
            errors.push("Max tokens must be greater than 0".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    pub fn export_config(&self, model_name: &str) -> Result<String, String> {
        if let Some(config) = self.models.get(model_name) {
            serde_json::to_string_pretty(config)
                .map_err(|e| format!("Failed to export config: {}", e))
        } else {
            Err(format!("Model '{}' not found", model_name))
        }
    }
    
    pub fn import_config(&mut self, json_str: &str) -> Result<(), String> {
        let config: ModelConfig = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
        
        self.validate_model_config(&config)
            .map_err(|errors| errors.join(", "))?;
        
        self.register_model(config);
        Ok(())
    }
    
    pub fn clear_cache(&mut self) {
        self.model_cache.clear();
    }
}