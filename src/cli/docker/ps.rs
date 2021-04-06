use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use regex::Regex;

pub struct Ps {}

impl Ps {
    pub fn new() -> App<'static> {
        App::new("ps")
            .arg(
                Arg::new("all")
                    .short('a')
                    .about("Show all containers (default shows just running)"),
            )
            .about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Ps::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // HEADERS
            Palette {
                regexp: Regex::new(
                    r"(?:\s|^)(CONTAINER ID|IMAGE|COMMAND|CREATED|STATUS|PORTS|NAMES)(?:\s|$)",
                )
                .unwrap(),
                colours: vec![&Colours::Default, &Colours::UnderlineDefault],
            },
            // IMAGE NAME (as docker image)
            Palette {
                regexp: Regex::new(r#"\s{2,}(?:([a-z\-_0-9]+)/)*([a-z\-_0-9]+)(:\S+)?\s{2,}"#)
                    .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::Yellow,
                    &Colours::BoldWhite,
                    &Colours::Cyan,
                ],
            },
            // Container ID
            Palette {
                regexp: Regex::new(r#"^\w+"#).unwrap(),
                colours: vec![&Colours::UnderlineBoldWhite],
            },
            // Statuses - Created
            Palette {
                regexp: Regex::new(r#"\s(\d+)(months|weeks|days|hours|minutes|seconds)\sago"#).unwrap(),
                colours: vec![&Colours::Blue],
            },
            // Statuses
            // https://github.com/docker/docker/blob/e5a3f86e447dd659da3c2e759f3c088a0bfcfe3d/container/state.go#L40
            // Up
            Palette {
                regexp: Regex::new(r#"(?:\s{2}|^)(?:Up|Restarting)(?:(?:\s[\w,\d]+)+)?"#).unwrap(),
                colours: vec![&Colours::BoldGreen],
            },
            // Health - healthy
            Palette {
                regexp: Regex::new(r#"\s\(healthy\)"#).unwrap(),
                colours: vec![&Colours::BoldGreen],
            },
            // Health -  starting
            Palette {
                regexp: Regex::new(r#"\s\(health: starting\)"#).unwrap(),
                colours: vec![&Colours::BoldYellow],
            },
            // Health -  unhealthy
            Palette {
                regexp: Regex::new(r#"\s\(unhealthy\)"#).unwrap(),
                colours: vec![&Colours::BoldRed],
            },
            // Statuses -  Exited
            Palette {
                regexp: Regex::new(r#"Exited\s.(\d+).+$"#).unwrap(),
                colours: vec![&Colours::BoldRed, &Colours::Red],
            },
            // Statuses -  Restarting
            Palette {
                regexp: Regex::new(r#"Restarting\s.(\d+).+$"#).unwrap(),
                colours: vec![&Colours::BoldBlue],
            },
            // Ip Addresses
            Palette {
                regexp: Regex::new(r#"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(:)?"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Blue, &Colours::Default],
            },
            // Ports
            Palette {
                regexp: Regex::new(
                    r#"(\d{1,5})?(-)?(\d{1,5})?(->)?(\d{1,5})(-)?(\d{1,5})?(/)(tcp|udp)"#,
                )
                .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BoldGreen,
                    &Colours::Default,
                    &Colours::BoldGreen,
                    &Colours::Default,
                    &Colours::BoldGreen,
                    &Colours::Default,
                    &Colours::BoldGreen,
                    &Colours::Default,
                    &Colours::Cyan,
                ],
            },
            // NAMES
            // Palette {
            //     regexp: Regex::new(r#"(?:([a-z\-_0-9]+)/)*([a-z\-_0-9]+)$"#).unwrap(),
            //     colours: vec![
            //         &Colours::Default,
            //         &Colours::Yellow,
            //         &Colours::OnBlue,
            //         &Colours::White,
            //     ],
            // },
        ]
    }
}
