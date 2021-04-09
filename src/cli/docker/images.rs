use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, ArgMatches};
use fancy_regex::Regex;
pub struct Images {}

impl Images {
    pub fn new() -> App<'static, 'static> {
        App::new("images").about("docker images")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Images::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            Palette {
                // REPO, TAG, IMAGE ID
                regexp: Regex::new(r#"^([a-z]+/?[^\s]+)\s+([^\s]+)\s+(\w+)"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BWhite,
                    &Colours::BCyan,
                    &Colours::BBlack,
                ],
            },
            Palette {
                // latest
                regexp: Regex::new(r#"(\s)latest(\s)"#).unwrap(),
                colours: vec![&Colours::DCyan],
            },
            Palette {
                // REPOSITORY (Image name)
                regexp: Regex::new(r#"^(?:(\S+)/)*(\S+)\s"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Yellow, &Colours::BWhite],
            },
            Palette {
                // images without name
                regexp: Regex::new(r#"^<none>.*$"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            Palette {
                // images without tag
                regexp: Regex::new(r#"\s+(<none>)\s+"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::BRed],
            },
            Palette {
                // Size 'G'
                regexp: Regex::new(r#"\s\d+[.,]?\d*\s?GB?"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            Palette {
                // Size 'M' 3+ digits
                regexp: Regex::new(r#"\s\d{3,4}[.,]?\d*\s?MB?"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            Palette {
                // Size 'M', 2 digits
                regexp: Regex::new(r#"\s\d{1,2}[.,]?\d*\s?MB?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            Palette {
                // Size 'K'
                regexp: Regex::new(r#"\s\d+[.,]?\d*\s?(KB?|B)"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            Palette {
                // CREATED seconds/minutes
                regexp: Regex::new(r#"[\da-f]{12}\s+((?:About a|\d+) (?:seconds?|minutes?) ago)"#)
                    .unwrap(),
                colours: vec![&Colours::Default, &Colours::OnGreen, &Colours::BWhite],
            },
            Palette {
                // CREATED About a minute ago
                regexp: Regex::new(r#"\s+(About a minute ago)\s\w+"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::OnGreen, &Colours::BWhite],
            },
            Palette {
                // CREATED hours
                regexp: Regex::new(r#"\s+(\d+\shours\s\w+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::BGreen],
            },
            Palette {
                // CREATED days
                regexp: Regex::new(r#"\s+(\d+\sdays\s\w+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Green],
            },
            Palette {
                // CREATED weeks
                regexp: Regex::new(r#"\s+(\d+\sweeks\s\w+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Yellow],
            },
            Palette {
                // CREATED months
                regexp: Regex::new(r#"\s+(\d+\smonths\s\w+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Red],
            },
            Palette {
                // HEADERS
                regexp: Regex::new(r#"(?:\s|^)(REPOSITORY|TAG|IMAGE ID|CREATED|SIZE)(?:\s|$)"#)
                    .unwrap(),
                colours: vec![&Colours::Default, &Colours::UDefault],
            },
        ]
    }
}
