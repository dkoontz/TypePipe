# Phase 3: Simplify Client Architecture

## Overview
This phase focuses on simplifying the client-side architecture by removing session switching, complex state management, mode switching, and complex keybindings while preserving basic input/output handling and terminal management.

## Goals
- Remove session switching and complex client state management
- Simplify input handling to basic passthrough
- Remove mode switching (keep normal mode only)
- Remove complex keybinding system
- Maintain terminal setup/teardown and basic I/O
- Preserve mouse support for status bar interaction

## Detailed Tasks

### 1. Modify `zellij-client/src/lib.rs`

#### Remove Session Switching
- [ ] Remove `ClientInstruction::SwitchSession` handling
- [ ] Remove `reconnect_to_session` logic in main client loop
- [ ] Remove session switching return value from `start_client()`
- [ ] Remove `ConnectToSession` parameter and logic
- [ ] Simplify client to single session only

#### Remove Complex Client State Management
- [ ] Remove `ClientInfo` enum complexity:
  - [ ] Remove `Attach` variant
  - [ ] Remove `Resurrect` variant  
  - [ ] Keep only `New` variant (simplified)
- [ ] Remove layout parameter from client startup
- [ ] Remove `tab_position_to_focus` parameter
- [ ] Remove `pane_id_to_focus` parameter
- [ ] Remove `is_a_reconnect` parameter
- [ ] Remove `layout_is_welcome_screen` parameter

#### Simplify Client Instructions
- [ ] Remove `ClientInstruction::SwitchSession`
- [ ] Remove `ClientInstruction::UnblockCliPipeInput`
- [ ] Remove `ClientInstruction::CliPipeOutput`
- [ ] Remove `ClientInstruction::WriteConfigToDisk`
- [ ] Remove `ClientInstruction::StartWebServer`
- [ ] Remove `ClientInstruction::RenamedSession`
- [ ] Keep only essential instructions:
  - [ ] `Render`
  - [ ] `Exit`
  - [ ] `Error`
  - [ ] `Connected`
  - [ ] `UnblockInputThread`

#### Remove Complex Terminal Setup
- [ ] Remove web server startup logic
- [ ] Remove complex configuration handling
- [ ] Remove layout processing
- [ ] Simplify to basic terminal setup:
  - [ ] Raw mode setup
  - [ ] Terminal snapshot
  - [ ] Basic ANSI setup
  - [ ] Cleanup on exit

#### Simplify Client Loop
- [ ] Remove loading screen logic
- [ ] Remove pending instructions buffering
- [ ] Remove synchronized output complexity
- [ ] Simplify to basic render/input loop
- [ ] Remove configuration change handling

### 2. Simplify Input Handling

#### Modify `zellij-client/src/input_handler.rs`

##### Remove Mode Switching
- [ ] Remove `InputMode` enum usage (except Normal)
- [ ] Remove `mode` field from `InputHandler`
- [ ] Remove mode-specific input handling
- [ ] Remove mode transition logic
- [ ] Keep only normal input mode behavior

##### Remove Complex Keybindings
- [ ] Remove `Config` keybinding processing
- [ ] Remove `Keybinds` struct usage
- [ ] Remove action dispatching system
- [ ] Remove `Action` enum processing
- [ ] Implement simple key passthrough to shell

##### Simplify Input Processing
- [ ] Remove `InputInstruction` complexity:
  - [ ] Remove `KeyWithModifierEvent`
  - [ ] Remove `AnsiStdinInstructions`
  - [ ] Keep only `KeyEvent` for basic passthrough
- [ ] Remove command execution detection
- [ ] Remove input mode coordination
- [ ] Implement direct key forwarding to PTY

##### Preserve Essential Input Features
- [ ] Keep mouse event handling (for status bar)
- [ ] Keep basic terminal control sequences
- [ ] Keep exit handling (Ctrl+C, etc.)
- [ ] Keep terminal resize handling

