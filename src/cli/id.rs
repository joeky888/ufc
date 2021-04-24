use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("id")
            .args(&[
                Arg::with_name("a")
                    .short("a")
                    .help("ignore, for compatibility with other versions"),
                Arg::with_name("context")
                    .long("context")
                    .short("Z")
                    .help("print only the security context of the process"),
                Arg::with_name("group")
                    .long("group")
                    .short("g")
                    .help("print only the effective group ID"),
                Arg::with_name("groups")
                    .long("groups")
                    .short("G")
                    .help("print all group IDs"),
                Arg::with_name("name")
                    .long("name")
                    .short("n")
                    .help("print a name instead of a number, for -ugG"),
                Arg::with_name("real")
                    .long("real")
                    .short("r")
                    .help("print the real ID instead of the effective ID, with -ugG"),
                Arg::with_name("user")
                    .long("user")
                    .short("u")
                    .help("print only the effective user ID"),
                Arg::with_name("zero")
                    .long("zero")
                    .short("z")
                    .help("delimit entries with NUL characters, not whitespace"),
                Arg::with_name("help")
                    .long("help")
                    .help("display this help and exit"),
                Arg::with_name("version")
                    .long("version")
                    .help("output version information and exit"),
            ])
            .about("id")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // SELinux
            Palette {
                regexp: Regex::new(r#"(\w+_u):(\w+_r):(\w+_t):([\w\-.:]+)"#).unwrap(),
                colours: vec![
                    &Colours::UnChanged,
                    &Colours::Green,
                    &Colours::Yellow,
                    &Colours::Cyan,
                    &Colours::Magenta,
                ],
            },
            // User
            Palette {
                regexp: Regex::new(r#"uid.(\d+)\((\w+)\)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::Green, &Colours::BGreen],
            },
            // Groups
            Palette {
                regexp: Regex::new(r#"(\d+)\((\w+)\)"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::Yellow, &Colours::BYellow],
            },
        ]
    }
}
