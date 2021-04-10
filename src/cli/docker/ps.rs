use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Ps {}

impl Ps {
    pub fn new() -> App<'static, 'static> {
        App::new("ps")
            .arg(
                Arg::with_name("all")
                    .short("a")
                    .required(false)
                    .help("Show all containers (default shows just running)"),
            )
            .about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Ps::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        let mut p = vec![
            // HEADERS
            Palette {
                regexp: Regex::new(
                    r#"(?:\s|^)(CONTAINER ID|IMAGE|COMMAND|CREATED|STATUS|PORTS|NAMES)(?:\s|$)"#,
                )
                .unwrap(),
                colours: vec![&Colours::Default, &Colours::UDefault],
            },
            // IMAGE NAME (as docker image)
            Palette {
                regexp: Regex::new(r#"\s{2,}(?:([a-z\-_0-9]+)\/)*([a-z\-_0-9]+)(:\S+)?\s{2,}\""#)
                    .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::Yellow,
                    &Colours::BWhite,
                    &Colours::Cyan,
                ],
            },
            // IMAGE
            Palette {
                regexp: Regex::new(r#"^(?!CONTAINER)(\w+)\s+([^\s]+)\s+(".*")\s+(.*(?=(?:Up|Exited|Created|Restarting)))"#)
                    .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::Yellow,
                    &Colours::BWhite,
                    &Colours::Cyan,
                ],
            },
            // Statuses - Created
            Palette {
                regexp: Regex::new(r#"\sCreated\s"#)
                    .unwrap(),
                colours: vec![&Colours::Blue],
            },
            // Statuses
            // https://github.com/docker/docker/blob/e5a3f86e447dd659da3c2e759f3c088a0bfcfe3d/container/state.go#L40
            // Up
            Palette {
                regexp: Regex::new(r#"(?:\s{2}|^)(?:Up|Restarting)(?:(?:\s[\w,\d]+)+)?"#).unwrap(),
                colours: vec![&Colours::BGreen],
            },
            // Health - healthy
            Palette {
                regexp: Regex::new(r#"\s\(healthy\)"#).unwrap(),
                colours: vec![&Colours::BGreen],
            },
            // Health -  starting
            Palette {
                regexp: Regex::new(r#"\s\(health: starting\)"#).unwrap(),
                colours: vec![&Colours::BYellow],
            },
            // Health -  unhealthy
            Palette {
                regexp: Regex::new(r#"\s\(unhealthy\)"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Statuses -  Exited
            Palette {
                regexp: Regex::new(r#"Exited\s.(\d+).+?(?=\s{2,})"#).unwrap(),
                colours: vec![&Colours::BRed, &Colours::Red],
            },
            // Statuses -  Restarting
            Palette {
                regexp: Regex::new(r#"Restarting\s.(\d+).+?(?=\s{2,})"#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
            // Ip Addresses
            Palette {
                regexp: Regex::new(r#"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(\:)?"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Blue, &Colours::Default],
            },
            // Ports
            Palette {
                regexp: Regex::new(
                    r#"(\d{1,5})?(-)?(\d{1,5})?(->)?(\d{1,5})(-)?(\d{1,5})?(\/)(tcp|udp)"#,
                )
                .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BGreen,
                    &Colours::Default,
                    &Colours::BGreen,
                    &Colours::Default,
                    &Colours::BGreen,
                    &Colours::Default,
                    &Colours::BGreen,
                    &Colours::Default,
                    &Colours::Cyan,
                ],
            },
            // NAMES
            Palette {
                regexp: Regex::new(r#"(?:([a-z\-_0-9]+)\/)*([a-z\-_0-9]+)$"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::Yellow,
                    &Colours::OnBlue,
                    &Colours::White,
                ],
            },
        ];
        p.reverse();
        p
    }
}
