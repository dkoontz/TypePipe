
use miette::{Diagnostic, LabeledSpan, NamedSource, SourceCode};
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use thiserror::Error;

use std::convert::TryFrom;

use super::options::Options;
use crate::cli::CliArgs;

use crate::{home, setup};

const DEFAULT_CONFIG_FILE_NAME: &str = "config.kdl";

type ConfigResult = Result<Config, ConfigError>;

/// Main configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Config {
    pub options: Options,
    // Stub fields for removed functionality - kept to avoid compilation errors
    pub themes: crate::data::Themes,
    pub keybinds: crate::input::keybinds::Keybinds,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Typey Pipe Configuration")
    }
}

impl Config {
    pub fn to_string(&self, _clear_defaults: bool) -> String {
        "Typey Pipe Configuration".to_string()
    }
}

#[derive(Error, Debug)]
pub struct KdlError {
    pub error_message: String,
    pub src: Option<NamedSource>,
    pub offset: Option<usize>,
    pub len: Option<usize>,
    pub help_message: Option<String>,
}

impl KdlError {
    pub fn add_src(mut self, src_name: String, src_input: String) -> Self {
        self.src = Some(NamedSource::new(src_name, src_input));
        self
    }
}

impl std::fmt::Display for KdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Failed to parse Zellij configuration")
    }
}
use std::fmt::Display;

