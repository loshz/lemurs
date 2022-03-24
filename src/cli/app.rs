use anyhow::Error;
use clap::{app_from_crate, App, AppSettings, Arg};

use super::*;

pub struct Cli {
    app: App<'static>,
}

impl Cli {
    pub fn new() -> Cli {
        // Install subcommand.
        let install = App::new(install::COMMAND_NAME)
            .setting(AppSettings::DisableVersionFlag)
            .about("Install default config and systemd service files.")
            .arg(
                Arg::new("config")
                    .long("conf")
                    .value_name("PATH")
                    .help("Path to the config file installation location")
                    .default_value(install::DEFAULT_CONFIG_PATH),
            )
            .arg(
                Arg::new("service")
                    .long("service")
                    .value_name("PATH")
                    .help("Path to the systemd service file installation location")
                    .default_value(install::DEFAULT_SERVICE_PATH),
            );

        // Start subcommand.
        let start = App::new(start::COMMAND_NAME)
            .setting(AppSettings::DisableVersionFlag)
            .about("Start the daemon.")
            .arg(
                Arg::new("config")
                    .long("conf")
                    .value_name("PATH")
                    .help("Path to the config file location")
                    .default_value(install::DEFAULT_CONFIG_PATH),
            );

        // Uninstall subcommand.
        let uninstall = App::new(uninstall::COMMAND_NAME)
            .setting(AppSettings::DisableVersionFlag)
            .about("Remove config and systemd service files.")
            .arg(
                Arg::new("config")
                    .long("conf")
                    .value_name("PATH")
                    .help("Path to the config file location")
                    .default_value(install::DEFAULT_CONFIG_PATH),
            )
            .arg(
                Arg::new("service")
                    .long("service")
                    .value_name("PATH")
                    .help("Path to the systemd service file location")
                    .default_value(install::DEFAULT_SERVICE_PATH),
            );

        // Create cli app from crate info.
        let app = app_from_crate!()
            .global_setting(AppSettings::PropagateVersion)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(install)
            .subcommand(start)
            .subcommand(uninstall);

        Cli { app }
    }

    pub fn run(self) -> Result<(), Error> {
        match self.app.get_matches().subcommand() {
            Some((install::COMMAND_NAME, args)) => install::run(args),
            Some((start::COMMAND_NAME, args)) => start::run(args),
            Some((uninstall::COMMAND_NAME, args)) => uninstall::run(args),
            _ => Err(Error::msg("exhausted list of subcommands".to_owned())),
        }
    }
}