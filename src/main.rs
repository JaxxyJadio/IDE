use eframe::egui;

// Import modules
mod backend;
mod frontend;

// Explicitly import required UI types
use frontend::top_menu_bar_ui::dropdownmenu::DropdownMenu;
use frontend::top_menu_bar_ui::searchbar::SearchBar;
use frontend::top_menu_bar_ui::specialbuttonsbar::SpecialButtonsBar;
use frontend::explorer_ui::file_explorer_activity_bar::ExplorerActivityBar;
use frontend::explorer_ui::file_explorer::Explorer;
use frontend::code_agent_ui::codeagentactivitybar::CodeAgentActivityBar;
use frontend::code_agent_ui::codeagent::CodeAgent;
use frontend::code_editor_ui::code_editor::Editor;
use frontend::shell_terminal_ui::shell_terminal::Terminal;
use frontend::status_bar_ui::statusbar::StatusBar;

use backend::{SettingsManager, ProjectManager, FileSystem};

// Main application structure
#[derive(Default)]
pub struct IDEApp {
    // Backend systems
    settings_manager: Option<SettingsManager>,
    project_manager: ProjectManager,
    file_system: FileSystem,
    
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
    
    // Error handling
    last_error: Option<String>,
}

impl IDEApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and styling
        frontend::style::configure_fonts(&cc.egui_ctx);
        frontend::style::configure_style(&cc.egui_ctx);
        
        // Initialize backend systems
        let settings_manager = SettingsManager::new().ok();
        let project_manager = ProjectManager::new(); // removed mut
        let file_system = FileSystem::new();
        
        // Load UI state from settings if available
        let (explorer_open, code_agent_open, terminal_open, status_bar_open) = 
            if let Some(ref sm) = settings_manager {
                let ui_settings = &sm.get_settings().ui;
                (ui_settings.show_explorer, ui_settings.show_code_agent, 
                 ui_settings.show_terminal, ui_settings.show_status_bar)
            } else {
                (true, true, true, true)
            };
        
        // Try to load last project if available
        // TODO: Implement recent project loading
        
        Self {
            settings_manager,
            project_manager,
            file_system,
            explorer_open,
            code_agent_open,
            terminal_open,
            status_bar_open,
            last_error: None,
            ..Default::default()
        }
    }
    
    pub fn handle_file_operation(&mut self, operation: FileOperation) {
        match operation {
            FileOperation::NewFile => {
                // TODO: Implement new file creation
                let new_filename = format!("untitled_{}.txt", chrono::Utc::now().timestamp());
                self.editor.open_file(new_filename, String::new());
            }
            FileOperation::OpenFile(path) => {
                match self.file_system.read_file(&path) {
                    Ok(content) => {
                        let filename = path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        self.editor.open_file(filename, content);
                    }
                    Err(e) => {
                        self.last_error = Some(format!("Failed to open file: {}", e));
                    }
                }
            }
            FileOperation::SaveFile(filename) => {
                if let Some(current_project) = self.project_manager.get_current_project() {
                    let _file_path = current_project.path.join(&filename); // prefix with _
                    // TODO: Get content from editor and save
                    // self.file_system.write_file(_file_path, content);
                }
            }
            FileOperation::OpenProject(path) => {
                match self.project_manager.open_project(&path) {
                    Ok(()) => {
                        let _ = self.file_system.set_workspace(&path);
                        self.explorer.open_workspace(path).ok();
                    }
                    Err(e) => {
                        self.last_error = Some(format!("Failed to open project: {}", e));
                    }
                }
            }
        }
    }
    
    fn show_error_popup(&mut self, ctx: &egui::Context) {
        // Take ownership of last_error to avoid double borrow
        if self.last_error.is_some() {
            let error = self.last_error.take().unwrap();
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&error);
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            // Already cleared
                        } else {
                            // If not dismissed, put it back
                            self.last_error = Some(error.clone());
                        }
                    });
                });
        }
    }
    
    fn update_ui_state_from_settings(&mut self) {
        if let Some(ref settings_manager) = self.settings_manager {
            let ui_settings = &settings_manager.get_settings().ui;
            self.explorer_open = ui_settings.show_explorer;
            self.code_agent_open = ui_settings.show_code_agent;
            self.terminal_open = ui_settings.show_terminal;
            self.status_bar_open = ui_settings.show_status_bar;
        }
    }
    
    fn save_ui_state_to_settings(&mut self) {
        if let Some(ref mut settings_manager) = self.settings_manager {
            let _ = settings_manager.update_ui_settings(|ui_settings| {
                ui_settings.show_explorer = self.explorer_open;
                ui_settings.show_code_agent = self.code_agent_open;
                ui_settings.show_terminal = self.terminal_open;
                ui_settings.show_status_bar = self.status_bar_open;
            });
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileOperation {
    NewFile,
    OpenFile(std::path::PathBuf),
    SaveFile(String),
    OpenProject(std::path::PathBuf),
}

impl eframe::App for IDEApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        ctx.input(|i| {
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::N) {
                    self.handle_file_operation(FileOperation::NewFile);
                }
                if i.key_pressed(egui::Key::O) {
                    // TODO: Implement file picker
                }
                if i.key_pressed(egui::Key::S) {
                    // TODO: Save current file
                }
                if i.modifiers.shift && i.key_pressed(egui::Key::E) {
                    self.explorer_open = !self.explorer_open;
                    self.save_ui_state_to_settings();
                }
                if i.key_pressed(egui::Key::Backtick) {
                    self.terminal_open = !self.terminal_open;
                    self.save_ui_state_to_settings();
                }
                if i.modifiers.shift && i.key_pressed(egui::Key::A) {
                    self.code_agent_open = !self.code_agent_open;
                    self.save_ui_state_to_settings();
                }
            }
        });

        // Top menu bar - contains dropdown menu, search bar, and special buttons
        egui::TopBottomPanel::top("top_menu_bar")
            .exact_height(40.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    .inner_margin(egui::Margin::same(4.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Dropdown menus on the left
                            self.dropdown_menu.show(ui);
                            
                            ui.separator();
                            
                            // Search bar in the center
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(ui.available_width() - 200.0, ui.available_height()),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| self.search_bar.show(ui)
                            );
                            
                            // Special buttons on the right
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                self.special_buttons_bar.show(ui);
                            });
                        });
                    });
            });

        // Bottom status bar
        if self.status_bar_open {
            egui::TopBottomPanel::bottom("status_bar")
                .exact_height(22.0)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .inner_margin(egui::Margin::symmetric(8.0, 2.0))
                        .show(ui, |ui| {
                            self.status_bar.show(ui);
                            
                            // Show current project info
                            if let Some(project) = self.project_manager.get_current_project() {
                                ui.separator();
                                ui.label(format!("📁 {}", project.name));
                                ui.label(format!("{:?}", project.project_type));
                            }
                        });
                });
        }

        // Terminal panel
        if self.terminal_open {
            let terminal_height = self.settings_manager
                .as_ref()
                .map(|sm| sm.get_settings().ui.terminal_height)
                .unwrap_or(200.0);
                
            egui::TopBottomPanel::bottom("terminal")
                .resizable(true)
                .default_height(terminal_height)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.terminal.show(ui);
                        });
                });
        }

        // Left activity bar
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

        // Left explorer panel
        if self.explorer_open {
            let explorer_width = self.settings_manager
                .as_ref()
                .map(|sm| sm.get_settings().ui.explorer_width)
                .unwrap_or(250.0);
                
            egui::SidePanel::left("explorer")
                .resizable(true)
                .default_width(explorer_width)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.explorer.show(ui);
                        });
                });
        }

        // Right activity bar
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

        // Right code agent panel
        if self.code_agent_open {
            let code_agent_width = self.settings_manager
                .as_ref()
                .map(|sm| sm.get_settings().ui.code_agent_width)
                .unwrap_or(300.0);
                
            egui::SidePanel::right("code_agent")
                .resizable(true)
                .default_width(code_agent_width)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                        .show(ui, |ui| {
                            self.code_agent.show(ui);
                        });
                });
        }

        // Central editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none()
                .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                .show(ui, |ui| {
                    self.editor.show(ui);
                });
        });
        
        // Show error popup if there's an error
        self.show_error_popup(ctx);
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save current UI state when the app closes
        self.save_ui_state_to_settings();
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