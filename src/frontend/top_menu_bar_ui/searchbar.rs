use eframe::egui;

#[derive(Default)]
pub struct SearchBar {
    search_text: String,
}

impl SearchBar {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍");
            let response = ui.text_edit_singleline(&mut self.search_text);
            
            if response.changed() {
                // TODO: Implement search functionality
            }
            
            if ui.button("⚙").clicked() {
                // TODO: Open search settings
            }
        });
    }
}