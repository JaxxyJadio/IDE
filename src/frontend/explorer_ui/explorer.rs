use eframe::egui;

#[derive(Default)]
pub struct Explorer {
    selected_tab: ExplorerTab,
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
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Files, "📁");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Search, "🔍");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::SourceControl, "🌿");
                ui.selectable_value(&mut self.selected_tab, ExplorerTab::Extensions, "🧩");
            });

            ui.separator();

            // Content area
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    ExplorerTab::Files => {
                        ui.heading("File Explorer");
                        ui.label("📂 src/");
                        ui.indent("src", |ui| {
                            ui.label("📄 main.rs");
                            ui.label("📂 frontend/");
                            ui.label("📂 backend/");
                        });
                        ui.label("📄 Cargo.toml");
                        ui.label("📄 README.md");
                    }
                    ExplorerTab::Search => {
                        ui.heading("Search");
                        ui.text_edit_singleline(&mut String::new());
                        ui.label("No search results");
                    }
                    ExplorerTab::SourceControl => {
                        ui.heading("Source Control");
                        ui.label("Git status here");
                    }
                    ExplorerTab::Extensions => {
                        ui.heading("Extensions");
                        ui.label("Extension list here");
                    }
                }
            });
        });
    }
}