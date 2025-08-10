use std::{collections::VecDeque, io::Write};

use crate::thread_bus::PluginId;
use log::{debug, error};
use zellij_utils::errors::prelude::*;

use serde::{Deserialize, Serialize};

// 16kB log buffer
const ZELLIJ_MAX_PIPE_BUFFER_SIZE: usize = 16_384;
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingPipe {
    buffer: VecDeque<u8>,
    plugin_name: String,
    plugin_id: PluginId,
}

impl LoggingPipe {


    fn log_message(&self, message: &str) {
        debug!(
            "|{:<25.25}| {} [{:<10.15}] {}",
            self.plugin_name,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
            format!("id: {}", self.plugin_id),
            message
        );
    }
}

impl Write for LoggingPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.buffer.len() + buf.len() > ZELLIJ_MAX_PIPE_BUFFER_SIZE {
            let error_msg =
                "Exceeded log buffer size. Make sure that your plugin calls flush on stderr on \
                valid UTF-8 symbol boundary. Additionally, make sure that your log message contains \
                endline \\n symbol.";
            error!("{}: {}", self.plugin_name, error_msg);
            self.buffer.clear();
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                error_msg,
            ));
        }

        self.buffer.extend(buf);

        Ok(buf.len())
    }

    // When we flush, check if current buffer is valid utf8 string, split by '\n' and truncate buffer in the process.
    // We assume that eventually, flush will be called on valid string boundary (i.e. std::str::from_utf8(..).is_ok() returns true at some point).
    // Above assumption might not be true, in which case we'll have to think about it. Make it simple for now.
    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.make_contiguous();

        match std::str::from_utf8(self.buffer.as_slices().0) {
            Ok(converted_buffer) => {
                if converted_buffer.contains('\n') {
                    let mut consumed_bytes = 0;
                    let mut split_converted_buffer = converted_buffer.split('\n').peekable();

                    while let Some(msg) = split_converted_buffer.next() {
                        if split_converted_buffer.peek().is_none() {
                            // Log last chunk iff the last char is endline. Otherwise do not do it.
                            if converted_buffer.ends_with('\n') && !msg.is_empty() {
                                self.log_message(msg);
                                consumed_bytes += msg.len() + 1;
                            }
                        } else {
                            self.log_message(msg);
                            consumed_bytes += msg.len() + 1;
                        }
                    }
                    drop(self.buffer.drain(..consumed_bytes));
                }
            },
            Err(e) => Err::<(), _>(e)
                .context("failed to flush logging pipe buffer")
                .non_fatal(),
        }

        Ok(())
    }
}

// Unit tests

