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
- [x] Remove `Sessions::ListSessions` handling (lines ~181-187)
- [x] Remove `Sessions::KillAllSessions` handling (lines ~190-191)
- [x] Remove `Sessions::KillSession` handling (lines ~192-195)
- [x] Remove `Sessions::DeleteAllSessions` handling (lines ~196-198)
- [x] Remove `Sessions::DeleteSession` handling (lines ~199-204)
- [x] Remove `Sessions::Action` handling (lines ~23-26)
- [x] Remove `Sessions::Run` handling (lines ~27-67)
- [x] Remove `Sessions::Plugin` handling (lines ~68-104)
- [x] Remove `Sessions::Edit` handling (lines ~105-141)

#### Remove Layout and Configuration Commands
- [x] Remove `Sessions::ConvertConfig` handling (lines ~142-145)
- [x] Remove `Sessions::ConvertLayout` handling (lines ~146-149)
- [x] Remove `Sessions::ConvertTheme` handling (lines ~150-153)
- [x] Remove layout parameter handling (lines ~207-225)
- [x] Remove `new_session_with_layout` handling (lines ~226-230)

#### Remove Plugin and Web Server Commands
- [x] Remove `Sessions::Pipe` handling (lines ~154-178)
- [x] Remove `Command::Web` handling (lines ~231-326)
- [x] Remove web server related imports and functions

#### Simplify Main Function
- [x] Keep only basic client startup path
- [x] Remove complex option parsing
- [x] Simplify to: parse basic args → start client
- [x] Remove server path handling (lines ~205-206)

### 2. Simplify `src/commands.rs`

#### Remove Session Management Functions
- [x] Remove `list_sessions()` function
- [x] Remove `list_aliases()` function  
- [x] Remove `kill_all_sessions()` function
- [x] Remove `kill_session()` function
- [x] Remove `delete_all_sessions()` function
- [x] Remove `delete_session()` function
- [x] Remove `send_action_to_session()` function

#### Remove Layout and Configuration Functions
- [x] Remove `convert_old_config_file()` function
- [x] Remove `convert_old_layout_file()` function
- [x] Remove `convert_old_theme_file()` function

#### Remove Web Server Functions
- [x] Remove `start_web_server()` function
- [x] Remove `stop_web_server()` function
- [x] Remove `web_server_status()` function
- [x] Remove `create_auth_token()` function
- [x] Remove `revoke_auth_token()` function
- [x] Remove `revoke_all_auth_tokens()` function
- [x] Remove `list_auth_tokens()` function

#### Keep Essential Functions
- [x] Keep `start_client()` function (simplified)
- [x] Keep `start_server()` function (simplified)
- [x] Remove complex client setup options
- [x] Simplify to basic shell wrapper startup

### 3. Update CLI Argument Structure

#### Modify `zellij-utils/src/cli.rs`
- [x] Remove session-related CLI arguments
- [x] Remove layout-related CLI arguments
- [x] Remove plugin-related CLI arguments
- [x] Remove web server CLI arguments
- [x] Keep only basic options:
  - [x] Debug flag
  - [x] Config file path (minimal)
  - [x] Help/version

#### Update Command Enum
- [x] Remove `Sessions` command variants
- [x] Remove `Web` command variants
- [x] Keep only essential commands for shell wrapping

### 4. Clean Up Imports and Dependencies

#### Remove Unused Imports in `src/main.rs`
- [x] Remove session management imports
- [x] Remove layout imports
- [x] Remove plugin imports
- [x] Remove web server imports
- [x] Keep only client startup imports

#### Remove Unused Imports in `src/commands.rs`
- [x] Remove session management imports
- [x] Remove layout conversion imports
- [x] Remove web server imports
- [x] Keep only basic client/server startup imports

## Testing Checklist

After completing Phase 1:

- [x] Project compiles without errors
- [x] Basic `typey-pipe` command shows correct help and version
- [x] Help text shows simplified options only
- [x] No session management commands are available
- [x] No layout or plugin options are present
- [ ] Shell input/output passes through correctly (requires Phase 2+)
- [ ] Terminal setup and teardown work properly (requires Phase 2+)

## Files Modified

- [x] `src/main.rs` - Simplified main entry point
- [x] `src/commands.rs` - Removed complex command handling
- [x] `zellij-utils/src/cli.rs` - Simplified CLI arguments

## Files to Review

Before proceeding to Phase 2, ensure these files compile and function:
- [x] `src/main.rs`
- [x] `src/commands.rs`
- [x] `zellij-utils/src/cli.rs`

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

### Completed Tasks

**Main Entry Point Simplification (`src/main.rs`)**
- Removed all session management commands (ListSessions, KillAllSessions, KillSession, DeleteAllSessions, DeleteSession, Action, Run, Plugin, Edit)
- Removed layout and configuration commands (ConvertConfig, ConvertLayout, ConvertTheme, layout parameter handling, new_session_with_layout)
- Removed plugin and web server commands (Pipe, Web command handling)
- Simplified main function to basic client startup path: parse args → start client or server
- Reduced from ~330 lines to ~30 lines

**Command Handling Simplification (`src/commands.rs`)**
- Removed all session management functions (kill_all_sessions, delete_all_sessions, kill_session, delete_session, send_action_to_session, list_aliases)
- Removed layout conversion functions (convert_old_config_file, convert_old_layout_file, convert_old_theme_file)
- Removed web server functions (start_web_server, stop_web_server, create_auth_token, revoke_auth_token, revoke_all_auth_tokens, list_auth_tokens, web_server_status)
- Simplified start_client function to basic shell wrapper startup
- Kept essential functions: start_server, start_client, generate_unique_session_name_or_exit

