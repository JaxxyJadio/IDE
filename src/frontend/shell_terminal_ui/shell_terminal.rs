use eframe::egui;
use crate::backend::{TerminalHandler, TerminalMessage, MessageType};

#[derive(Default)]
pub struct Terminal {
    selected_tab: TerminalTab,
    terminal_handler: TerminalHandler,
    terminal_input: String,
    auto_scroll: bool,
    show_timestamps: bool,
}

#[derive(Default, PartialEq)]
enum TerminalTab {
    Problems,
    Output,
    Debug,
    #[default]
    Terminal,
    Ports,
    LlmHelp,
}

impl Terminal {
    pub fn new() -> Self {
        let mut terminal = Self {
            auto_scroll: true,
            show_timestamps: false,
            ..Default::default()
        };
        
        // Start default shell
        if let Err(e) = terminal.terminal_handler.start_shell(None) {
            eprintln!("Failed to start shell: {}", e);
        }
        
        terminal
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar with controls
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Problems, "Problems");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Output, "Output");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Debug, "Debug");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Terminal, "Terminal");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Ports, "Ports");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::LlmHelp, "LLM Help");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.selected_tab == TerminalTab::Terminal {
                        if ui.button("🗑").on_hover_text("Clear").clicked() {
                            self.terminal_handler.clear_history();
                        }
                        
                        ui.checkbox(&mut self.auto_scroll, "Auto-scroll");
                        ui.checkbox(&mut self.show_timestamps, "Timestamps");
                        
                        if self.terminal_handler.is_running() {
                            if ui.button("⏹").on_hover_text("Stop Shell").clicked() {
                                self.terminal_handler.stop_shell();
                            }
                        } else {
                            if ui.button("▶").on_hover_text("Start Shell").clicked() {
                                let _ = self.terminal_handler.start_shell(None);
                            }
                        }
                    }
                });
            });

            ui.separator();

            // Content area
            match self.selected_tab {
                TerminalTab::Problems => {
                    ui.label("No problems detected");
                }
                TerminalTab::Output => {
                    ui.label("Build output will appear here");
                }
                TerminalTab::Debug => {
                    ui.label("Debug console");
                }
                TerminalTab::Terminal => {
                    // Update terminal handler to receive new messages
                    self.terminal_handler.update();
                    
                    // Terminal output area
                    let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
                    let num_rows = 20;
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(self.auto_scroll)
                        .max_height(text_height * num_rows as f32)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                for message in self.terminal_handler.get_history() {
                                    self.show_terminal_message(ui, message);
                                }
                            });
                        });
                    
                    ui.separator();
                    
                    // Terminal input
                    ui.horizontal(|ui| {
                        // Show current directory
                        let cwd = self.terminal_handler.get_working_directory();
                        ui.monospace(format!("{} $", cwd.display()));
                        
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.terminal_input)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(ui.available_width())
                        );
                        
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if !self.terminal_input.trim().is_empty() {
                                let command = self.terminal_input.clone();
                                self.terminal_input.clear();
                                
                                if let Err(e) = self.terminal_handler.execute_command(command) {
                                    eprintln!("Failed to execute command: {}", e);
                                }
                            }
                            response.request_focus();
                        }
                    });
                }
                TerminalTab::Ports => {
                    ui.label("Server ports monitoring coming soon...");
                }
                TerminalTab::LlmHelp => {
                    ui.label("LLM assistance and documentation coming soon...");
                }
            }
        });
    }
    
    fn show_terminal_message(&self, ui: &mut egui::Ui, message: &TerminalMessage) {
        ui.horizontal(|ui| {
            if self.show_timestamps {
                ui.weak(message.timestamp.format("[%H:%M:%S]").to_string());
            }
            
            let (color, prefix) = match message.message_type {
                MessageType::Output => (egui::Color32::from_gray(200), ""),
                MessageType::Error => (egui::Color32::from_rgb(255, 100, 100), "ERROR: "),
                MessageType::Input => (egui::Color32::from_rgb(100, 200, 255), ""),
                MessageType::System => (egui::Color32::from_rgb(255, 255, 100), "SYSTEM: "),
            };
            
            ui.colored_label(color, format!("{}{}", prefix, message.content));
        });
    }
}