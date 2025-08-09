//! Definitions and helpers for sending and receiving messages between threads.

use crate::{
    background_jobs::BackgroundJob,
    os_input_output::ServerOsApi,
    pty::PtyInstruction, 
    pty_writer::PtyWriteInstruction,
    screen::ScreenInstruction,
    ServerInstruction,
};

// Stub for removed plugin functionality
#[derive(Debug, Clone)]
pub enum PluginInstruction {
    Update(Vec<(Option<u32>, Option<u16>, Event)>),
    CliPipe { 
        pipe_id: String,
        name: String, 
        payload: Option<String>,
        plugin: Option<String>,
        args: Option<std::collections::BTreeMap<String, String>>,
        configuration: Option<std::collections::BTreeMap<String, String>>,
        floating: Option<bool>,
        pane_id_to_replace: Option<PaneId>,
        cwd: Option<std::path::PathBuf>,
        pane_title: Option<String>,
        skip_cache: bool,
        cli_client_id: Option<u16>,
    },
    KeybindPipe { 
        name: String, 
        payload: Option<String>,
        plugin: Option<String>,
        args: Option<std::collections::BTreeMap<String, String>>,
        configuration: Option<std::collections::BTreeMap<String, String>>,
        floating: Option<bool>,
        pane_id_to_replace: Option<PaneId>,
        cwd: Option<std::path::PathBuf>,
        pane_title: Option<String>,
        skip_cache: bool,
        cli_client_id: Option<u16>,
        plugin_and_client_id: Option<(PluginId, u16)>,
    },
    Resize(u32, usize, usize), // plugin_id, rows, cols - stub parameters
    PermissionRequestResult(u32, Option<u16>, Vec<PermissionType>, PermissionStatus, Option<()>), // stub parameters
    DumpLayoutToPlugin(String, PluginId), // layout, plugin_id
    ListClientsToPlugin(String, PluginId, Option<u16>), // layout, plugin_id, client_id
    Unload(u32), // plugin_id
    Load(u32, String), // plugin_id, path
    UnblockCliPipes(u32), // plugin_id
    LogLayoutToHd(String), // layout
    NewTab(Option<String>, Option<String>, Option<String>, Vec<String>, usize, bool, (u16, bool)), // Stub with all parameters
    DumpLayout(String), // layout
    ListClientsMetadata(Vec<String>), // clients
    Reload(u32), // plugin_id
    Exit, // for shutdown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PluginId(pub u32);

impl std::fmt::Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PluginId> for u32 {
    fn from(plugin_id: PluginId) -> u32 {
        plugin_id.0
    }
}

impl From<u32> for PluginId {
    fn from(id: u32) -> PluginId {
        PluginId(id)
    }
}

#[derive(Debug, Clone)]
pub struct PluginRenderAsset {
    pub plugin_id: PluginId,
    pub client_id: Option<u16>,
    pub bytes: Vec<u8>,
}
use zellij_utils::errors::prelude::*;
use zellij_utils::{channels, channels::SenderWithContext, errors::ErrorContext};
use zellij_utils::data::{Event, PermissionStatus, PermissionType};
use crate::panes::PaneId;

/// A container for senders to the different threads in zellij on the server side
#[derive(Default, Clone)]
pub struct ThreadSenders {
    pub to_screen: Option<SenderWithContext<ScreenInstruction>>,
    pub to_pty: Option<SenderWithContext<PtyInstruction>>,
    pub to_plugin: Option<SenderWithContext<PluginInstruction>>,
    pub to_server: Option<SenderWithContext<ServerInstruction>>,
    pub to_pty_writer: Option<SenderWithContext<PtyWriteInstruction>>,
    pub to_background_jobs: Option<SenderWithContext<BackgroundJob>>,
    // this is a convenience for the unit tests
    // it's not advisable to set it to true in production code
    pub should_silently_fail: bool,
}

impl ThreadSenders {
    pub fn send_to_screen(&self, instruction: ScreenInstruction) -> Result<()> {
        if self.should_silently_fail {
            let _ = self
                .to_screen
                .as_ref()
                .map(|sender| sender.send(instruction))
                .unwrap_or_else(|| Ok(()));
            Ok(())
        } else {
            self.to_screen
                .as_ref()
                .context("failed to get screen sender")?
                .send(instruction)
                .to_anyhow()
                .context("failed to send message to screen")
        }
    }

