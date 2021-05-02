use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;
pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("images")
            .args(&[
                Arg::new("all")
                    .long("all")
                    .short('a')
                    .about("Show all images (default hides intermediate images)"),
                Arg::new("digests").long("digests").about("Show digests"),
                Arg::new("filter")
                    .long("filter")
                    .short('f')
                    .takes_value(true)
                    .about("Filter output based on conditions provided"),
                Arg::new("format")
                    .long("format")
                    .takes_value(true)
                    .about("Pretty-print containers using a Go template"),
                Arg::new("no-trunc")
                    .long("no-trunc")
                    .about("Don't truncate output"),
                Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .about("Only display numeric IDs"),
            ])
            .about("docker images")
    }

    pub fn parse(_args: &ArgMatches) {
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        let mut p = vec![
            Palette {
                // REPO, TAG, IMAGE ID
                regexp: Regex::new(r#"^([a-z]+\/?[^\s]+)\s+([^\s]+)\s+(\w+)"#).unwrap(),
                colors: vec![
                    &Colors::Default,
                    &Colors::BWhite,
                    &Colors::BCyan,
                    &Colors::BBlack,
                ],
            },
            Palette {
                // latest
                regexp: Regex::new(r#"(?<=\s)latest(?=\s+)"#).unwrap(),
                colors: vec![&Colors::DCyan],
            },
            Palette {
                // REPOSITORY (Image name)
                regexp: Regex::new(r#"^(?:(\S+)\/)*(\S+)\s"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::Yellow, &Colors::BWhite],
            },
            Palette {
                // images without name
                regexp: Regex::new(r#"^<none>.*$"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            Palette {
                // images without tag
                regexp: Regex::new(r#"\s+(<none>)\s+"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::BRed],
            },
            Palette {
                // Size 'K'
                regexp: Regex::new(r#"(?<=\s)\d+[.,]?\d*\s?(KB?|B)"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            Palette {
                // Size 'M', 2 digits
                regexp: Regex::new(r#"(?<=\s)\d{1,2}[.,]?\d*\s?MB?"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            Palette {
                // Size 'M' 3+ digits
                regexp: Regex::new(r#"(?<=\s)\d{3,4}[.,]?\d*\s?MB?"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            Palette {
                // Size 'G'
                regexp: Regex::new(r#"(?<=\s)\d+[.,]?\d*\s?GB?"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            Palette {
                // CREATED seconds/minutes
                regexp: Regex::new(r#"[\da-f]{12}\s+((?:About a|\d+) (?:seconds?|minutes?) ago)"#)
                    .unwrap(),
                colors: vec![&Colors::Default, &Colors::OnGreen, &Colors::BWhite],
            },
            Palette {
                // CREATED About a minute ago
                regexp: Regex::new(r#"\s+(About a minute ago)\s\w+"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::OnGreen, &Colors::BWhite],
            },
            Palette {
                // CREATED hours
                regexp: Regex::new(r#"\s+(\d+\shours\s\w+)"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::BGreen],
            },
            Palette {
                // CREATED days
                regexp: Regex::new(r#"\s+(\d+\sdays\s\w+)"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::Green],
            },
            Palette {
                // CREATED weeks
                regexp: Regex::new(r#"\s+(\d+\sweeks\s\w+)"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::Yellow],
            },
            Palette {
                // CREATED months
                regexp: Regex::new(r#"\s+(\d+\smonths\s\w+)"#).unwrap(),
                colors: vec![&Colors::Default, &Colors::Red],
            },
            Palette {
                // HEADERS
                regexp: Regex::new(r#"(?:\s|^)(REPOSITORY|TAG|IMAGE ID|CREATED|SIZE)(?:\s|$)"#)
                    .unwrap(),
                colors: vec![&Colors::Default, &Colors::UDefault],
            },
        ];
        p.reverse();
        p
    }
}
