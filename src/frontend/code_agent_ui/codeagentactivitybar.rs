use eframe::egui;

#[derive(Default)]
pub struct CodeAgentActivityBar {
    // State for code agent activity bar
}

impl CodeAgentActivityBar {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            
            if ui.button("🤖").on_hover_text("AI Assistant").clicked() {
                // TODO: Toggle AI assistant
            }
            
            if ui.button("🧠").on_hover_text("Code Analysis").clicked() {
                // TODO: Toggle code analysis
            }
            
            if ui.button("💡").on_hover_text("Suggestions").clicked() {
                // TODO: Toggle suggestions
            }
            
            if ui.button("📊").on_hover_text("Code Metrics").clicked() {
                // TODO: Toggle code metrics
            }
            
            if ui.button("🔍").on_hover_text("Code Search").clicked() {
                // TODO: Toggle code search
            }
        });
    }
}