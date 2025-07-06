// WHAT I WANT: Project management logic for Jadio IDE, including detection, creation, opening, and settings for various project types.
// WHAT IT DOES: Handles project lifecycle, type detection, file scaffolding, and project-specific settings for Rust, Python, JS/TS, HTML, and mixed projects.
// TODO: Implement persistent project settings, project migration, and advanced detection heuristics.
// FIXME: Handle edge cases for file permissions, partial/corrupt projects, and cross-platform issues.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a user project in the IDE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub last_opened: std::time::SystemTime,
    pub settings: ProjectSettings,
}

/// Supported project types for detection and scaffolding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    HTML,
    Mixed,
    Unknown,
}

/// Per-project settings (formatting, AI, linting, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub auto_save: bool,
    pub format_on_save: bool,
    pub line_endings: LineEndings,
    pub tab_size: usize,
    pub use_spaces: bool,
    pub enable_linting: bool,
    pub enable_ai_assistance: bool,
}

/// Supported line ending styles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineEndings {
    Unix,    // LF
    Windows, // CRLF
    Mac,     // CR
}

impl Default for ProjectSettings {
    /// Returns default project settings.
    fn default() -> Self {
        Self {
            auto_save: true,
            format_on_save: true,
            line_endings: if cfg!(windows) { 
                LineEndings::Windows 
            } else { 
                LineEndings::Unix 
            },
            tab_size: 4,
            use_spaces: true,
            enable_linting: true,
            enable_ai_assistance: true,
        }
    }
}

/// Manages project lifecycle, detection, and scaffolding.
pub struct ProjectManager {
    current_project: Option<Project>,
    recent_projects: Vec<Project>,
    project_cache: HashMap<PathBuf, Project>,
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectManager {
    /// Create a new ProjectManager instance.
    pub fn new() -> Self {
        Self {
            current_project: None,
            recent_projects: Vec::new(),
            project_cache: HashMap::new(),
        }
    }

    /// Detect the type of project in a given directory.
    pub fn detect_project_type<P: AsRef<Path>>(path: P) -> ProjectType {
        let path = path.as_ref();
        
        // Check for specific project files
        if path.join("Cargo.toml").exists() {
            return ProjectType::Rust;
        }
        
        if path.join("package.json").exists() {
            // Check if it's TypeScript
            if path.join("tsconfig.json").exists() {
                return ProjectType::TypeScript;
            }
            return ProjectType::JavaScript;
        }
        
        if path.join("requirements.txt").exists() || 
           path.join("setup.py").exists() ||
           path.join("pyproject.toml").exists() {
            return ProjectType::Python;
        }
        
        if path.join("index.html").exists() {
            return ProjectType::HTML;
        }
        
        // Check for common file extensions
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut rust_files = 0;
            let mut python_files = 0;
            let mut js_files = 0;
            let mut ts_files = 0;
            let mut html_files = 0;
            let mut total_files = 0;
            
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    total_files += 1;
                    match ext.to_str().unwrap_or("") {
                        "rs" => rust_files += 1,
                        "py" => python_files += 1,
                        "js" => js_files += 1,
                        "ts" => ts_files += 1,
                        "html" => html_files += 1,
                        _ => {}
                    }
                }
            }
            
