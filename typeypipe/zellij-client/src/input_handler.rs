//! Main input logic.
use crate::{
    os_input_output::ClientOsApi, InputInstruction,
};
use termwiz::input::{InputEvent, Modifiers, MouseButtons, MouseEvent as TermwizMouseEvent};
use zellij_utils::{
    channels::{Receiver, OPENCALLS},
    errors::{ContextType, ErrorContext, FatalError},
    input::mouse::{MouseEvent, MouseEventType},
    ipc::ClientToServerMsg,
    position::Position,
};

/// Handles basic input forwarding to the server
struct InputHandler {
    os_input: Box<dyn ClientOsApi>,
    should_exit: bool,
    receive_input_instructions: Receiver<(InputInstruction, ErrorContext)>,
    mouse_old_event: MouseEvent,
}

fn termwiz_mouse_convert(original_event: &mut MouseEvent, event: &TermwizMouseEvent) {
    let button_bits = &event.mouse_buttons;
    original_event.left = button_bits.contains(MouseButtons::LEFT);
    original_event.right = button_bits.contains(MouseButtons::RIGHT);
    original_event.middle = button_bits.contains(MouseButtons::MIDDLE);
    original_event.wheel_up = button_bits.contains(MouseButtons::VERT_WHEEL)
        && button_bits.contains(MouseButtons::WHEEL_POSITIVE);
    original_event.wheel_down = button_bits.contains(MouseButtons::VERT_WHEEL)
        && !button_bits.contains(MouseButtons::WHEEL_POSITIVE);

    let mods = &event.modifiers;
    original_event.shift = mods.contains(Modifiers::SHIFT);
    original_event.alt = mods.contains(Modifiers::ALT);
    original_event.ctrl = mods.contains(Modifiers::CTRL);
}

pub fn from_termwiz(old_event: &mut MouseEvent, event: TermwizMouseEvent) -> MouseEvent {
    // We use the state of old_event vs new_event to determine if this
    // event is a Press, Release, or Motion.  This is an unfortunate
    // side effect of the pre-SGR-encoded X10 mouse protocol design in
    // which release events don't carry information about WHICH
    // button(s) were released, so we have to maintain a wee bit of
    // state in between events.
    //
    // Note that only Left, Right, and Middle are saved in between
    // calls.  WheelUp/WheelDown typically do not generate Release
    // events.
    let mut new_event = MouseEvent::new();
    termwiz_mouse_convert(&mut new_event, &event);
    new_event.position = Position::new(event.y.saturating_sub(1) as i32, event.x.saturating_sub(1));

    if (new_event.left && !old_event.left)
        || (new_event.right && !old_event.right)
        || (new_event.middle && !old_event.middle)
        || new_event.wheel_up
        || new_event.wheel_down
    {
        // This is a mouse Press event.
        new_event.event_type = MouseEventType::Press;

        // Hang onto the button state.
        *old_event = new_event;
    } else if event.mouse_buttons.is_empty()
        && !old_event.left
        && !old_event.right
        && !old_event.middle
    {
        // This is a mouse Motion event (no buttons are down).
        new_event.event_type = MouseEventType::Motion;

        // Hang onto the button state.
        *old_event = new_event;
    } else if event.mouse_buttons.is_empty()
        && (old_event.left || old_event.right || old_event.middle)
    {
        // This is a mouse Release event.  Note that we set
        // old_event.{button} to false (to release), but set ONLY the
        // new_event that were released to true before sending the
        // event up.
        if old_event.left {
            old_event.left = false;
            new_event.left = true;
        }
        if old_event.right {
            old_event.right = false;
            new_event.right = true;
        }
        if old_event.middle {
            old_event.middle = false;
            new_event.middle = true;
        }
        new_event.event_type = MouseEventType::Release;
    } else {
        // Dragging with some button down.  Return it as a Motion
        // event, and hang on to the button state.
        new_event.event_type = MouseEventType::Motion;
        *old_event = new_event;
    }

    new_event
}

impl InputHandler {
    /// Returns a new [`InputHandler`] with the attributes specified as arguments.
    fn new(
        os_input: Box<dyn ClientOsApi>,
        receive_input_instructions: Receiver<(InputInstruction, ErrorContext)>,
    ) -> Self {
        InputHandler {
            os_input,
            should_exit: false,
            receive_input_instructions,
            mouse_old_event: MouseEvent::new(),
        }
    }

