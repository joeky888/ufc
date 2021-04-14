use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, AppSettings, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("fdisk")
            .args(&[
                Arg::with_name("FILE").help("[FILE]"),
                Arg::with_name("list").long("list").short("l").help("List the partition tables for the specified devices and then exit. If no devices are given, those mentioned in /proc/partitions (if that file exists) are used."),
            ])
            .setting(AppSettings::ArgRequiredElseHelp)
            .about("fdisk")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d+[.,]?\d*\s?Gi?B?"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d*\s?Mi?B?"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d*\s?Ki?B?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // ID
            Palette {
                regexp: Regex::new(r#"identifier: (.*)$"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::Cyan],
            },
            // Type
            Palette {
                regexp: Regex::new(r#"type: (.*)$"#).unwrap(),
                colours: vec![&Colours::UnChanged,&Colours::BCyan],
            },
            // Partitions
            Palette {
                regexp: Regex::new(r#"^(?:\/([^\/: ]+))+"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
            // Boot?
            Palette {
                regexp: Regex::new(r#"\*\s\s\s"#).unwrap(),
                colours: vec![&Colours::OnRed, &Colours::BWhite],
            },
            // Disk
            Palette {
                regexp: Regex::new(r#"^(Disk) (?:\/([^\/: ]+))+"#).unwrap(),
                colours: vec![&Colours::Yellow, &Colours::OnYellow, &Colours::BYellow, &Colours::BYellow],
            },
            // Error
            Palette {
                regexp: Regex::new(r#"fdisk: cannot open ([^:]+).*$"#).unwrap(),
                colours: vec![&Colours::Red, &Colours::BRed],
            },
        ]
    }
}
