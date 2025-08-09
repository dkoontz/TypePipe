pub mod os_input_output;

pub mod cli_client;
mod input_handler;
mod keyboard_parser;
pub mod old_config_converter;
mod stdin_ansi_parser;
mod stdin_handler;
#[cfg(feature = "web_server_capability")]
pub mod web_client;

use log::info;
use std::env::current_exe;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::stdin_ansi_parser::{AnsiStdinInstruction, StdinAnsiParser};
use crate::{
    input_handler::input_loop,
    os_input_output::ClientOsApi, stdin_handler::stdin_loop,
};
use termwiz::input::InputEvent;
use zellij_utils::{
    channels::{self, ChannelWithContext, SenderWithContext},
    consts::{set_permissions, ZELLIJ_SOCK_DIR},
    data::{KeyWithModifier, Style, Layout},
    envs,
    errors::{ClientContext, ContextType, ErrorInstruction, FatalError},
    input::{config::Config, options::Options},
    ipc::{ClientAttributes, ClientToServerMsg, ExitReason, ServerToClientMsg},
};
use zellij_utils::cli::CliArgs;

/// Instructions related to the client-side application
#[derive(Debug, Clone)]
pub(crate) enum ClientInstruction {
    Error(String),
    Render(String),
    UnblockInputThread,
    Exit(ExitReason),
    Connected,
}

impl From<ServerToClientMsg> for ClientInstruction {
    fn from(instruction: ServerToClientMsg) -> Self {
        match instruction {
            ServerToClientMsg::Exit(e) => ClientInstruction::Exit(e),
            ServerToClientMsg::Render(buffer) => ClientInstruction::Render(buffer),
            ServerToClientMsg::UnblockInputThread => ClientInstruction::UnblockInputThread,
            ServerToClientMsg::Connected => ClientInstruction::Connected,
            _ => ClientInstruction::Error("Unsupported server message".to_string()),
        }
    }
}

impl From<&ClientInstruction> for ClientContext {
    fn from(client_instruction: &ClientInstruction) -> Self {
        match *client_instruction {
            ClientInstruction::Exit(_) => ClientContext::Exit,
            ClientInstruction::Error(_) => ClientContext::Error,
            ClientInstruction::Render(_) => ClientContext::Render,
            ClientInstruction::UnblockInputThread => ClientContext::UnblockInputThread,
            ClientInstruction::Connected => ClientContext::Connected,
        }
    }
}

impl ErrorInstruction for ClientInstruction {
    fn error(err: String) -> Self {
        ClientInstruction::Error(err)
    }
}



pub fn spawn_server(socket_path: &Path, debug: bool) -> io::Result<()> {
    let mut cmd = Command::new(current_exe()?);
    cmd.arg("--server");
    cmd.arg(socket_path);
    if debug {
        cmd.arg("--debug");
    }
    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        let msg = "Process returned non-zero exit code";
        let err_msg = match status.code() {
            Some(c) => format!("{}: {}", msg, c),
            None => msg.to_string(),
        };
        Err(io::Error::new(io::ErrorKind::Other, err_msg))
    }
}

#[derive(Debug, Clone)]
pub enum ClientInfo {
    New(String),
}

