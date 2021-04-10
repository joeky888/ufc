use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, AppSettings, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Ping {}

impl Ping {
    pub fn new() -> App<'static, 'static> {
        App::new("ping")
            .args(&[
                Arg::with_name("url").help("URL destination").required(true),
                Arg::with_name("count").short("c").takes_value(true).help("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
            ])
            .setting(AppSettings::ArgRequiredElseHelp)
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
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
            // ipv6 number
            Palette {
                regexp: Regex::new(r#"(([0-9a-fA-F]{1,4})?\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colours: vec![&Colours::Magenta],
            },
            // icmp_seq=
            Palette {
                regexp: Regex::new(r#"icmp_seq=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // ttl=
            Palette {
                regexp: Regex::new(r#"ttl=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // name
            Palette {
                regexp: Regex::new(r#"(?:[fF]rom|PING)\s(\S+)\s"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Blue],
            },
            // DUP
            Palette {
                regexp: Regex::new(r#"DUP\!"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // OK
            Palette {
                regexp: Regex::new(r#" 0(\.0)?% packet loss"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Errors
            Palette {
                regexp: Regex::new(r#"(Destination Host Unreachable|100(\.0)?% packet loss)"#)
                    .unwrap(),
                colours: vec![&Colours::Red],
            },
            // unknown host
            Palette {
                regexp: Regex::new(r#".+unknown\shost\s(.+)"#).unwrap(),
                colours: vec![&Colours::Red, &Colours::BRed],
            },
            // statistics header
            Palette {
                regexp: Regex::new(r#"--- (\S+) ping statistics ---"#).unwrap(),
                colours: vec![&Colours::BDefault, &Colours::BBlue],
            },
            // last line min/avg/max/mdev
            Palette {
                regexp: Regex::new(r#"rtt (min)/(avg)/(max)/(mdev)"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BYellow,
                    &Colours::BBlue,
                    &Colours::BRed,
                    &Colours::BMagenta,
                ],
            },
            // last line values
            Palette {
                regexp: Regex::new(r#"\=\s([0-9\.]+)\/([0-9\.]+)\/([0-9\.]+)\/([0-9\.]+)"#)
                    .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BYellow,
                    &Colours::BBlue,
                    &Colours::BRed,
                    &Colours::BMagenta,
                ],
            },
            // these are good for nping
            Palette {
                regexp: Regex::new(r#"SENT|RCVD"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // nping
            Palette {
                regexp: Regex::new(r#"unreachable"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // time
            Palette {
                regexp: Regex::new(r#"([0-9\.]+)\s?ms"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
        ]
    }
}
