# Phase 6: Simplify Configuration

## Overview
This phase focuses on drastically simplifying the configuration system by removing complex features like keybinding configuration, theme systems, layout configuration, and plugin settings while keeping only essential options for basic shell wrapper functionality.

## Goals
- Remove complex keybinding configuration system
- Remove theme and styling configuration
- Remove layout configuration system
- Remove plugin-related configuration
- Keep only minimal essential configuration options
- Simplify configuration parsing and validation
- Reduce configuration file complexity

## Detailed Tasks

### 1. Simplify Configuration Structure

#### Modify `typeypipe/zellij-utils/src/input/config.rs`
- [ ] Remove `keybinds: Keybinds` field from `Config`
- [ ] Remove `themes: Option<Themes>` field
- [ ] Remove `theme: Option<String>` field
- [ ] Remove `plugins: PluginAliases` field
- [ ] Remove `ui: UiConfig` field (keep minimal styling)
- [ ] Remove `env: EnvVarsFromConfig` field (or simplify)
- [ ] Remove `layout: Option<Layout>` field

#### Keep Essential Configuration Fields
- [ ] Keep `options: Options` (simplified)
- [ ] Keep basic terminal settings
- [ ] Keep minimal shell configuration
- [ ] Keep status bar configuration (from Phase 4)
- [ ] Keep debug/logging options

#### Simplify Options Structure
- [ ] Remove `default_mode: Option<InputMode>`
- [ ] Remove `default_shell: Option<PathBuf>` (use system default)
- [ ] Remove `default_cwd: Option<PathBuf>`
- [ ] Remove `default_layout: Option<PathBuf>`
- [ ] Remove `layout_dir: Option<PathBuf>`
- [ ] Remove `theme: Option<String>`
- [ ] Remove `mouse_mode: Option<bool>`
- [ ] Remove `pane_frames: Option<bool>`
- [ ] Remove `mirror_session: Option<bool>`
- [ ] Remove complex UI options

### 2. Remove Keybinding System

#### Delete Keybinding Infrastructure
- [ ] Remove `typeypipe/zellij-utils/src/input/keybinds.rs` file
- [ ] Remove `Keybinds` struct definition
- [ ] Remove `KeybindConfig` struct
- [ ] Remove keybinding parsing logic
- [ ] Remove keybinding validation
- [ ] Remove default keybinding definitions

#### Remove Input Mode System
- [ ] Remove `InputMode` enum (keep only Normal)
- [ ] Remove mode-specific keybinding handling
- [ ] Remove mode transition logic
- [ ] Remove mode-related configuration
- [ ] Simplify input handling to single mode

#### Remove Action System
- [ ] Remove `Action` enum entirely
- [ ] Remove action dispatching logic
- [ ] Remove action-related configuration
- [ ] Remove action validation
- [ ] Remove action serialization

### 3. Remove Theme System

#### Delete Theme Infrastructure
- [ ] Remove theme-related files from `typeypipe/zellij-utils/src/`
- [ ] Remove `Palette` struct complexity
- [ ] Remove `PaletteColor` enum
- [ ] Remove theme parsing logic
- [ ] Remove theme validation
- [ ] Remove default theme definitions

#### Simplify Color Handling
- [ ] Use hardcoded default colors
- [ ] Remove color customization options
- [ ] Remove theme switching capability
- [ ] Keep minimal ANSI color support
- [ ] Remove complex styling options

#### Remove UI Configuration
- [ ] Remove `UiConfig` struct
- [ ] Remove pane frame configuration
- [ ] Remove border styling options
- [ ] Remove UI element customization
- [ ] Keep only essential display options

### 4. Remove Layout Configuration

#### Delete Layout System
- [ ] Remove `typeypipe/zellij-utils/src/input/layout/` directory
- [ ] Remove `Layout` struct and related types
- [ ] Remove layout parsing (KDL format)
- [ ] Remove layout validation
- [ ] Remove layout serialization
- [ ] Remove layout template system

#### Remove Layout-Related Configuration
- [ ] Remove layout directory configuration
- [ ] Remove default layout settings
- [ ] Remove layout switching options
- [ ] Remove layout-related CLI arguments
- [ ] Remove layout file handling

### 5. Simplify Configuration Parsing

#### Simplify Configuration File Format
- [ ] Remove KDL parsing complexity
- [ ] Consider switching to simpler format (TOML/JSON)
- [ ] Remove complex configuration validation
- [ ] Remove configuration merging logic
- [ ] Simplify default configuration generation

#### Reduce Configuration Options
- [ ] Keep only essential options:
  - [ ] Debug mode
  - [ ] Status bar enable/disable
  - [ ] Basic terminal settings
  - [ ] Log level
- [ ] Remove all complex feature configurations
- [ ] Remove plugin-related configurations
- [ ] Remove layout-related configurations

