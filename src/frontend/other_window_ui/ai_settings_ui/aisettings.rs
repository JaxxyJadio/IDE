use eframe::egui;

#[derive(Default)]
pub struct AiSettings {
    model_name: String,
    api_key: String,
    temperature: f32,
    max_tokens: i32,
    auto_complete: bool,
    code_suggestions: bool,
}

impl AiSettings {
    pub fn new() -> Self {
        Self {
            model_name: "claude-3-sonnet".to_string(),
            api_key: String::new(),
            temperature: 0.7,
            max_tokens: 4096,
            auto_complete: true,
            code_suggestions: true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("🤖 AI Settings");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.group(|ui| {
                ui.label("Model Configuration");
                
                ui.horizontal(|ui| {
                    ui.label("Model:");
                    ui.text_edit_singleline(&mut self.model_name);
                });

                ui.horizontal(|ui| {
                    ui.label("API Key:");
                    ui.add(egui::TextEdit::singleline(&mut self.api_key).password(true));
                });

                ui.horizontal(|ui| {
                    ui.label("Temperature:");
                    ui.add(egui::Slider::new(&mut self.temperature, 0.0..=2.0).step_by(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Max Tokens:");
                    ui.add(egui::Slider::new(&mut self.max_tokens, 100..=8192).step_by(100.0));
                });
            });

            ui.group(|ui| {
                ui.label("Features");
                
                ui.checkbox(&mut self.auto_complete, "Auto-complete suggestions");
                ui.checkbox(&mut self.code_suggestions, "Real-time code suggestions");
            });

            ui.horizontal(|ui| {
                if ui.button("Save Settings").clicked() {
                    // TODO: Save settings to config file
                }
                
                if ui.button("Test Connection").clicked() {
                    // TODO: Test API connection
                }
                
                if ui.button("Reset to Defaults").clicked() {
                    *self = Self::new();
                }
            });
        });
    }
}