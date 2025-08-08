# AGENTS.md - Development Guide for Typey Pipe

## Project Overview

Typey Pipe is a transparent shell wrapper that provides advanced asynchronous messaging capabilities from external processes. It acts as an invisible layer around your shell, maintaining full interactive access while enabling programmatic control through a file-based message queue system.

We're not building a terminal emulator, we're building a transparent shell proxy that needs just enough ANSI handling to not break existing tools.

## Tool Use

The `rust-analyzer` and `serena` MCP servers contain tools for working with code files. Use them instead of `ls`, `grep`, `sed`, `awk` to find files and symbols in the project. Utilize their refactoring tools instead of manually finding and replacing lines of text. Since this is a Rust project prefer using `rust-analyzer` and fall back to `serena` for tools rust-analyzer lacks.

## Testing

Be sure to clean up any temporary test_* files that you create when troubleshooting once you have implemented your fix.

## Working on Tasks

When you are working off of a story or planning document, be sure to mark off the items you are completing in the file as you go. Do not wait until the very end and mark all the items at once. The document should reflect your actual status in case there is a crash or other problem and we need to resume mid-process.
