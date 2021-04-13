use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("ps")
            .args(&[
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help("Show all containers (default shows just running)"),
                Arg::with_name("filter")
                    .long("filter")
                    .short("f")
                    .takes_value(true)
                    .help("Filter output based on conditions provided"),
                Arg::with_name("format")
                    .long("format")
                    .takes_value(true)
                    .help("Pretty-print containers using a Go template"),
                Arg::with_name("last")
                    .long("last")
                    .short("n")
                    .takes_value(true)
                    .help("Show n last created containers (includes all states) (default -1)"),
                Arg::with_name("no-trunc")
                    .long("no-trunc")
                    .help("Don't truncate output"),
                Arg::with_name("quiet")
                    .long("quiet")
                    .short("q")
                    .help("Only display numeric IDs"),
                Arg::with_name("size")
                    .long("size")
                    .short("s")
                    .help("Display total file sizes"),
            ])
            .about("docker ps")
    }

    pub fn parse(_args: &ArgMatches) {
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
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
                    &Colours::UnChanged,
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
                    &Colours::UnChanged,
                    &Colours::BBlack,
                    &Colours::UnChanged,
                    &Colours::BBlack,
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
                regexp: Regex::new(r#"Exited\s.\d+."#).unwrap(),
                colours: vec![&Colours::BRed, &Colours::Red],
            },
            // Statuses -  Restarting
            Palette {
                regexp: Regex::new(r#"Restarting\s.\d+."#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
            Palette {
                // CREATED seconds
                regexp: Regex::new(r#"\s+(\d+\sseconds?\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED About a minute ago
                regexp: Regex::new(r#"\s+(About a minute ago)\s"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED minutes
                regexp: Regex::new(r#"\s+(\d+\sminutes\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED hours
                regexp: Regex::new(r#"\s+(\d+\shours?\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED days
                regexp: Regex::new(r#"\s+(\d+\sdays?\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED weeks
                regexp: Regex::new(r#"\s+(\d+\sweeks?\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged],
            },
            Palette {
                // CREATED months
                regexp: Regex::new(r#"\s+(\d+\smonths?\s\w+)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::UnChanged],
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
        ]
    }
}
