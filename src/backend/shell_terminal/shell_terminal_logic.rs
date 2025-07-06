use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TerminalMessage {
    pub content: String,
    pub message_type: MessageType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Output,
    Error,
    Input,
    System,
}

pub struct TerminalHandler {
    process: Option<Child>,
    output_receiver: Option<Receiver<TerminalMessage>>,
    input_sender: Option<Sender<String>>,
    history: VecDeque<TerminalMessage>,
    max_history: usize,
    current_directory: std::path::PathBuf,
}

impl Default for TerminalHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalHandler {
    pub fn new() -> Self {
        Self {
            process: None,
            output_receiver: None,
            input_sender: None,
            history: VecDeque::new(),
            max_history: 1000,
            current_directory: std::env::current_dir().unwrap_or_default(),
        }
    }

    pub fn start_shell(&mut self, shell_command: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // Stop existing process if any
        self.stop_shell();

        let shell = shell_command.unwrap_or_else(|| {
            if cfg!(windows) {
                "powershell.exe".to_string()
            } else {
                "/bin/bash".to_string()
            }
        });

        // Create channels for communication
        let (output_tx, output_rx) = mpsc::channel();
        let (input_tx, input_rx) = mpsc::channel();

        // Start the shell process
        let mut process = Command::new(&shell)
            .current_dir(&self.current_directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Get handles to stdin, stdout, and stderr
        let stdin = process.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = process.stdout.take().ok_or("Failed to open stdout")?;
        let stderr = process.stderr.take().ok_or("Failed to open stderr")?;

        // Start thread to handle stdout
        let output_tx_stdout = output_tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let message = TerminalMessage {
                        content: line,
                        message_type: MessageType::Output,
                        timestamp: chrono::Utc::now(),
                    };
                    if output_tx_stdout.send(message).is_err() {
                        break;
                    }
                }
            }
        });

        // Start thread to handle stderr
        let output_tx_stderr = output_tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let message = TerminalMessage {
                        content: line,
                        message_type: MessageType::Error,
                        timestamp: chrono::Utc::now(),
                    };
                    if output_tx_stderr.send(message).is_err() {
                        break;
                    }
                }
            }
        });

        // Start thread to handle stdin
        let mut stdin_writer = stdin;
        thread::spawn(move || {
            while let Ok(input) = input_rx.recv() {
                if writeln!(stdin_writer, "{}", input).is_err() {
                    break;
                }
                if stdin_writer.flush().is_err() {
                    break;
                }
            }
        });

        self.process = Some(process);
        self.output_receiver = Some(output_rx);
        self.input_sender = Some(input_tx);

        // Add system message
        self.add_system_message(format!("Started shell: {}", shell));

        Ok(())
    }

    pub fn stop_shell(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
        
        self.output_receiver = None;
        self.input_sender = None;
        
        self.add_system_message("Shell stopped".to_string());
    }

    pub fn send_command(&mut self, command: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref sender) = self.input_sender {
            // Add to history as input
            let message = TerminalMessage {
                content: format!("$ {}", command),
                message_type: MessageType::Input,
                timestamp: chrono::Utc::now(),
            };
            // Send to process first, then add to history to avoid borrow conflict
            sender.send(command)?;
            self.add_message(message);
            Ok(())
        } else {
            Err("No active shell process".into())
        }
    }

    pub fn execute_command(&mut self, command: String) -> Result<(), Box<dyn std::error::Error>> {
        // Handle built-in commands
        if self.handle_builtin_command(&command) {
            return Ok(());
        }

        // Send to shell process
        self.send_command(command)
    }

    fn handle_builtin_command(&mut self, command: &str) -> bool {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        match parts[0] {
            "cd" => {
                let new_dir = if parts.len() > 1 {
                    std::path::PathBuf::from(parts[1])
                } else {
                    dirs::home_dir().unwrap_or_default()
                };

                match std::env::set_current_dir(&new_dir) {
                    Ok(()) => {
                        self.current_directory = std::env::current_dir().unwrap_or_default();
                        self.add_system_message(format!("Changed directory to: {}", self.current_directory.display()));
                    }
                    Err(e) => {
                        self.add_error_message(format!("cd: {}", e));
                    }
                }
                true
            }
            "pwd" => {
                self.add_output_message(self.current_directory.display().to_string());
                true
            }
            "clear" => {
                self.history.clear();
                true
            }
            "exit" => {
                self.stop_shell();
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        // Process any new messages from the shell
        let mut messages = Vec::new();
        if let Some(ref receiver) = self.output_receiver {
            while let Ok(message) = receiver.try_recv() {
                messages.push(message);
            }
        }
        for message in messages {
            self.add_message(message);
        }
    }

    pub fn get_history(&self) -> &VecDeque<TerminalMessage> {
        &self.history
    }

    pub fn get_recent_output(&self, count: usize) -> Vec<&TerminalMessage> {
        self.history.iter().rev().take(count).collect()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn set_working_directory<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.current_directory = path.as_ref().to_path_buf();
        if std::env::set_current_dir(&self.current_directory).is_ok() {
            self.add_system_message(format!("Working directory set to: {}", self.current_directory.display()));
        }
    }

    pub fn get_working_directory(&self) -> &std::path::Path {
        &self.current_directory
    }

    fn add_message(&mut self, message: TerminalMessage) {
        self.history.push_back(message);
        
        // Limit history size
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    fn add_system_message(&mut self, content: String) {
        let message = TerminalMessage {
            content,
            message_type: MessageType::System,
            timestamp: chrono::Utc::now(),
        };
        self.add_message(message);
    }

    fn add_output_message(&mut self, content: String) {
        let message = TerminalMessage {
            content,
            message_type: MessageType::Output,
            timestamp: chrono::Utc::now(),
        };
        self.add_message(message);
    }

    fn add_error_message(&mut self, content: String) {
        let message = TerminalMessage {
            content,
            message_type: MessageType::Error,
            timestamp: chrono::Utc::now(),
        };
        self.add_message(message);
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    pub fn run_script<P: AsRef<std::path::Path>>(&mut self, script_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let script_path = script_path.as_ref();
        
        if !script_path.exists() {
            return Err(format!("Script not found: {}", script_path.display()).into());
        }

        let extension = script_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let command = match extension {
            "rs" => format!("cargo run --manifest-path {}", script_path.display()),
            "py" => format!("python {}", script_path.display()),
            "js" => format!("node {}", script_path.display()),
            "sh" | "bash" => format!("bash {}", script_path.display()),
            "ps1" => format!("powershell -File {}", script_path.display()),
            _ => script_path.display().to_string(),
        };

        self.execute_command(command)
    }
}

impl Drop for TerminalHandler {
    fn drop(&mut self) {
        self.stop_shell();
    }
}