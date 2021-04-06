use crate::cli::cli::{exec, Palette};
use clap::{App, ArgMatches};
pub struct Images {}

impl Images {
    pub fn new() -> App<'static, 'static> {
        App::new("images").about("docker images")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Images::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![]
    }
}
