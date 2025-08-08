# Phase 5: Remove Plugin System

## Overview
This phase focuses on completely removing the plugin system infrastructure, including WASM runtime, plugin API, plugin communication, and all default plugins while preserving any essential status bar logic that was extracted in Phase 4.

## Goals
- Delete all plugin infrastructure and directories
- Remove WASM runtime and related dependencies
- Remove plugin API and communication systems
- Clean up plugin-related message types and data structures
- Remove plugin workspace members from Cargo.toml
- Ensure no plugin-related code remains in the codebase

## Detailed Tasks

### 1. Delete Plugin Infrastructure

#### Remove Plugin Directories
- [ ] Delete `default-plugins/` directory entirely
- [ ] Delete `zellij-tile/` directory entirely  
- [ ] Delete `zellij-tile-utils/` directory entirely
- [ ] Delete `zellij-server/src/plugins/` directory entirely
- [ ] Verify no plugin-related files remain

#### Remove Plugin Source Files
- [ ] Remove `zellij-server/src/plugins.rs` (if exists as single file)
- [ ] Remove any plugin-related modules from `zellij-server/src/lib.rs`
- [ ] Remove plugin imports from all server files
- [ ] Remove plugin-related test files

### 2. Remove WASM Runtime and Dependencies

#### Clean Up Cargo.toml Dependencies
- [ ] Remove `wasmtime` dependency from root Cargo.toml
- [ ] Remove `wasmtime` from workspace dependencies
- [ ] Remove any WASM-related dependencies:
  - [ ] `wasmtime-wasi`
  - [ ] `wasi-common`
  - [ ] `wasmtime-cranelift`
  - [ ] Any other WASM-related crates
- [ ] Remove plugin compilation features

#### Remove Plugin Workspace Members
- [ ] Remove all `default-plugins/*` from workspace members
- [ ] Remove `zellij-tile` from workspace members
- [ ] Remove `zellij-tile-utils` from workspace members
- [ ] Update workspace member list to only include:
  - [ ] Main binary (`.`)
  - [ ] `zellij-client`
  - [ ] `zellij-server`
  - [ ] `zellij-utils`

### 3. Remove Plugin API and Communication

#### Clean Up Message Types
- [ ] Remove `PluginInstruction` enum entirely
- [ ] Remove plugin-related variants from `ServerInstruction`
- [ ] Remove plugin-related variants from `ScreenInstruction`
- [ ] Remove plugin-related variants from `ClientInstruction`
- [ ] Remove `PluginId` type definitions
- [ ] Remove `PluginCapabilities` struct

#### Remove Plugin Communication Channels
- [ ] Remove `to_plugin` channel creation in server
- [ ] Remove `plugin_receiver` channel handling
- [ ] Remove plugin message routing
- [ ] Remove plugin thread communication
- [ ] Clean up `ThreadSenders` to remove plugin sender

#### Remove Plugin Data Structures
- [ ] Remove `PluginAliases` type
- [ ] Remove `PluginAlias` struct
- [ ] Remove `RunPluginOrAlias` enum
- [ ] Remove plugin-related fields from `Layout`
- [ ] Remove plugin-related fields from configuration

### 4. Clean Up Server Code

#### Remove Plugin Handling from `zellij-server/src/lib.rs`
- [ ] Remove plugin thread spawning in `init_session()`
- [ ] Remove plugin-related imports
- [ ] Remove `PluginInstruction` handling in server loop
- [ ] Remove plugin initialization code
- [ ] Remove plugin cleanup code in `Drop` implementation

#### Clean Up Screen Code
- [ ] Remove plugin coordination from `zellij-server/src/screen.rs`
- [ ] Remove plugin pane handling
- [ ] Remove plugin event forwarding
- [ ] Remove plugin render coordination
- [ ] Remove plugin-related screen instructions

#### Clean Up PTY Code
- [ ] Remove plugin-related PTY instructions
- [ ] Remove plugin spawn handling
- [ ] Remove plugin communication from PTY thread

### 5. Remove Plugin-Related Configuration

#### Clean Up Configuration System
- [ ] Remove plugin configuration from `Config` struct
- [ ] Remove plugin aliases from configuration
- [ ] Remove plugin-related CLI arguments
- [ ] Remove plugin directory configuration
- [ ] Remove plugin permission system

#### Remove Plugin Utilities
- [ ] Remove plugin-related utilities from `zellij-utils`
- [ ] Remove plugin API definitions
- [ ] Remove plugin event types
- [ ] Remove plugin permission types
- [ ] Remove plugin loading utilities

