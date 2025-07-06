use std::collections::HashMap;
use std::process::{Command, Output};
use std::io;

/// Represents a user-configured script.
#[derive(Debug, Clone)]
pub struct ScriptConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub shortcut: Option<String>, // e.g. "Ctrl+Alt+R"
}

/// Represents the result of running a script.
#[derive(Debug, Clone)]
pub struct ScriptRunResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Manages and runs user scripts for the script runner panel.
pub struct ScriptRunnerLogic {
    pub scripts: HashMap<String, ScriptConfig>,
}

impl ScriptRunnerLogic {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
        }
    }

    /// Add or update a script configuration.
    pub fn add_script(&mut self, config: ScriptConfig) {
        self.scripts.insert(config.name.clone(), config);
    }

    /// Remove a script by name.
    pub fn remove_script(&mut self, name: &str) {
        self.scripts.remove(name);
    }

    /// Run a script by name.
    pub fn run_script(&self, name: &str) -> io::Result<ScriptRunResult> {
        if let Some(config) = self.scripts.get(name) {
            let output: Output = Command::new(&config.command)
                .args(&config.args)
                .output()?;
            Ok(ScriptRunResult {
                success: output.status.success(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Script not found"))
        }
    }

    /// List all configured scripts.
    pub fn list_scripts(&self) -> Vec<&ScriptConfig> {
        self.scripts.values().collect()
    }
}
