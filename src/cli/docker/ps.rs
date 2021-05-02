use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("ps")
            .args(&[
                Arg::new("all")
                    .long("all")
                    .short('a')
                    .about("Show all containers (default shows just running)"),
                Arg::new("filter")
                    .long("filter")
                    .short('f')
                    .takes_value(true)
                    .about("Filter output based on conditions provided"),
                Arg::new("format")
                    .long("format")
                    .takes_value(true)
                    .about("Pretty-print containers using a Go template"),
                Arg::new("last")
                    .long("last")
                    .short('n')
                    .takes_value(true)
                    .about("Show n last created containers (includes all states) (default -1)"),
                Arg::new("no-trunc")
                    .long("no-trunc")
                    .about("Don't truncate output"),
                Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .about("Only display numeric IDs"),
                Arg::new("size")
                    .long("size")
                    .short('s')
                    .about("Display total file sizes"),
            ])
            .about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // HEADERS
            Palette {
                regexp: Regex::new(
                    r#"(?:\s|^)(CONTAINER ID|IMAGE|COMMAND|CREATED|STATUS|PORTS|NAMES)(?:\s|$)"#,
                )
                .unwrap(),
                colors: vec![&Colors::Default, &Colors::UDefault],
            },
            // IMAGE NAME (as docker image)
            Palette {
                regexp: Regex::new(r#"\s{2,}(?:([a-z\-_0-9]+)\/)*([a-z\-_0-9]+)(:\S+)?\s{2,}\""#)
                    .unwrap(),
                colors: vec![
                    &Colors::UnChanged,
                    &Colors::Yellow,
                    &Colors::BWhite,
                    &Colors::Cyan,
                ],
            },
            // IMAGE
            Palette {
                regexp: Regex::new(r#"^(?!CONTAINER)(\w+)\s+([^\s]+)\s+(".*")\s+(.*(?=(?:Up|Exited|Created|Restarting)))"#)
                    .unwrap(),
                colors: vec![
                    &Colors::UnChanged,
                    &Colors::BBlack,
                    &Colors::UnChanged,
                    &Colors::BBlack,
                    &Colors::Cyan,
                ],
            },
            // Statuses - Created
            Palette {
                regexp: Regex::new(r#"\sCreated\s"#)
                    .unwrap(),
                colors: vec![&Colors::Blue],
            },
            // Statuses
            // https://github.com/docker/docker/blob/e5a3f86e447dd659da3c2e759f3c088a0bfcfe3d/container/state.go#L40
            // Up
            Palette {
                regexp: Regex::new(r#"(?:\s{2}|^)(?:Up|Restarting)(?:(?:\s[\w,\d]+)+)?"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
            // Health - healthy
            Palette {
                regexp: Regex::new(r#"\s\(healthy\)"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
            // Health -  starting
            Palette {
                regexp: Regex::new(r#"\s\(health: starting\)"#).unwrap(),
                colors: vec![&Colors::BYellow],
            },
            // Health -  unhealthy
            Palette {
                regexp: Regex::new(r#"\s\(unhealthy\)"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Statuses -  Exited
            Palette {
                regexp: Regex::new(r#"Exited\s.\d+."#).unwrap(),
                colors: vec![&Colors::BRed, &Colors::Red],
            },
            // Statuses -  Restarting
            Palette {
                regexp: Regex::new(r#"Restarting\s.\d+."#).unwrap(),
                colors: vec![&Colors::BBlue],
            },
            Palette {
                // CREATED seconds
                regexp: Regex::new(r#"\s+(\d+\sseconds?\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED About a minute ago
                regexp: Regex::new(r#"\s+(About a minute ago)\s"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED minutes
                regexp: Regex::new(r#"\s+(\d+\sminutes\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED hours
                regexp: Regex::new(r#"\s+(\d+\shours?\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED days
                regexp: Regex::new(r#"\s+(\d+\sdays?\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED weeks
                regexp: Regex::new(r#"\s+(\d+\sweeks?\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged],
            },
            Palette {
                // CREATED months
                regexp: Regex::new(r#"\s+(\d+\smonths?\s\w+)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::UnChanged],
            },
            // Ip Addresses
            Palette {
                regexp: Regex::new(r#"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(\:)?"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::Blue, &Colors::Default],
            },
            // Ports
            Palette {
                regexp: Regex::new(
                    r#"(\d{1,5})?(-)?(\d{1,5})?(->)?(\d{1,5})(-)?(\d{1,5})?(\/)(tcp|udp)"#,
                )
                .unwrap(),
                colors: vec![
                    &Colors::Default,
                    &Colors::BGreen,
                    &Colors::Default,
                    &Colors::BGreen,
                    &Colors::Default,
                    &Colors::BGreen,
                    &Colors::Default,
                    &Colors::BGreen,
                    &Colors::Default,
                    &Colors::Cyan,
                ],
            },
            // NAMES
            Palette {
                regexp: Regex::new(r#"(?:([a-z\-_0-9]+)\/)*([a-z\-_0-9]+)$"#).unwrap(),
                colors: vec![
                    &Colors::Default,
                    &Colors::Yellow,
                    &Colors::OnBlue,
                    &Colors::White,
                ],
            },
        ]
    }
}
