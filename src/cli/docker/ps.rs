use crate::cli::cli::exec;
use clap::{App, ArgMatches};

pub struct Ps {}

impl Ps {
    pub fn new() -> App<'static> {
        App::new("ps").about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        exec();
    }
}
