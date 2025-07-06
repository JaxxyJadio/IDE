use eframe::egui;

#[derive(Default)]
pub struct CodeAgent {
    chat_input: String,
    messages: Vec<String>,
}

impl CodeAgent {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("🤖 Code Agent");
            
            // Chat history
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in &self.messages {
                        ui.label(message);
                        ui.separator();
                    }
                    
                    if self.messages.is_empty() {
                        ui.label("Welcome! I'm your AI coding assistant.");
                        ui.label("Ask me anything about your code!");
                    }
                });

            ui.separator();

            // Input area
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut self.chat_input);
                
                if ui.button("Send").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.chat_input.trim().is_empty() {
                        self.messages.push(format!("You: {}", self.chat_input));
                        self.messages.push(format!("AI: I received your message: '{}'", self.chat_input));
                        self.chat_input.clear();
                    }
                }
            });

            ui.separator();

            // Quick action buttons
            ui.horizontal_wrapped(|ui| {
                if ui.button("📝 Review Code").clicked() {
                    self.messages.push("AI: I'll review your code for improvements.".to_string());
                }
                
                if ui.button("🐛 Find Bugs").clicked() {
                    self.messages.push("AI: Scanning for potential bugs...".to_string());
                }
                
                if ui.button("📚 Explain Code").clicked() {
                    self.messages.push("AI: I'll explain the current code for you.".to_string());
                }
                
                if ui.button("⚡ Optimize").clicked() {
                    self.messages.push("AI: Looking for optimization opportunities...".to_string());
                }
            });
        });
    }
}