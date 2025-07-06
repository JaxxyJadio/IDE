// WHAT I WANT: Core logic for managing, starting, stopping, and monitoring servers in the Jadio IDE backend.
// WHAT IT DOES: Provides abstractions and utilities for server lifecycle management, status tracking, and integration with the IDE's UI and plugin system.
// TODO: Implement server process management, logging, and error handling.
// FIXME: Address cross-platform process issues and improve robustness for edge cases.

use std::process::{Child, Command, Stdio};
use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

/// Represents the status of a managed server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
    Error(String),
}

/// Represents a managed server process.
pub struct ManagedServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub status: ServerStatus,
    pub process: Option<Child>,
}

impl ManagedServer {
    pub fn new(name: &str, command: &str, args: &[String]) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            args: args.to_vec(),
            status: ServerStatus::Stopped,
            process: None,
        }
    }

    /// Start the server process.
    pub fn start(&mut self) -> io::Result<()> {
        if self.status == ServerStatus::Running {
            return Ok(());
        }
        self.status = ServerStatus::Starting;
        let child = Command::new(&self.command)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        match child {
            Ok(child) => {
                self.process = Some(child);
                self.status = ServerStatus::Running;
                Ok(())
            }
            Err(e) => {
                self.status = ServerStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// Stop the server process.
    pub fn stop(&mut self) -> io::Result<()> {
        if let Some(child) = &mut self.process {
            let _ = child.kill();
            self.process = None;
            self.status = ServerStatus::Stopped;
        }
        Ok(())
    }

    /// Check if the server is running.
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.status = ServerStatus::Stopped;
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Get the latest output from the server's stdout and stderr.
    pub fn get_output(&mut self) -> io::Result<(String, String)> {
        let mut stdout = String::new();
        let mut stderr = String::new();
        if let Some(child) = &mut self.process {
            if let Some(out) = &mut child.stdout {
                let mut buf = [0u8; 4096];
                let n = out.read(&mut buf).unwrap_or(0);
                stdout.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
            if let Some(err) = &mut child.stderr {
                let mut buf = [0u8; 4096];
                let n = err.read(&mut buf).unwrap_or(0);
                stderr.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
        }
        Ok((stdout, stderr))
    }
}

/// Manages multiple servers in the IDE.
pub struct ServerManager {
    pub servers: HashMap<String, ManagedServer>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    pub fn add_server(&mut self, name: &str, command: &str, args: &[String]) {
        self.servers.insert(name.to_string(), ManagedServer::new(name, command, args));
    }

    pub fn start_server(&mut self, name: &str) -> io::Result<()> {
        if let Some(server) = self.servers.get_mut(name) {
            server.start()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Server not found"))
        }
    }

    pub fn stop_server(&mut self, name: &str) -> io::Result<()> {
        if let Some(server) = self.servers.get_mut(name) {
            server.stop()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Server not found"))
        }
    }

    pub fn status(&mut self, name: &str) -> Option<ServerStatus> {
        self.servers.get_mut(name).map(|s| {
            if s.is_running() {
                ServerStatus::Running
            } else {
                s.status.clone()
            }
        })
    }

    pub fn get_output(&mut self, name: &str) -> io::Result<(String, String)> {
        if let Some(server) = self.servers.get_mut(name) {
            server.get_output()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Server not found"))
        }
    }
}
