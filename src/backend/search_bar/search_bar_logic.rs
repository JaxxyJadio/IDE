use std::fs;
use std::path::PathBuf;

/// Represents a single search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub match_indices: Vec<(usize, usize)>,
}

/// Logic for the search bar (VS Code style, top center).
pub struct SearchBarLogic;

impl SearchBarLogic {
    /// Search for a query string in all files under the given root directory.
    pub fn search(root: &PathBuf, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let walker = walkdir::WalkDir::new(root).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    for (i, line) in content.lines().enumerate() {
                        let mut match_indices = Vec::new();
                        let mut start = 0;
                        while let Some(pos) = line[start..].find(query) {
                            let begin = start + pos;
                            let end = begin + query.len();
                            match_indices.push((begin, end));
                            start = end;
                        }
                        if !match_indices.is_empty() {
                            results.push(SearchResult {
                                file: entry.path().to_path_buf(),
                                line_number: i + 1,
                                line: line.to_string(),
                                match_indices,
                            });
                        }
                    }
                }
            }
        }
        results
    }
}