            if total_files > 0 {
                let threshold = total_files / 3; // At least 1/3 of files should be of the same type
                
                if rust_files >= threshold { return ProjectType::Rust; }
                if python_files >= threshold { return ProjectType::Python; }
                if ts_files >= threshold { return ProjectType::TypeScript; }
                if js_files >= threshold { return ProjectType::JavaScript; }
                if html_files >= threshold { return ProjectType::HTML; }
                
                if rust_files + python_files + js_files + ts_files + html_files > 1 {
                    return ProjectType::Mixed;
                }
            }
        }
        
        ProjectType::Unknown
    }

    /// Open a project at the given path, updating recent projects and cache.
    pub fn open_project<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() || !path.is_dir() {
            return Err("Invalid project path".into());
        }
        
        let project_type = Self::detect_project_type(&path);
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let project = Project {
            name,
            path: path.clone(),
            project_type,
            last_opened: std::time::SystemTime::now(),
            settings: ProjectSettings::default(),
        };
        
        // Add to recent projects if not already there
        self.recent_projects.retain(|p| p.path != path);
        self.recent_projects.insert(0, project.clone());
        
        // Keep only last 10 recent projects
        self.recent_projects.truncate(10);
        
        self.current_project = Some(project.clone());
        self.project_cache.insert(path, project);
        
        Ok(())
    }

    /// Close the current project.
    pub fn close_project(&mut self) {
        self.current_project = None;
    }

    /// Get a reference to the current project, if any.
    pub fn get_current_project(&self) -> Option<&Project> {
        self.current_project.as_ref()
    }

    /// Get a slice of recent projects.
    pub fn get_recent_projects(&self) -> &[Project] {
        &self.recent_projects
    }

    /// Create a new project of the given type, scaffolding files as needed.
    pub fn create_new_project<P: AsRef<Path>>(
        &mut self, 
        path: P, 
        name: String, 
        project_type: ProjectType
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref().join(&name);
        
        if path.exists() {
            return Err("Project directory already exists".into());
        }
        
        std::fs::create_dir_all(&path)?;
        
        // Create project-specific files based on type
        match project_type {
            ProjectType::Rust => {
                self.create_rust_project(&path, &name)?;
            }
            ProjectType::Python => {
                self.create_python_project(&path, &name)?;
            }
            ProjectType::JavaScript => {
                self.create_javascript_project(&path, &name)?;
            }
            ProjectType::TypeScript => {
                self.create_typescript_project(&path, &name)?;
            }
            ProjectType::HTML => {
                self.create_html_project(&path, &name)?;
            }
            _ => {
                // Create a basic README for unknown/mixed projects
                std::fs::write(path.join("README.md"), format!("# {}\n\nProject description here.", name))?;
            }
        }
        
        self.open_project(path)?;
        Ok(())
    }

    /// Scaffold a new Rust project (Cargo.toml, src/main.rs).
    fn create_rust_project(&self, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create Cargo.toml
        let cargo_toml = format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#, name.replace('-', "_"));
        
        std::fs::write(path.join("Cargo.toml"), cargo_toml)?;
        
        // Create src directory and main.rs
        std::fs::create_dir_all(path.join("src"))?;
        std::fs::write(
            path.join("src").join("main.rs"),
            "fn main() {\n    println!(\"Hello, world!\");\n}"
        )?;
        
        Ok(())
    }

    /// Scaffold a new Python project (main.py, requirements.txt, README.md).
    fn create_python_project(&self, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create main.py
        std::fs::write(
            path.join("main.py"),
            "#!/usr/bin/env python3\n\ndef main():\n    print(\"Hello, world!\")\n\nif __name__ == \"__main__\":\n    main()\n"
        )?;
        
        // Create requirements.txt
        std::fs::write(path.join("requirements.txt"), "")?;
        
        // Create README.md
        std::fs::write(
            path.join("README.md"),
            format!("# {}\n\nA Python project.\n\n## Setup\n\n```bash\npip install -r requirements.txt\npython main.py\n```\n", name)
        )?;
        
        Ok(())
    }

    /// Scaffold a new JavaScript project (package.json, index.js).
    fn create_javascript_project(&self, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create package.json
        let package_json = format!(
r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {{
    "start": "node index.js"
  }},
  "dependencies": {{}}
}}
"#, name);
        
        std::fs::write(path.join("package.json"), package_json)?;
        
        // Create index.js
        std::fs::write(
            path.join("index.js"),
            "console.log('Hello, world!');\n"
        )?;
        
        Ok(())
    }

    /// Scaffold a new TypeScript project (package.json, tsconfig.json, src/index.ts).
    fn create_typescript_project(&self, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.create_javascript_project(path, name)?;
        
        // Update package.json for TypeScript
        let package_json = format!(
r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "dist/index.js",
  "scripts": {{
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "tsc --watch"
  }},
  "devDependencies": {{
    "typescript": "^4.0.0",
    "@types/node": "^14.0.0"
  }}
}}
"#, name);
        
        std::fs::write(path.join("package.json"), package_json)?;
        
        // Create tsconfig.json
        let tsconfig = r#"{
  "compilerOptions": {
    "target": "es2020",
    "module": "commonjs",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true
  }
}
"#;
        std::fs::write(path.join("tsconfig.json"), tsconfig)?;
        
        // Create src directory and index.ts
        std::fs::create_dir_all(path.join("src"))?;
        std::fs::write(
            path.join("src").join("index.ts"),
            "console.log('Hello, TypeScript world!');\n"
        )?;
        
        // Remove the JavaScript index.js
        let _ = std::fs::remove_file(path.join("index.js"));
        
        Ok(())
    }

    /// Scaffold a new HTML project (index.html, style.css, script.js).
    fn create_html_project(&self, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create index.html
        let html = format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <h1>Welcome to {}</h1>
    <p>Your web project starts here!</p>
    <script src="script.js"></script>
</body>
</html>
"#, name, name);
        
        std::fs::write(path.join("index.html"), html)?;
        
        // Create style.css
        let css = "body {\n    font-family: Arial, sans-serif;\n    margin: 0;\n    padding: 20px;\n    background-color: #f0f0f0;\n}\nh1 { color: #333; text-align: center; }\np { text-align: center; font-size: 18px; }\n";
        std::fs::write(path.join("style.css"), css)?;
        // Create script.js
        std::fs::write(
            path.join("script.js"),
            "console.log('Hello, web world!');\n"
        )?;
        Ok(())
    }

    /// Save current project settings (TODO: persist to disk).
    pub fn save_project_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(project) = &self.current_project {
            // TODO: Implement saving project settings to .jadio/project.json
            // For now, just update the cache
            self.project_cache.insert(project.path.clone(), project.clone());
        }
        Ok(())
    }

    /// Load project settings from disk (TODO: implement real loading).
    pub fn load_project_settings<P: AsRef<Path>>(&mut self, _path: P) -> Result<ProjectSettings, Box<dyn std::error::Error>> {
        // TODO: Implement loading project settings from .jadio/project.json
        // For now, return default settings
        Ok(ProjectSettings::default())
    }
}