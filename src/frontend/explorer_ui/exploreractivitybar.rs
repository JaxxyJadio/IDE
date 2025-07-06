use eframe::egui;

#[derive(Default)]
pub struct ExplorerActivityBar {
    // State for activity bar
}

impl ExplorerActivityBar {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            
            if ui.button("📁").on_hover_text("Explorer").clicked() {
                // TODO: Toggle explorer
            }
            
            if ui.button("🔍").on_hover_text("Search").clicked() {
                // TODO: Toggle search
            }
            
            if ui.button("🌿").on_hover_text("Source Control").clicked() {
                // TODO: Toggle source control
            }
            
            if ui.button("🐛").on_hover_text("Debug").clicked() {
                // TODO: Toggle debug
            }
            
            if ui.button("🧩").on_hover_text("Extensions").clicked() {
                // TODO: Toggle extensions
            }
            
            // Spacer to push settings to bottom
            ui.allocate_space(egui::Vec2::new(0.0, ui.available_height() - 40.0));
            
            if ui.button("⚙").on_hover_text("Settings").clicked() {
                // TODO: Open settings
            }
        });
    }
}