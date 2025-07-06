use std::collections::HashMap;

/// Represents a style for a token type (e.g., color, bold, italic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub color: String, // Hex color, e.g. "#ff0000"
    pub bold: bool,
    pub italic: bool,
}

/// Token types for syntax highlighting.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    Identifier,
    String,
    Number,
    Comment,
    Symbol,
    Whitespace,
    Other,
}

/// Represents a highlighted token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightedToken {
    pub text: String,
    pub token_type: TokenType,
    pub style: Style,
}

/// Syntax highlighter for a simple code editor.
pub struct SyntaxHighlighter {
    pub theme: HashMap<TokenType, Style>,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with a default theme.
    pub fn new() -> Self {
        let mut theme = HashMap::new();
        theme.insert(TokenType::Keyword, Style { color: "#569CD6".to_string(), bold: true, italic: false });
        theme.insert(TokenType::Identifier, Style { color: "#d4d4d4".to_string(), bold: false, italic: false });
        theme.insert(TokenType::String, Style { color: "#ce9178".to_string(), bold: false, italic: false });
        theme.insert(TokenType::Number, Style { color: "#b5cea8".to_string(), bold: false, italic: false });
        theme.insert(TokenType::Comment, Style { color: "#6a9955".to_string(), bold: false, italic: true });
        theme.insert(TokenType::Symbol, Style { color: "#d4d4d4".to_string(), bold: false, italic: false });
        theme.insert(TokenType::Whitespace, Style { color: "#d4d4d4".to_string(), bold: false, italic: false });
        theme.insert(TokenType::Other, Style { color: "#d4d4d4".to_string(), bold: false, italic: false });
        Self { theme }
    }

    /// Highlight a line of code (very basic, language-agnostic).
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightedToken> {
        let keywords = ["fn", "let", "pub", "struct", "enum", "impl", "use", "mod", "if", "else", "for", "while", "loop", "match", "return", "true", "false", "const", "static", "mut", "as", "in", "break", "continue", "crate", "super", "self", "Self", "type", "where", "ref", "move", "async", "await", "dyn", "trait", "extern"];
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();
        let mut buf = String::new();
        let mut current_type = TokenType::Other;
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                if !buf.is_empty() {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                buf.push(c);
                chars.next();
                while let Some(&c2) = chars.peek() {
                    if c2.is_whitespace() {
                        buf.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(self.make_token(&buf, &TokenType::Whitespace));
                buf.clear();
                current_type = TokenType::Other;
            } else if c == '/' && chars.clone().nth(1) == Some('/') {
                if !buf.is_empty() {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                let comment: String = chars.by_ref().collect();
                tokens.push(self.make_token(&comment, &TokenType::Comment));
                break;
            } else if c == '"' {
                if !buf.is_empty() {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                buf.push(c);
                chars.next();
                while let Some(&c2) = chars.peek() {
                    buf.push(c2);
                    chars.next();
                    if c2 == '"' {
                        break;
                    }
                }
                tokens.push(self.make_token(&buf, &TokenType::String));
                buf.clear();
                current_type = TokenType::Other;
            } else if c.is_ascii_digit() {
                if !buf.is_empty() && current_type != TokenType::Number {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                current_type = TokenType::Number;
                buf.push(c);
                chars.next();
                while let Some(&c2) = chars.peek() {
                    if c2.is_ascii_digit() || c2 == '.' {
                        buf.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
            } else if c.is_alphabetic() || c == '_' {
                if !buf.is_empty() && current_type != TokenType::Identifier {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                current_type = TokenType::Identifier;
                buf.push(c);
                chars.next();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' {
                        buf.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if keywords.contains(&buf.as_str()) {
                    tokens.push(self.make_token(&buf, &TokenType::Keyword));
                } else {
                    tokens.push(self.make_token(&buf, &TokenType::Identifier));
                }
                buf.clear();
                current_type = TokenType::Other;
            } else {
                if !buf.is_empty() {
                    tokens.push(self.make_token(&buf, &current_type));
                    buf.clear();
                }
                buf.push(c);
                chars.next();
                tokens.push(self.make_token(&buf, &TokenType::Symbol));
                buf.clear();
                current_type = TokenType::Other;
            }
        }
        if !buf.is_empty() {
            tokens.push(self.make_token(&buf, &current_type));
        }
        tokens
    }

    fn make_token(&self, text: &str, token_type: &TokenType) -> HighlightedToken {
        let style = self.theme.get(token_type).cloned().unwrap_or_else(|| Style {
            color: "#d4d4d4".to_string(),
            bold: false,
            italic: false,
        });
        HighlightedToken {
            text: text.to_string(),
            token_type: token_type.clone(),
            style,
        }
    }
}