impl ClientInfo {
    pub fn get_session_name(&self) -> &str {
        match self {
            Self::New(ref name) => name,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum InputInstruction {
    KeyEvent(InputEvent, Vec<u8>),
    KeyWithModifierEvent(KeyWithModifier, Vec<u8>),
    AnsiStdinInstructions(Vec<AnsiStdinInstruction>),
    StartedParsing,
    DoneParsing,
    Exit,
}

pub fn start_client(
    mut os_input: Box<dyn ClientOsApi>,
    opts: CliArgs,
    info: ClientInfo,
) {
    info!("Starting Typey Pipe client!");

    let explicitly_disable_kitty_keyboard_protocol = false;
    let clear_client_terminal_attributes = "\u{1b}[?1l\u{1b}=\u{1b}[r\u{1b}[?1000l\u{1b}[?1002l\u{1b}[?1003l\u{1b}[?1005l\u{1b}[?1006l\u{1b}[?12l";
    let take_snapshot = "\u{1b}[?1049h";
    let bracketed_paste = "\u{1b}[?2004h";
    let enter_kitty_keyboard_mode = "\u{1b}[>1u";
    os_input.unset_raw_mode(0).unwrap();

    // Basic terminal setup
    let _ = os_input
        .get_stdout_writer()
        .write(take_snapshot.as_bytes())
        .unwrap();
    let _ = os_input
        .get_stdout_writer()
        .write(clear_client_terminal_attributes.as_bytes())
        .unwrap();
    if !explicitly_disable_kitty_keyboard_protocol {
        let _ = os_input
            .get_stdout_writer()
            .write(enter_kitty_keyboard_mode.as_bytes())
            .unwrap();
    }
    envs::set_zellij("0".to_string());

    let palette = os_input.load_palette().into();
    let full_screen_ws = os_input.get_terminal_size_using_fd(0);
    let client_attributes = ClientAttributes {
        size: full_screen_ws,
        style: Style {
            colors: palette,
            rounded_corners: true,
            hide_session_name: false,
        },
    };

    let create_ipc_pipe = || -> std::path::PathBuf {
        let mut sock_dir = ZELLIJ_SOCK_DIR.clone();
        std::fs::create_dir_all(&sock_dir).unwrap();
        set_permissions(&sock_dir, 0o700).unwrap();
        sock_dir.push(envs::get_session_name().unwrap());
        sock_dir
    };

    let (first_msg, ipc_pipe) = match info {
        ClientInfo::New(name) => {
            envs::set_session_name(name.clone());
            os_input.update_session_name(name);
            let ipc_pipe = create_ipc_pipe();

            spawn_server(&*ipc_pipe, opts.debug).unwrap();

            (
                ClientToServerMsg::NewClient(
                    client_attributes,
                    Box::new(opts.clone()),
                    Box::new(Config::default()),
                    Box::new(Options::default()),
                    Box::new(Layout::default()),
                    Box::new(Default::default()),
                    false, // is_web_client
                    false, // should_launch_setup_wizard
                    false, // layout_is_welcome_screen
                ),
                ipc_pipe,
            )
        },
    };

    os_input.connect_to_server(&*ipc_pipe);
    os_input.send_to_server(first_msg);

    os_input.set_raw_mode(0);
    let _ = os_input
        .get_stdout_writer()
        .write(bracketed_paste.as_bytes())
        .unwrap();

    let (send_client_instructions, receive_client_instructions): ChannelWithContext<
        ClientInstruction,
    > = channels::bounded(50);
    let send_client_instructions = SenderWithContext::new(send_client_instructions);

    let (send_input_instructions, receive_input_instructions): ChannelWithContext<
        InputInstruction,
    > = channels::bounded(50);
    let send_input_instructions = SenderWithContext::new(send_input_instructions);

    std::panic::set_hook({
        use zellij_utils::errors::handle_panic;
        let send_client_instructions = send_client_instructions.clone();
        let os_input = os_input.clone();
        Box::new(move |info| {
            if let Ok(()) = os_input.unset_raw_mode(0) {
                handle_panic(info, &send_client_instructions);
            }
        })
    });

    let stdin_ansi_parser = Arc::new(Mutex::new(StdinAnsiParser::new()));

    let _stdin_thread = thread::Builder::new()
        .name("stdin_handler".to_string())
        .spawn({
            let os_input = os_input.clone();
            let send_input_instructions = send_input_instructions.clone();
            let stdin_ansi_parser = stdin_ansi_parser.clone();
            move || {
                stdin_loop(
                    os_input,
                    send_input_instructions,
                    stdin_ansi_parser,
                    explicitly_disable_kitty_keyboard_protocol,
                )
            }
        });

    let _input_thread = thread::Builder::new()
        .name("input_handler".to_string())
        .spawn({
            let os_input = os_input.clone();
            move || {
                input_loop(
                    os_input,
                    receive_input_instructions,
                )
            }
        });

    let _signal_thread = thread::Builder::new()
        .name("signal_listener".to_string())
        .spawn({
            let os_input = os_input.clone();
            move || {
                os_input.handle_signals(
                    Box::new({
                        let os_api = os_input.clone();
                        move || {
                            os_api.send_to_server(ClientToServerMsg::TerminalResize(
                                os_api.get_terminal_size_using_fd(0),
                            ));
                        }
                    }),
                    Box::new({
                        let os_api = os_input.clone();
                        move || {
                            os_api.send_to_server(ClientToServerMsg::ClientExited);
                        }
                    }),
                );
            }
        })
        .unwrap();

    let router_thread = thread::Builder::new()
        .name("router".to_string())
        .spawn({
            let os_input = os_input.clone();
            let mut should_break = false;
            move || loop {
                match os_input.recv_from_server() {
                    Some((instruction, err_ctx)) => {
                        err_ctx.update_thread_ctx();
                        if let ServerToClientMsg::Exit(_) = instruction {
                            should_break = true;
                        }
                        send_client_instructions.send(instruction.into()).unwrap();
                        if should_break {
                            break;
                        }
                    },
                    None => {
                        send_client_instructions
                            .send(ClientInstruction::UnblockInputThread)
                            .unwrap();
                        log::error!("Received empty message from server");
                        send_client_instructions
                            .send(ClientInstruction::Error(
                                "Received empty message from server".to_string(),
                            ))
                            .unwrap();
                        break;
                    },
                }
            }
        })
        .unwrap();

    let handle_error = |backtrace: String| {
        os_input.unset_raw_mode(0).unwrap();
        let goto_start_of_last_line = format!("\u{1b}[{};{}H", full_screen_ws.rows, 1);
        let restore_snapshot = "\u{1b}[?1049l";
        os_input.disable_mouse().non_fatal();
        let error = format!(
            "{}\n{}{}\n",
            restore_snapshot, goto_start_of_last_line, backtrace
        );
        let _ = os_input
            .get_stdout_writer()
            .write(error.as_bytes())
            .unwrap();
        let _ = os_input.get_stdout_writer().flush().unwrap();
        std::process::exit(1);
    };

    let exit_msg;

    loop {
        let (client_instruction, mut err_ctx) = receive_client_instructions
            .recv()
            .expect("failed to receive app instruction on channel");

        err_ctx.add_call(ContextType::Client((&client_instruction).into()));

        match client_instruction {
            ClientInstruction::Exit(reason) => {
                os_input.send_to_server(ClientToServerMsg::ClientExited);

                if let ExitReason::Error(_) = reason {
                    handle_error(reason.to_string());
                }
                exit_msg = reason.to_string();
                break;
            },
            ClientInstruction::Error(backtrace) => {
                handle_error(backtrace);
            },
            ClientInstruction::Render(output) => {
                let mut stdout = os_input.get_stdout_writer();
                stdout
                    .write_all(output.as_bytes())
                    .expect("cannot write to stdout");
                stdout.flush().expect("could not flush");
            },
            ClientInstruction::UnblockInputThread => {
                // Input thread unblocked
            },
            ClientInstruction::Connected => {
                // Client connected successfully
            },
        }
    }

    router_thread.join().unwrap();

    // Terminal cleanup
    let reset_style = "\u{1b}[m";
    let show_cursor = "\u{1b}[?25h";
    let restore_snapshot = "\u{1b}[?1049l";
    let goto_start_of_last_line = format!("\u{1b}[{};{}H", full_screen_ws.rows, 1);
    let goodbye_message = format!(
        "{}\n{}{}{}{}\n",
        goto_start_of_last_line, restore_snapshot, reset_style, show_cursor, exit_msg
    );

    os_input.disable_mouse().non_fatal();
    info!("{}", exit_msg);
    os_input.unset_raw_mode(0).unwrap();
    let mut stdout = os_input.get_stdout_writer();
    let exit_kitty_keyboard_mode = "\u{1b}[<1u";
    if !explicitly_disable_kitty_keyboard_protocol {
        let _ = stdout.write(exit_kitty_keyboard_mode.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
    let _ = stdout.write(goodbye_message.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let _ = send_input_instructions.send(InputInstruction::Exit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use zellij_utils::ipc::{ClientToServerMsg, ServerToClientMsg};

    #[test]
    fn test_client_instruction_from_server_msg() {
        // Test Exit message conversion
        let exit_msg = ServerToClientMsg::Exit(ExitReason::Normal);
        let client_instruction = ClientInstruction::from(exit_msg);
        match client_instruction {
            ClientInstruction::Exit(ExitReason::Normal) => {},
            _ => panic!("Expected Exit instruction"),
        }

        // Test Render message conversion
        let render_msg = ServerToClientMsg::Render("test output".to_string());
        let client_instruction = ClientInstruction::from(render_msg);
        match client_instruction {
            ClientInstruction::Render(output) => assert_eq!(output, "test output"),
            _ => panic!("Expected Render instruction"),
        }

        // Test Connected message conversion
        let connected_msg = ServerToClientMsg::Connected;
        let client_instruction = ClientInstruction::from(connected_msg);
        match client_instruction {
            ClientInstruction::Connected => {},
            _ => panic!("Expected Connected instruction"),
        }

        // Test UnblockInputThread message conversion
        let unblock_msg = ServerToClientMsg::UnblockInputThread;
        let client_instruction = ClientInstruction::from(unblock_msg);
        match client_instruction {
            ClientInstruction::UnblockInputThread => {},
            _ => panic!("Expected UnblockInputThread instruction"),
        }
    }

    #[test]
    fn test_client_info_session_name() {
        let client_info = ClientInfo::New("test_session".to_string());
        assert_eq!(client_info.get_session_name(), "test_session");
    }

    #[test]
    fn test_input_instruction_variants() {
        // Test that InputInstruction enum has the expected variants
        use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};
        
        let key_event = InputEvent::Key(KeyEvent {
            key: KeyCode::Char('a'),
            modifiers: Modifiers::NONE,
        });
        let input_instruction = InputInstruction::KeyEvent(key_event, vec![97]);
        
        match input_instruction {
            InputInstruction::KeyEvent(_, bytes) => assert_eq!(bytes, vec![97]),
            _ => panic!("Expected KeyEvent instruction"),
        }

        let exit_instruction = InputInstruction::Exit;
        match exit_instruction {
            InputInstruction::Exit => {},
            _ => panic!("Expected Exit instruction"),
        }
    }

    #[test]
    fn test_terminal_bytes_message() {
        // Test that TerminalBytes message can be created and contains expected data
        let test_bytes = vec![65, 66, 67]; // "ABC"
        let terminal_bytes_msg = ClientToServerMsg::TerminalBytes(test_bytes.clone());
        
        match terminal_bytes_msg {
            ClientToServerMsg::TerminalBytes(bytes) => assert_eq!(bytes, test_bytes),
            _ => panic!("Expected TerminalBytes message"),
        }
    }

    #[test]
    fn test_client_instruction_context_conversion() {
        let exit_instruction = ClientInstruction::Exit(ExitReason::Normal);
        let context = ClientContext::from(&exit_instruction);
        match context {
            ClientContext::Exit => {},
            _ => panic!("Expected Exit context"),
        }

        let render_instruction = ClientInstruction::Render("test".to_string());
        let context = ClientContext::from(&render_instruction);
        match context {
            ClientContext::Render => {},
            _ => panic!("Expected Render context"),
        }

        let error_instruction = ClientInstruction::Error("test error".to_string());
        let context = ClientContext::from(&error_instruction);
        match context {
            ClientContext::Error => {},
            _ => panic!("Expected Error context"),
        }
    }
}


