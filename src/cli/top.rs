use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("top").about("top")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Header
            Palette {
                regexp: Regex::new(r#"\s+PID.+COMMAND.+$"#).unwrap(),
                colors: vec![&Colors::BlackOnGreen],
            },
            // Swap
            Palette {
                regexp: Regex::new(r#"Swap"#).unwrap(),
                colors: vec![&Colors::BMagenta],
            },
            // Mem
            Palette {
                regexp: Regex::new(r#"Mem"#).unwrap(),
                colors: vec![&Colors::BCyan],
            },
            // Time
            Palette {
                regexp: Regex::new(r#"\d+:\d+[:\.]\d+"#).unwrap(),
                colors: vec![&Colors::BBlue],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dt|\b\d{10,12}\b"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dg|\b\d{7,9}\b"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dm|\b\d{4,6}\b"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dk?"#).unwrap(),
                colors: vec![&Colors::Green],
            },
        ]
    }
}
