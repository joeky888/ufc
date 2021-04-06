use crate::cli::cli::{Palette, exec};
use clap::{App, ArgMatches};

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
        ]
    }
}
