use eframe::egui;

#[derive(Default)]
pub struct StatusBar {
    // Status bar state
}

impl StatusBar {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left side - Git branch and status
            ui.label("🌿 main");
            ui.separator();
            ui.label("✓ Clean");
            
            // Spacer to push right side content to the right
            ui.allocate_space(egui::Vec2::new(ui.available_width() - 200.0, 0.0));
            
            // Right side - Line/column, language, encoding
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("UTF-8");
                ui.separator();
                ui.label("Rust");
                ui.separator();
                ui.label("Ln 1, Col 1");
                ui.separator();
                ui.label("🔔");  // Notifications
            });
        });
    }
}