impl Diagnostic for KdlError {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self.src.as_ref() {
            Some(src) => Some(src),
            None => None,
        }
    }
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match &self.help_message {
            Some(help_message) => Some(Box::new(help_message)),
            None => Some(Box::new(format!("For more information, please see our configuration guide: https://zellij.dev/documentation/configuration.html")))
        }
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        if let (Some(offset), Some(len)) = (self.offset, self.len) {
            let label = LabeledSpan::new(Some(self.error_message.clone()), offset, len);
            Some(Box::new(std::iter::once(label)))
        } else {
            None
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum ConfigError {
    // Deserialization error
    #[error("Deserialization error: {0}")]
    KdlDeserializationError(#[from] kdl::KdlError),
    #[error("KdlDeserialization error: {0}")]
    KdlError(KdlError), // TODO: consolidate these
    #[error("Config error: {0}")]
    Std(#[from] Box<dyn std::error::Error>),
    // Io error with path context
    #[error("IoError: {0}, File: {1}")]
    IoPath(io::Error, PathBuf),
    // Internal Deserialization Error
    #[error("FromUtf8Error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    // Plugins have a semantic error, usually trying to parse two of the same tag

    #[error("{0}")]
    ConversionError(#[from] ConversionError),
    #[error("{0}")]
    DownloadError(String),
}

impl ConfigError {
    pub fn new_kdl_error(error_message: String, offset: usize, len: usize) -> Self {
        ConfigError::KdlError(KdlError {
            error_message,
            src: None,
            offset: Some(offset),
            len: Some(len),
            help_message: None,
        })
    }
    pub fn new_layout_kdl_error(error_message: String, offset: usize, len: usize) -> Self {
        ConfigError::KdlError(KdlError {
            error_message,
            src: None,
            offset: Some(offset),
            len: Some(len),
            help_message: Some(format!("For more information, please see our layout guide: https://zellij.dev/documentation/creating-a-layout.html")),
        })
    }
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("{0}")]
    UnknownInputMode(String),
}

impl TryFrom<&CliArgs> for Config {
    type Error = ConfigError;

    fn try_from(opts: &CliArgs) -> ConfigResult {
        if let Some(ref path) = opts.config {
            let default_config = Config::from_default_assets()?;
            return Config::from_path(path, Some(default_config));
        }



        let config_dir = opts
            .config_dir
            .clone()
            .or_else(home::find_default_config_dir);

        if let Some(ref config) = config_dir {
            let path = config.join(DEFAULT_CONFIG_FILE_NAME);
            if path.exists() {
                let default_config = Config::from_default_assets()?;
                Config::from_path(&path, Some(default_config))
            } else {
                Config::from_default_assets()
            }
        } else {
            Config::from_default_assets()
        }
    }
}

impl Config {
    pub fn from_kdl(
        kdl_config: &str,
        base_config: Option<Config>,
    ) -> Result<Config, ConfigError> {
        let mut config = base_config.unwrap_or_default();
        
        // Simplified KDL parsing for Typey Pipe - only handle basic options
        // Parse line by line for simple key-value pairs
        for line in kdl_config.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            // Simple parsing for basic options
            if line.starts_with("on_force_close") {
                if line.contains("\"quit\"") {
                    config.options.on_force_close = Some(crate::input::options::OnForceClose::Quit);
                } else {
                    config.options.on_force_close = Some(crate::input::options::OnForceClose::Detach);
                }
            } else if line.starts_with("scroll_buffer_size") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    if let Ok(value) = value_str.parse::<usize>() {
                        config.options.scroll_buffer_size = Some(value);
                    }
                }
            } else if line.starts_with("status_bar ") {
                if line.contains("true") {
                    config.options.status_bar = Some(true);
                } else if line.contains("false") {
                    config.options.status_bar = Some(false);
                }
            } else if line.starts_with("status_bar_refresh_interval") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    if let Ok(value) = value_str.parse::<u64>() {
                        config.options.status_bar_refresh_interval = Some(value);
                    }
                }
            }
            // Ignore other complex configuration options that we've removed
        }
        
        Ok(config)
    }

    /// Gets default configuration from assets
    pub fn from_default_assets() -> ConfigResult {
        let cfg = String::from_utf8(setup::DEFAULT_CONFIG.to_vec())?;
        match Self::from_kdl(&cfg, None) {
            Ok(config) => Ok(config),
            Err(ConfigError::KdlError(kdl_error)) => Err(ConfigError::KdlError(
                kdl_error.add_src("Default built-in-configuration".into(), cfg),
            )),
            Err(e) => Err(e),
        }
    }
    pub fn from_path(path: &PathBuf, default_config: Option<Config>) -> ConfigResult {
        match File::open(path) {
            Ok(mut file) => {
                let mut kdl_config = String::new();
                file.read_to_string(&mut kdl_config)
                    .map_err(|e| ConfigError::IoPath(e, path.to_path_buf()))?;
                match Config::from_kdl(&kdl_config, default_config) {
                    Ok(config) => Ok(config),
                    Err(ConfigError::KdlDeserializationError(kdl_error)) => {
                        let error_message = match kdl_error.kind {
                            kdl::KdlErrorKind::Context("valid node terminator") => {
                                format!("Failed to deserialize KDL node. \nPossible reasons:\n{}\n{}\n{}\n{}",
                                "- Missing `;` after a node name, eg. { node; another_node; }",
                                "- Missing quotations (\") around an argument node eg. { first_node \"argument_node\"; }",
                                "- Missing an equal sign (=) between node arguments on a title line. eg. argument=\"value\"",
                                "- Found an extraneous equal sign (=) between node child arguments and their values. eg. { argument=\"value\" }")
                            },
                            _ => {
                                String::from(kdl_error.help.unwrap_or("Kdl Deserialization Error"))
                            },
                        };
                        let kdl_error = KdlError {
                            error_message,
                            src: Some(NamedSource::new(
                                path.as_path().as_os_str().to_string_lossy(),
                                kdl_config,
                            )),
                            offset: Some(kdl_error.span.offset()),
                            len: Some(kdl_error.span.len()),
                            help_message: None,
                        };
                        Err(ConfigError::KdlError(kdl_error))
                    },
                    Err(ConfigError::KdlError(kdl_error)) => {
                        Err(ConfigError::KdlError(kdl_error.add_src(
                            path.as_path().as_os_str().to_string_lossy().to_string(),
                            kdl_config,
                        )))
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(ConfigError::IoPath(e, path.into())),
        }
    }
    pub fn merge(&mut self, other: Config) -> Result<(), ConfigError> {
        self.options = self.options.merge(other.options);
        Ok(())
    }
    pub fn config_file_path(opts: &CliArgs) -> Option<PathBuf> {
        opts.config.clone().or_else(|| {
            opts.config_dir
                .clone()
                .or_else(home::find_default_config_dir)
                .map(|config_dir| config_dir.join(DEFAULT_CONFIG_FILE_NAME))
        })
    }
    pub fn default_config_file_path() -> Option<PathBuf> {
        home::find_default_config_dir().map(|config_dir| config_dir.join(DEFAULT_CONFIG_FILE_NAME))
    }
    pub fn write_config_to_disk(config: String, opts: &CliArgs) -> Result<Config, Option<PathBuf>> {
        // if we fail, try to return the PathBuf of the file we were not able to write to
        Config::from_kdl(&config, None)
            .map_err(|e| {
                log::error!("Failed to parse config: {}", e);
                None
            })
            .and_then(|parsed_config| {
                let backed_up_file_name = Config::backup_current_config(&opts)?;
                let config_file_path = Config::config_file_path(&opts).ok_or_else(|| {
                    log::error!("Config file path not found");
                    None
                })?;
                let config = match backed_up_file_name {
                    Some(backed_up_file_name) => {
                        format!(
                            "{}{}",
                            Config::autogen_config_message(backed_up_file_name),
                            config
                        )
                    },
                    None => config,
                };
                std::fs::write(&config_file_path, config.as_bytes()).map_err(|e| {
                    log::error!("Failed to write config: {}", e);
                    Some(config_file_path.clone())
                })?;
                let written_config = std::fs::read_to_string(&config_file_path).map_err(|e| {
                    log::error!("Failed to read written config: {}", e);
                    Some(config_file_path.clone())
                })?;
                let parsed_written_config =
                    Config::from_kdl(&written_config, None).map_err(|e| {
                        log::error!("Failed to parse written config: {}", e);
                        None
                    })?;
                if parsed_written_config == parsed_config {
                    Ok(parsed_config)
                } else {
                    log::error!("Configuration corrupted when writing to disk");
                    Err(Some(config_file_path))
                }
            })
    }
    // returns true if the config was not previouly written to disk and we successfully wrote it
    pub fn write_config_to_disk_if_it_does_not_exist(config: String, opts: &CliArgs) -> bool {
        if opts.config.is_none() {
            // if a config file path wasn't explicitly specified, we try to create the default
            // config folder
            home::try_create_home_config_dir();
        }
        match Config::config_file_path(opts) {
            Some(config_file_path) => {
                if config_file_path.exists() {
                    false
                } else {
                    if let Err(e) = std::fs::write(&config_file_path, config.as_bytes()) {
                        log::error!("Failed to write config to disk: {}", e);
                        return false;
                    }
                    match std::fs::read_to_string(&config_file_path) {
                        Ok(written_config) => written_config == config,
                        Err(e) => {
                            log::error!("Failed to read written config: {}", e);
                            false
                        },
                    }
                }
            },
            None => false,
        }
    }
    fn find_free_backup_file_name(config_file_path: &PathBuf) -> Option<PathBuf> {
        let mut backup_config_path = None;
        let config_file_name = config_file_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_else(|| DEFAULT_CONFIG_FILE_NAME);
        for i in 0..100 {
            let new_file_name = if i == 0 {
                format!("{}.bak", config_file_name)
            } else {
                format!("{}.bak.{}", config_file_name, i)
            };
            let mut potential_config_path = config_file_path.clone();
            potential_config_path.set_file_name(new_file_name);
            if !potential_config_path.exists() {
                backup_config_path = Some(potential_config_path);
                break;
            }
        }
        backup_config_path
    }
    fn backup_config_with_written_content_confirmation(
        current_config: &str,
        current_config_file_path: &PathBuf,
        backup_config_path: &PathBuf,
    ) -> bool {
        let _ = std::fs::copy(current_config_file_path, &backup_config_path);
        match std::fs::read_to_string(&backup_config_path) {
            Ok(backed_up_config) => current_config == &backed_up_config,
            Err(e) => {
                log::error!(
                    "Failed to back up config file {}: {:?}",
                    backup_config_path.display(),
                    e
                );
                false
            },
        }
    }
    fn backup_current_config(opts: &CliArgs) -> Result<Option<PathBuf>, Option<PathBuf>> {
        // if we fail, try to return the PathBuf of the file we were not able to write to
        if let Some(config_file_path) = Config::config_file_path(&opts) {
            match std::fs::read_to_string(&config_file_path) {
                Ok(current_config) => {
                    let Some(backup_config_path) =
                        Config::find_free_backup_file_name(&config_file_path)
                    else {
                        log::error!("Failed to find a file name to back up the configuration to, ran out of files.");
                        return Err(None);
                    };
                    if Config::backup_config_with_written_content_confirmation(
                        &current_config,
                        &config_file_path,
                        &backup_config_path,
                    ) {
                        Ok(Some(backup_config_path))
                    } else {
                        log::error!(
                            "Failed to back up config file: {}",
                            backup_config_path.display()
                        );
                        Err(Some(backup_config_path))
                    }
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        Ok(None)
                    } else {
                        log::error!(
                            "Failed to read current config {}: {}",
                            config_file_path.display(),
                            e
                        );
                        Err(Some(config_file_path))
                    }
                },
            }
        } else {
            log::error!("No config file path found?");
            Err(None)
        }
    }
    fn autogen_config_message(backed_up_file_name: PathBuf) -> String {
        format!("//\n// THIS FILE WAS AUTOGENERATED BY ZELLIJ, THE PREVIOUS FILE AT THIS LOCATION WAS COPIED TO: {}\n//\n\n", backed_up_file_name.display())
    }
}

#[cfg(not(target_family = "wasm"))]
pub async fn watch_config_file_changes<F, Fut>(config_file_path: PathBuf, on_config_change: F)
where
    F: Fn(Config) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    // in a gist, what we do here is fire the `on_config_change` function whenever there is a
    // change in the config file, we do this by:
    // 1. Trying to watch the provided config file for changes
    // 2. If the file is deleted or does not exist, we periodically poll for it (manually, not
    //    through filesystem events)
    // 3. Once it exists, we start watching it for changes again
    //
    // we do this because the alternative is to watch its parent folder and this might cause the
    // classic "too many open files" issue if there are a lot of files there and/or lots of Zellij
    // instances
    use crate::setup::Setup;
    use notify::{self, Config as WatcherConfig, Event, PollWatcher, RecursiveMode, Watcher};
    use std::time::Duration;
    use tokio::sync::mpsc;
    loop {
        if config_file_path.exists() {
            let (tx, mut rx) = mpsc::unbounded_channel();

            let mut watcher = match PollWatcher::new(
                move |res: Result<Event, notify::Error>| {
                    let _ = tx.send(res);
                },
                WatcherConfig::default().with_poll_interval(Duration::from_secs(1)),
            ) {
                Ok(watcher) => watcher,
                Err(_) => break,
            };

            if watcher
                .watch(&config_file_path, RecursiveMode::NonRecursive)
                .is_err()
            {
                break;
            }

            while let Some(event_result) = rx.recv().await {
                match event_result {
                    Ok(event) => {
                        if event.paths.contains(&config_file_path) {
                            if event.kind.is_remove() {
                                break;
                            } else if event.kind.is_create() || event.kind.is_modify() {
                                tokio::time::sleep(Duration::from_millis(100)).await;

                                if !config_file_path.exists() {
                                    continue;
                                }

                                let mut cli_args_for_config = CliArgs::default();
                                cli_args_for_config.config = Some(PathBuf::from(&config_file_path));
                                if let Ok(new_config) = Setup::from_cli_args(&cli_args_for_config)
                                    .map_err(|e| e.to_string())
                                {
                                    on_config_change(new_config.0).await;
                                }
                            }
                        }
                    },
                    Err(_) => break,
                }
            }
        }

        while !config_file_path.exists() {
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }
}

#[cfg(test)]
mod config_test {
    use super::*;
    use crate::input::options::OnForceClose;

    use tempfile::tempdir;

    #[test]
    fn try_from_cli_args_with_config() {
        // makes sure loading a config file with --config tries to load the config
        let arbitrary_config = PathBuf::from("nonexistent.yaml");
        let opts = CliArgs {
            config: Some(arbitrary_config),
            ..Default::default()
        };
        let result = Config::try_from(&opts);
        assert!(result.is_err());
    }



    #[test]
    fn try_from_cli_args_with_config_dir_without_config() {
        let mut opts = CliArgs::default();
        let tmp = tempdir().unwrap();
        opts.config_dir = Some(tmp.path().to_path_buf());
        let result = Config::try_from(&opts);
        assert_eq!(result.unwrap(), Config::from_default_assets().unwrap());
    }

    #[test]
    fn try_from_cli_args_default() {
        let opts = CliArgs::default();
        let result = Config::try_from(&opts);
        assert_eq!(result.unwrap(), Config::from_default_assets().unwrap());
    }

    #[test]
    fn can_define_simplified_options_in_configfile() {
        let config_contents = r#"
            on_force_close "quit"
            scroll_buffer_size 100000
            status_bar true
            status_bar_refresh_interval 2
        "#;
        let config = Config::from_kdl(config_contents, None).unwrap();
        assert_eq!(
            config.options.on_force_close,
            Some(OnForceClose::Quit),
            "Option set in config"
        );
        assert_eq!(
            config.options.scroll_buffer_size,
            Some(100000),
            "Option set in config"
        );
        assert_eq!(
            config.options.status_bar,
            Some(true),
            "Option set in config"
        );
        assert_eq!(
            config.options.status_bar_refresh_interval,
            Some(2),
            "Option set in config"
        );
    }
}
