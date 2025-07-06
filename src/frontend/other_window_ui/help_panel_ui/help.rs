use eframe::egui;

#[derive(Default)]
pub struct HelpPanel {
    selected_category: HelpCategory,
}

#[derive(Default, PartialEq)]
enum HelpCategory {
    #[default]
    GettingStarted,
    KeyboardShortcuts,
    CodeAgent,
    Features,
    Troubleshooting,
}

impl HelpPanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("❓ Help & Documentation");
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(150.0);
                ui.selectable_value(&mut self.selected_category, HelpCategory::GettingStarted, "Getting Started");
                ui.selectable_value(&mut self.selected_category, HelpCategory::KeyboardShortcuts, "Shortcuts");
                ui.selectable_value(&mut self.selected_category, HelpCategory::CodeAgent, "Code Agent");
                ui.selectable_value(&mut self.selected_category, HelpCategory::Features, "Features");
                ui.selectable_value(&mut self.selected_category, HelpCategory::Troubleshooting, "Troubleshooting");
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_category {
                    HelpCategory::GettingStarted => {
                        ui.heading("Welcome to JadioAI IDE!");
                        ui.label("This is a modern IDE built with Rust and egui.");
                        ui.separator();
                        
                        ui.label("Quick Start:");
                        ui.label("• Use the File menu to open or create projects");
                        ui.label("• The Explorer shows your project files");
                        ui.label("• The Code Agent provides AI assistance");
                        ui.label("• The Terminal runs commands and shows output");
                    }
                    HelpCategory::KeyboardShortcuts => {
                        ui.heading("Keyboard Shortcuts");
                        ui.separator();
                        
                        ui.monospace("Ctrl+N     - New File");
                        ui.monospace("Ctrl+O     - Open File");
                        ui.monospace("Ctrl+S     - Save File");
                        ui.monospace("Ctrl+Shift+P - Command Palette");
                        ui.monospace("Ctrl+`     - Toggle Terminal");
                        ui.monospace("Ctrl+Shift+E - Toggle Explorer");
                        ui.monospace("F5         - Run/Debug");
                    }
                    HelpCategory::CodeAgent => {
                        ui.heading("AI Code Agent");
                        ui.separator();
                        
                        ui.label("The Code Agent is your AI programming assistant.");
                        ui.label("Features:");
                        ui.label("• Code review and suggestions");
                        ui.label("• Bug detection and fixes");
                        ui.label("• Code explanation and documentation");
                        ui.label("• Refactoring assistance");
                        ui.label("• Natural language to code conversion");
                    }
                    HelpCategory::Features => {
                        ui.heading("IDE Features");
                        ui.separator();
                        
                        ui.label("🎨 Syntax highlighting for multiple languages");
                        ui.label("🔍 Global search and replace");
                        ui.label("🌿 Git integration");
                        ui.label("🔌 Plugin system");
                        ui.label("🖥️ Integrated terminal");
                        ui.label("🤖 AI-powered coding assistance");
                        ui.label("⚡ Script runner");
                        ui.label("🌐 Development server management");
                    }
                    HelpCategory::Troubleshooting => {
                        ui.heading("Common Issues");
                        ui.separator();
                        
                        ui.label("Q: The Code Agent isn't responding");
                        ui.label("A: Check your API key in AI Settings");
                        ui.separator();
                        
                        ui.label("Q: Syntax highlighting not working");
                        ui.label("A: Ensure the file extension is recognized");
                        ui.separator();
                        
                        ui.label("Q: Terminal commands not executing");
                        ui.label("A: Check your system PATH configuration");
                    }
                }
            });
        });
    }
}