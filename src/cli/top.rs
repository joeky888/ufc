use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("top").args(&[]).about("top")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Header
            Palette {
                regexp: Regex::new(r#"\s+PID.+COMMAND.+$"#).unwrap(),
                colours: vec![&Colours::BlackOnGreen],
            },
            // Swap
            Palette {
                regexp: Regex::new(r#"Swap"#).unwrap(),
                colours: vec![&Colours::BMagenta],
            },
            // Mem
            Palette {
                regexp: Regex::new(r#"Mem"#).unwrap(),
                colours: vec![&Colours::BCyan],
            },
            // Time
            Palette {
                regexp: Regex::new(r#"\d+:\d+[:\.]\d+"#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dt|\b\d{10,12}\b"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dg|\b\d{7,9}\b"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dm|\b\d{4,6}\b"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dk?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
        ]
    }
}
