use eframe::egui;

#[derive(Default)]
pub struct Editor {
    code_text: String,
    open_files: Vec<String>,
    active_file: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            code_text: "// Welcome to JadioAI IDE\n// Start coding here!\n\nfn main() {\n    println!(\"Hello, World!\");\n}".to_string(),
            open_files: vec!["main.rs".to_string()],
            active_file: 0,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar for open files
            ui.horizontal(|ui| {
                for (i, file) in self.open_files.iter().enumerate() {
                    let is_active = i == self.active_file;
                    if ui.selectable_label(is_active, file).clicked() {
                        self.active_file = i;
                    }
                    
                    // Close button for tab
                    if ui.small_button("×").clicked() {
                        // TODO: Handle closing files
                    }
                }
                
                // Add new tab button
                if ui.button("+").clicked() {
                    // TODO: Handle opening new files
                }
            });

            ui.separator();

            // Main editor area
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code_text)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(50)
                    );
                });
        });
    }
}