# Jadio IDE

A Rust application using [egui](https://github.com/emilk/egui) for a modern, extensible IDE experience.

## Project Structure

```
Cargo.lock
Cargo.toml
readme.md
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
            memory/
            tools/
        code_editor/
            backup.rs
            code_editor_logic.rs
            saving.rs
            syntax_highlighting.rs
        dropdown_menu/
            dropdown_menu_logic.rs
        explorer_bar/
            explorer_bar_logic.rs
        other_logic/
            debug/
            llm_cli_helper/
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
        editor_ui/
            editor.rs
        explorer_ui/
            explorer.rs
            exploreractivitybar.rs
        icons/
        other_window_ui/
            ai_settings_ui/
            help_panel_ui/
            plugin_panel_ui/
            script_runner_ui/
            server_panel_ui/
            settings_panel_ui/
        shell_terminal_ui/
            shell_terminal.rs
        status_bar_ui/
            statusbar.rs
        top_menu_bar_ui/
            dropdownmenu.rs
            searchbar.rs
            specialbuttonsbar.rs
target/
    CACHEDIR.TAG
    debug/
        jadio_ide.d
        jadio_ide.exe
        jadio_ide.pdb
        build/
            ...
        deps/
            ...
        examples/
        incremental/
```

## Getting Started

1. **Build and Run:**
   ```sh
   cargo run
   ```
2. **Requirements:**
   - Rust toolchain (https://rustup.rs)

## Features
- Modular backend and frontend
- Extensible code agent and plugin system
- Integrated terminal, explorer, and script runner

---

*For more details, see source code and module comments.*