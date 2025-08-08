# Zellij to Typey Pipe Conversion Plan

## Overview

This document outlines a systematic plan to strip down Zellij from a full terminal multiplexer to a transparent shell wrapper with a single-row status footer. The goal is to preserve the core shell wrapping and ANSI handling capabilities while removing all multiplexing functionality.

## Architecture Analysis

### Core Components to Preserve

1. **Shell Process Management** (`zellij-server/src/pty.rs`)
   - PTY creation and management
   - Shell process spawning
   - Input/output handling between shell and terminal

2. **ANSI/VT Terminal Handling** (`zellij-server/src/panes/terminal_pane.rs`)
   - VTE parser for ANSI escape sequences
   - Terminal character rendering
   - Cursor position tracking
   - Basic terminal emulation

3. **Input Processing** (`zellij-client/src/input_handler.rs`)
   - Keyboard input capture
   - Mouse event handling
   - Raw terminal mode management

4. **Client-Server Communication** (`zellij-client/src/lib.rs`, `zellij-server/src/lib.rs`)
   - IPC mechanism for client-server communication
   - Message passing infrastructure

5. **Status Bar Rendering** (from `default-plugins/status-bar/`)
   - Single row status display
   - Basic UI rendering for footer

### Components to Remove

1. **Multi-tab System** (`zellij-server/src/tab/`)
   - Tab management
   - Tab switching logic
   - Multiple tab rendering

2. **Multi-pane System** (`zellij-server/src/panes/`)
   - Pane splitting and resizing
   - Multiple pane coordination
   - Pane boundaries and frames

3. **Plugin System** (`zellij-server/src/plugins/`, `default-plugins/`)
   - WASM plugin loading
   - Plugin API
   - Plugin communication
   - All default plugins except minimal status bar

4. **Layout System** (`zellij-utils/src/input/layout/`)
   - Complex layout definitions
   - Layout serialization
   - Multi-pane layouts

5. **Session Management** 
   - Session persistence
   - Session switching
   - Multiple session support

6. **Advanced Features**
   - Floating panes
   - Stacked panes
   - Pane groups
   - Web server capability
   - Configuration system (keep minimal)

## Systematic Removal Plan

### Phase 1: Simplify Entry Point and CLI

1. **Modify `src/main.rs`**
   - Remove all session management commands
   - Remove layout options
   - Remove plugin-related commands
   - Keep only basic shell wrapping functionality

2. **Simplify `src/commands.rs`**
   - Remove session listing, killing, switching
   - Remove layout conversion
   - Remove plugin management
   - Keep only basic client startup

### Phase 2: Strip Server Architecture

1. **Simplify `zellij-server/src/lib.rs`**
   - Remove multi-client support (keep single client)
   - Remove session metadata
   - Remove plugin thread initialization
   - Remove background jobs thread
   - Keep only: PTY thread, screen thread, basic server loop

2. **Modify `zellij-server/src/screen.rs`**
   - Remove tab management
   - Remove multi-pane support
   - Keep single pane with status footer
   - Remove layout processing
   - Remove plugin coordination

3. **Simplify PTY Management**
   - Keep single shell process
   - Remove pane-specific PTY handling
   - Maintain direct shell I/O

### Phase 3: Simplify Client Architecture

1. **Modify `zellij-client/src/lib.rs`**
   - Remove session switching
   - Remove complex client state management
   - Keep basic input/output handling
   - Maintain terminal setup/teardown

2. **Simplify Input Handling**
   - Remove mode switching (keep normal mode only)
   - Remove complex keybindings
   - Keep basic input passthrough
   - Maintain mouse support for status bar

### Phase 4: Create Minimal Status Bar

1. **Create Simple Status Component**
   - Single row at bottom
   - Basic system information
   - No plugin system - direct rendering
   - Minimal configuration

2. **Integrate Status Rendering**
   - Modify screen rendering to reserve bottom row
   - Direct status bar rendering (no plugins)
   - Simple text-based status display

### Phase 5: Remove Plugin System

