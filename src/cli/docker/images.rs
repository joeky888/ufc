use crate::cli::cli::exec;
use clap::{App, ArgMatches};
pub struct Images {}

impl Images {
    pub fn new() -> App<'static> {
        App::new("images").about("docker images")
    }

    pub fn parse(_args: &ArgMatches) {
        exec();
    }
}
