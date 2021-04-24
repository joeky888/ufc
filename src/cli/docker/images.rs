use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;
pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("images")
            .args(&[
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help("Show all images (default hides intermediate images)"),
                Arg::with_name("digests")
                    .long("digests")
                    .help("Show digests"),
                Arg::with_name("filter")
                    .long("filter")
                    .short("f")
                    .takes_value(true)
                    .help("Filter output based on conditions provided"),
                Arg::with_name("format")
                    .long("format")
                    .takes_value(true)
                    .help("Pretty-print containers using a Go template"),
                Arg::with_name("no-trunc")
                    .long("no-trunc")
                    .help("Don't truncate output"),
                Arg::with_name("quiet")
                    .long("quiet")
                    .short("q")
                    .help("Only display numeric IDs"),
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
                colours: vec![
                    &Colours::Default,
                    &Colours::BWhite,
                    &Colours::BCyan,
                    &Colours::BBlack,
                ],
            },
            Palette {
                // latest
                regexp: Regex::new(r#"(?<=\s)latest(?=\s+)"#).unwrap(),
                colours: vec![&Colours::DCyan],
            },
            Palette {
                // REPOSITORY (Image name)
                regexp: Regex::new(r#"^(?:(\S+)\/)*(\S+)\s"#).unwrap(),
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
                colours: vec![&Colours::UnChanged, &Colours::BRed],
            },
            Palette {
                // Size 'K'
                regexp: Regex::new(r#"(?<=\s)\d+[.,]?\d*\s?(KB?|B)"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            Palette {
                // Size 'M', 2 digits
                regexp: Regex::new(r#"(?<=\s)\d{1,2}[.,]?\d*\s?MB?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            Palette {
                // Size 'M' 3+ digits
                regexp: Regex::new(r#"(?<=\s)\d{3,4}[.,]?\d*\s?MB?"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            Palette {
                // Size 'G'
                regexp: Regex::new(r#"(?<=\s)\d+[.,]?\d*\s?GB?"#).unwrap(),
                colours: vec![&Colours::Red],
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
        ];
        p.reverse();
        p
    }
}
