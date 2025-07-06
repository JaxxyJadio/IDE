use eframe::egui;
use std::collections::HashMap;

#[derive(Default)]
pub struct Editor {
    files: HashMap<String, FileContent>,
    open_files: Vec<String>,
    active_file: Option<String>,
    unsaved_changes: std::collections::HashSet<String>,
}

#[derive(Clone)]
struct FileContent {
    content: String,
    language: String,
    cursor_position: usize,
    scroll_offset: egui::Vec2,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = Self::default();
        
        // Create a default file
        let default_content = FileContent {
            content: "// Welcome to JadioAI IDE\n// Start coding here!\n\nfn main() {\n    println!(\"Hello, World!\");\n}".to_string(),
            language: "rust".to_string(),
            cursor_position: 0,
            scroll_offset: egui::Vec2::ZERO,
        };
        
        editor.files.insert("main.rs".to_string(), default_content);
        editor.open_files.push("main.rs".to_string());
        editor.active_file = Some("main.rs".to_string());
        
        editor
    }

    pub fn open_file(&mut self, filename: String, content: String) {
        let file_content = FileContent {
            content,
            language: Self::detect_language(&filename),
            cursor_position: 0,
            scroll_offset: egui::Vec2::ZERO,
        };
        
        self.files.insert(filename.clone(), file_content);
        
        if !self.open_files.contains(&filename) {
            self.open_files.push(filename.clone());
        }
        
        self.active_file = Some(filename);
    }

    fn detect_language(filename: &str) -> String {
        match filename.split('.').last().unwrap_or("") {
            "rs" => "rust".to_string(),
            "py" => "python".to_string(),
            "js" => "javascript".to_string(),
            "ts" => "typescript".to_string(),
            "html" => "html".to_string(),
            "css" => "css".to_string(),
            "json" => "json".to_string(),
            "yaml" | "yml" => "yaml".to_string(),
            "toml" => "toml".to_string(),
            "md" => "markdown".to_string(),
            _ => "text".to_string(),
        }
    }

    pub fn close_file(&mut self, filename: &str) {
        if let Some(pos) = self.open_files.iter().position(|f| f == filename) {
            self.open_files.remove(pos);
            
            // Switch to another file if this was the active one
            if self.active_file.as_ref() == Some(&filename.to_string()) {
                self.active_file = self.open_files.first().cloned();
            }
        }
    }

    pub fn save_file(&mut self, filename: &str) {
        // TODO: Implement actual file saving
        self.unsaved_changes.remove(filename);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar for open files
            if !self.open_files.is_empty() {
                ui.horizontal(|ui| {
                    for filename in self.open_files.clone() {
                        let is_active = self.active_file.as_ref() == Some(&filename);
                        let has_changes = self.unsaved_changes.contains(&filename);
                        
                        let tab_text = if has_changes {
                            format!("● {}", filename)
                        } else {
                            filename.clone()
                        };
                        
                        if ui.selectable_label(is_active, tab_text).clicked() {
                            self.active_file = Some(filename.clone());
                        }
                        
                        // Close button for tab
                        ui.small_button("×").clicked().then(|| {
                            self.close_file(&filename);
                        });
                    }
                    
                    // Add new tab button
                    if ui.button("+").clicked() {
                        let new_filename = format!("untitled_{}.txt", self.open_files.len() + 1);
                        self.open_file(new_filename, String::new());
                    }
                });

                ui.separator();
            }

            // Main editor area
            if let Some(active_filename) = &self.active_file.clone() {
                if let Some(file_content) = self.files.get_mut(active_filename) {
                    // Language indicator
                    ui.horizontal(|ui| {
                        ui.label(format!("Language: {}", file_content.language));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Lines: {}", file_content.content.lines().count()));
                            ui.label(format!("Chars: {}", file_content.content.len()));
                        });
                    });
                    
                    ui.separator();

                    // Editor with line numbers
                    egui::ScrollArea::both()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.horizontal_top(|ui| {
                                // Line numbers
                                let line_count = file_content.content.lines().count();
                                ui.vertical(|ui| {
                                    ui.set_width(30.0);
                                    for i in 1..=line_count.max(1) {
                                        ui.label(format!("{:3}", i));
                                    }
                                });

                                ui.separator();

                                // Main text editor
                                let response = ui.add(
                                    egui::TextEdit::multiline(&mut file_content.content)
                                        .font(egui::TextStyle::Monospace)
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(30)
                                );

                                if response.changed() {
                                    self.unsaved_changes.insert(active_filename.clone());
                                }
                            });
                        });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("No file selected");
                    });
                }
            } else {
                // Welcome screen when no files are open
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Welcome to JadioAI IDE");
                        ui.add_space(20.0);
                        
                        if ui.button("📄 New File").clicked() {
                            self.open_file("untitled.rs".to_string(), String::new());
                        }
                        
                        if ui.button("📁 Open File").clicked() {
                            // TODO: Implement file dialog
                        }
                        
                        if ui.button("📂 Open Folder").clicked() {
                            // TODO: Implement folder dialog
                        }
                    });
                });
            }
        });
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}