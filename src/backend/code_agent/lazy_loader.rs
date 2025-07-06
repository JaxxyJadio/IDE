use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LazyResource<T> {
    pub id: String,
    pub loader: Arc<dyn Fn() -> Result<T, String> + Send + Sync>,
    pub cached_value: Arc<Mutex<Option<T>>>,
    pub last_loaded: Arc<Mutex<Option<std::time::Instant>>>,
    pub ttl_seconds: Option<u64>,
}

pub struct LazyLoader {
    resources: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    loading_strategies: HashMap<String, LoadingStrategy>,
    cache_size_limit: usize,
    current_cache_size: usize,
}

#[derive(Debug, Clone)]
pub enum LoadingStrategy {
    Eager,      // Load immediately
    Lazy,       // Load on first access
    Background, // Load in background thread
    Scheduled,  // Load at specific intervals
}

impl Default for LazyLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl LazyLoader {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            loading_strategies: HashMap::new(),
            cache_size_limit: 100 * 1024 * 1024, // 100MB default
            current_cache_size: 0,
        }
    }
    
    pub fn register_file_loader(&mut self, id: String, path: PathBuf, strategy: LoadingStrategy) {
        let loader = Arc::new(move || -> Result<String, String> {
            std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to load file: {}", e))
        });
        
        let resource = LazyResource {
            id: id.clone(),
            loader,
            cached_value: Arc::new(Mutex::new(None)),
            last_loaded: Arc::new(Mutex::new(None)),
            ttl_seconds: Some(300), // 5 minute TTL
        };
        
        self.resources.insert(id.clone(), Box::new(resource));
        self.loading_strategies.insert(id, strategy);
    }
    
    pub fn register_model_loader(&mut self, id: String, model_path: PathBuf) {
        let loader = Arc::new(move || -> Result<ModelData, String> {
            // Simulate loading a model file
            Ok(ModelData {
                name: model_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                size: std::fs::metadata(&model_path)
                    .map(|m| m.len())
                    .unwrap_or(0),
                loaded: true,
            })
        });
        
        let resource = LazyResource {
            id: id.clone(),
            loader,
            cached_value: Arc::new(Mutex::new(None)),
            last_loaded: Arc::new(Mutex::new(None)),
            ttl_seconds: None, // Models don't expire
        };
        
        self.resources.insert(id.clone(), Box::new(resource));
        self.loading_strategies.insert(id, LoadingStrategy::Lazy);
    }
    
    pub fn get_file(&mut self, id: &str) -> Result<String, String> {
        if let Some(resource) = self.resources.get(id) {
            if let Some(lazy_resource) = resource.downcast_ref::<LazyResource<String>>() {
                self.load_if_needed(lazy_resource)
            } else {
                Err("Resource type mismatch".to_string())
            }
        } else {
            Err(format!("Resource '{}' not found", id))
        }
    }
    
    pub fn get_model(&mut self, id: &str) -> Result<ModelData, String> {
        if let Some(resource) = self.resources.get(id) {
            if let Some(lazy_resource) = resource.downcast_ref::<LazyResource<ModelData>>() {
                self.load_if_needed(lazy_resource)
            } else {
                Err("Resource type mismatch".to_string())
            }
        } else {
            Err(format!("Model '{}' not found", id))
        }
    }
    
    fn load_if_needed<T: Clone>(&self, resource: &LazyResource<T>) -> Result<T, String> {
        let mut cached = resource.cached_value.lock().unwrap();
        let mut last_loaded = resource.last_loaded.lock().unwrap();
        
        // Check if we need to reload
        let should_reload = if let Some(ref value) = *cached {
            if let Some(ttl) = resource.ttl_seconds {
                if let Some(loaded_time) = *last_loaded {
                    loaded_time.elapsed().as_secs() > ttl
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            true
        };
        
        if should_reload {
            // Load the resource
            let loaded_value = (resource.loader)()?;
            *cached = Some(loaded_value.clone());
            *last_loaded = Some(std::time::Instant::now());
            Ok(loaded_value)
        } else {
            Ok(cached.as_ref().unwrap().clone())
        }
    }
    
    pub fn preload_all(&mut self) {
        let ids: Vec<String> = self.resources.keys().cloned().collect();
        
        for id in ids {
            if let Some(strategy) = self.loading_strategies.get(&id) {
                match strategy {
                    LoadingStrategy::Eager | LoadingStrategy::Background => {
                        // Try to load the resource
                        let _ = self.get_file(&id).or_else(|_| self.get_model(&id).map(|_| String::new()));
                    }
                    _ => {}
                }
            }
        }
    }
    
    pub fn invalidate(&mut self, id: &str) {
        if let Some(resource) = self.resources.get(id) {
            // This is a simplified version - in reality, we'd need to handle different types
            if let Some(lazy_resource) = resource.downcast_ref::<LazyResource<String>>() {
                let mut cached = lazy_resource.cached_value.lock().unwrap();
                *cached = None;
            }
        }
    }
    
    pub fn invalidate_all(&mut self) {
        let ids: Vec<String> = self.resources.keys().cloned().collect();
        for id in ids {
            self.invalidate(&id);
        }
    }
    
    pub fn set_cache_limit(&mut self, limit_bytes: usize) {
        self.cache_size_limit = limit_bytes;
    }
    
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            total_resources: self.resources.len(),
            cache_size_bytes: self.current_cache_size,
            cache_limit_bytes: self.cache_size_limit,
            loading_strategies: self.loading_strategies.clone(),
        }
    }
    
    pub fn cleanup_expired(&mut self) {
        let ids: Vec<String> = self.resources.keys().cloned().collect();
        
        for id in ids {
            if let Some(resource) = self.resources.get(&id) {
                // Check if resource has expired TTL
                // This is simplified - would need to handle different types properly
                if let Some(lazy_resource) = resource.downcast_ref::<LazyResource<String>>() {
                    if let Some(ttl) = lazy_resource.ttl_seconds {
                        let last_loaded = lazy_resource.last_loaded.lock().unwrap();
                        if let Some(loaded_time) = *last_loaded {
                            if loaded_time.elapsed().as_secs() > ttl * 2 {
                                // Expired for more than double TTL, remove from cache
                                let mut cached = lazy_resource.cached_value.lock().unwrap();
                                *cached = None;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelData {
    pub name: String,
    pub size: u64,
    pub loaded: bool,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_resources: usize,
    pub cache_size_bytes: usize,
    pub cache_limit_bytes: usize,
    pub loading_strategies: HashMap<String, LoadingStrategy>,
}

// Helper trait for downcasting
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for LazyResource<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}