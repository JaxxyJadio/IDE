# Jadio IDE

Jadio IDE is a modern, extensible Integrated Development Environment (IDE) built in Rust using [egui](https://github.com/emilk/egui) for a fast, native, and highly modular user experience. It is designed to empower developers with advanced code intelligence, plugin support, and a rich set of built-in tools for code editing, project management, and automation.

## Features

- **Modular Backend and Frontend:** Clean separation of logic and UI, making it easy to extend and maintain.
- **Code Agent System:** Integrates AI-powered code assistance, context-aware suggestions, and automation tools.
- **Plugin Architecture:** Easily add or develop plugins to extend IDE functionality.
- **Integrated Terminal:** Run shell commands and scripts directly within the IDE.
- **Project Explorer:** Navigate and manage files and folders efficiently.
- **Script Runner:** Execute and manage scripts with ease.
- **Customizable Panels:** Activity bars, status bars, and menus for a tailored workflow.
- **Rust/egui Foundation:** Fast, cross-platform, and open source.

## Project Structure

```
Cargo.lock
Cargo.toml
.gitignore
readme.md
.vscode/
    tasks.json
src/
    backend/
        mod.rs
        activity_bar_left/
            activity_bar_left_logic.rs
        activity_bar_right/
            activity_bar_right_logic.rs
        code_agent/
            agent.rs
            autoprompt.rs
            chat.rs
            code_agent_logic.rs
            context.rs
            files_changed.rs
            hot_swapper.rs
            lazy_loader.rs
            model_loader.rs
            server_logic.rs
            instructions/
                agent_instruction_logic.rs
                instructions.yaml
            memory/
                agent_memory_logic.rs
                memory.yaml
            tools/
                base/
                    agent_tools.yaml
                    docstring_audit.rs
                    document.rs
                    lint.rs
                    parse.rs
                plugins/
                    agent_plugins.yaml
                tool_sets/
                    toolset1.yaml
        code_editor/
            backup.rs
            code_editor_logic.rs
            saving.rs
            syntax_highlighting.rs
        dropdown_menu/
            dropdown_menu_logic.rs
        explorer_bar/
            explorer_bar_logic.rs
        file_system/
            file_system_logic.rs
        other_logic/
            debug/
                debug_logic.rs
            llm_cli_helper/
                llm_cli_helper_logic.rs
        plugin_control/
            plugin_control_logic.rs
        script_runner/
            script_runner_logic.rs
        search_bar/
            search_bar_logic.rs
        server/
            server_logic.rs
            server_port_logic.rs
        shell_terminal/
            shell_terminal_list_logic.rs
            shell_terminal_logic.rs
            shell_terminal_menu_logic.rs
        special_buttons/
            special_buttons_logic.rs
        status_bar/
            status_bar_logic.rs
    frontend/
        main.rs
        mod.rs
        style.rs
        code_agent_ui/
            codeagent.rs
            codeagentactivitybar.rs
            mod.rs
        code_editor_ui/
            code_editor.rs
            mod.rs
        explorer_ui/
            explorer.rs
            exploreractivitybar.rs
            mod.rs
        icons/
        other_window_ui/
            ai_settings_ui/
                aisettings.rs
                mod.rs
            help_panel_ui/
                help.rs
                mod.rs
            plugin_panel_ui/
                pluginpanel.rs
                mod.rs
            script_runner_ui/
                scriptrunner.rs
                mod.rs
            server_panel_ui/
                serverpanel.rs
                mod.rs
            settings_panel_ui/
                settings_panel.rs
                mod.rs
        shell_terminal_ui/
            shell_terminal.rs
            mod.rs
        status_bar_ui/
            statusbar.rs
            mod.rs
        top_menu_bar_ui/
            dropdownmenu.rs
            searchbar.rs
            specialbuttonsbar.rs
            mod.rs
target/
    CACHEDIR.TAG
    debug/
        .cargo-lock
        jadio_ide.d
        jadio_ide.exe
        jadio_ide.pdb
        build/
        deps/
        examples/
        fingerprint/
        incremental/
        ...
```

## Getting Started

1. **Install Rust:**
   - Get the Rust toolchain from [rustup.rs](https://rustup.rs)
2. **Build and Run:**
   ```sh
   cargo run
   ```
3. **Open in VS Code:**
   - Recommended: Use the provided VS Code tasks for easy build/run.

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, new features, or suggestions. See the code for module-level documentation and comments.

## License

This project is open source and available under the MIT License.

---

*For more details, see source code and module comments.*