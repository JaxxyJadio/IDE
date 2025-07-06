use std::fs;
use std::path::Path;

/// Represents the result of a docstring audit.
#[derive(Debug, Clone)]
pub struct DocstringAuditResult {
    pub file: String,
    pub missing_docstrings: Vec<String>,
    pub total_functions: usize,
    pub documented_functions: usize,
}

/// Tool for auditing and suggesting docstrings for Rust source files.
pub struct DocstringAuditTool;

impl DocstringAuditTool {
    /// Audit a Rust file for missing docstrings on functions.
    pub fn audit_file<P: AsRef<Path>>(path: P) -> std::io::Result<DocstringAuditResult> {
        let content = fs::read_to_string(&path)?;
        let mut missing_docstrings = Vec::new();
        let mut total_functions = 0;
        let mut documented_functions = 0;
        let mut lines = content.lines().peekable();
        while let Some(line) = lines.next() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("fn ") {
                total_functions += 1;
                // Look for docstring above
                let mut has_doc = false;
                for prev in content[..content.find(line).unwrap_or(0)].lines().rev() {
                    let prev_trim = prev.trim_start();
                    if prev_trim.is_empty() {
                        continue;
                    }
                    if prev_trim.starts_with("///") || prev_trim.starts_with("/**") {
                        has_doc = true;
                        break;
                    }
                    if !prev_trim.starts_with("#") && !prev_trim.starts_with("pub") && !prev_trim.starts_with("fn ") {
                        break;
                    }
                }
                if has_doc {
                    documented_functions += 1;
                } else {
                    // Extract function signature
                    let sig = trimmed.split('{').next().unwrap_or("").trim().to_string();
                    missing_docstrings.push(sig);
                }
            }
        }
        Ok(DocstringAuditResult {
            file: path.as_ref().to_string_lossy().to_string(),
            missing_docstrings,
            total_functions,
            documented_functions,
        })
    }

    /// Suggest a docstring for a given function signature.
    pub fn suggest_docstring(signature: &str) -> String {
        format!("/// TODO: Document this function\n{}", signature)
    }
}
