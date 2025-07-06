use eframe::egui;

// Import modules
mod backend;
mod frontend;

use frontend::*;

// Main application structure
#[derive(Default)]
pub struct IDEApp {
    // Top menu components (all in one top bar)
    dropdown_menu: DropdownMenu,
    search_bar: SearchBar,
    special_buttons_bar: SpecialButtonsBar,
    
    // Left side panels
    explorer_activity_bar: ExplorerActivityBar,
    explorer: Explorer,
    
    // Right side panels
    code_agent_activity_bar: CodeAgentActivityBar,
    code_agent: CodeAgent,
    
    // Center and bottom panels
    editor: Editor,
    terminal: Terminal,
    status_bar: StatusBar,
    
    // UI state
    explorer_open: bool,
    code_agent_open: bool,
    terminal_open: bool,
    status_bar_open: bool,
}

impl IDEApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and styling
        frontend::style::configure_fonts(&cc.egui_ctx);
        frontend::style::configure_style(&cc.egui_ctx);
        
        Self {
            explorer_open: true,
            code_agent_open: true,
            terminal_open: true,
            status_bar_open: true,
            ..Default::default()
        }
    }
}

impl eframe::App for IDEApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar - contains dropdown menu, search bar, and special buttons
        egui::TopBottomPanel::top("top_menu_bar")
            .exact_height(40.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .inner_margin(egui::Margin::same(4.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Dropdown menus on the left (File, Edit, Selection, View, Go, Run, Terminal, LLM, Server, Help)
                            self.dropdown_menu.show(ui);
                            
                            ui.separator();
                            
                            // Search bar in the center (takes most space)
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(ui.available_width() - 200.0, ui.available_height()),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| self.search_bar.show(ui)
                            );
                            
                            // Special buttons on the right (6-8 square buttons)
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                self.special_buttons_bar.show(ui);
                            });
                        });
                    });
            });

        // Bottom status bar (very thin, like VS Code - shows line/col, Git branch, etc.)
        if self.status_bar_open {
            egui::TopBottomPanel::bottom("status_bar")
                .exact_height(22.0)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .inner_margin(egui::Margin::symmetric(8.0, 2.0))
                        .show(ui, |ui| {
                            self.status_bar.show(ui);
                        });
                });
        }

        // Terminal panel (Problems, Output, Debug, Ports, Terminal, LLM Help tabs)
        if self.terminal_open {
            egui::TopBottomPanel::bottom("terminal")
                .resizable(true)
                .default_height(200.0)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.terminal.show(ui);
                        });
                });
        }

        // Left activity bar (narrow vertical bar with icons)
        egui::SidePanel::left("explorer_activity_bar")
            .resizable(false)
            .exact_width(48.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .show(ui, |ui| {
                        self.explorer_activity_bar.show(ui);
                    });
            });

        // Left explorer panel (file tree, search, source control, etc.)
        if self.explorer_open {
            egui::SidePanel::left("explorer")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.explorer.show(ui);
                        });
                });
        }

        // Right activity bar (narrow vertical bar with AI/code agent icons)
        egui::SidePanel::right("code_agent_activity_bar")
            .resizable(false)
            .exact_width(48.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .show(ui, |ui| {
                        self.code_agent_activity_bar.show(ui);
                    });
            });

        // Right code agent panel (AI assistant, code analysis, suggestions, chat)
        if self.code_agent_open {
            egui::SidePanel::right("code_agent")
                .resizable(true)
                .default_width(300.0)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.code_agent.show(ui);
                        });
                });
        }

        // Central editor area (main code editing space with tabs)
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none()
                .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                .show(ui, |ui| {
                    self.editor.show(ui);
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("JadioAI IDE"),
        ..Default::default()
    };

    eframe::run_native(
        "JadioAI IDE",
        options,
        Box::new(|cc| Box::new(IDEApp::new(cc))),
    )
}