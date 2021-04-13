use clap::{App, AppSettings, Arg, ArgMatches};
mod images;
mod ps;

use super::cli::exec;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("docker")
            .args(&[
                Arg::with_name("config").long("config").takes_value(true).help("Print version information and quit"),
                Arg::with_name("version").long("version").short("v").help("Print version information and quit"),
                Arg::with_name("debug").long("debug").short("D").help("Enable debug mode"),
                Arg::with_name("host").long("host").short("H").takes_value(true).help("Daemon socket(s) to connect to"),
                Arg::with_name("log").long("log-level").short("l").takes_value(true).help(r#"Set the logging level ("debug"|"info"|"warn"|"error"|"fatal") (default "info")"#),
            ])
            // .setting(AppSettings::NeedsSubcommandHelp)
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommands(vec![ps::Cmd::new(), images::Cmd::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        match app.subcommand() {
            ("ps", Some(args)) => ps::Cmd::parse(args),
            ("images", Some(args)) => images::Cmd::parse(args),
            _ => exec(vec![]),
        }
    }
}
