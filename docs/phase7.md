# Phase 7: Clean Up Dependencies

## Overview
This final phase focuses on removing unused dependencies, simplifying the workspace structure, and cleaning up the build system to create a lean, focused codebase for the transparent shell wrapper.

## Goals
- Remove unused crate dependencies
- Simplify workspace structure to essential components only
- Clean up build scripts and compilation features
- Reduce binary size and build time
- Remove development and testing dependencies that are no longer needed
- Optimize remaining dependencies for the simplified use case

## Detailed Tasks

### 1. Remove Unused Crate Dependencies

#### Clean Up Root Cargo.toml Dependencies
- [ ] Remove `wasmtime` and all WASM-related dependencies
- [ ] Remove `dialoguer` (used for interactive prompts)
- [ ] Remove `names` (used for session name generation)
- [ ] Remove `suggest` (used for command suggestions)
- [ ] Remove `isahc` (used for web server functionality)
- [ ] Remove `humantime` if not essential
- [ ] Remove `interprocess` if simplified IPC is sufficient

#### Clean Up Workspace Dependencies
- [ ] Remove `async-std` (if not needed for simplified architecture)
- [ ] Remove `notify-debouncer-full` and `notify` (file watching)
- [ ] Remove `prost` (protobuf, if not used)
- [ ] Remove `url` (if web functionality removed)
- [ ] Remove `uuid` (if session management removed)
- [ ] Remove `rmp-serde` (MessagePack serialization)
- [ ] Remove `sha2` (if not needed for simplified use)
- [ ] Remove `include_dir` (if asset embedding not needed)

#### Review and Keep Essential Dependencies
- [ ] Keep `ansi_term` for terminal color handling
- [ ] Keep `anyhow` for error handling
- [ ] Keep `clap` for CLI parsing (simplified)
- [ ] Keep `log` for logging
- [ ] Keep `nix` for Unix system calls
- [ ] Keep `serde` and `serde_json` for basic serialization
- [ ] Keep `signal-hook` for signal handling
- [ ] Keep `termwiz` for terminal handling
- [ ] Keep `thiserror` for error types
- [ ] Keep `vte` for terminal emulation
- [ ] Keep `unicode-width` for text handling

### 2. Simplify Workspace Structure

#### Update Workspace Members
- [ ] Remove all `default-plugins/*` entries
- [ ] Remove `zellij-tile` entry
- [ ] Remove `zellij-tile-utils` entry
- [ ] Remove `xtask` entry (if build tasks simplified)
- [ ] Keep only essential workspace members:
  - [ ] `.` (main binary)
  - [ ] `zellij-client`
  - [ ] `zellij-server`
  - [ ] `zellij-utils`

#### Clean Up Workspace Configuration
- [ ] Remove plugin-related workspace dependencies
- [ ] Remove WASM-related workspace dependencies
- [ ] Remove complex UI dependencies
- [ ] Remove session serialization dependencies
- [ ] Keep only dependencies used by remaining workspace members

### 3. Remove Complex UI Dependencies

#### Remove Advanced Terminal UI Dependencies
- [ ] Remove complex rendering dependencies
- [ ] Remove advanced color handling (keep basic ANSI)
- [ ] Remove complex layout rendering dependencies
- [ ] Remove mouse handling complexity (keep basic support)
- [ ] Remove advanced terminal feature dependencies

#### Simplify Terminal Handling
- [ ] Keep essential terminal emulation (`vte`)
- [ ] Keep basic terminal control (`termwiz`)
- [ ] Remove advanced terminal features
- [ ] Remove complex terminal capability detection
- [ ] Keep basic ANSI sequence handling

### 4. Remove Session and State Management Dependencies

#### Remove Session Persistence Dependencies
- [ ] Remove session serialization dependencies
- [ ] Remove state persistence dependencies
- [ ] Remove session metadata dependencies
- [ ] Remove session sharing dependencies
- [ ] Remove session synchronization dependencies

#### Remove Complex State Management
- [ ] Remove complex data structure dependencies
- [ ] Remove advanced synchronization primitives
- [ ] Remove distributed state dependencies
- [ ] Keep only basic state management for single session

### 5. Clean Up Build System

#### Remove Complex Build Features
- [ ] Remove plugin compilation features
- [ ] Remove WASM compilation features
- [ ] Remove web server compilation features
- [ ] Remove complex asset bundling
- [ ] Remove advanced optimization features

#### Simplify Build Configuration
- [ ] Remove plugin-related build scripts
- [ ] Remove asset generation scripts
- [ ] Remove complex feature flags
- [ ] Keep only essential build configuration
- [ ] Simplify release profile settings

#### Update Feature Flags
- [ ] Remove `plugins_from_target` feature
- [ ] Remove `disable_automatic_asset_installation` feature
- [ ] Remove `vendored_curl` feature
- [ ] Remove `unstable` feature
- [ ] Remove `singlepass` feature
- [ ] Remove `web_server_capability` feature
- [ ] Keep only essential features (if any)

### 6. Remove Development Dependencies

#### Clean Up Dev Dependencies
- [ ] Remove `insta` (snapshot testing)
- [ ] Remove `ssh2` (SSH testing)
- [ ] Remove `rand` (random testing)
- [ ] Remove `regex` (if not used in main code)
- [ ] Remove plugin development dependencies
- [ ] Keep only essential testing dependencies

