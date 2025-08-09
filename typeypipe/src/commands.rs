use std::{path::PathBuf, process};

use nix;
use zellij_client::{
    os_input_output::get_client_os_input,
    start_client as start_client_impl, ClientInfo,
};
use zellij_utils::sessions::generate_unique_session_name;

use miette::Report;
use zellij_server::{os_input_output::get_server_os_input, start_server as start_server_impl};
use zellij_utils::{
    cli::CliArgs,
    input::config::ConfigError,
    setup::Setup,
};



fn get_os_input<OsInputOutput>(
    fn_get_os_input: fn() -> Result<OsInputOutput, nix::Error>,
) -> OsInputOutput {
    match fn_get_os_input() {
        Ok(os_input) => os_input,
        Err(e) => {
            eprintln!("failed to open terminal:\n{}", e);
            process::exit(1);
        },
    }
}

pub(crate) fn start_server(path: PathBuf, debug: bool) {
    // Set instance-wide debug mode
    zellij_utils::consts::DEBUG_MODE.set(debug).unwrap();
    let os_input = get_os_input(get_server_os_input);
    start_server_impl(Box::new(os_input), path);
}



fn generate_unique_session_name_or_exit() -> String {
    let Some(unique_session_name) = generate_unique_session_name() else {
        eprintln!("Failed to generate a unique session name, giving up");
        process::exit(1);
    };
    unique_session_name
}
























pub(crate) fn start_client(opts: CliArgs) {
    let (_config, _layout, _config_options, _, _) = match Setup::from_cli_args(&opts) {
        Ok(results) => results,
        Err(e) => {
            if let ConfigError::KdlError(error) = e {
                let report: Report = error.into();
                eprintln!("{:?}", report);
            } else {
                eprintln!("{}", e);
            }
            process::exit(1);
        },
    };

    let os_input = get_os_input(get_client_os_input);
    let session_name = opts.session.clone().unwrap_or_else(|| generate_unique_session_name_or_exit());
    let client = ClientInfo::New(session_name);

    start_client_impl(
        Box::new(os_input),
        opts,
        client,
    );
}




