# Phase 4: Create Minimal Status Bar

## Overview
This phase focuses on creating a simple, direct-rendered status bar that displays basic system information in the bottom row of the terminal, without using the complex plugin system.

## Goals
- Create a simple status component that renders directly
- Reserve bottom row of terminal for status display
- Display basic system information (time, shell info, etc.)
- Integrate status rendering into screen output
- Remove dependency on plugin system for status bar
- Keep status bar minimal and configurable

## Detailed Tasks

### 1. Create Simple Status Component

#### Create `zellij-server/src/status_bar.rs`
- [ ] Create new file for status bar logic
- [ ] Define `StatusBar` struct with basic fields:
  - [ ] Terminal width
  - [ ] Current time
  - [ ] Shell information
  - [ ] System load (optional)
- [ ] Implement `new()` constructor
- [ ] Implement `update()` method for refreshing data
- [ ] Implement `render()` method returning formatted string

#### Implement Status Bar Content
- [ ] Add current time display (HH:MM format)
- [ ] Add shell name/type display
- [ ] Add current working directory (truncated if needed)
- [ ] Add basic system info (load average, memory usage - optional)
- [ ] Implement text truncation for narrow terminals
- [ ] Add padding and alignment logic

#### Create Status Bar Styling
- [ ] Define simple color scheme (using ANSI codes)
- [ ] Implement background color for status bar
- [ ] Add text styling (bold, colors)
- [ ] Create separator characters between sections
- [ ] Implement responsive layout for different terminal widths

### 2. Integrate Status Rendering into Screen

#### Modify `zellij-server/src/screen.rs`
- [ ] Add `status_bar: StatusBar` field to `Screen` struct
- [ ] Initialize status bar in `Screen::new()`
- [ ] Reserve bottom row for status bar in terminal calculations
- [ ] Modify terminal size calculations to account for status bar

#### Update Screen Rendering Logic
- [ ] Modify `render()` method to include status bar
- [ ] Ensure terminal pane height is reduced by 1 row
- [ ] Add status bar rendering to screen output
- [ ] Handle terminal resize with status bar considerations
- [ ] Update cursor positioning to account for status bar

#### Implement Status Bar Updates
- [ ] Add periodic status bar refresh (every second for time)
- [ ] Update status bar on terminal resize
- [ ] Update status bar on shell changes (if detectable)
- [ ] Trigger screen re-render when status changes

### 3. Modify Terminal Pane Integration

#### Update `zellij-server/src/panes/terminal_pane.rs`
- [ ] Modify terminal pane to work with reduced height
- [ ] Ensure terminal pane doesn't overwrite status bar
- [ ] Update scroll calculations for status bar space
- [ ] Modify cursor positioning relative to status bar

#### Update Screen Size Calculations
- [ ] Modify `get_terminal_size()` to reserve status bar row
- [ ] Update pane geometry calculations
- [ ] Ensure proper terminal size reporting to shell
- [ ] Handle edge cases with very small terminals

### 4. Add Status Bar Configuration

#### Create Minimal Status Configuration
- [ ] Add status bar enable/disable option
- [ ] Add configurable status bar format
- [ ] Add color customization options
- [ ] Add refresh interval setting
- [ ] Keep configuration minimal and optional

#### Integrate with Existing Config System
- [ ] Add status bar options to `Config` struct
- [ ] Provide sensible defaults
- [ ] Allow runtime configuration changes
- [ ] Handle configuration validation

### 5. Implement Status Bar Data Collection

#### Add System Information Gathering
- [ ] Implement current time retrieval
- [ ] Add shell process information gathering
- [ ] Add current working directory detection
- [ ] Add optional system metrics (load, memory)
- [ ] Handle errors gracefully (show fallback info)

#### Create Data Update Mechanism
- [ ] Implement periodic data refresh
- [ ] Add efficient change detection
- [ ] Minimize system calls and overhead
- [ ] Cache data appropriately

### 6. Handle Status Bar Interactions

#### Add Mouse Support for Status Bar
- [ ] Detect mouse clicks in status bar area
- [ ] Implement basic status bar interactions (optional)
- [ ] Handle mouse events without interfering with terminal
- [ ] Add click-to-copy functionality (optional)

#### Status Bar Responsiveness
- [ ] Handle terminal width changes gracefully
- [ ] Implement text truncation strategies
- [ ] Add scrolling for long content (optional)
- [ ] Ensure status bar works on narrow terminals

## Testing Checklist

After completing Phase 4:

- [ ] Status bar appears at bottom of terminal
- [ ] Status bar displays current time correctly
- [ ] Status bar shows shell information
- [ ] Status bar updates periodically
- [ ] Terminal pane height is correctly reduced
- [ ] Terminal resizing works with status bar
- [ ] Status bar doesn't interfere with shell output
- [ ] Status bar styling renders correctly
- [ ] Mouse clicks in status bar area are handled
- [ ] Status bar works on various terminal sizes

## Files Created

- [ ] `zellij-server/src/status_bar.rs` - Status bar implementation

## Files Modified

- [ ] `zellij-server/src/screen.rs` - Status bar integration
- [ ] `zellij-server/src/panes/terminal_pane.rs` - Height adjustments
- [ ] `zellij-server/src/lib.rs` - Add status bar module
- [ ] `zellij-utils/src/input/config.rs` - Status bar configuration

## Files to Review

Before proceeding to Phase 5:
- [ ] `zellij-server/src/status_bar.rs`
- [ ] `zellij-server/src/screen.rs`
- [ ] Status bar rendering output
- [ ] Terminal size calculations

## Success Criteria

Phase 4 is complete when:
1. Status bar renders correctly at bottom of terminal
2. Status bar displays basic system information
3. Status bar updates periodically without flickering
4. Terminal pane works correctly with reduced height
5. Terminal resizing handles status bar properly
6. Status bar doesn't interfere with shell functionality
7. Status bar is visually appealing and readable


## Optional Enhancements

Consider for future phases:
- [ ] Clickable status bar sections
- [ ] Customizable status bar format
- [ ] Additional system metrics
- [ ] Status bar themes
- [ ] Notification area in status bar

## Notes

- Keep status bar simple and lightweight
- Avoid complex plugin architecture
- Ensure status bar doesn't impact performance
- Test with various terminal sizes and types
- Make status bar optional/configurable
- Consider accessibility (color contrast, etc.)
- Ensure status bar works with different shells
- Test terminal resizing thoroughly
- Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