**CLI Structure Simplification (`zellij-utils/src/cli.rs`)**
- Removed complex CLI arguments: max_panes, data_dir, layout, new_session_with_layout
- Removed Command enum and all subcommands (Sessions, Web, Options, Setup)
- Removed WebCli, SessionCommand, Sessions, and CliAction enums
- Simplified CliArgs to basic options: server, session, config, config_dir, debug
- Changed application name from "zellij" to "typey-pipe"

**Additional Changes**
- Removed plugin asset map from consts.rs (no longer includes WASM plugins)
- Simplified actions_from_cli function to return error (CliAction functionality removed)
- Fixed import issues and removed unused dependencies
- Removed tests that depended on removed functionality
- Fixed compilation errors in server and client code related to removed fields

### Issues Encountered and Resolved

1. **Plugin Asset Loading**: The original code tried to include WASM plugin files at compile time. Replaced with empty asset map since plugins are not needed for shell wrapper.

2. **Complex CLI Action System**: The CliAction enum and actions_from_cli function were heavily integrated throughout the codebase. Replaced with stub that returns error message.

3. **Interdependent Configuration**: Layout, session, and plugin systems were tightly coupled. Simplified by removing layout handling and using basic configuration.

4. **Test Dependencies**: Many tests depended on removed functionality. Removed tests that referenced removed CLI fields and commands.

5. **Import Cleanup**: Removed numerous unused imports after simplifying functionality.

### Current State

The project now compiles successfully with only warnings about unused imports and functions. The core shell wrapper functionality is preserved while all multiplexer features have been removed. The application can be built and should start as a basic shell wrapper.


## QA Notes (to be filled in by QA agent)

### QA Review Summary

**Overall Assessment: PASSED with Minor Issues**

The developer has successfully completed Phase 1 requirements. The CLI has been simplified, session management commands removed, and the application compiles and runs correctly.

### Verification Results

#### ✅ Requirements Met

1. **Main Entry Point Simplification (`zellij/src/main.rs`)**
   - Successfully reduced from ~330 lines to 23 lines
   - Removed all session management, layout, plugin, and web server command handling
   - Simplified to basic client/server startup path
   - Clean, minimal implementation

2. **Command Handling Simplification (`zellij/src/commands.rs`)**
   - Removed all session management functions as required
   - Removed layout conversion functions
   - Removed web server functions
   - Kept essential functions: `start_server`, `start_client`, `generate_unique_session_name_or_exit`

3. **CLI Structure Simplification (`zellij/zellij-utils/src/cli.rs`)**
   - Successfully removed complex CLI arguments and command enums
   - Application name changed from "zellij" to "typey-pipe" ✅
   - Simplified to basic options: server, session, config, config_dir, debug
   - Clean, minimal CLI structure

4. **Compilation and Basic Functionality**
   - Project compiles successfully ✅
   - Help output shows simplified options only ✅
   - Version output shows "typey-pipe 0.44.0" ✅
   - No session management or complex commands available ✅

5. **Plugin Asset Cleanup**
   - ASSET_MAP properly emptied with comment explaining removal ✅
   - Plugin loading infrastructure disabled

### ⚠️ Minor Issues Found

1. **Unused Import Warnings**
   - Multiple unused imports in `zellij-utils/src/input/actions.rs` (Layout, PluginAlias, RunPlugin, etc.)
   - Unused imports in `zellij-utils/src/setup.rs`
   - These are warnings, not errors, but should be cleaned up

2. **Dead Code Warnings**
   - Functions `reload_config_from_disk` and `get_config_options_from_cli_args` in `src/commands.rs` are unused
   - Should be removed or marked with `#[allow(dead_code)]` if needed for future phases

3. **Redundant Code**
   - Unnecessary trailing semicolon in `zellij-utils/src/setup.rs:411`
   - Unused variable `cli_args` in setup.rs

### ✅ Issues Addressed

**Priority: Low (Cleanup) - COMPLETED**
1. ✅ Removed unused imports in `zellij-utils/src/input/actions.rs`:
   - `Layout`, `PluginAlias`, `RunPlugin`, `RunPluginLocation`
   - `find_default_config_dir`, `get_layout_dir`
   - `ConfigError`, `KdlError`
   - `NamedSource`, `Report`
   - `uuid::Uuid`

2. ✅ Removed unused imports in `zellij-utils/src/setup.rs`:
   - `process`

3. ✅ Removed dead code in `src/commands.rs`:
   - `reload_config_from_disk` function (was only used by removed web client)
   - `get_config_options_from_cli_args` function (unused)

4. ✅ Fixed minor syntax issues:
   - Removed trailing semicolon in `zellij-utils/src/setup.rs:411`
   - Prefixed unused variables with underscore: `_cli_args`

### ✅ Success Criteria Verification

All Phase 1 success criteria have been met:
1. ✅ CLI simplified to basic shell wrapper functionality
2. ✅ All session management commands removed
3. ✅ Layout and plugin commands removed  
4. ✅ Application compiles and can start basic shell wrapper
5. ✅ No complex multiplexer features accessible via CLI

### 🎯 Final Recommendation

**APPROVED** for Phase 2 progression. All QA issues have been resolved:

- ✅ All unused imports removed
- ✅ Dead code functions removed  
- ✅ Minor syntax issues fixed
- ✅ Project compiles cleanly without warnings
- ✅ Core functionality preserved and simplified

Phase 1 is now complete and ready for Phase 2 development.
