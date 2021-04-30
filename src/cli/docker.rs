use clap::{App, AppSettings, Arg, ArgMatches};
mod images;
mod ps;

use super::cli::pre_exec;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("docker")
            .args(&[
                Arg::new("config").long("config").takes_value(true).about("Print version information and quit"),
                Arg::new("version").long("version").short('v').about("Print version information and quit"),
                Arg::new("debug").long("debug").short('D').about("Enable debug mode"),
                Arg::new("host").long("host").short('H').takes_value(true).about("Daemon socket(s) to connect to"),
                Arg::new("log").long("log-level").short('l').takes_value(true).about(r#"Set the logging level ("debug"|"info"|"warn"|"error"|"fatal") (default "info")"#),
            ])
            // .setting(AppSettings::NeedsSubcommandHelp)
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommands(vec![ps::Cmd::new(), images::Cmd::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        match app.subcommand() {
            Some(("ps", args)) => ps::Cmd::parse(args),
            Some(("images", args)) => images::Cmd::parse(args),
            _ => pre_exec(vec![]),
        }
    }
}
