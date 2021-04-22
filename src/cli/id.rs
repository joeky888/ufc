use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("id").args(&[]).about("id")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
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
