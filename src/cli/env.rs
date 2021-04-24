use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("env")
            .args(&[
                Arg::with_name("ignore-environment").long("ignore-environment").short("i").help("start with an empty environment"),
                Arg::with_name("null").long("null").short("0").help("end each output line with NUL, not newline"),
                Arg::with_name("unset").long("unset").short("u").takes_value(true).help("remove variable from the environment"),
                Arg::with_name("chdir").long("chdir").short("C").takes_value(true).help("change working directory to DIR"),
                Arg::with_name("split-string").long("split-string").short("S").takes_value(true).help("process and split S into separate arguments; used to pass multiple arguments on shebang lines"),
                Arg::with_name("block-signal").long("block-signal").takes_value(true).help("block delivery of SIG signal(s) to COMMAND"),
                Arg::with_name("default-signal").long("default-signal").takes_value(true).help("reset handling of SIG signal(s) to the default"),
                Arg::with_name("ignore-signal").long("ignore-signal").takes_value(true).help("set handling of SIG signals(s) to do nothing"),
                Arg::with_name("list-signal-handling").long("list-signal-handling").help("list non default signal handling to stderr"),
                Arg::with_name("debug").long("debug").short("v").help("print verbose information for each processing step"),
                Arg::with_name("help").long("help").help("display this help and exit"),
                Arg::with_name("version").long("version").help("output version information and exit"),
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