    /// Main input event loop. Forwards input directly to the server.
    fn handle_input(&mut self) {
        let mut err_ctx = OPENCALLS.with(|ctx| *ctx.borrow());
        err_ctx.add_call(ContextType::StdinHandler);
        
        // Enable mouse support for status bar interaction
        self.os_input.enable_mouse().non_fatal();
        
        loop {
            if self.should_exit {
                break;
            }
            match self.receive_input_instructions.recv() {
                Ok((InputInstruction::KeyEvent(input_event, raw_bytes), _error_context)) => {
                    match input_event {
                        InputEvent::Key(_key_event) => {
                            // Forward raw bytes directly to server for shell input
                            self.os_input.send_to_server(ClientToServerMsg::TerminalBytes(raw_bytes));
                        },
                        InputEvent::Mouse(mouse_event) => {
                            let mouse_event = from_termwiz(&mut self.mouse_old_event, mouse_event);
                            self.handle_mouse_event(&mouse_event);
                        },
                        InputEvent::Paste(pasted_text) => {
                            // Forward paste directly to shell
                            let bracketed_paste_start = vec![27, 91, 50, 48, 48, 126]; // \u{1b}[200~
                            let bracketed_paste_end = vec![27, 91, 50, 48, 49, 126]; // \u{1b}[201~
                            self.os_input.send_to_server(ClientToServerMsg::TerminalBytes(bracketed_paste_start));
                            self.os_input.send_to_server(ClientToServerMsg::TerminalBytes(pasted_text.as_bytes().to_vec()));
                            self.os_input.send_to_server(ClientToServerMsg::TerminalBytes(bracketed_paste_end));
                        },
                        _ => {},
                    }
                },
                Ok((InputInstruction::KeyWithModifierEvent(_key_with_modifier, raw_bytes), _error_context)) => {
                    // Forward raw bytes directly to server for shell input
                    self.os_input.send_to_server(ClientToServerMsg::TerminalBytes(raw_bytes));
                },
                Ok((InputInstruction::AnsiStdinInstructions(_ansi_stdin_instructions), _error_context)) => {
                    // Ignore ANSI stdin instructions for now in simplified mode
                },
                Ok((InputInstruction::StartedParsing, _error_context)) => {
                    // Ignore parsing events in simplified mode
                },
                Ok((InputInstruction::DoneParsing, _error_context)) => {
                    // Ignore parsing events in simplified mode
                },
                Ok((InputInstruction::Exit, _error_context)) => {
                    self.should_exit = true;
                },
                Err(err) => panic!("Encountered read error: {:?}", err),
            }
        }
    }

    fn handle_mouse_event(&mut self, _mouse_event: &MouseEvent) {
        // For now, just handle basic mouse events for status bar interaction
        // In a full implementation, this would forward mouse events to the server
        // for status bar interaction
    }


}

/// Entry point to the module. Instantiates an [`InputHandler`] and starts
/// its [`InputHandler::handle_input()`] loop.
pub(crate) fn input_loop(
    os_input: Box<dyn ClientOsApi>,
    receive_input_instructions: Receiver<(InputInstruction, ErrorContext)>,
) {
    let _handler = InputHandler::new(
        os_input,
        receive_input_instructions,
    )
    .handle_input();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InputInstruction;
    use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

    #[test]
    fn test_mouse_event_conversion() {
        use termwiz::input::{MouseButtons, MouseEvent as TermwizMouseEvent};
        
        let mut old_event = MouseEvent::new();
        let termwiz_event = TermwizMouseEvent {
            x: 10,
            y: 5,
            mouse_buttons: MouseButtons::LEFT,
            modifiers: Modifiers::NONE,
        };
        
        let converted_event = from_termwiz(&mut old_event, termwiz_event);
        
        assert_eq!(converted_event.position.column.0, 9); // x - 1
        assert_eq!(converted_event.position.line.0, 4);   // y - 1
        assert!(converted_event.left);
        assert_eq!(converted_event.event_type, MouseEventType::Press);
    }

    #[test]
    fn test_input_instruction_key_event() {
        let key_event = InputEvent::Key(KeyEvent {
            key: KeyCode::Char('a'),
            modifiers: Modifiers::NONE,
        });
        let raw_bytes = vec![97]; // 'a'
        
        let instruction = InputInstruction::KeyEvent(key_event, raw_bytes.clone());
        
        match instruction {
            InputInstruction::KeyEvent(event, bytes) => {
                assert_eq!(bytes, raw_bytes);
                match event {
                    InputEvent::Key(key_event) => {
                        assert_eq!(key_event.key, KeyCode::Char('a'));
                    },
                    _ => panic!("Expected Key event"),
                }
            },
            _ => panic!("Expected KeyEvent instruction"),
        }
    }

    #[test]
    fn test_input_instruction_exit() {
        let instruction = InputInstruction::Exit;
        match instruction {
            InputInstruction::Exit => {},
            _ => panic!("Expected Exit instruction"),
        }
    }

    #[test]
    fn test_mouse_event_new() {
        let mouse_event = MouseEvent::new();
        assert!(!mouse_event.left);
        assert!(!mouse_event.right);
        assert!(!mouse_event.middle);
        assert!(!mouse_event.wheel_up);
        assert!(!mouse_event.wheel_down);
        assert_eq!(mouse_event.position.line.0, 0);
        assert_eq!(mouse_event.position.column.0, 0);
    }
}
