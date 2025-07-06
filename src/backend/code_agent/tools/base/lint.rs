use std::process::{Command, Output};
use std::path::Path;

/// Represents the result of a lint run.
#[derive(Debug, Clone)]
pub struct LintResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Tool for running lints on the entire Rust project.
pub struct LintTool;

impl LintTool {
    /// Run `cargo clippy` on the given project directory.
    pub fn run_lint<P: AsRef<Path>>(project_dir: P) -> std::io::Result<LintResult> {
        let output: Output = Command::new("cargo")
            .arg("clippy")
            .arg("--all-targets")
            .arg("--all-features")
            .current_dir(project_dir)
            .output()?;
        Ok(LintResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
