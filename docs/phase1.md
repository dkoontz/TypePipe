# Phase 1: Simplify Entry Point and CLI

## Overview
This phase focuses on stripping down the command-line interface and main entry point to remove all multiplexer functionality while preserving basic shell wrapping capabilities.

## Goals
- Remove session management commands
- Remove layout and plugin-related CLI options
- Simplify the main entry point to focus on single shell wrapping
- Clean up command handling to remove complex features

## Detailed Tasks

### 1. Modify `src/main.rs`

#### Remove Session Management Commands
- [ ] Remove `Sessions::ListSessions` handling (lines ~181-187)
- [ ] Remove `Sessions::KillAllSessions` handling (lines ~190-191)
- [ ] Remove `Sessions::KillSession` handling (lines ~192-195)
- [ ] Remove `Sessions::DeleteAllSessions` handling (lines ~196-198)
- [ ] Remove `Sessions::DeleteSession` handling (lines ~199-204)
- [ ] Remove `Sessions::Action` handling (lines ~23-26)
- [ ] Remove `Sessions::Run` handling (lines ~27-67)
- [ ] Remove `Sessions::Plugin` handling (lines ~68-104)
- [ ] Remove `Sessions::Edit` handling (lines ~105-141)

#### Remove Layout and Configuration Commands
- [ ] Remove `Sessions::ConvertConfig` handling (lines ~142-145)
- [ ] Remove `Sessions::ConvertLayout` handling (lines ~146-149)
- [ ] Remove `Sessions::ConvertTheme` handling (lines ~150-153)
- [ ] Remove layout parameter handling (lines ~207-225)
- [ ] Remove `new_session_with_layout` handling (lines ~226-230)

#### Remove Plugin and Web Server Commands
- [ ] Remove `Sessions::Pipe` handling (lines ~154-178)
- [ ] Remove `Command::Web` handling (lines ~231-326)
- [ ] Remove web server related imports and functions

#### Simplify Main Function
- [ ] Keep only basic client startup path
- [ ] Remove complex option parsing
- [ ] Simplify to: parse basic args → start client
- [ ] Remove server path handling (lines ~205-206)

### 2. Simplify `src/commands.rs`

#### Remove Session Management Functions
- [ ] Remove `list_sessions()` function
- [ ] Remove `list_aliases()` function  
- [ ] Remove `kill_all_sessions()` function
- [ ] Remove `kill_session()` function
- [ ] Remove `delete_all_sessions()` function
- [ ] Remove `delete_session()` function
- [ ] Remove `send_action_to_session()` function

#### Remove Layout and Configuration Functions
- [ ] Remove `convert_old_config_file()` function
- [ ] Remove `convert_old_layout_file()` function
- [ ] Remove `convert_old_theme_file()` function

#### Remove Web Server Functions
- [ ] Remove `start_web_server()` function
- [ ] Remove `stop_web_server()` function
- [ ] Remove `web_server_status()` function
- [ ] Remove `create_auth_token()` function
- [ ] Remove `revoke_auth_token()` function
- [ ] Remove `revoke_all_auth_tokens()` function
- [ ] Remove `list_auth_tokens()` function

#### Keep Essential Functions
- [ ] Keep `start_client()` function (simplified)
- [ ] Keep `start_server()` function (simplified)
- [ ] Remove complex client setup options
- [ ] Simplify to basic shell wrapper startup

### 3. Update CLI Argument Structure

#### Modify `zellij-utils/src/cli.rs`
- [ ] Remove session-related CLI arguments
- [ ] Remove layout-related CLI arguments
- [ ] Remove plugin-related CLI arguments
- [ ] Remove web server CLI arguments
- [ ] Keep only basic options:
  - [ ] Debug flag
  - [ ] Config file path (minimal)
  - [ ] Help/version

#### Update Command Enum
- [ ] Remove `Sessions` command variants
- [ ] Remove `Web` command variants
- [ ] Keep only essential commands for shell wrapping

### 4. Clean Up Imports and Dependencies

#### Remove Unused Imports in `src/main.rs`
- [ ] Remove session management imports
- [ ] Remove layout imports
- [ ] Remove plugin imports
- [ ] Remove web server imports
- [ ] Keep only client startup imports

#### Remove Unused Imports in `src/commands.rs`
- [ ] Remove session management imports
- [ ] Remove layout conversion imports
- [ ] Remove web server imports
- [ ] Keep only basic client/server startup imports

## Testing Checklist

After completing Phase 1:

- [ ] Project compiles without errors
- [ ] Basic `typey-pipe` command starts a shell wrapper
- [ ] Help text shows simplified options only
- [ ] No session management commands are available
- [ ] No layout or plugin options are present
- [ ] Shell input/output passes through correctly
- [ ] Terminal setup and teardown work properly

## Files Modified

- [ ] `src/main.rs` - Simplified main entry point
- [ ] `src/commands.rs` - Removed complex command handling
- [ ] `zellij-utils/src/cli.rs` - Simplified CLI arguments

## Files to Review

Before proceeding to Phase 2, ensure these files compile and function:
- [ ] `src/main.rs`
- [ ] `src/commands.rs`
- [ ] `zellij-utils/src/cli.rs`

## Success Criteria

Phase 1 is complete when:
1. The CLI is simplified to basic shell wrapper functionality
2. All session management commands are removed
3. Layout and plugin commands are removed
4. The application compiles and can start a basic shell wrapper
5. No complex multiplexer features are accessible via CLI

## Notes

- Keep the client-server architecture intact for now
- Don't modify the actual client/server implementation yet
- Focus only on the entry point and command parsing
- Preserve the basic shell spawning capability
- Test frequently to ensure the basic shell wrapper still works
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
