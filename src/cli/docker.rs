use clap::{App, Arg, ArgMatches};
mod images;
mod ps;
use images::Images;
use ps::Ps;

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
            .subcommands(vec![Ps::new(), Images::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        match app.subcommand() {
            ("ps", Some(args)) => Ps::parse(args),
            ("images", Some(args)) => Images::parse(args),
            _ => exec(vec![]),
        }
    }
}