#### Remove Testing Infrastructure
- [ ] Remove plugin testing infrastructure
- [ ] Remove complex integration tests
- [ ] Remove session management tests
- [ ] Remove layout testing
- [ ] Keep only basic functionality tests

### 7. Optimize Remaining Dependencies

#### Review Dependency Versions
- [ ] Update remaining dependencies to latest stable versions
- [ ] Remove version constraints that were plugin-specific
- [ ] Optimize dependency features (disable unused features)
- [ ] Remove transitive dependencies that are no longer needed

#### Minimize Dependency Features
- [ ] Review each dependency's features
- [ ] Disable unused features to reduce compilation time
- [ ] Enable only essential features
- [ ] Document why each dependency is needed

### 8. Clean Up Package Metadata

#### Update Package Information
- [ ] Update package description to reflect shell wrapper purpose
- [ ] Remove plugin-related keywords
- [ ] Remove multiplexer-related keywords
- [ ] Update repository and homepage URLs
- [ ] Update license and author information

#### Clean Up Package Assets
- [ ] Remove plugin assets from package
- [ ] Remove complex layout examples
- [ ] Remove theme assets
- [ ] Remove documentation for removed features
- [ ] Keep only essential assets

### 9. Verify Dependency Cleanup

#### Test Compilation
- [ ] Ensure project compiles with reduced dependencies
- [ ] Verify no missing dependency errors
- [ ] Check that all features still work
- [ ] Test debug and release builds
- [ ] Verify cross-compilation still works (if needed)

#### Measure Improvements
- [ ] Measure compilation time improvement
- [ ] Measure binary size reduction
- [ ] Measure dependency count reduction
- [ ] Measure build cache size reduction
- [ ] Document performance improvements

## Testing Checklist

After completing Phase 7:

- [ ] Project compiles successfully with reduced dependencies
- [ ] All essential functionality works correctly
- [ ] Binary size is significantly reduced
- [ ] Compilation time is improved
- [ ] No unused dependencies remain
- [ ] Workspace structure is clean and minimal
- [ ] Build system is simplified
- [ ] Package metadata is accurate

## Files Modified

- [ ] `typeypipe/Cargo.toml` - Removed unused dependencies and workspace members
- [ ] `typeypipe/zellij-client/Cargo.toml` - Cleaned up client dependencies
- [ ] `typeypipe/zellij-server/Cargo.toml` - Cleaned up server dependencies
- [ ] `typeypipe/zellij-utils/Cargo.toml` - Cleaned up utils dependencies
- [ ] Build scripts and CI configuration
- [ ] Package metadata and documentation

## Files to Review

Final review of the entire project:
- [ ] All `Cargo.toml` files
- [ ] Build configuration
- [ ] CI/CD configuration
- [ ] Documentation
- [ ] README files

## Success Criteria

Phase 7 is complete when:
1. All unused dependencies are removed
2. Workspace contains only essential components
3. Build system is simplified and optimized
4. Binary size is significantly reduced
5. Compilation time is improved
6. No unused code or dependencies remain
7. Package metadata accurately reflects the simplified project
8. All functionality still works correctly

## Final Verification

### Dependency Audit
- [ ] Run `cargo tree` to verify dependency tree is minimal
- [ ] Run `cargo audit` to check for security issues
- [ ] Run `cargo bloat` to analyze binary size
- [ ] Run `cargo unused-features` to find unused features

### Functionality Test
- [ ] Test basic shell wrapper functionality
- [ ] Test terminal input/output passthrough
- [ ] Test terminal resizing
- [ ] Test status bar (if implemented)
- [ ] Test exit handling
- [ ] Test error handling

### Performance Measurement
- [ ] Measure startup time
- [ ] Measure memory usage
- [ ] Measure CPU usage during normal operation
- [ ] Compare with original Zellij metrics
- [ ] Document performance improvements

## Expected Outcomes

After completing all 7 phases:

### Codebase Reduction
- [ ] ~80% reduction in lines of code
- [ ] ~70% reduction in dependencies
- [ ] ~60% reduction in binary size
- [ ] ~50% reduction in compilation time

### Simplified Architecture
- [ ] Single shell process management
- [ ] Direct input/output passthrough
- [ ] Minimal status bar display
- [ ] Clean client-server architecture
- [ ] No complex multiplexer features

### Ready for Extension
- [ ] Clean foundation for queue processing system
- [ ] Minimal, focused codebase
- [ ] Clear separation of concerns
- [ ] Easy to understand and modify
- [ ] Well-documented simplified architecture

## Notes

- This is the final phase - be thorough in cleanup
- Test extensively to ensure no functionality is broken
- Document all changes and improvements
- Consider creating migration guide from original Zellij
- Prepare codebase for queue processing integration
- Ensure code is well-documented for future development

## Post-Phase 7 Next Steps

After completing Phase 7:
1. Create comprehensive documentation for the simplified architecture
2. Set up testing infrastructure for the shell wrapper
3. Create integration points for queue processing system
4. Establish development workflow for the new codebase
5. Plan queue processing system integration
6. Use the Implementation Notes section to communicate any problems or blockers you encounter.

## Implementation Notes (to be filled in by developer agent)


## QA Notes (to be filled in by QA agent)