#### Simplify Configuration Loading
- [ ] Remove configuration file watching
- [ ] Remove runtime configuration changes
- [ ] Remove configuration validation complexity
- [ ] Simplify configuration error handling
- [ ] Remove configuration migration logic

### 6. Update Configuration Usage

#### Modify Server Configuration Usage
- [ ] Remove keybinding configuration from server
- [ ] Remove theme configuration from server
- [ ] Remove layout configuration from server
- [ ] Keep only essential server configuration
- [ ] Simplify configuration propagation

#### Modify Client Configuration Usage
- [ ] Remove keybinding configuration from client
- [ ] Remove theme configuration from client
- [ ] Remove mode configuration from client
- [ ] Keep only essential client configuration
- [ ] Simplify configuration handling

### 7. Clean Up Configuration Files

#### Remove Configuration Examples
- [ ] Remove complex configuration examples
- [ ] Remove theme configuration examples
- [ ] Remove keybinding configuration examples
- [ ] Remove layout configuration examples
- [ ] Create simple minimal configuration example

#### Update Default Configuration
- [ ] Create minimal default configuration
- [ ] Remove complex default settings
- [ ] Ensure defaults work without configuration file
- [ ] Simplify configuration documentation

### 8. Remove Configuration-Related CLI

#### Simplify CLI Configuration Options
- [ ] Remove `--config-dir` option complexity
- [ ] Remove theme-related CLI options
- [ ] Remove layout-related CLI options
- [ ] Remove keybinding-related CLI options
- [ ] Keep only basic configuration file option

#### Remove Configuration Commands
- [ ] Remove configuration conversion commands
- [ ] Remove configuration validation commands
- [ ] Remove configuration generation commands
- [ ] Keep only basic configuration handling

## Testing Checklist

After completing Phase 6:

- [ ] Application starts without configuration file
- [ ] Minimal configuration file works correctly
- [ ] No keybinding configuration is processed
- [ ] No theme configuration is processed
- [ ] No layout configuration is processed
- [ ] Status bar configuration works (if implemented)
- [ ] Default settings provide functional shell wrapper
- [ ] Configuration parsing is fast and simple
- [ ] No complex configuration validation occurs

## Files Deleted

- [ ] `typeypipe/zellij-utils/src/input/keybinds.rs`
- [ ] `typeypipe/zellij-utils/src/input/layout/` (entire directory)
- [ ] Theme-related configuration files
- [ ] Complex configuration examples
- [ ] Configuration migration utilities

## Files Modified

- [ ] `typeypipe/zellij-utils/src/input/config.rs` - Simplified configuration
- [ ] `typeypipe/zellij-utils/src/input/options.rs` - Reduced options
- [ ] `typeypipe/zellij-utils/src/cli.rs` - Simplified CLI arguments
- [ ] `typeypipe/src/main.rs` - Simplified configuration handling
- [ ] `typeypipe/zellij-client/src/lib.rs` - Removed configuration complexity
- [ ] `typeypipe/zellij-server/src/lib.rs` - Removed configuration complexity

## Files to Review

Before proceeding to Phase 7:
- [ ] `typeypipe/zellij-utils/src/input/config.rs`
- [ ] `typeypipe/zellij-utils/src/input/options.rs`
- [ ] Configuration loading and parsing code
- [ ] Default configuration generation

## Success Criteria

Phase 6 is complete when:
1. Configuration system is drastically simplified
2. No keybinding configuration exists
3. No theme configuration exists
4. No layout configuration exists
5. Application works with minimal or no configuration
6. Configuration parsing is fast and simple
7. Only essential options remain configurable
8. Default settings provide full functionality


## Minimal Configuration Example

After Phase 6, configuration should be minimal:

```toml
# Optional minimal configuration
[status_bar]
enabled = true

[debug]
enabled = false
```

## Potential Issues

Watch out for:
- [ ] Code that expects complex configuration options
- [ ] Default value handling for removed options
- [ ] Configuration file parsing errors
- [ ] Missing imports after configuration removal
- [ ] Tests that depend on complex configuration
- [ ] CLI argument handling for removed options

## Verification Steps

To ensure configuration is properly simplified:
- [ ] Search for references to removed configuration fields
- [ ] Verify no keybinding parsing occurs
- [ ] Verify no theme parsing occurs
- [ ] Verify no layout parsing occurs
- [ ] Test application with no configuration file
- [ ] Test application with minimal configuration
- [ ] Check configuration loading performance

## Notes

- Ensure application works without any configuration file
- Test with minimal configuration options
- Verify no complex configuration parsing occurs
- Check that defaults provide good user experience
- Ensure status bar configuration still works
- Test configuration error handling
- Document the simplified configuration format
- Consider future extensibility needs
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
