use clap::{App, ArgMatches};
mod images;
mod ps;
use images::Images;
use ps::Ps;

pub struct Docker {}

impl Docker {
    pub fn new() -> App<'static> {
        App::new("docker")
            .subcommands(vec![Ps::new(), Images::new()])
            .about("docker")
    }

    pub fn parse(app: &ArgMatches) {
        // print!("{:?}", app);
        // args.subcommand()
        match app.subcommand() {
            Some(("ps", args)) => Ps::parse(args),
            Some(("images", args)) => Images::parse(args),
            None => println!("No subcommand was used"),
            _ => {}
        }
    }
}
