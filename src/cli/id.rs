use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("id")
            .args(&[
                Arg::new("a")
                    .short('a')
                    .about("ignore, for compatibility with other versions"),
                Arg::new("context")
                    .long("context")
                    .short('Z')
                    .about("print only the security context of the process"),
                Arg::new("group")
                    .long("group")
                    .short('g')
                    .about("print only the effective group ID"),
                Arg::new("groups")
                    .long("groups")
                    .short('G')
                    .about("print all group IDs"),
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .about("print a name instead of a number, for -ugG"),
                Arg::new("real")
                    .long("real")
                    .short('r')
                    .about("print the real ID instead of the effective ID, with -ugG"),
                Arg::new("user")
                    .long("user")
                    .short('u')
                    .about("print only the effective user ID"),
                Arg::new("zero")
                    .long("zero")
                    .short('z')
                    .about("delimit entries with NUL characters, not whitespace"),
                Arg::new("help")
                    .long("help")
                    .about("display this help and exit"),
                Arg::new("version")
                    .long("version")
                    .about("output version information and exit"),
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
                colors: vec![
                    &Colors::UnChanged,
                    &Colors::Green,
                    &Colors::Yellow,
                    &Colors::Cyan,
                    &Colors::Magenta,
                ],
            },
            // User
            Palette {
                regexp: Regex::new(r#"uid.(\d+)\((\w+)\)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::Green, &Colors::BGreen],
            },
            // Groups
            Palette {
                regexp: Regex::new(r#"(\d+)\((\w+)\)"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::Yellow, &Colors::BYellow],
            },
        ]
    }
}
