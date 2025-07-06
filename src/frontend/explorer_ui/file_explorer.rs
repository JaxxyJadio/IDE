use eframe::egui;
use crate::backend::{FileSystem, FileEntry};
use std::path::PathBuf;

#[derive(Default)]
pub struct Explorer {
    selected_tab: ExplorerTab,
    file_system: FileSystem,
    current_entries: Vec<FileEntry>,
    expanded_dirs: std::collections::HashSet<PathBuf>,
    search_query: String,
    selected_file: Option<PathBuf>,
    // Channel to communicate with editor
    file_open_sender: Option<std::sync::mpsc::Sender<(String, String)>>,
}

#[derive(Default, PartialEq)]
enum ExplorerTab {
    #[default]
    Files,
    Search,
    SourceControl,
    Extensions,
}

impl Explorer {
    pub fn new() -> Self {
        let mut explorer = Self::default();
        explorer.file_system = FileSystem::new();
        
        // Set initial workspace to current directory
        if let Ok(cwd) = std::env::current_dir() {
            let _ = explorer.file_system.set_workspace(&cwd);
            explorer.refresh_entries();
        }
        
        explorer
    }

    pub fn set_file_open_channel(&mut self, sender: std::sync::mpsc::Sender<(String, String)>) {
        self.file_open_sender = Some(sender);
    }

    pub fn open_workspace(&mut self, path: PathBuf) -> std::io::Result<()> {
        self.file_system.set_workspace(&path)?;
        self.expanded_dirs.clear();
        self.expanded_dirs.insert(path);
        self.refresh_entries();
        Ok(())
    }

    fn refresh_entries(&mut self) {
        if let Some(workspace) = self.file_system.get_workspace() {
            match self.file_system.list_directory(workspace) {
                Ok(entries) => self.current_entries = entries,
                Err(e) => eprintln!("Failed to list directory: {}", e),
            }
        }
    }

    fn get_directory_contents(&self, path: &PathBuf) -> Vec<FileEntry> {
        match self.file_system.list_directory(path) {
            Ok(entries) => entries,
            Err(_) => vec![],
        }
    }

    fn show_file_tree(&mut self, ui: &mut egui::Ui, entries: &[FileEntry], level: usize) {
        for entry in entries {
            let indent = level as f32 * 20.0;
            
            ui.horizontal(|ui| {
                ui.add_space(indent);
                
                if entry.is_directory {
                    let is_expanded = self.expanded_dirs.contains(&entry.path);
                    let icon = if is_expanded { "📂" } else { "📁" };
                    
                    if ui.button(format!("{} {}", icon, entry.name)).clicked() {
                        if is_expanded {
                            self.expanded_dirs.remove(&entry.path);
                        } else {
                            self.expanded_dirs.insert(entry.path.clone());
                        }
                    }
                    
                    // Show directory contents if expanded
                    if is_expanded {
                        let contents = self.get_directory_contents(&entry.path);
                        ui.vertical(|ui| {
                            self.show_file_tree(ui, &contents, level + 1);
                        });
                    }
                } else {
                    let icon = self.get_file_icon(&entry.name);
                    let is_selected = self.selected_file.as_ref() == Some(&entry.path);
                    
                    if ui.selectable_label(is_selected, format!("{} {}", icon, entry.name)).clicked() {
                        self.selected_file = Some(entry.path.clone());
                        
                        // Open file in editor
                        if let Some(sender) = &self.file_open_sender {
                            if let Ok(content) = self.file_system.read_file(&entry.path) {
                                let _ = sender.send((entry.name.clone(), content));
                            }
                        }
                    }
                }
            });
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
            _ => "📄",
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Files, "📁 Files");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Search, "🔍 Search");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::SourceControl, "🌿 Git");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Extensions, "🧩 Extensions");
            });

            ui.separator();

            // Content area
            let workspace = self.file_system.get_workspace().cloned();
            let file_entries = self.current_entries.clone();
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    ExplorerTab::Files => {
                        // Workspace header
                        if let Some(workspace) = workspace {
                            ui.horizontal(|ui| {
                                ui.strong(workspace.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string());
                                if ui.small_button("🔄").on_hover_text("Refresh").clicked() {
                                    self.refresh_entries();
                                }
                            });
                            ui.separator();
                            // File tree
                            self.show_file_tree(ui, &file_entries, 0);
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label("No workspace open");
                                if ui.button("Open Folder").clicked() {
                                    // TODO: Implement folder picker
                                }
                            });
                        }
                    }
                    ExplorerTab::Search => {
                        ui.heading("Search");
                        ui.horizontal(|ui| {
                            ui.label("🔍");
                            ui.text_edit_singleline(&mut self.search_query);
                        });
                        
                        if !self.search_query.is_empty() {
                            ui.separator();
                            // TODO: Implement file search
                            ui.label("Search functionality coming soon...");
                        }
                    }
                    ExplorerTab::SourceControl => {
                        ui.heading("Source Control");
                        ui.label("Git integration coming soon...");
                        // TODO: Implement git integration
                    }
                    ExplorerTab::Extensions => {
                        ui.heading("Extensions");
                        ui.label("Extension management coming soon...");
                        // TODO: Implement extension management
                    }
                }
            });
        });
    }
}