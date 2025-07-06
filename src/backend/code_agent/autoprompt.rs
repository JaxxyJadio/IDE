// WHAT I WANT: 
// WHAT IT DOES: 
// TODO: 
// FIXME: 

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AutoPromptTemplate {
    pub name: String,
    pub description: String,
    pub template: String,
    pub variables: Vec<String>,
    pub category: PromptCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PromptCategory {
    CodeGeneration,
    BugFix,
    Refactoring,
    Documentation,
    Testing,
    Optimization,
    Security,
    Custom,
}

pub struct AutoPromptEngine {
    templates: HashMap<String, AutoPromptTemplate>,
    recent_prompts: Vec<String>,
    custom_templates: HashMap<String, AutoPromptTemplate>,
}

impl Default for AutoPromptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoPromptEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
            recent_prompts: Vec::new(),
            custom_templates: HashMap::new(),
        };
        
        engine.load_default_templates();
        engine
    }
    
    fn load_default_templates(&mut self) {
        // Code generation templates
        self.add_template(AutoPromptTemplate {
            name: "generate_function".to_string(),
            description: "Generate a function with given specifications".to_string(),
            template: "Generate a {language} function that {description}. The function should {requirements}.".to_string(),
            variables: vec!["language".to_string(), "description".to_string(), "requirements".to_string()],
            category: PromptCategory::CodeGeneration,
        });
        
        // Bug fix templates
        self.add_template(AutoPromptTemplate {
            name: "fix_bug".to_string(),
            description: "Fix a bug in the code".to_string(),
            template: "Fix the following bug in this {language} code:\n{code}\nError: {error}\nProvide the corrected code and explain the fix.".to_string(),
            variables: vec!["language".to_string(), "code".to_string(), "error".to_string()],
            category: PromptCategory::BugFix,
        });
        
        // Refactoring templates
        self.add_template(AutoPromptTemplate {
            name: "refactor_code".to_string(),
            description: "Refactor code for better quality".to_string(),
            template: "Refactor this {language} code to improve {aspect}:\n{code}\nMaintain the same functionality.".to_string(),
            variables: vec!["language".to_string(), "aspect".to_string(), "code".to_string()],
            category: PromptCategory::Refactoring,
        });
        
        // Documentation templates
        self.add_template(AutoPromptTemplate {
            name: "add_documentation".to_string(),
            description: "Add documentation to code".to_string(),
            template: "Add comprehensive documentation to this {language} code:\n{code}\nInclude {doc_type} documentation.".to_string(),
            variables: vec!["language".to_string(), "code".to_string(), "doc_type".to_string()],
            category: PromptCategory::Documentation,
        });
        
        // Testing templates
        self.add_template(AutoPromptTemplate {
            name: "write_tests".to_string(),
            description: "Write tests for code".to_string(),
            template: "Write {test_type} tests for this {language} code:\n{code}\nCover edge cases and common scenarios.".to_string(),
            variables: vec!["test_type".to_string(), "language".to_string(), "code".to_string()],
            category: PromptCategory::Testing,
        });
        
        // Optimization templates
        self.add_template(AutoPromptTemplate {
            name: "optimize_performance".to_string(),
            description: "Optimize code for performance".to_string(),
            template: "Optimize this {language} code for {optimization_goal}:\n{code}\nProvide benchmarks if possible.".to_string(),
            variables: vec!["language".to_string(), "optimization_goal".to_string(), "code".to_string()],
            category: PromptCategory::Optimization,
        });
        
        // Security templates
        self.add_template(AutoPromptTemplate {
            name: "security_review".to_string(),
            description: "Review code for security issues".to_string(),
            template: "Review this {language} code for security vulnerabilities:\n{code}\nSuggest fixes for any issues found.".to_string(),
            variables: vec!["language".to_string(), "code".to_string()],
            category: PromptCategory::Security,
        });
    }
    
    fn add_template(&mut self, template: AutoPromptTemplate) {
        self.templates.insert(template.name.clone(), template);
    }
    
    pub fn get_template(&self, name: &str) -> Option<&AutoPromptTemplate> {
        self.templates.get(name).or_else(|| self.custom_templates.get(name))
    }
    
    pub fn list_templates(&self, category: Option<PromptCategory>) -> Vec<&AutoPromptTemplate> {
        let mut templates: Vec<_> = self.templates.values()
            .chain(self.custom_templates.values())
            .collect();
            
        if let Some(cat) = category {
            templates.retain(|t| t.category == cat);
        }
        
        templates
    }
    
    pub fn generate_prompt(&mut self, template_name: &str, variables: HashMap<String, String>) -> Result<String, String> {
        let template = self.get_template(template_name)
            .ok_or_else(|| format!("Template '{}' not found", template_name))?;
        
        let mut prompt = template.template.clone();
        
        for var in &template.variables {
            if let Some(value) = variables.get(var) {
                prompt = prompt.replace(&format!("{{{}}}", var), value);
            } else {
                return Err(format!("Missing variable: {}", var));
            }
        }
        
        self.recent_prompts.push(prompt.clone());
        if self.recent_prompts.len() > 50 {
            self.recent_prompts.remove(0);
        }
        
        Ok(prompt)
    }
    
    pub fn add_custom_template(&mut self, template: AutoPromptTemplate) {
        self.custom_templates.insert(template.name.clone(), template);
    }
    
    pub fn get_recent_prompts(&self) -> &[String] {
        &self.recent_prompts
    }
    
    pub fn suggest_prompt(&self, context: &str) -> Vec<String> {
        // TODO: Implement intelligent prompt suggestions based on context
        vec![
            "Explain this code".to_string(),
            "Find potential bugs".to_string(),
            "Suggest improvements".to_string(),
            "Add documentation".to_string(),
        ]
    }
}