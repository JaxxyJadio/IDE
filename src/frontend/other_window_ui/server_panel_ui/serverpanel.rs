use eframe::egui;
use std::collections::HashMap;

#[derive(Default)]
pub struct ServerPanel {
    servers: Vec<Server>,
    server_templates: HashMap<String, ServerTemplate>,
    show_new_server_dialog: bool,
    new_server_name: String,
    new_server_port: String,
    new_server_command: String,
    selected_template: Option<String>,
}

#[derive(Clone)]
struct Server {
    name: String,
    port: u16,
    command: String,
    status: ServerStatus,
    pid: Option<u32>,
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    logs: Vec<String>,
    auto_restart: bool,
}

#[derive(Clone, PartialEq)]
enum ServerStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error(String),
}

struct ServerTemplate {
    name: String,
    description: String,
    default_port: u16,
    command: String,
    icon: String,
}

impl ServerPanel {
    pub fn new() -> Self {
        let mut panel = ServerPanel {
            servers: Vec::new(),
            server_templates: HashMap::new(),
            show_new_server_dialog: false,
            new_server_name: String::new(),
            new_server_port: String::new(),
            new_server_command: String::new(),
            selected_template: None,
        };
        panel.init_templates();
        panel.init_servers();
        panel
    }

    fn init_templates(&mut self) {
        let templates = vec![
            ServerTemplate {
                name: "Rust Web Server".to_string(),
                description: "Cargo web server (cargo run)".to_string(),
                default_port: 8080,
                command: "cargo run".to_string(),
                icon: "🦀".to_string(),
            },
            ServerTemplate {
                name: "Python HTTP Server".to_string(),
                description: "Simple Python HTTP server".to_string(),
                default_port: 8000,
                command: "python -m http.server".to_string(),
                icon: "🐍".to_string(),
            },
            ServerTemplate {
                name: "Node.js Server".to_string(),
                description: "Node.js development server".to_string(),
                default_port: 3000,
                command: "npm start".to_string(),
                icon: "📜".to_string(),
            },
            ServerTemplate {
                name: "Live Server".to_string(),
                description: "Static file server with live reload".to_string(),
                default_port: 5500,
                command: "live-server".to_string(),
                icon: "🌐".to_string(),
            },
            ServerTemplate {
                name: "Webpack Dev Server".to_string(),
                description: "Webpack development server".to_string(),
                default_port: 8080,
                command: "npx webpack serve".to_string(),
                icon: "📦".to_string(),
            },
        ];

        for template in templates {
            self.server_templates.insert(template.name.clone(), template);
        }
    }

    fn init_servers(&mut self) {
        // Add some example servers
        self.servers = vec![
            Server {
                name: "Main App".to_string(),
                port: 8080,
                command: "cargo run".to_string(),
                status: ServerStatus::Stopped,
                pid: None,
                start_time: None,
                logs: vec![],
                auto_restart: true,
            },
            Server {
                name: "API Server".to_string(),
                port: 3000,
                command: "npm run dev".to_string(),
                status: ServerStatus::Running,
                pid: Some(12345),
                start_time: Some(chrono::Utc::now() - chrono::Duration::minutes(30)),
                logs: vec![
                    "Server starting...".to_string(),
                    "Listening on port 3000".to_string(),
                    "✅ API server ready".to_string(),
                ],
                auto_restart: false,
            },
            Server {
                name: "Static Files".to_string(),
                port: 8000,
                command: "python -m http.server 8000".to_string(),
                status: ServerStatus::Error("Port already in use".to_string()),
                pid: None,
                start_time: None,
                logs: vec![
                    "Starting static file server...".to_string(),
                    "❌ Error: Port 8000 already in use".to_string(),
                ],
                auto_restart: false,
            },
        ];
    }

    fn get_status_color(&self, status: &ServerStatus) -> egui::Color32 {
        match status {
            ServerStatus::Stopped => egui::Color32::GRAY,
            ServerStatus::Starting => egui::Color32::YELLOW,
            ServerStatus::Running => egui::Color32::GREEN,
            ServerStatus::Stopping => egui::Color32::YELLOW,
            ServerStatus::Error(_) => egui::Color32::RED,
        }
    }

