# Phase 2: Strip Server Architecture

## Overview
This phase focuses on simplifying the server-side architecture by removing multi-client support, session metadata, plugin threads, and complex coordination while preserving the core PTY and screen management.

## Goals
- Remove multi-client support (support single client only)
- Remove session metadata and complex state management
- Remove plugin thread initialization and coordination
- Remove background jobs thread
- Simplify screen management to single pane with status footer
- Maintain core PTY and terminal handling

## Detailed Tasks

### 1. Simplify `typeypipe/zellij-server/src/lib.rs`

#### Remove Multi-Client Support
- [ ] Modify `SessionState` struct to support single client only
- [ ] Remove `clients: HashMap<ClientId, Option<(Size, bool)>>` - use single client
- [ ] Remove `pipes: HashMap<String, ClientId>` - no pipe support needed
- [ ] Simplify `new_client()` to return fixed client ID (1)
- [ ] Remove `associate_pipe_with_client()` method
- [ ] Remove `web_client_ids()` method
- [ ] Remove `get_pipe()` method
- [ ] Remove `active_clients_are_connected()` method

#### Remove Session Metadata Complexity
- [ ] Remove `SessionConfiguration` struct entirely
- [ ] Remove `current_input_modes: HashMap<ClientId, InputMode>`
- [ ] Remove `session_configuration: SessionConfiguration`
- [ ] Remove `web_sharing: WebSharing` field
- [ ] Simplify `SessionMetaData` to essential fields only
- [ ] Remove configuration change propagation methods

#### Remove Plugin Thread Infrastructure
- [ ] Remove `plugin_thread: Option<thread::JoinHandle<()>>` from `SessionMetaData`
- [ ] Remove plugin thread spawning in `init_session()`
- [ ] Remove `to_plugin` channel creation
- [ ] Remove `PluginInstruction` message handling
- [ ] Remove plugin-related imports

#### Remove Background Jobs Thread
- [ ] Remove `background_jobs_thread: Option<thread::JoinHandle<()>>` from `SessionMetaData`
- [ ] Remove background jobs thread spawning
- [ ] Remove `to_background_jobs` channel creation
- [ ] Remove `BackgroundJob` message handling
- [ ] Remove background jobs imports

#### Simplify Server Main Loop
- [ ] Remove `ServerInstruction::NewClient` complex handling
- [ ] Remove `ServerInstruction::AttachClient` (single client only)
- [ ] Remove `ServerInstruction::RemoveClient` multi-client logic
- [ ] Remove `ServerInstruction::DetachSession`
- [ ] Remove `ServerInstruction::SwitchSession`
- [ ] Remove `ServerInstruction::AssociatePipeWithClient`
- [ ] Remove `ServerInstruction::DisconnectAllClientsExcept`
- [ ] Remove web server related instructions
- [ ] Remove configuration change instructions
- [ ] Simplify to: client connect, render, client disconnect, exit

### 2. Modify `typeypipe/zellij-server/src/screen.rs`

#### Remove Tab Management
- [ ] Remove `tabs: BTreeMap<usize, Tab>` field from `Screen`
- [ ] Remove `active_tab_indices: HashMap<ClientId, usize>`
- [ ] Remove `tab_history: HashMap<ClientId, Vec<usize>>`
- [ ] Remove all tab-related methods:
  - [ ] `new_tab()`
  - [ ] `switch_tab_to()`
  - [ ] `switch_tab_next()`
  - [ ] `switch_tab_prev()`
  - [ ] `close_tab()`
  - [ ] `get_active_tab()`
  - [ ] `get_active_tab_mut()`

#### Remove Multi-Pane Support
- [ ] Replace tab system with single `TerminalPane`
- [ ] Remove pane splitting logic
- [ ] Remove pane resizing coordination
- [ ] Remove pane boundaries and frames
- [ ] Remove floating pane support
- [ ] Remove stacked pane support
- [ ] Keep only single terminal pane + status area

#### Simplify Screen Structure
- [ ] Replace complex screen state with:
  - [ ] Single `terminal_pane: TerminalPane`
  - [ ] Status bar area (reserve bottom row)
  - [ ] Basic client information
  - [ ] Terminal size information
- [ ] Remove layout processing entirely
- [ ] Remove pane group management

