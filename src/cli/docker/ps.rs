use crate::cli::cli::{Colours, Palette, exec};
use clap::{App, ArgMatches};
use regex::Regex;

pub struct Ps {}

impl Ps {
    pub fn new() -> App<'static> {
        App::new("ps").about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Ps::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // HEADERS
            Palette {
                regexp: Regex::new(r"(?:\s|^)(CONTAINER ID|IMAGE|COMMAND|CREATED|STATUS|PORTS|NAMES)(?:\s|$)").unwrap(),
                colours: vec![&Colours::Default, &Colours::UnderlineDefault],
            },
        ]
    }
}
