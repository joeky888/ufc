use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("ifconfig")
            .args(&[
                Arg::new("interface")
                    .min_values(0)
                    .about("The name of the interface."),
                Arg::new("all")
                    .short('a')
                    .about("Display all interfaces which are currently available, even if down"),
                Arg::new("short")
                    .short('s')
                    .about("Display a short list (like netstat -i)"),
                Arg::new("verbose")
                    .short('v')
                    .about("Be more verbose for some error conditions"),
            ])
            .about("ifconfig")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            Palette {
                regexp: Regex::new(r#"collisions[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            Palette {
                regexp: Regex::new(r#"carrier[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::Cyan],
            },
            Palette {
                regexp: Regex::new(r#"frame[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::White],
            },
            Palette {
                regexp: Regex::new(r#"overruns[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            Palette {
                regexp: Regex::new(r#"dropped[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::White],
            },
            // errors
            Palette {
                regexp: Regex::new(r#"errors[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // mtu
            Palette {
                regexp: Regex::new(r#"(?i)mtu[\s|\:]\d+"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // flags
            Palette {
                regexp: Regex::new(r#"(?<=[,<])[^,]+?(?=[,>])"#).unwrap(),
                colors: vec![&Colors::Blue],
            },
            // ip disc
            Palette {
                regexp: Regex::new(r#"(inet6?|netmask|broadcast)"#).unwrap(),
                colors: vec![&Colors::Cyan],
            },
            // interface
            Palette {
                regexp: Regex::new(r#"^([a-z0-9.]{2,}\d*):?\s"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
            // size [T\|G\|M\|K]i?B
            Palette {
                regexp: Regex::new(r#"\d+\.?\d*\s+[T|G|M|K]?i?B"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // hwaddr
            Palette {
                regexp: Regex::new(r#"[\d[a-f]]{2}(\:[\d[a-f]]{2}){5}"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // ipv6
            Palette {
                regexp: Regex::new(r#"\b[0-9a-fA-F]{1,4}(\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
            // ipv4
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
        ]
    }
}