#### Remove Plugin Coordination
- [ ] Remove all `PluginInstruction` sending
- [ ] Remove plugin event handling
- [ ] Remove plugin render coordination
- [ ] Remove plugin pane management

#### Simplify Screen Instructions
- [ ] Keep only essential `ScreenInstruction` variants:
  - [ ] `Render`
  - [ ] `WriteCharacter`
  - [ ] `Resize`
  - [ ] `Exit`
- [ ] Remove tab-related instructions
- [ ] Remove pane-related instructions
- [ ] Remove plugin-related instructions
- [ ] Remove layout-related instructions

### 3. Simplify PTY Management

#### Modify `typeypipe/zellij-server/src/pty.rs`
- [ ] Remove `PtyInstruction::SpawnTerminal` multi-pane support
- [ ] Remove `PtyInstruction::NewTab`
- [ ] Remove `PtyInstruction::ClosePane`
- [ ] Remove `PtyInstruction::SpawnInPlace`
- [ ] Keep only single shell PTY management:
  - [ ] `SpawnTerminal` (single instance)
  - [ ] `WriteToTerminal`
  - [ ] `Exit`

#### Simplify PTY State
- [ ] Remove `terminals: HashMap<RawFd, TerminalBytes>`
- [ ] Use single terminal state
- [ ] Remove pane ID mapping
- [ ] Remove terminal multiplexing logic

#### Remove Complex PTY Features
- [ ] Remove `NewPanePlacement` enum complexity
- [ ] Remove `ClientTabIndexOrPaneId` enum
- [ ] Remove layout-based terminal spawning
- [ ] Keep simple shell process spawning

### 4. Update Message Types

#### Simplify `ServerInstruction` Enum
- [ ] Remove session management instructions
- [ ] Remove multi-client instructions
- [ ] Remove plugin instructions
- [ ] Remove web server instructions
- [ ] Remove configuration instructions
- [ ] Keep only:
  - [ ] `NewClient` (simplified)
  - [ ] `Render`
  - [ ] `ClientExit`
  - [ ] `Error`
  - [ ] `KillSession`

#### Simplify `ScreenInstruction` Enum
- [ ] Remove tab instructions
- [ ] Remove pane instructions
- [ ] Remove plugin instructions
- [ ] Remove layout instructions
- [ ] Keep only:
  - [ ] `WriteCharacter`
  - [ ] `Render`
  - [ ] `Resize`
  - [ ] `Exit`

#### Simplify `PtyInstruction` Enum
- [ ] Remove multi-terminal instructions
- [ ] Keep only:
  - [ ] `SpawnTerminal`
  - [ ] `WriteToTerminal`
  - [ ] `Exit`

## Testing Checklist

After completing Phase 2:

- [ ] Server starts with single client support
- [ ] Single shell process spawns correctly
- [ ] Terminal input/output works
- [ ] Screen renders single pane correctly
- [ ] No plugin threads are created
- [ ] No background job threads are created
- [ ] Terminal resizing works
- [ ] Client can connect and disconnect cleanly

## Files Modified

- [ ] `typeypipe/zellij-server/src/lib.rs` - Simplified server architecture
- [ ] `typeypipe/zellij-server/src/screen.rs` - Single pane screen management
- [ ] `typeypipe/zellij-server/src/pty.rs` - Single shell PTY management
- [ ] `typeypipe/zellij-server/src/route.rs` - Simplified message routing
- [ ] `typeypipe/zellij-utils/src/ipc.rs` - Simplified message types

## Files to Review

Before proceeding to Phase 3:
- [ ] `typeypipe/zellij-server/src/lib.rs`
- [ ] `typeypipe/zellij-server/src/screen.rs`
- [ ] `typeypipe/zellij-server/src/pty.rs`
- [ ] Message type definitions

## Success Criteria

Phase 2 is complete when:
1. Server supports single client only
2. Single terminal pane with status area renders correctly
3. No plugin or background job threads are created
4. PTY management handles single shell process
5. Basic terminal functionality works (input/output, resize)
6. Complex multiplexer features are removed from server

## Notes

- Preserve the basic client-server communication
- Keep terminal character handling and ANSI processing intact
- Maintain proper terminal setup and cleanup
- Reserve space for status bar (bottom row)
- Test terminal functionality frequently
- Don't break the basic shell wrapping capability
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
