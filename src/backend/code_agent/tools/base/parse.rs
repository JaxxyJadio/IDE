use std::fs;
use std::path::Path;

/// Represents a parsed Rust item (function, struct, enum, etc.).
#[derive(Debug, Clone)]
pub enum ParsedItem {
    Function { signature: String },
    Struct { name: String },
    Enum { name: String },
    Other { line: String },
}

/// Result of parsing a Rust file.
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub file: String,
    pub items: Vec<ParsedItem>,
}

/// Tool for parsing Rust source files for high-level items.
pub struct ParseTool;

impl ParseTool {
    /// Parse a Rust file and extract top-level items (functions, structs, enums).
    pub fn parse_file<P: AsRef<Path>>(path: P) -> std::io::Result<ParseResult> {
        let content = fs::read_to_string(&path)?;
        let mut items = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                let sig = trimmed.split('{').next().unwrap_or("").trim().to_string();
                items.push(ParsedItem::Function { signature: sig });
            } else if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
                let name = trimmed.split_whitespace().nth(1).unwrap_or("").to_string();
                items.push(ParsedItem::Struct { name });
            } else if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") {
                let name = trimmed.split_whitespace().nth(1).unwrap_or("").to_string();
                items.push(ParsedItem::Enum { name });
            } else if !trimmed.is_empty() {
                items.push(ParsedItem::Other { line: trimmed.to_string() });
            }
        }
        Ok(ParseResult {
            file: path.as_ref().to_string_lossy().to_string(),
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_tool() {
        let test_file = "test_parse_input.rs";
        std::fs::write(test_file, r#"
            pub struct Foo {}
            struct Bar {}
            pub enum Baz { A, B }
            fn private_func() {}
            pub fn public_func(x: i32) -> i32 { x }
            // Not a function
            let x = 5;
        "#).unwrap();

        let result = ParseTool::parse_file(Path::new(test_file)).unwrap();
        let mut found_struct = false;
        let mut found_enum = false;
        let mut found_fn = false;
        for item in result.items {
            match item {
                ParsedItem::Function { .. } => found_fn = true,
                ParsedItem::Struct { .. } => found_struct = true,
                ParsedItem::Enum { .. } => found_enum = true,
                _ => {}
            }
        }
        assert!(found_struct);
        assert!(found_enum);
        assert!(found_fn);
        std::fs::remove_file(test_file).unwrap();
    }
}
