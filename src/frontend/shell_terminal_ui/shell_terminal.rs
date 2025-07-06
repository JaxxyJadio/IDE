use eframe::egui;

#[derive(Default)]
pub struct Terminal {
    selected_tab: TerminalTab,
    terminal_output: String,
    terminal_input: String,
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
        Self {
            terminal_output: "Welcome to JadioAI IDE Terminal\n$ ".to_string(),
            ..Default::default()
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Problems, "Problems");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Output, "Output");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Debug, "Debug");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Terminal, "Terminal");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::Ports, "Ports");
                ui.selectable_value(&mut self.selected_tab, TerminalTab::LlmHelp, "LLM Help");
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
                    // Terminal content
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.terminal_output)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(f32::INFINITY)
                                    .interactive(false)
                            );
                        });
                    
                    // Terminal input
                    ui.horizontal(|ui| {
                        ui.label("$ ");
                        let response = ui.text_edit_singleline(&mut self.terminal_input);
                        
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if !self.terminal_input.trim().is_empty() {
                                self.terminal_output.push_str(&format!("{}\n", self.terminal_input));
                                self.terminal_output.push_str("Command executed (placeholder)\n$ ");
                                self.terminal_input.clear();
                            }
                        }
                    });
                }
                TerminalTab::Ports => {
                    ui.label("Server ports will be listed here");
                }
                TerminalTab::LlmHelp => {
                    ui.label("LLM assistance and documentation");
                }
            }
        });
    }
}