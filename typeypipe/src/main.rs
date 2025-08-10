mod commands;

use clap::Parser;
use zellij_utils::{
    cli::CliArgs,
    consts::create_config_and_cache_folders,
    logging::*,
};

fn main() {
    configure_logger();
    create_config_and_cache_folders();
    let opts = CliArgs::parse();

    if let Some(path) = opts.server {
        commands::start_server(path, opts.debug);
    } else {
        commands::start_client(opts);
    }
}
