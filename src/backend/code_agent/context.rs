// WHAT I WANT: 
// WHAT IT DOES: 
// TODO: 
// FIXME: 

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileContext {
    pub path: PathBuf,
    pub content: String,
    pub language: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub line: usize,
    pub column: usize,
    pub scope: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Class,
    Method,
    Variable,
    Constant,
    Interface,
    Enum,
    Module,
}

#[derive(Debug)]
pub struct ContextManager {
    current_file: Option<String>,
    current_project: Option<String>,
    open_files: HashMap<String, FileContext>,
    recent_files: VecDeque<String>,
    project_symbols: HashMap<String, Vec<Symbol>>,
    max_recent_files: usize,
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_project: None,
            open_files: HashMap::new(),
            recent_files: VecDeque::new(),
            project_symbols: HashMap::new(),
            max_recent_files: 20,
        }
    }
    
    pub fn set_current_file(&mut self, file_path: String) {
        self.current_file = Some(file_path.clone());
        
        // Add to recent files
        if !self.recent_files.contains(&file_path) {
            self.recent_files.push_front(file_path);
            while self.recent_files.len() > self.max_recent_files {
                self.recent_files.pop_back();
            }
        }
    }
    
    pub fn get_current_file(&self) -> Option<&String> {
        self.current_file.as_ref()
    }
    
    pub fn set_current_project(&mut self, project_path: String) {
        self.current_project = Some(project_path);
    }
    
    pub fn get_current_project(&self) -> Option<&String> {
        self.current_project.as_ref()
    }
    
    pub fn add_file_context(&mut self, path: String, content: String, language: String) {
        let context = FileContext {
            path: PathBuf::from(&path),
            content: content.clone(),
            language: language.clone(),
            last_modified: chrono::Utc::now(),
            symbols: self.extract_symbols(&content, &language),
        };
        
        self.open_files.insert(path.clone(), context);
        self.set_current_file(path);
    }
    
    pub fn update_file_content(&mut self, path: &str, content: String) {
        if let Some(context) = self.open_files.get_mut(path) {
            context.content = content.clone();
            context.last_modified = chrono::Utc::now();
            context.symbols = self.extract_symbols(&content, &context.language);
        }
    }
    
    pub fn remove_file_context(&mut self, path: &str) {
        self.open_files.remove(path);
        if self.current_file.as_ref() == Some(&path.to_string()) {
            self.current_file = None;
        }
    }
    
    pub fn get_file_context(&self, path: &str) -> Option<&FileContext> {
        self.open_files.get(path)
    }
    
    pub fn get_open_files(&self) -> Vec<String> {
        self.open_files.keys().cloned().collect()
    }
    
    pub fn get_recent_files(&self) -> Vec<String> {
        self.recent_files.iter().cloned().collect()
    }
    
    fn extract_symbols(&self, content: &str, language: &str) -> Vec<Symbol> {
        // Basic symbol extraction - would be replaced with proper parsing
        let mut symbols = Vec::new();
        
        match language {
            "rust" => {
                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    
                    if trimmed.starts_with("fn ") {
                        if let Some(name) = self.extract_rust_function_name(trimmed) {
                            symbols.push(Symbol {
                                name,
                                kind: SymbolKind::Function,
                                line: line_num + 1,
                                column: line.find("fn").unwrap_or(0) + 1,
                                scope: "global".to_string(),
                            });
                        }
                    } else if trimmed.starts_with("struct ") {
                        if let Some(name) = self.extract_rust_type_name(trimmed, "struct") {
                            symbols.push(Symbol {
                                name,
                                kind: SymbolKind::Class,
                                line: line_num + 1,
                                column: line.find("struct").unwrap_or(0) + 1,
                                scope: "global".to_string(),
                            });
                        }
                    } else if trimmed.starts_with("impl ") {
                        if let Some(name) = self.extract_rust_impl_name(trimmed) {
                            symbols.push(Symbol {
                                name: format!("impl {}", name),
                                kind: SymbolKind::Class,
                                line: line_num + 1,
                                column: line.find("impl").unwrap_or(0) + 1,
                                scope: "global".to_string(),
                            });
                        }
                    }
                }
            }
            "python" => {
                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    
                    if trimmed.starts_with("def ") {
                        if let Some(name) = self.extract_python_function_name(trimmed) {
                            symbols.push(Symbol {
                                name,
                                kind: SymbolKind::Function,
                                line: line_num + 1,
                                column: line.find("def").unwrap_or(0) + 1,
                                scope: "global".to_string(),
                            });
                        }
                    } else if trimmed.starts_with("class ") {
                        if let Some(name) = self.extract_python_class_name(trimmed) {
                            symbols.push(Symbol {
                                name,
                                kind: SymbolKind::Class,
                                line: line_num + 1,
                                column: line.find("class").unwrap_or(0) + 1,
                                scope: "global".to_string(),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        
        symbols
    }
    
    fn extract_rust_function_name(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "fn" {
            let name_part = parts[1];
            if let Some(paren_pos) = name_part.find('(') {
                Some(name_part[..paren_pos].to_string())
            } else if let Some(angle_pos) = name_part.find('<') {
                Some(name_part[..angle_pos].to_string())
            } else {
                Some(name_part.to_string())
            }
        } else {
            None
        }
    }
    
    fn extract_rust_type_name(&self, line: &str, keyword: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == keyword {
            let name_part = parts[1];
            if let Some(angle_pos) = name_part.find('<') {
                Some(name_part[..angle_pos].to_string())
            } else if let Some(brace_pos) = name_part.find('{') {
                Some(name_part[..brace_pos].to_string())
            } else {
                Some(name_part.to_string())
            }
        } else {
            None
        }
    }
    
    fn extract_rust_impl_name(&self, line: &str) -> Option<String> {
        let after_impl = line.strip_prefix("impl")?.trim();
        let name = if after_impl.contains(" for ") {
            after_impl.split(" for ").nth(1)?
        } else {
            after_impl
        };
        
        let name = name.split_whitespace().next()?;
        Some(name.split('{').next()?.trim().to_string())
    }
    
    fn extract_python_function_name(&self, line: &str) -> Option<String> {
        let after_def = line.strip_prefix("def")?.trim();
        let name = after_def.split('(').next()?;
        Some(name.trim().to_string())
    }
    
    fn extract_python_class_name(&self, line: &str) -> Option<String> {
        let after_class = line.strip_prefix("class")?.trim();
        let name = after_class.split(&['(', ':'][..]).next()?;
        Some(name.trim().to_string())
    }
    
    pub fn get_symbols_for_file(&self, path: &str) -> Vec<&Symbol> {
        if let Some(context) = self.open_files.get(path) {
            context.symbols.iter().collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn search_symbols(&self, query: &str) -> Vec<(&String, &Symbol)> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (path, context) in &self.open_files {
            for symbol in &context.symbols {
                if symbol.name.to_lowercase().contains(&query_lower) {
                    results.push((path, symbol));
                }
            }
        }
        
        results
    }
}