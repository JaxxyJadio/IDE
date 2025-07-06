use eframe::egui;

#[derive(Default)]
pub struct PluginPanel {
    search_text: String,
    plugins: Vec<Plugin>,
}

#[derive(Clone)]
struct Plugin {
    name: String,
    description: String,
    version: String,
    enabled: bool,
    installed: bool,
}

impl PluginPanel {
    pub fn new() -> Self {
        Self {
            search_text: String::new(),
            plugins: vec![
                Plugin {
                    name: "Rust Language Server".to_string(),
                    description: "Enhanced Rust support with IntelliSense".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    installed: true,
                },
                Plugin {
                    name: "Git Integration Plus".to_string(),
                    description: "Advanced Git features and visual diff".to_string(),
                    version: "2.1.3".to_string(),
                    enabled: true,
                    installed: true,
                },
                Plugin {
                    name: "Theme Pack".to_string(),
                    description: "Additional color themes for the IDE".to_string(),
                    version: "1.5.2".to_string(),
                    enabled: false,
                    installed: true,
                },
                Plugin {
                    name: "Docker Helper".to_string(),
                    description: "Docker container management and deployment".to_string(),
                    version: "3.0.1".to_string(),
                    enabled: false,
                    installed: false,
                },
                Plugin {
                    name: "Database Explorer".to_string(),
                    description: "Connect and query databases directly".to_string(),
                    version: "1.2.7".to_string(),
                    enabled: false,
                    installed: false,
                },
            ],
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔌 Plugin Manager");
        ui.separator();

        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_text);
            if ui.button("Refresh").clicked() {
                // TODO: Refresh plugin list
            }
        });

        ui.separator();

        // Plugin list
        egui::ScrollArea::vertical().show(ui, |ui| {
            for plugin in &mut self.plugins {
                if !self.search_text.is_empty() && 
                   !plugin.name.to_lowercase().contains(&self.search_text.to_lowercase()) {
                    continue;
                }

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.strong(&plugin.name);
                            ui.label(&plugin.description);
                            ui.horizontal(|ui| {
                                ui.label(format!("v{}", plugin.version));
                                if plugin.installed {
                                    ui.colored_label(egui::Color32::GREEN, "✓ Installed");
                                } else {
                                    ui.colored_label(egui::Color32::GRAY, "Not installed");
                                }
                            });
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if plugin.installed {
                                ui.checkbox(&mut plugin.enabled, "Enabled");
                                
                                if ui.button("Uninstall").clicked() {
                                    plugin.installed = false;
                                    plugin.enabled = false;
                                }
                            } else {
                                if ui.button("Install").clicked() {
                                    plugin.installed = true;
                                }
                            }
                        });
                    });
                });

                ui.add_space(5.0);
            }
        });

        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Browse Marketplace").clicked() {
                // TODO: Open plugin marketplace
            }
            
            if ui.button("Install from File").clicked() {
                // TODO: Install plugin from file
            }
            
            if ui.button("Create Plugin").clicked() {
                // TODO: Open plugin development guide
            }
        });
    }
}

impl Default for PluginPanel {
    fn default() -> Self {
        Self::new()
    }
}