### 6. Clean Up Layout System

#### Remove Plugin Layout Support
- [ ] Remove `Run::Plugin` variant from layout system
- [ ] Remove plugin-related layout parsing
- [ ] Remove plugin spawn from layout processing
- [ ] Update layout system to handle only terminal commands
- [ ] Remove plugin-related layout validation

#### Simplify Run Commands
- [ ] Keep only `Run::Command` variant
- [ ] Remove plugin-related run command processing
- [ ] Simplify layout parsing to exclude plugins
- [ ] Update layout examples to remove plugin references

### 7. Remove Plugin Assets and Resources

#### Clean Up Asset Files
- [ ] Remove plugin WASM files from `zellij-utils/assets/plugins/`
- [ ] Remove plugin-related asset loading code
- [ ] Remove plugin asset installation code
- [ ] Remove plugin asset validation

#### Remove Plugin Documentation
- [ ] Remove plugin-related documentation files
- [ ] Remove plugin API documentation
- [ ] Remove plugin development guides
- [ ] Update main documentation to remove plugin references

### 8. Update Build System

#### Clean Up Build Configuration
- [ ] Remove plugin compilation from build scripts
- [ ] Remove WASM target compilation
- [ ] Remove plugin-related build features
- [ ] Update CI/CD to exclude plugin builds
- [ ] Remove plugin-related test configurations

#### Update Installation Scripts
- [ ] Remove plugin installation from install scripts
- [ ] Remove plugin asset copying
- [ ] Update package configurations to exclude plugins
- [ ] Remove plugin-related file permissions

## Testing Checklist

After completing Phase 5:

- [ ] Project compiles without any plugin-related errors
- [ ] No WASM runtime is initialized
- [ ] No plugin threads are created
- [ ] Server starts without plugin infrastructure
- [ ] Client works without plugin communication
- [ ] Status bar still functions (from Phase 4)
- [ ] Terminal functionality is unaffected
- [ ] No plugin-related memory usage
- [ ] Build time is reduced (no WASM compilation)

## Files Deleted

- [ ] `default-plugins/` (entire directory)
- [ ] `zellij-tile/` (entire directory)
- [ ] `zellij-tile-utils/` (entire directory)
- [ ] `zellij-server/src/plugins/` (entire directory)
- [ ] Plugin WASM assets
- [ ] Plugin-related documentation

## Files Modified

- [ ] `Cargo.toml` - Remove plugin dependencies and workspace members
- [ ] `zellij-server/src/lib.rs` - Remove plugin infrastructure
- [ ] `zellij-server/src/screen.rs` - Remove plugin coordination
- [ ] `zellij-server/src/pty.rs` - Remove plugin PTY handling
- [ ] `zellij-utils/src/` - Remove plugin utilities and types
- [ ] `zellij-client/src/lib.rs` - Remove plugin client handling

## Files to Review

Before proceeding to Phase 6:
- [ ] `Cargo.toml` workspace configuration
- [ ] Server initialization code
- [ ] Message type definitions
- [ ] Build scripts and CI configuration

## Success Criteria

Phase 5 is complete when:
1. No plugin-related code exists in the codebase
2. WASM runtime is completely removed
3. Plugin dependencies are removed from Cargo.toml
4. Project compiles without plugin infrastructure
5. Application runs without any plugin functionality
6. Status bar continues to work (direct rendering)
7. Build time and binary size are reduced
8. No plugin-related memory allocation occurs

## Potential Issues

Watch out for:
- [ ] Circular dependencies when removing plugin types
- [ ] Configuration parsing errors with plugin references
- [ ] Layout parsing failures with plugin commands
- [ ] Missing imports after plugin removal
- [ ] Build script failures
- [ ] Test failures due to missing plugin infrastructure

## Verification Steps

To ensure complete plugin removal:
- [ ] Search codebase for "plugin" (case-insensitive)
- [ ] Search for "wasm" references
- [ ] Search for "wasmtime" references
- [ ] Verify no plugin-related types remain
- [ ] Check that all plugin directories are deleted
- [ ] Confirm no plugin-related dependencies in Cargo.toml

## Notes

- Be thorough in removing all plugin references
- Test compilation frequently during removal
- Ensure status bar functionality is preserved
- Check for any remaining plugin imports or types
- Verify no plugin-related configuration remains
- Test that the application still functions correctly
- Document any functionality that was lost
- Consider any plugin features that need alternative implementation
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
