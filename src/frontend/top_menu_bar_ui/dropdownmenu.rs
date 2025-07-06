use eframe::egui;

#[derive(Default)]
pub struct DropdownMenu {
    // State for dropdown menus
}

impl DropdownMenu {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New File").clicked() {
                    // TODO: Implement new file
                }
                if ui.button("Open File").clicked() {
                    // TODO: Implement open file
                }
                if ui.button("Save").clicked() {
                    // TODO: Implement save
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    // TODO: Implement exit
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // TODO: Implement undo
                }
                if ui.button("Redo").clicked() {
                    // TODO: Implement redo
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    // TODO: Implement cut
                }
                if ui.button("Copy").clicked() {
                    // TODO: Implement copy
                }
                if ui.button("Paste").clicked() {
                    // TODO: Implement paste
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("Explorer").clicked() {
                    // TODO: Toggle explorer
                }
                if ui.button("Terminal").clicked() {
                    // TODO: Toggle terminal
                }
                if ui.button("Code Agent").clicked() {
                    // TODO: Toggle code agent
                }
            });

            ui.menu_button("Terminal", |ui| {
                if ui.button("New Terminal").clicked() {
                    // TODO: Create new terminal
                }
                if ui.button("Split Terminal").clicked() {
                    // TODO: Split terminal
                }
            });

            ui.menu_button("LLM", |ui| {
                if ui.button("Chat").clicked() {
                    // TODO: Open LLM chat
                }
                if ui.button("Code Assistant").clicked() {
                    // TODO: Open code assistant
                }
                if ui.button("Settings").clicked() {
                    // TODO: Open LLM settings
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Documentation").clicked() {
                    // TODO: Open documentation
                }
                if ui.button("About").clicked() {
                    // TODO: Show about dialog
                }
            });
        });
    }
}