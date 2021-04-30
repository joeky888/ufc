use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("env")
            .args(&[
                Arg::new("ignore-environment").long("ignore-environment").short('i').about("start with an empty environment"),
                Arg::new("null").long("null").short('0').about("end each output line with NUL, not newline"),
                Arg::new("unset").long("unset").short('u').takes_value(true).about("remove variable from the environment"),
                Arg::new("chdir").long("chdir").short('C').takes_value(true).about("change working directory to DIR"),
                Arg::new("split-string").long("split-string").short('S').takes_value(true).about("process and split S into separate arguments; used to pass multiple arguments on shebang lines"),
                Arg::new("block-signal").long("block-signal").takes_value(true).about("block delivery of SIG signal(s) to COMMAND"),
                Arg::new("default-signal").long("default-signal").takes_value(true).about("reset handling of SIG signal(s) to the default"),
                Arg::new("ignore-signal").long("ignore-signal").takes_value(true).about("set handling of SIG signals(s) to do nothing"),
                Arg::new("list-signal-handling").long("list-signal-handling").about("list non default signal handling to stderr"),
                Arg::new("debug").long("debug").short('v').about("print verbose information for each processing step"),
                Arg::new("help").long("help").about("display this help and exit"),
                Arg::new("version").long("version").about("output version information and exit"),
            ])
            .about("env")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Main
            Palette {
                regexp: Regex::new(r#"^([^=]+)(=)(.*)$"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::Cyan,
                    &Colours::White,
                    &Colours::Yellow,
                ],
            },
        ]
    }
}
