use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("dig")
            .args(&[
                Arg::with_name("server").help("server is the name or IP address of the name server to query"),
                // Arg::with_name("name").help("name is the name of the resource record that is to be looked up"),
                // Arg::with_name("type").help("type indicates what type of query is required - ANY, A, MX, SIG, etc.  type can be any valid query type"),
                Arg::with_name("4").short("4").help("IPv4 should be used"),
                Arg::with_name("6").short("6").help("IPv4 should be used"),
                Arg::with_name("b").short("b").takes_value(true).help("This option sets the source IP address of the query"),
                Arg::with_name("class").short("c").takes_value(true).help("This option sets the query class. The default class is IN"),
                Arg::with_name("file").short("f").takes_value(true).help("This option sets batch mode, in which dig reads a list of lookup requests to process from the given file"),
                Arg::with_name("keyfile").short("k").takes_value(true).help("This option tells named to sign queries using TSIG using a key read from the given file"),
                Arg::with_name("m").short("m").help("This option enables memory usage debugging"),
                Arg::with_name("port").short("p").takes_value(true).help("This option sends the query to a non-standard port on the server, instead of the default port 53"),
                Arg::with_name("name").short("q").takes_value(true).help("This option specifies the domain name to query"),
                Arg::with_name("r").short("r").help("This option indicates that options from ${HOME}/.digrc should not be read"),
                Arg::with_name("type").short("t").takes_value(true).help("This option indicates the resource record type to query, which can be any valid query type"),
                Arg::with_name("u").short("u").help("This option indicates that print query times should be provided in microseconds instead of milliseconds"),
                Arg::with_name("v").short("v").help("This option prints the version number and exits"),
                Arg::with_name("addr").short("x").help("This option sets simplified reverse lookups, for mapping addresses to names"),
                Arg::with_name("TSIG").short("y").help("[hmac:]keyname:secret This option signs queries using TSIG with the given authentication key"),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("dig")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Title
            Palette {
                regexp: Regex::new(r#"; <<>> DiG.* <<>> (\S+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::BMagenta],
            },
            // comments
            Palette {
                regexp: Regex::new(r#"^;;[\s\w]+"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // ipv6
            Palette {
                regexp: Regex::new(r#"\t(([0-9a-fA-F]{1,4})?\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colours: vec![&Colours::DGreen],
            },
            // ip4 address
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // line
            Palette {
                regexp: Regex::new(r#"^(\S+).*?(\d+)\t(\w+)\t(\w+)\t"#).unwrap(),
                colours: vec![
                    &Colours::UnChanged,
                    &Colours::Magenta,
                    &Colours::Red,
                    &Colours::Yellow,
                    &Colours::Cyan,
                ],
            },
            // domain
            Palette {
                regexp: Regex::new(r#"[\S]+\."#).unwrap(),
                colours: vec![&Colours::BMagenta],
            },
        ]
    }
}
