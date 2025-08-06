# Typey Pipe

A simple wrapper with PTY support that provides transparent shell access with advanced asynchronous messaging from external processes.

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap dkoontz/typeypipe
brew install typeypipe
```

### Manual Installation

Download the latest release for your platform from [GitHub Releases](https://github.com/dkoontz/TypeyPipe/releases/latest):


## Features

### 🚀 **Transparent Shell Integration**
Typey Pipe acts as an invisible wrapper around your shell, providing full interactive access while adding programmatic control capabilities. All your existing shell features, aliases, and workflows continue to work exactly as before.

### 📨 **File-Based Message Queue**
Send commands to any running shell instance by simply writing files to a directory. Any process that can create a file can control the shell - no complex APIs or network protocols required.

```bash
# From any process, send a command:
echo "ls -la" > .tp/myshell/command.txt
```

### 🔄 **Asynchronous Command Processing**
Commands are processed asynchronously without blocking your interactive session. The queue system intelligently pauses when you're actively typing to prevent conflicts.

### 🛠 **Perfect for Automation & Orchestration**
- **Agent-based systems**: AI agents can control shell environments
- **Deployment scripts**: Automated workflows can interact with shells
- **Monitoring systems**: Send alerts and commands based on system events  
- **Development workflows**: Hot-reload, testing, and build automation
- **Multi-process coordination**: Orchestrate complex multi-step operations

### 📡 **Cross-Platform PTY Support**
Built on portable-pty for reliable cross-platform terminal handling on Linux, macOS, and Windows with full feature parity.

### 🔒 **Safe Concurrent Access**
Multiple processes can safely send commands simultaneously. The system uses proper locking and atomic file operations to prevent conflicts.

### 🎯 **Unix Philosophy Compliant**
- **Do one thing well**: Provides shell access + message queue
- **Work with other tools**: Integrates seamlessly with existing workflows
- **Text-based interfaces**: Simple file-based communication protocol


## Usage

### Basic Usage
```bash
# Run with default bash shell and PID for name
typeypipe

# Use a different shell
typeypipe --shell /bin/zsh

# Custom name for queue directory and log file
typeypipe --queue-dir my-custom-name
```

### Command Line Options
```
-s, --shell <SHELL>            Shell to use (default: /bin/bash)
-q, --queue-dir <NAME>         Queue directory name under .tp/ directory (default: process ID)
-t, --input-timeout <SECONDS>  Seconds to wait after user input before resuming queue processing (default: 30)
-u, --quiet                    Suppress startup messages
-h, --help                     Print help
-V, --version                  Print version
```

### Key Bindings
- **Ctrl+C**: Interrupt/cancel running processes in shell (passes through to shell)
- **All other keys**: Pass through directly to shell with full terminal feature support
- **Arrow keys, function keys**: Full support for command history, tab completion, etc.
- **Exit**: Use standard shell exit commands (`exit`, `logout`) or Ctrl+D

## Programmatic Command Queue

Typey Pipe supports programmatic command input through a file-based queue system. External processes can send commands by writing files to a queue directory.

### Queue Directory Structure

Each shell instance creates its own queue directory inside the `.tp` directory that is created wherever you ran Typey Pipe from:
- **Default**: `.tp/<process-id>/` (uses process ID for uniqueness)
- **Named**: `.tp/<custom-name>/` (when using `--queue-dir`)

### Sending Commands

External processes send commands using this HIGHLY TECHNICAL PROCESS:

1. **Write command to temporary file**
2. **Move file to queue directory**

Advanced users can take advantage of this even more advanced process:

1. **Write a command directly to a file in the queue directory**

```bash
# Basic example - echo adds newline (executes command)
echo "ls -la" > cmd.txt
mv cmd.txt .tp/12345/  # Replace 12345 with actual process ID

# Multi-line commands work naturally, file extensions are totally optional but they do keep from accidentily colliding with commands
cat << EOF > script
cd /tmp
ls -la
pwd
echo "Done!"
EOF
mv script .tp/webapp/
```

### Command Formatting

**Important:** The queue system sends file contents exactly as stored. Understanding newline behavior is crucial:

```bash
# echo adds a newline - command executes immediately
echo "pwd" > cmd.txt
# File contains: "pwd\n" - shell executes and shows result

# printf without newline - text appears at prompt but doesn't execute  
printf "pwd" > cmd.txt  
# File contains: "pwd" - appears at shell prompt, waits for Enter

# echo -n suppresses newline (same as printf)
echo -n "pwd" > cmd.txt
# File contains: "pwd" - appears at shell prompt, waits for Enter
```

**For most use cases, you want `echo` (with newline) to execute commands immediately.**

### Advanced Use Cases

#### Multiple Shell Instances
```bash
# Terminal 1: Start webapp shell
typeypipe --queue-dir webapp --shell /bin/zsh

# Terminal 2: Start background processing shell  
typeypipe --queue-dir background --shell /bin/bash

# Terminal 3: Send commands to specific shells
echo "cd /var/www && ls -la" > temp-cmd.txt && mv temp-cmd.txt .tp/webapp/
echo "cd /var/log && tail -f app.log" > temp-cmd.txt && mv temp-cmd.txt .tp/background/
```

#### Cron Job Integration
```bash
# In crontab: */5 * * * * /path/to/monitor.sh
# monitor.sh
if [ -f /tmp/error.log ]; then
    echo "echo 'Error detected at $(date)'" > temp-alert.txt
    mv temp-alert.txt .tp/alerts/
    echo "tail -20 /tmp/error.log" > temp-alert.txt
    mv temp-alert.txt .tp/alerts/
fi
```

#### Development Workflow
```bash
# watch.sh - monitors file changes and runs tests
inotifywait -m -e modify src/ | while read file; do
    echo "cargo test --quiet" > temp-test.txt
    mv temp-test.txt .tp/test/
