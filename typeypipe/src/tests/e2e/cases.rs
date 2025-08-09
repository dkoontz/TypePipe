#![allow(unused)]

use ::insta::assert_snapshot;
use zellij_utils::{pane_size::Size, position::Position};

use rand::Rng;
use regex::Regex;

use std::fmt::Write;
use std::path::Path;

use super::remote_runner::{RemoteRunner, RemoteTerminal, Step};

pub const QUIT: [u8; 1] = [17]; // ctrl-q
pub const ESC: [u8; 1] = [27];
pub const ENTER: [u8; 2] = [10, 13]; // '\n\r'
pub const SPACE: [u8; 1] = [32];
pub const LOCK_MODE: [u8; 1] = [7]; // ctrl-g

pub const BRACKETED_PASTE_START: [u8; 6] = [27, 91, 50, 48, 48, 126]; // \u{1b}[200~
pub const BRACKETED_PASTE_END: [u8; 6] = [27, 91, 50, 48, 49, 126]; // \u{1b}[201~
pub const SLEEP: [u8; 0] = [];

pub fn sgr_mouse_report(position: Position, button: u8) -> Vec<u8> {
    // button: (release is with lower case m, not supported here yet)
    // M = press, m = release
    // button: 0 = left, 1 = middle, 2 = right
    // position: 1-indexed
    format!("\u{1b}[<{};{};{}M", button, position.column.0 + 1, position.line.0 + 1)
        .as_bytes()
        .to_vec()
}

pub fn account_for_races_in_snapshot(snapshot: String) -> String {
    // this is a bit of a hack, but we need to account for the fact that
    // the snapshot might be taken at different times, and the cursor
    // might be in different positions
    let cursor_coordinates_in_snapshot = Regex::new(r"\u{1b}\[[0-9]+;[0-9]+H").unwrap();
    let cursor_coordinates_in_snapshot = cursor_coordinates_in_snapshot.replace_all(&snapshot, "");
    let hide_cursor_in_snapshot = Regex::new(r"\u{1b}\[\?25l").unwrap();
    let hide_cursor_in_snapshot = hide_cursor_in_snapshot.replace_all(&cursor_coordinates_in_snapshot, "");
    let show_cursor_in_snapshot = Regex::new(r"\u{1b}\[\?25h").unwrap();
    let show_cursor_in_snapshot = show_cursor_in_snapshot.replace_all(&hide_cursor_in_snapshot, "");
    show_cursor_in_snapshot.to_string()
}

// All the E2E tests are marked as "ignored" so that they can be run separately from the normal
// tests

#[test]
#[ignore]
pub fn starts_with_one_terminal() {
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };

    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}

#[test]
#[ignore]
pub fn exit_zellij() {
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Send quit command",
            instruction: |mut remote_terminal: RemoteTerminal| -> bool {
                remote_terminal.send_key(&QUIT);
                true
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };
    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}

#[test]
#[ignore]
pub fn typing_exit_closes_pane() {
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Type exit command",
            instruction: |mut remote_terminal: RemoteTerminal| -> bool {
                remote_terminal.send_key("exit".as_bytes());
                remote_terminal.send_key(&ENTER);
                true
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };
    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}

#[test]
#[ignore]
pub fn lock_mode() {
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Enter lock mode",
            instruction: |mut remote_terminal: RemoteTerminal| -> bool {
                remote_terminal.send_key(&LOCK_MODE);
                std::thread::sleep(std::time::Duration::from_millis(100));
                // Check if lock mode is active by looking for specific text
                remote_terminal.snapshot_contains("LOCKED") || remote_terminal.ctrl_plus_appears()
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };

    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}

#[test]
#[ignore]
pub fn resize_terminal_window() {
    // this checks the resizing of the whole terminal window (reaction to SIGWINCH) and not just one pane
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Resize terminal window",
            instruction: |mut remote_terminal: RemoteTerminal| -> bool {
                remote_terminal.change_size(100, 20);
                std::thread::sleep(std::time::Duration::from_millis(200));
                // Check if resize was processed
                remote_terminal.status_bar_appears()
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };

    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}

#[test]
#[ignore]
pub fn bracketed_paste() {
    let fake_win_size = Size {
        cols: 120,
        rows: 24,
    };
    // here we enter some text, before which we invoke "bracketed paste mode"
    // this is a special mode that allows us to paste text without it being
    // interpreted as commands - this way we can paste text that contains
    // special characters (eg. ctrl sequences) without them being interpreted
    let mut test_attempts = 10;
    let last_snapshot = loop {
        RemoteRunner::kill_running_sessions(fake_win_size);
        let mut runner = RemoteRunner::new(fake_win_size);
        runner.take_snapshot_after(Step {
            name: "Wait for app to load",
            instruction: |remote_terminal: RemoteTerminal| -> bool {
                let mut step_is_complete = false;
                if remote_terminal.status_bar_appears() && remote_terminal.cursor_position_is(3, 2)
                {
                    step_is_complete = true;
                }
                step_is_complete
            },
        });
        let last_snapshot = runner.take_snapshot_after(Step {
            name: "Send bracketed paste",
            instruction: |mut remote_terminal: RemoteTerminal| -> bool {
                remote_terminal.send_key(&BRACKETED_PASTE_START);
                remote_terminal.send_key("echo \"I am pasted text\"".as_bytes());
                remote_terminal.send_key(&BRACKETED_PASTE_END);
                std::thread::sleep(std::time::Duration::from_millis(100));
                remote_terminal.cursor_position_is(25, 2)
            },
        });
        if runner.test_timed_out && test_attempts > 0 {
            test_attempts -= 1;
            continue;
        } else {
            break last_snapshot;
        }
    };

    let last_snapshot = account_for_races_in_snapshot(last_snapshot);
    assert_snapshot!(last_snapshot);
}