#### Update Input Loop
- [ ] Simplify `input_loop()` function
- [ ] Remove action processing
- [ ] Remove mode management
- [ ] Implement direct input forwarding
- [ ] Keep mouse event processing for status bar

### 3. Remove Complex Client Features

#### Remove Configuration Management
- [ ] Remove `Config` parameter from client startup
- [ ] Remove `Options` parameter complexity
- [ ] Remove runtime configuration changes
- [ ] Keep only minimal essential config (if any)

#### Remove Layout Processing
- [ ] Remove `Layout` parameter from client
- [ ] Remove layout-based client setup
- [ ] Remove tab/pane focus parameters
- [ ] Simplify to single shell startup

#### Remove Plugin Communication
- [ ] Remove plugin-related client instructions
- [ ] Remove plugin event forwarding
- [ ] Remove plugin state management
- [ ] Remove plugin message handling

### 4. Update Client Message Types

#### Simplify `ClientToServerMsg` Enum
- [ ] Remove `AttachClient` variant
- [ ] Remove `Action` variant
- [ ] Remove `ClientExited` (keep simple disconnect)
- [ ] Remove `TerminalResize` complexity
- [ ] Remove configuration-related messages
- [ ] Keep only:
  - [ ] `NewClient` (simplified)
  - [ ] `TerminalBytes` (input forwarding)
  - [ ] `TerminalResize`
  - [ ] `ClientExited`

#### Simplify `ServerToClientMsg` Enum
- [ ] Remove session switching messages
- [ ] Remove plugin messages
- [ ] Remove configuration messages
- [ ] Remove web server messages
- [ ] Keep only:
  - [ ] `Render`
  - [ ] `Exit`
  - [ ] `Connected`
  - [ ] `UnblockInputThread`

### 5. Simplify Client Threads

#### Reduce Thread Complexity
- [ ] Keep stdin thread (simplified input forwarding)
- [ ] Keep input thread (basic processing)
- [ ] Keep signal thread (terminal resize, exit)
- [ ] Keep router thread (server message handling)
- [ ] Remove complex thread coordination
- [ ] Remove plugin-related threads

#### Simplify Thread Communication
- [ ] Reduce channel complexity
- [ ] Remove plugin instruction channels
- [ ] Remove complex client instruction channels
- [ ] Keep basic input/output channels

## Testing Checklist

After completing Phase 3:

- [ ] Client connects to server successfully
- [ ] Keyboard input passes through to shell correctly
- [ ] Shell output displays correctly
- [ ] Terminal resizing works
- [ ] Mouse events work for status bar area
- [ ] Exit handling works (Ctrl+C, etc.)
- [ ] No mode switching occurs
- [ ] No complex keybinding processing
- [ ] Terminal setup and cleanup work properly

## Files Modified

- [ ] `zellij-client/src/lib.rs` - Simplified client architecture
- [ ] `zellij-client/src/input_handler.rs` - Basic input passthrough
- [ ] `zellij-client/src/stdin_handler.rs` - Simplified stdin processing
- [ ] `zellij-utils/src/ipc.rs` - Simplified message types
- [ ] `zellij-utils/src/input/` - Removed complex input processing

## Files to Review

Before proceeding to Phase 4:
- [ ] `zellij-client/src/lib.rs`
- [ ] `zellij-client/src/input_handler.rs`
- [ ] `zellij-client/src/stdin_handler.rs`
- [ ] Client-server message definitions

## Success Criteria

Phase 3 is complete when:
1. Client handles single session only
2. Input passes through directly to shell
3. No mode switching or complex keybindings
4. Basic terminal functionality preserved
5. Mouse support maintained for status bar
6. Terminal setup/teardown works correctly
7. Client-server communication simplified

## Notes

- Preserve the basic terminal emulation capabilities
- Keep mouse support for future status bar interaction
- Maintain proper terminal control sequence handling
- Test input/output passthrough thoroughly
- Ensure terminal resizing still works
- Keep exit handling functional
- Don't break ANSI sequence processing
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
