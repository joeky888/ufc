use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("dig")
            .args(&[
                Arg::new("server").about("server is the name or IP address of the name server to query"),
                // Arg::new("name").about("name is the name of the resource record that is to be looked up"),
                // Arg::new("type").about("type indicates what type of query is required - ANY, A, MX, SIG, etc.  type can be any valid query type"),
                Arg::new("4").short('4').about("IPv4 should be used"),
                Arg::new("6").short('6').about("IPv4 should be used"),
                Arg::new("b").short('b').takes_value(true).about("This option sets the source IP address of the query"),
                Arg::new("class").short('c').takes_value(true).about("This option sets the query class. The default class is IN"),
                Arg::new("file").short('f').takes_value(true).about("This option sets batch mode, in which dig reads a list of lookup requests to process from the given file"),
                Arg::new("keyfile").short('k').takes_value(true).about("This option tells named to sign queries using TSIG using a key read from the given file"),
                Arg::new("m").short('m').about("This option enables memory usage debugging"),
                Arg::new("port").short('p').takes_value(true).about("This option sends the query to a non-standard port on the server, instead of the default port 53"),
                Arg::new("name").short('q').takes_value(true).about("This option specifies the domain name to query"),
                Arg::new("r").short('r').about("This option indicates that options from ${HOME}/.digrc should not be read"),
                Arg::new("type").short('t').takes_value(true).about("This option indicates the resource record type to query, which can be any valid query type"),
                Arg::new("u").short('u').about("This option indicates that print query times should be provided in microseconds instead of milliseconds"),
                Arg::new("v").short('v').about("This option prints the version number and exits"),
                Arg::new("addr").short('x').about("This option sets simplified reverse lookups, for mapping addresses to names"),
                Arg::new("TSIG").short('y').about("[hmac:]keyname:secret This option signs queries using TSIG with the given authentication key"),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("dig")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
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
