use std::fs;
use std::path::Path;

/// Represents a documentation generation result.
#[derive(Debug, Clone)]
pub struct DocumentationResult {
    pub file: String,
    pub doc_comments: Vec<String>,
}

/// Tool for generating and extracting documentation from Rust source files.
pub struct DocumentationTool;

impl DocumentationTool {
    /// Extract all doc comments from a Rust file.
    pub fn extract_doc_comments<P: AsRef<Path>>(path: P) -> std::io::Result<DocumentationResult> {
        let content = fs::read_to_string(&path)?;
        let mut doc_comments = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("///") || trimmed.starts_with("/**") {
                doc_comments.push(trimmed.to_string());
            }
        }
        Ok(DocumentationResult {
            file: path.as_ref().to_string_lossy().to_string(),
            doc_comments,
        })
    }

    /// Generate a Markdown documentation stub for a Rust file.
    pub fn generate_markdown_stub<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
        let content = fs::read_to_string(&path)?;
        let mut doc = String::new();
        doc.push_str(&format!("# Documentation for {}\n\n", path.as_ref().to_string_lossy()));
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("///") {
                doc.push_str(trimmed.trim_start_matches("///").trim());
                doc.push_str("\n");
            }
        }
        Ok(doc)
    }

    /// Suggest a documentation template for a function or struct.
    pub fn suggest_template(name: &str, kind: &str) -> String {
        match kind {
            "function" => format!("/// {}: Describe what this function does.\npub fn {}() {{}}", name, name),
            "struct" => format!("/// {}: Describe the purpose of this struct.\npub struct {} {{}}", name, name),
            _ => format!("/// {}: Add documentation.", name),
        }
    }
}
