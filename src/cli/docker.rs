use clap::{App, AppSettings, ArgMatches};
mod images;
mod ps;
use images::Images;
use ps::Ps;

pub struct Docker {}

impl Docker {
    pub fn new() -> App<'static, 'static> {
        App::new("docker")
            .setting(AppSettings::NeedsSubcommandHelp)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommands(vec![Ps::new(), Images::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        match app.subcommand() {
            ("ps", Some(args)) => Ps::parse(args),
            ("images", Some(args)) => Images::parse(args),
            _ => println!("Unsupported command"),
        }
    }
}
