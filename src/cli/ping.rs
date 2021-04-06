use crate::cli::cli::exec;
use clap::{App, Arg, ArgMatches};

pub struct Ping {}

impl Ping {
    pub fn new() -> App<'static> {
        App::new("ping")
            .args(&[
                Arg::new("count").short('c').about("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
            ])
            .about("ping")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec();
    }
}
