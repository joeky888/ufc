// use crate::cli::cli::exec;
// use crate::cli::cli::Colours;
// use crate::cli::cli::Palette;
use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use regex::Regex;

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
        exec(Ping::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // IP
            Palette {
                regexp: Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap(),
                colours: vec![&Colours::BoldBlue],
            },
            // icmp_seq=
            Palette {
                regexp: Regex::new(r"icmp_seq=(\d+)").unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // ttl=
            Palette {
                regexp: Regex::new(r"ttl=(\d+)").unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // time
            Palette {
                regexp: Regex::new(r"([0-9\.]+)\s?ms").unwrap(),
                colours: vec![&Colours::Green, &Colours::BoldGreen],
            },
        ]
    }
}