done
```

#### Control Characters and Special Input
```bash
# Send Ctrl+C to interrupt running processes
printf "\003" > interrupt.txt && mv interrupt.txt .tp/webapp/

# Send Ctrl+D to exit interactive programs  
printf "\004" > eof.txt && mv eof.txt .tp/webapp/

# Interactive sequences with exact control
echo "python3" > temp.txt && mv temp.txt .tp/idle/           # Start Python
echo "" > temp.txt && mv temp.txt .tp/idle/                  # Press Enter to execute
echo "print('Hello')" > temp.txt && mv temp.txt .tp/idle/    # Send Python command
printf "\004" > exit.txt && mv exit.txt .tp/idle/            # Ctrl+D to exit Python
```

#### Language-Agnostic Examples

**Python:**
```python
import os
import time

def send_command(queue_name, command):
    """Send command with newline (executes immediately)"""
    temp_file = f"cmd-{os.getpid()}-{int(time.time() * 1000000000)}.txt"
    with open(temp_file, 'w') as f:
        f.write(command + '\n')  # Add newline for execution
    os.rename(temp_file, f".tp/{queue_name}/{temp_file}")

def send_input(queue_name, text):
    """Send input without newline (waits at prompt)"""
    temp_file = f"input-{os.getpid()}-{int(time.time() * 1000000000)}.txt"
    with open(temp_file, 'w') as f:
        f.write(text)  # No newline
    os.rename(temp_file, f".tp/{queue_name}/{temp_file}")

# Usage examples
send_command("webapp", "ls -la")           # Executes immediately
send_input("webapp", "python3")            # Waits at prompt
send_command("webapp", "")                 # Press Enter
```

**Node.js:**
```javascript
const fs = require('fs');
const path = require('path');

function sendCommand(queueName, command) {
    // Send command with newline (executes immediately)
    const tempFile = `cmd-${process.pid}-${Date.now()}-${Math.random().toString(36)}.txt`;
    fs.writeFileSync(tempFile, command + '\n');  // Add newline
    fs.renameSync(tempFile, path.join(`.tp/${queueName}`, tempFile));
}

function sendInput(queueName, text) {
    // Send input without newline (waits at prompt)
    const tempFile = `input-${process.pid}-${Date.now()}-${Math.random().toString(36)}.txt`;
    fs.writeFileSync(tempFile, text);  // No newline
    fs.renameSync(tempFile, path.join(`.tp/${queueName}`, tempFile));
}

// Usage examples
sendCommand('webapp', 'npm run dev');          // Executes immediately
sendInput('webapp', 'node');                   // Waits at prompt
sendCommand('webapp', '');                     // Press Enter
```

## How It Works

The wrapper creates a pseudo-terminal (PTY) and spawns your chosen shell inside it. Input comes from two sources: interactive terminal and programmatic queue files. All output flows transparently to your terminal.

### Core Functionality
- **Background Output Forwarding**: Shell output appears immediately without buffering
- **Smart Exit Detection**: Automatically exits when shell process terminates
- **Signal Passthrough**: Ctrl+C cancels shell processes instead of killing wrapper

### Queue Processing
- **File System Monitoring**: Watches `.tp/<name>/` directory for new files
- **Atomic Processing**: Files are processed in chronological order
- **Raw Text Forwarding**: File contents are sent exactly as stored (no modification)
- **Concurrent Safety**: Queue processor and interactive input both use mutex-protected PTY, the queue processor pauses when interactive input is detected
- **Automatic Cleanup**: Queue directories are created on startup and removed on exit

### Key Behavior Notes
- **Exact Content**: Queue system sends file contents exactly as stored - no additions or modifications
- **Newline Behavior**: `echo` adds newlines (commands execute), `printf` doesn't (waits at prompt)
- **Control Characters**: Can send Ctrl+C, Ctrl+D, etc. using `printf "\003"` or similar
- **Multi-line Support**: Files can contain multiple lines, scripts, or complex input sequences

## Examples

### Running Different Shells
```bash
# Bash (default)
typeypipe

# Zsh with custom queue directory
typeypipe --shell /bin/zsh --queue-dir zsh-session

# Fish shell with quiet startup
typeypipe --shell /usr/bin/fish --quiet

# Custom shell with longer input timeout
typeypipe --shell /bin/dash --input-timeout 60
```

### Interactive Usage
Once running, use the shell normally:
```bash
# Regular commands work as expected
ls -la
cd /tmp
echo "Hello World"

# Long-running processes can be cancelled with Ctrl+C
sleep 30
# Press Ctrl+C to cancel

# Exit the shell normally
exit
# or use Ctrl+D
```

## Technical Details

### Building

```bash
# Clone and build
git clone <repository>
cd <repository>
cargo build --release
```

### Architecture
- **Tokio async runtime** for non-blocking I/O
- **pty-process crate** for cross-platform PTY support  
- **Raw terminal mode** using libc for proper signal handling
- **Mutex-protected PTY** for safe concurrent access from multiple sources
- **File system watcher** using notify crate for queue monitoring
- **Atomic file operations** for conflict-free command queuing

### Dependencies
- `tokio` - Async runtime and I/O
- `pty-process` - PTY creation and management
- `libc` - Raw terminal control
- `clap` - Command line argument parsing
- `notify` - File system watching for queue monitoring
- `thiserror` & `anyhow` - Error handling

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Attribution

Some of the implementation in this project was inspired by the [Zellij](https://github.com/zellij-org/zellij) terminal multiplexer project, particularly for PTY handling and terminal integration. We acknowledge and thank the Zellij maintainers for their excellent work.

## License

MIT License - see LICENSE file for details.