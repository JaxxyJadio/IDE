Jadio IDE
Overview
Jadio IDE is a native Rust IDE built with egui for fast, cross-platform UI. No Electron. No
bloat. Just Rust speed, plugin-powered customization, and an integrated AI Code Agent for
smarter coding.
Why Jadio
• Rust/egui Core: Native-speed, cross-platform UI
• AI Code Agent: Context-aware code generation and automation
• Plugin Architecture: Easily extend your IDE
• Lightweight: No heavyweight runtimes
Features
Highlights:
• AI Code Agent System
– Automate repetitive tasks
– Customizable instructions
– Context-aware suggestions
• Plugin System
– Build or install plugins
– Clean, modular design
• Rust/egui Foundation
– Fast native rendering
– Cross-platform
Built-in Tools:
• Integrated Terminal
• Project Explorer
• Script Runner
• Code Editor with syntax highlighting
1
• Customizable Panels and Buttons
• Status Bar with Git integration
• Search Bar (VS Code-style)
• 5 Configurable Shortcut Buttons
Project Structure
Jadio is organized for modular development. Backend logic is cleanly separated from frontend
UI.
Top-level layout:
• src/
– backend/ — Core logic, AI Agent, server
– frontend/ — egui UI, panels, styling
• Cargo.toml — Rust manifest
For the complete module breakdown, see PROJECT STRUCTURE.md.
Getting Started
1. Install Rust
Visit: https://rustup.rs
Recommended version: rustc 1.75+
2. Build and Run
cargo run
For a release build:
cargo build --release
3. Recommended Workflow
Open the project in your favorite Rust IDE or editor. Build, run, and modify.
Contributing
We welcome contributions:
• Fix bugs
• Develop new plugins
• Improve the AI Code Agent
• Enhance documentation
Pull Requests are welcome. Please open issues to discuss features or ideas.
2
License
MIT License — free as in freedom.
Jadio is built for developers who want an IDE that works the way they do. Hackable. Fast.
Yours.
3