    pub fn send_to_pty(&self, instruction: PtyInstruction) -> Result<()> {
        if self.should_silently_fail {
            let _ = self
                .to_pty
                .as_ref()
                .map(|sender| sender.send(instruction))
                .unwrap_or_else(|| Ok(()));
            Ok(())
        } else {
            self.to_pty
                .as_ref()
                .context("failed to get pty sender")?
                .send(instruction)
                .to_anyhow()
                .context("failed to send message to pty")
        }
    }



    pub fn send_to_server(&self, instruction: ServerInstruction) -> Result<()> {
        if self.should_silently_fail {
            let _ = self
                .to_server
                .as_ref()
                .map(|sender| sender.send(instruction))
                .unwrap_or_else(|| Ok(()));
            Ok(())
        } else {
            self.to_server
                .as_ref()
                .context("failed to get server sender")?
                .send(instruction)
                .to_anyhow()
                .context("failed to send message to server")
        }
    }


    pub fn send_to_plugin(&self, _instruction: PluginInstruction) -> Result<()> {
        // Plugin functionality removed - this is now a no-op
        Ok(())
    }

    pub fn send_to_pty_writer(&self, instruction: PtyWriteInstruction) -> Result<()> {
        if self.should_silently_fail {
            let _ = self
                .to_pty_writer
                .as_ref()
                .map(|sender| sender.send(instruction))
                .unwrap_or_else(|| Ok(()));
            Ok(())
        } else {
            self.to_pty_writer
                .as_ref()
                .context("failed to get pty writer sender")?
                .send(instruction)
                .to_anyhow()
                .context("failed to send message to pty writer")
        }
    }

    pub fn send_to_background_jobs(&self, background_job: BackgroundJob) -> Result<()> {
        if self.should_silently_fail {
            let _ = self
                .to_background_jobs
                .as_ref()
                .map(|sender| sender.send(background_job))
                .unwrap_or_else(|| Ok(()));
            Ok(())
        } else {
            self.to_background_jobs
                .as_ref()
                .context("failed to get background jobs sender")?
                .send(background_job)
                .to_anyhow()
                .context("failed to send message to background jobs")
        }
    }

    #[allow(unused)]
    pub fn silently_fail_on_send(mut self) -> Self {
        // this is mostly used for the tests, see struct
        self.should_silently_fail = true;
        self
    }

}

/// A container for a receiver, OS input and the senders to a given thread
#[derive(Default)]
pub(crate) struct Bus<T> {
    receivers: Vec<channels::Receiver<(T, ErrorContext)>>,
    pub senders: ThreadSenders,
    pub os_input: Option<Box<dyn ServerOsApi>>,
}

impl<T> Bus<T> {
    pub fn new(
        receivers: Vec<channels::Receiver<(T, ErrorContext)>>,
        to_screen: Option<&SenderWithContext<ScreenInstruction>>,
        to_pty: Option<&SenderWithContext<PtyInstruction>>,
        to_plugin: Option<&SenderWithContext<PluginInstruction>>,
        to_server: Option<&SenderWithContext<ServerInstruction>>,
        to_pty_writer: Option<&SenderWithContext<PtyWriteInstruction>>,
        to_background_jobs: Option<&SenderWithContext<BackgroundJob>>,
        os_input: Option<Box<dyn ServerOsApi>>,
    ) -> Self {
        Bus {
            receivers,
            senders: ThreadSenders {
                to_screen: to_screen.cloned(),
                to_pty: to_pty.cloned(),
                to_plugin: to_plugin.cloned(),
                to_server: to_server.cloned(),
                to_pty_writer: to_pty_writer.cloned(),
                to_background_jobs: to_background_jobs.cloned(),
                should_silently_fail: false,
            },
            os_input: os_input.clone(),
        }
    }
    #[allow(unused)]
    pub fn should_silently_fail(mut self) -> Self {
        // this is mostly used for the tests
        self.senders.should_silently_fail = true;
        self
    }
    #[allow(unused)]
    pub fn empty() -> Self {
        // this is mostly used for the tests
        Bus {
            receivers: vec![],
            senders: ThreadSenders {
                to_screen: None,
                to_pty: None,
                to_plugin: None,
                to_server: None,
                to_pty_writer: None,
                to_background_jobs: None,
                should_silently_fail: true,
            },
            os_input: None,
        }
    }

    pub fn recv(&self) -> Result<(T, ErrorContext), channels::RecvError> {
        let mut selector = channels::Select::new();
        self.receivers.iter().for_each(|r| {
            selector.recv(r);
        });
        let oper = selector.select();
        let idx = oper.index();
        oper.recv(&self.receivers[idx])
    }
}
