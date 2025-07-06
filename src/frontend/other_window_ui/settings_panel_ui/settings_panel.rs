use eframe::egui;
use crate::backend::settings_manager::{SettingsManager, Theme, AIProvider, CursorStyle};

#[derive(Default)]
pub struct SettingsPanel {
    selected_category: SettingsCategory,
    settings_manager: Option<SettingsManager>,
    temp_api_key: String,
    show_api_key: bool,
}

#[derive(Default, PartialEq)]
enum SettingsCategory {
    #[default]
    General,
    Editor,
    UI,
    AI,
    Terminal,
    Git,
    KeyBindings,
}

impl SettingsPanel {
    pub fn new() -> Self {
        let settings_manager = SettingsManager::new().ok();
        let temp_api_key = settings_manager
            .as_ref()
            .map(|sm| sm.get_settings().ai.api_key.clone())
            .unwrap_or_default();
            
        Self {
            selected_category: SettingsCategory::default(),
            settings_manager,
            temp_api_key,
            show_api_key: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.separator();

        ui.horizontal(|ui| {
            // Settings categories sidebar
            ui.vertical(|ui| {
                ui.set_width(120.0);
                ui.selectable_value(&mut self.selected_category, SettingsCategory::General, "General");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::Editor, "Editor");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::UI, "UI & Theme");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::AI, "AI Assistant");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::Terminal, "Terminal");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::Git, "Git");
                ui.selectable_value(&mut self.selected_category, SettingsCategory::KeyBindings, "Key Bindings");
            });

            ui.separator();

            // Settings content
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(settings_manager) = &mut self.settings_manager {
                    match self.selected_category {
                        SettingsCategory::General => self.show_general_settings(ui, settings_manager),
                        SettingsCategory::Editor => self.show_editor_settings(ui, settings_manager),
                        SettingsCategory::UI => self.show_ui_settings(ui, settings_manager),
                        SettingsCategory::AI => self.show_ai_settings(ui, settings_manager),
                        SettingsCategory::Terminal => self.show_terminal_settings(ui, settings_manager),
                        SettingsCategory::Git => self.show_git_settings(ui, settings_manager),
                        SettingsCategory::KeyBindings => self.show_keybindings_settings(ui),
                    }
                } else {
                    ui.label("Failed to load settings manager");
                }
            });
        });

        ui.separator();

        // Bottom buttons
        ui.horizontal(|ui| {
            if ui.button("Reset to Defaults").clicked() {
                if let Some(settings_manager) = &mut self.settings_manager {
                    let _ = settings_manager.reset_to_defaults();
                }
            }

            if ui.button("Export Settings").clicked() {
                if let Some(settings_manager) = &self.settings_manager {
                    let _ = settings_manager.export_settings("jadio_settings_export.json");
                }
            }

            if ui.button("Import Settings").clicked() {
                // TODO: Implement file picker for importing settings
            }
        });
    }

    fn show_general_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("General Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("Startup");
            ui.checkbox(&mut settings.editor.auto_save, "Auto-save files");
            
            ui.horizontal(|ui| {
                ui.label("Auto-save delay (seconds):");
                ui.add(egui::Slider::new(&mut settings.editor.auto_save_delay, 1..=60));
            });
        });

        ui.group(|ui| {
            ui.label("File Handling");
            ui.checkbox(&mut settings.editor.format_on_save, "Format files on save");
            ui.checkbox(&mut settings.editor.trim_whitespace_on_save, "Trim whitespace on save");
        });
    }

    fn show_editor_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("Editor Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("Font");
            ui.horizontal(|ui| {
                ui.label("Font family:");
                ui.text_edit_singleline(&mut settings.editor.font_family);
            });
            
            ui.horizontal(|ui| {
                ui.label("Font size:");
                ui.add(egui::Slider::new(&mut settings.editor.font_size, 8.0..=24.0).step_by(1.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Line height:");
                ui.add(egui::Slider::new(&mut settings.editor.line_height, 1.0..=2.0).step_by(0.1));
            });
        });

        ui.group(|ui| {
            ui.label("Indentation");
            ui.horizontal(|ui| {
                ui.label("Tab size:");
                ui.add(egui::Slider::new(&mut settings.editor.tab_size, 2..=8));
            });
            ui.checkbox(&mut settings.editor.use_spaces, "Use spaces instead of tabs");
        });

        ui.group(|ui| {
            ui.label("Display");
            ui.checkbox(&mut settings.editor.show_line_numbers, "Show line numbers");
            ui.checkbox(&mut settings.editor.highlight_current_line, "Highlight current line");
            ui.checkbox(&mut settings.editor.word_wrap, "Word wrap");
        });
    }

    fn show_ui_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("UI & Theme Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("Theme");
            ui.horizontal(|ui| {
                ui.radio_value(&mut settings.ui.theme, Theme::Dark, "Dark");
                ui.radio_value(&mut settings.ui.theme, Theme::Light, "Light");
                ui.radio_value(&mut settings.ui.theme, Theme::HighContrast, "High Contrast");
            });
        });

        ui.group(|ui| {
            ui.label("Panel Visibility");
            ui.checkbox(&mut settings.ui.show_explorer, "Show Explorer");
            ui.checkbox(&mut settings.ui.show_terminal, "Show Terminal");
            ui.checkbox(&mut settings.ui.show_code_agent, "Show Code Agent");
            ui.checkbox(&mut settings.ui.show_status_bar, "Show Status Bar");
        });

        ui.group(|ui| {
            ui.label("Panel Sizes");
            ui.horizontal(|ui| {
                ui.label("Explorer width:");
                ui.add(egui::Slider::new(&mut settings.ui.explorer_width, 200.0..=400.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Code Agent width:");
                ui.add(egui::Slider::new(&mut settings.ui.code_agent_width, 250.0..=500.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Terminal height:");
                ui.add(egui::Slider::new(&mut settings.ui.terminal_height, 150.0..=400.0));
            });
        });
    }

    fn show_ai_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("AI Assistant Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("Provider Configuration");
            
            ui.horizontal(|ui| {
                ui.label("Provider:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", settings.ai.provider))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.ai.provider, AIProvider::Anthropic, "Anthropic");
                        ui.selectable_value(&mut settings.ai.provider, AIProvider::OpenAI, "OpenAI");
                        ui.selectable_value(&mut settings.ai.provider, AIProvider::Local, "Local");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Model:");
                ui.text_edit_singleline(&mut settings.ai.model);
            });

            ui.horizontal(|ui| {
                ui.label("API Key:");
                if self.show_api_key {
                    ui.text_edit_singleline(&mut self.temp_api_key);
                } else {
                    ui.add(egui::TextEdit::singleline(&mut self.temp_api_key).password(true));
                }
                
                if ui.button(if self.show_api_key { "👁" } else { "👁‍🗨" }).clicked() {
                    self.show_api_key = !self.show_api_key;
                }
            });

            if ui.button("Save API Key").clicked() {
                settings.ai.api_key = self.temp_api_key.clone();
                let _ = settings_manager.save_settings();
            }
        });

        ui.group(|ui| {
            ui.label("Model Parameters");
            
            ui.horizontal(|ui| {
                ui.label("Temperature:");
                ui.add(egui::Slider::new(&mut settings.ai.temperature, 0.0..=2.0).step_by(0.1));
            });
            
            ui.horizontal(|ui| {
                ui.label("Max tokens:");
                ui.add(egui::Slider::new(&mut settings.ai.max_tokens, 100..=8192).step_by(100.0));
            });
        });

        ui.group(|ui| {
            ui.label("Features");
            ui.checkbox(&mut settings.ai.enable_auto_complete, "Enable auto-completion");
            ui.checkbox(&mut settings.ai.enable_code_suggestions, "Enable code suggestions");
            ui.checkbox(&mut settings.ai.enable_code_review, "Enable automatic code review");
        });

        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Test Connection").clicked() {
                // TODO: Implement API connection test
            }
        });
    }

    fn show_terminal_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("Terminal Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("Shell Configuration");
            ui.horizontal(|ui| {
                ui.label("Default shell:");
                ui.text_edit_singleline(&mut settings.terminal.shell);
            });
        });

        ui.group(|ui| {
            ui.label("Appearance");
            ui.horizontal(|ui| {
                ui.label("Font family:");
                ui.text_edit_singleline(&mut settings.terminal.font_family);
            });
            
            ui.horizontal(|ui| {
                ui.label("Font size:");
                ui.add(egui::Slider::new(&mut settings.terminal.font_size, 8.0..=20.0).step_by(1.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Cursor style:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", settings.terminal.cursor_style))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.terminal.cursor_style, CursorStyle::Block, "Block");
                        ui.selectable_value(&mut settings.terminal.cursor_style, CursorStyle::Line, "Line");
                        ui.selectable_value(&mut settings.terminal.cursor_style, CursorStyle::Underline, "Underline");
                    });
            });
        });

        ui.group(|ui| {
            ui.label("Behavior");
            ui.horizontal(|ui| {
                ui.label("Scroll back limit:");
                ui.add(egui::Slider::new(&mut settings.terminal.scroll_back_limit, 1000..=50000).step_by(1000.0));
            });
        });
    }

    fn show_git_settings(&mut self, ui: &mut egui::Ui, settings_manager: &mut SettingsManager) {
        ui.heading("Git Settings");
        
        let settings = settings_manager.get_settings_mut();
        
        ui.group(|ui| {
            ui.label("User Configuration");
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut settings.git.user_name);
            });
            
            ui.horizontal(|ui| {
                ui.label("Email:");
                ui.text_edit_singleline(&mut settings.git.user_email);
            });
        });

        ui.group(|ui| {
            ui.label("Automatic Actions");
            ui.checkbox(&mut settings.git.auto_fetch, "Auto-fetch from remote");
            ui.checkbox(&mut settings.git.auto_push, "Auto-push on commit");
            ui.checkbox(&mut settings.git.show_diff_in_editor, "Show diff in editor gutter");
        });
    }

    fn show_keybindings_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Key Bindings");
        
        ui.label("Customize keyboard shortcuts:");
        
        ui.group(|ui| {
            ui.label("File Operations");
            self.show_keybinding(ui, "New File", "Ctrl+N");
            self.show_keybinding(ui, "Open File", "Ctrl+O");
            self.show_keybinding(ui, "Save File", "Ctrl+S");
            self.show_keybinding(ui, "Save All", "Ctrl+Shift+S");
        });

        ui.group(|ui| {
            ui.label("Edit Operations");
            self.show_keybinding(ui, "Undo", "Ctrl+Z");
            self.show_keybinding(ui, "Redo", "Ctrl+Y");
            self.show_keybinding(ui, "Cut", "Ctrl+X");
            self.show_keybinding(ui, "Copy", "Ctrl+C");
            self.show_keybinding(ui, "Paste", "Ctrl+V");
        });

        ui.group(|ui| {
            ui.label("View Operations");
            self.show_keybinding(ui, "Toggle Explorer", "Ctrl+Shift+E");
            self.show_keybinding(ui, "Toggle Terminal", "Ctrl+`");
            self.show_keybinding(ui, "Toggle Code Agent", "Ctrl+Shift+A");
            self.show_keybinding(ui, "Command Palette", "Ctrl+Shift+P");
        });
    }

    fn show_keybinding(&self, ui: &mut egui::Ui, action: &str, binding: &str) {
        ui.horizontal(|ui| {
            ui.label(action);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Edit").clicked() {
                    // TODO: Implement keybinding editor
                }
                ui.monospace(binding);
            });
        });
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}