    fn get_status_icon(&self, status: &ServerStatus) -> &'static str {
        match status {
            ServerStatus::Stopped => "⏹️",
            ServerStatus::Starting => "🔄",
            ServerStatus::Running => "✅",
            ServerStatus::Stopping => "🔄",
            ServerStatus::Error(_) => "❌",
        }
    }

    fn start_server(&mut self, index: usize) {
        if let Some(server) = self.servers.get_mut(index) {
            server.status = ServerStatus::Starting;
            server.logs.push(format!("Starting server: {}", server.name));
            
            // TODO: Implement actual server starting
            // For now, simulate starting
            server.status = ServerStatus::Running;
            server.start_time = Some(chrono::Utc::now());
            // Use a simple cast from u64 to u32 for pseudo-random PID
            let pid = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() % (u32::MAX as u128)) as u32;
            server.pid = Some(pid % 10000 + 1000);
            server.logs.push(format!("✅ Server started on port {}", server.port));
        }
    }

    fn stop_server(&mut self, index: usize) {
        if let Some(server) = self.servers.get_mut(index) {
            server.status = ServerStatus::Stopping;
            server.logs.push("Stopping server...".to_string());
            
            // TODO: Implement actual server stopping
            // For now, simulate stopping
            server.status = ServerStatus::Stopped;
            server.start_time = None;
            server.pid = None;
            server.logs.push("🛑 Server stopped".to_string());
        }
    }

    fn restart_server(&mut self, index: usize) {
        self.stop_server(index);
        // Add a small delay simulation
        self.start_server(index);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("🌐 Development Servers");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Manage your development servers");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("➕ New Server").clicked() {
                    self.show_new_server_dialog = true;
                    self.new_server_name.clear();
                    self.new_server_port.clear();
                    self.new_server_command.clear();
                    self.selected_template = None;
                }
                if ui.button("🔄 Refresh").clicked() {
                    // TODO: Refresh server status
                }
            });
        });

        ui.separator();

        // Server list
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Collect actions to perform after UI loop
            let mut actions: Vec<(usize, &'static str)> = Vec::new();
            // Collect server info for UI rendering
            let _server_count = self.servers.len();
            let servers_snapshot: Vec<_> = self.servers.iter().cloned().collect();
            for (i, server) in servers_snapshot.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Status indicator
                        let status_icon = self.get_status_icon(&server.status);
                        let status_color = self.get_status_color(&server.status);
                        ui.colored_label(status_color, status_icon);
                        // Server info
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.strong(&server.name);
                                ui.label(format!(":{}", server.port));
                                if let Some(pid) = server.pid {
                                    ui.small(format!("PID: {}", pid));
                                }
                            });
                            ui.small(&server.command);
                            if let Some(start_time) = server.start_time {
                                let uptime = chrono::Utc::now() - start_time;
                                ui.small(format!("Uptime: {}m {}s", 
                                    uptime.num_minutes(), 
                                    uptime.num_seconds() % 60));
                            }
                            match &server.status {
                                ServerStatus::Error(msg) => {
                                    ui.colored_label(egui::Color32::RED, format!("Error: {}", msg));
                                }
                                _ => {}
                            }
                        });
                        // Control buttons
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            match server.status {
                                ServerStatus::Stopped | ServerStatus::Error(_) => {
                                    if ui.button("▶ Start").clicked() {
                                        actions.push((i, "start"));
                                    }
                                }
                                ServerStatus::Running => {
                                    if ui.button("⏹ Stop").clicked() {
                                        actions.push((i, "stop"));
                                    }
                                    if ui.button("🔄 Restart").clicked() {
                                        actions.push((i, "restart"));
                                    }
                                    if ui.button("🌐").on_hover_text("Open in Browser").clicked() {
                                        // TODO: Open URL in browser
                                        println!("Opening http://localhost:{}", server.port);
                                    }
                                }
                                ServerStatus::Starting | ServerStatus::Stopping => {
                                    ui.label("Processing...");
                                }
                            }
                            // Settings menu
                            ui.menu_button("⚙", |ui| {
                                // Note: auto_restart is not updated in snapshot, so this is view-only
                                let _ = server.auto_restart;
                                ui.checkbox(&mut false, "Auto-restart");
                                if ui.button("Edit").clicked() {
                                    // TODO: Edit server configuration
                                }
                                if ui.button("Delete").clicked() {
                                    // TODO: Delete server (with confirmation)
                                }
                                if ui.button("View Logs").clicked() {
                                    // TODO: Show detailed logs
                                }
                            });
                        });
                    });
                    // Show recent logs
                    if !server.logs.is_empty() {
                        ui.separator();
                        ui.collapsing("Recent Logs", |ui| {
                            let recent_logs: Vec<_> = server.logs.iter().rev().take(5).collect();
                            for log in recent_logs.iter().rev() {
                                ui.small(log.as_str());
                            }
                        });
                    }
                });
                ui.add_space(5.0);
            }
            // Process actions after the UI loop to avoid borrow checker issues
            for (i, act) in actions {
                match act {
                    "start" => self.start_server(i),
                    "stop" => self.stop_server(i),
                    "restart" => self.restart_server(i),
                    _ => {}
                }
            }
            if self.servers.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("No servers configured");
                        ui.label("Click 'New Server' to add your first development server");
                    });
                });
            }
        });

        // New server dialog
        if self.show_new_server_dialog {
            egui::Window::new("Create New Server")
                .collapsible(false)
                .resizable(true)
                .default_width(400.0)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_server_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.text_edit_singleline(&mut self.new_server_port);
                    });

                    ui.separator();

                    ui.label("Templates:");
                    ui.horizontal_wrapped(|ui| {
                        for (name, template) in &self.server_templates {
                            let is_selected = self.selected_template.as_ref() == Some(name);
                            
                            if ui.selectable_label(is_selected, format!("{} {}", template.icon, name)).clicked() {
                                self.selected_template = Some(name.clone());
                                self.new_server_port = template.default_port.to_string();
                                self.new_server_command = template.command.clone();
                                if self.new_server_name.is_empty() {
                                    self.new_server_name = name.clone();
                                }
                            }
                        }
                    });

                    if let Some(template_name) = &self.selected_template {
                        if let Some(template) = self.server_templates.get(template_name) {
                            ui.separator();
                            ui.small(&template.description);
                        }
                    }

                    ui.separator();

                    ui.label("Command:");
                    ui.text_edit_multiline(&mut self.new_server_command);

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            if !self.new_server_name.trim().is_empty() && 
                               !self.new_server_command.trim().is_empty() {
                                
                                let port = self.new_server_port.parse::<u16>().unwrap_or(8080);
                                
                                let new_server = Server {
                                    name: self.new_server_name.clone(),
                                    port,
                                    command: self.new_server_command.clone(),
                                    status: ServerStatus::Stopped,
                                    pid: None,
                                    start_time: None,
                                    logs: vec![],
                                    auto_restart: false,
                                };
                                
                                self.servers.push(new_server);
                                self.show_new_server_dialog = false;
                            }
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.show_new_server_dialog = false;
                        }
                    });
                });
        }
    }
}

// Mock random number generation since we don't have rand crate
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    pub fn random<T: From<u64>>() -> T {
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        T::from(hasher.finish())
    }
}