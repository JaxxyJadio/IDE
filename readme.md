# Jadio IDE

Jadio IDE is a modern, extensible Integrated Development Environment (IDE) built in Rust using [egui](https://github.com/emilk/egui) for a fast, native, and highly modular user experience. It empowers developers with advanced code intelligence, plugin support, and a rich set of built-in tools for code editing, project management, and automation.

## Features

- **Modular Backend and Frontend:** Clean separation of logic and UI for easy extension and maintenance.
- **Code Agent System:** AI-powered code assistance, context-aware suggestions, and automation tools.
- **Plugin Architecture:** Add or develop plugins to extend IDE functionality.
- **Integrated Terminal:** Run shell commands and scripts directly within the IDE.
- **Project Explorer:** Navigate and manage files and folders efficiently.
- **Script Runner:** Configure, execute, and manage scripts with shortcuts and buttons.
- **Customizable Panels:** Activity bars, status bars, and menus for a tailored workflow.
- **Special Buttons:** 5 configurable shortcut buttons for any action.
- **Status Bar:** Bottom bar showing git status, settings, and more.
- **Search Bar:** VS Code-style top-center search for fast navigation.
- **Rust/egui Foundation:** Fast, cross-platform, and open source.

## Project Structure

```
src/
  main.rs
  mod.rs
  backend/
    mod.rs
    activity_bar_left/
      mod.rs
      activity_bar_left_logic.rs
    activity_bar_right/
      mod.rs
      activity_bar_right_logic.rs
    code_agent/
      mod.rs
      agent.rs
      agent_server_logic.rs
      autoprompt.rs
      chat.rs
      code_agent_logic.rs
      context.rs
      files_changed.rs
      hot_swapper.rs
      lazy_loader.rs
      model_loader.rs
      instructions/
        mod.rs
        agent_instruction_logic.rs
        instructions.yaml
      memory/
        mod.rs
        agent_memory_logic.rs
        memory.yaml
      tools/
        mod.rs
        base/
          mod.rs
          agent_tools.yaml
          docstring_audit.rs
          document.rs
          lint.rs
          parse.rs
          parse_test.rs
        plugins/
          mod.rs
          agent_plugins.yaml
        tool_sets/
          mod.rs
          toolset1.yaml
    code_editor/
      mod.rs
      code_editor_logic.rs
      syntax_highlighting.rs
      saving.rs
      backup.rs
    dropdown_menu/
      mod.rs
      dropdown_menu_logic.rs
    explorer_bar/
      mod.rs
      explorer_bar_logic.rs
    file_system/
      mod.rs
      file_system_logic.rs
    other_logic/
      debug/
      llm_cli_helper/
    plugin_control/
      mod.rs
      plugin_control_logic.rs
    project_manager/
      mod.rs
      project_manager_logic.rs
    script_runner/
      mod.rs
      script_runner_logic.rs
    search_bar/
      mod.rs
      search_bar_logic.rs
    server/
      mod.rs
      server_logic.rs
      server_port_logic.rs
    settings_manager/
      mod.rs
      settings_manager_logic.rs
    shell_terminal/
      mod.rs
      shell_terminal_list_logic.rs
      shell_terminal_logic.rs
      shell_terminal_menu_logic.rs
    special_buttons/
      mod.rs
      special_buttons_logic.rs
    status_bar/
      mod.rs
      status_bar_logic.rs
  frontend/
    main.rs
    mod.rs
    style.rs
    code_agent_ui/
      mod.rs
      codeagent.rs
      codeagentactivitybar.rs
    code_editor_ui/
      mod.rs
      code_editor.rs
    explorer_ui/
      mod.rs
      file_explorer.rs
      file_explorer_activity_bar.rs
    icons/
    other_window_ui/
      mod.rs
      ai_settings_ui/
        mod.rs
        aisettings.rs
      help_panel_ui/
        mod.rs
        help.rs
      plugin_panel_ui/
        mod.rs
        pluginpanel.rs
      script_runner_ui/
        mod.rs
        scriptrunner.rs
      server_panel_ui/
        mod.rs
        serverpanel.rs
      settings_panel_ui/
        mod.rs
        settings_panel.rs
    shell_terminal_ui/
      mod.rs
      shell_terminal.rs
    status_bar_ui/
      mod.rs
      statusbar.rs
    top_menu_bar_ui/
      mod.rs
      dropdownmenu.rs
      searchbar.rs
      specialbuttonsbar.rs
Cargo.toml
Cargo.lock
readme.md
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

Contributions are welcome! Please open issues or pull requests for bug fixes, new features, or suggestions.

## License

This project is open source and available under the MIT License.

---

*For more details, see source code and module comments.*