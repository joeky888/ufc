use clap::{App, ArgMatches};
mod images;
mod ps;
use images::Images;
use ps::Ps;

pub struct Docker {}

impl Docker {
    pub fn new() -> App<'static, 'static> {
        App::new("docker")
            .subcommands(vec![Ps::new(), Images::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        // print!("{:?}", app);
        // args.subcommand()
        match app.subcommand() {
            ("ps", Some(args)) => Ps::parse(args),
            ("images", Some(args)) => Images::parse(args),
            _ => println!("Unsupported command"),
        }
    }
}