1. **Delete Plugin Infrastructure**
   - Remove `zellij-server/src/plugins/`
   - Remove `default-plugins/` (except extract status logic)
   - Remove WASM runtime
   - Remove plugin API

2. **Remove Plugin Dependencies**
   - Remove wasmtime from Cargo.toml
   - Remove plugin-related crates
   - Clean up plugin message types

### Phase 6: Simplify Configuration

1. **Minimal Config System**
   - Keep only essential options
   - Remove keybinding configuration
   - Remove theme system (use defaults)
   - Remove layout configuration

2. **Clean Up Utils Crate**
   - Remove layout parsing
   - Remove plugin utilities
   - Keep only basic data structures
   - Remove complex configuration options

### Phase 7: Clean Up Dependencies

1. **Remove Unused Crates**
   - Remove wasmtime and WASM-related deps
   - Remove complex UI dependencies
   - Remove session serialization deps
   - Keep only essential terminal handling

2. **Simplify Workspace**
   - Remove plugin workspace members
   - Keep only: main binary, client, server, utils
   - Remove tile and tile-utils crates

## Implementation Strategy

### Step-by-Step Approach

1. **Start with a Copy**
   - Copy Zellij from `./zellij/` to `./typeypipe/` directory
   - Rename project to "typey-pipe" in the copied directory
   - Update Cargo.toml metadata in `./typeypipe/`
   - Keep original Zellij source in `./zellij/` unmodified for reference

2. **Remove from Outside In**
   - Delete entire plugin directories first
   - Remove unused workspace members
   - Strip CLI commands
   - Work inward to core functionality

3. **Preserve Core Loop**
   - Keep the basic client-server architecture
   - Maintain PTY handling
   - Preserve ANSI processing
   - Keep input passthrough

4. **Test at Each Step**
   - Ensure basic shell wrapping works
   - Verify input/output passthrough
   - Test terminal setup/teardown
   - Validate ANSI handling

### Key Files to Modify

**Note: All modifications are done in the `./typeypipe/` directory. The original `./zellij/` directory remains unmodified for reference.**

1. **Entry Points**
   - `typeypipe/src/main.rs` - Simplify CLI
   - `typeypipe/src/commands.rs` - Remove complex commands

2. **Server Core**
   - `typeypipe/zellij-server/src/lib.rs` - Strip to essentials
   - `typeypipe/zellij-server/src/screen.rs` - Single pane + status
   - `typeypipe/zellij-server/src/pty.rs` - Single shell management

3. **Client Core**
   - `typeypipe/zellij-client/src/lib.rs` - Basic client
   - `typeypipe/zellij-client/src/input_handler.rs` - Passthrough input

4. **Configuration**
   - `typeypipe/zellij-utils/src/` - Minimal config and data structures

### Files to Delete

**Note: All deletions are done in the `./typeypipe/` directory. The original `./zellij/` directory remains unmodified.**

1. **Plugin System**
   - `typeypipe/default-plugins/` (entire directory)
   - `typeypipe/zellij-tile/` and `typeypipe/zellij-tile-utils/`
   - `typeypipe/zellij-server/src/plugins/`

2. **Complex Features**
   - Layout-related files in `typeypipe/`
   - Session management files in `typeypipe/`
   - Multi-pane/tab files in `typeypipe/`
   - Web server files in `typeypipe/`

## Expected Outcome

After this conversion, you'll have:

1. **Transparent Shell Wrapper**
   - Spawns single shell process
   - Passes all input directly to shell
   - Displays all shell output directly

2. **Status Footer**
   - Single row at bottom
   - Basic system information
   - No complex plugin system

3. **Preserved Capabilities**
   - Full ANSI/VT compatibility
   - Mouse support
   - Terminal resizing
   - Proper terminal setup/teardown

4. **Clean Architecture**
   - Minimal codebase
   - Clear separation of concerns
   - Ready for queue processing integration

This stripped-down version will provide the foundation for adding your queue processing system while maintaining the robust terminal handling that Zellij provides.