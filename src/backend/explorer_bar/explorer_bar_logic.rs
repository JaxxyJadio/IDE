use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub struct Explorer {
    selected_tab: ExplorerTab,
    current_directory: Option<PathBuf>,
    expanded_folders: std::collections::HashSet<PathBuf>,
    search_text: String,
    search_results: Vec<PathBuf>,
    git_status: GitStatus,
    file_tree: Vec<FileTreeNode>,
}

#[derive(Default, PartialEq)]
enum ExplorerTab {
    #[default]
    Files,
    Search,
    SourceControl,
    Extensions,
}

#[derive(Default)]
struct GitStatus {
    branch: String,
    modified_files: Vec<String>,
    staged_files: Vec<String>,
    untracked_files: Vec<String>,
}

#[derive(Clone)]
struct FileTreeNode {
    name: String,
    path: PathBuf,
    is_directory: bool,
    children: Vec<FileTreeNode>,
    is_expanded: bool,
    size: Option<u64>,
}

impl Explorer {
    pub fn new() -> Self {
        let mut explorer = Self::default();
        
        // Set initial directory to current working directory
        if let Ok(current_dir) = std::env::current_dir() {
            explorer.current_directory = Some(current_dir.clone());
            explorer.load_file_tree();
        }
        
        // Mock git status
        explorer.git_status = GitStatus {
            branch: "main".to_string(),
            modified_files: vec!["src/main.rs".to_string()],
            staged_files: vec![],
            untracked_files: vec!["temp.txt".to_string()],
        };
        
        explorer
    }

    pub fn set_workspace(&mut self, path: PathBuf) {
        self.current_directory = Some(path);
        self.expanded_folders.clear();
        self.load_file_tree();
    }

    fn load_file_tree(&mut self) {
        if let Some(ref workspace) = self.current_directory {
            self.file_tree = self.build_file_tree(workspace, 0);
        }
    }

    fn build_file_tree(&self, path: &PathBuf, depth: usize) -> Vec<FileTreeNode> {
        if depth > 3 { // Limit recursion depth for performance
            return vec![];
        }

        let mut nodes = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            entries.sort_by(|a, b| {
                match (a.file_type().map(|t| t.is_dir()).unwrap_or(false),
                       b.file_type().map(|t| t.is_dir()).unwrap_or(false)) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.file_name().cmp(&b.file_name()),
                }
            });

            for entry in entries {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Skip hidden files and common ignore patterns
                if name.starts_with('.') || name == "target" || name == "node_modules" {
                    continue;
                }

                let is_directory = path.is_dir();
                let metadata = entry.metadata().ok();
                let size = metadata.as_ref().map(|m| m.len());
                
                let children = if is_directory && self.expanded_folders.contains(&path) {
                    self.build_file_tree(&path, depth + 1)
                } else {
                    vec![]
                };

                let node = FileTreeNode {
                    name,
                    path: path.clone(),
                    is_directory,
                    children,
                    is_expanded: self.expanded_folders.contains(&path),
                    size,
                };

                nodes.push(node);
            }
        }

        nodes
    }

    fn show_file_tree_nodes(&mut self, ui: &mut egui::Ui, nodes: &[FileTreeNode], level: usize) {
        for node in nodes {
            let indent = level as f32 * 15.0;
            
            ui.horizontal(|ui| {
                ui.add_space(indent);
                
                if node.is_directory {
                    let arrow = if node.is_expanded { "📂" } else { "📁" };
                    
                    if ui.button(format!("{} {}", arrow, node.name))
                        .on_hover_text(format!("Path: {}", node.path.display()))
                        .clicked() 
                    {
                        if node.is_expanded {
                            self.expanded_folders.remove(&node.path);
                        } else {
                            self.expanded_folders.insert(node.path.clone());
                        }
                        self.load_file_tree(); // Reload to update expansion state
                    }
                } else {
                    let icon = self.get_file_icon(&node.name);
                    
                    if ui.button(format!("{} {}", icon, node.name))
                        .on_hover_text(format!("Path: {}", node.path.display()))
                        .clicked() 
                    {
                        // TODO: Signal to open file in editor
                        println!("Opening file: {:?}", node.path);
                    }
                    
                    // Show file size
                    if let Some(size) = node.size {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.small(self.format_file_size(size));
                        });
                    }
                }
            });

            // Recursively show children if expanded
            if node.is_directory && node.is_expanded {
                self.show_file_tree_nodes(ui, &node.children, level + 1);
            }
        }
    }

    fn get_file_icon(&self, filename: &str) -> &'static str {
        match filename.split('.').last().unwrap_or("") {
            "rs" => "🦀",
            "py" => "🐍", 
            "js" => "📜",
            "ts" => "📘",
            "html" => "🌐",
            "css" => "🎨",
            "json" => "📋",
            "md" => "📝",
            "txt" => "📄",
            "toml" => "⚙️",
            "yaml" | "yml" => "📊",
            "png" | "jpg" | "jpeg" | "gif" => "🖼️",
            "mp4" | "mov" | "avi" => "🎬",
            "mp3" | "wav" | "flac" => "🎵",
            "zip" | "rar" | "7z" => "📦",
            "pdf" => "📕",
            _ => "📄",
        }
    }

    fn format_file_size(&self, size: u64) -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Files, "📁");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Search, "🔍");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::SourceControl, "🌿");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Extensions, "🧩");
            });

            ui.separator();

            // Content area
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    ExplorerTab::Files => {
                        ui.horizontal(|ui| {
                            ui.heading("Files");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("🔄").on_hover_text("Refresh").clicked() {
                                    self.load_file_tree();
                                }
                                if