use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Df {}

impl Df {
    pub fn new() -> App<'static, 'static> {
        App::new("df")
            .args(&[
                Arg::with_name("FILE").help("URL destination"),
                Arg::with_name("human-readable").long("human-readable").short("h").help("print sizes in powers of 1024 (e.g., 1023M)"),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("df")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Df::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // FS
            Palette {
                regexp: Regex::new(r#"^(?!Filesystem)(\/[-\w\d.]+)+\s"#).unwrap(),
                colours: vec![&Colours::Blue, &Colours::BBlue],
            },
            // tmpfs lines
            Palette {
                regexp: Regex::new(r#"^tmpfs.*"#).unwrap(),
                colours: vec![&Colours::BBlack],
            },
            // Mounted on
            Palette {
                regexp: Regex::new(r#"\/$|(\/[-\w\d. ]+)+$"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
            // Use 0-60%
            Palette {
                regexp: Regex::new(r#"\s[1-6]?[0-9]%\s"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // 70-89%
            Palette {
                regexp: Regex::new(r#"\s[78][0-9]%\s"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // 90-97%
            Palette {
                regexp: Regex::new(r#"\s9[0-7]%\s"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Use 98-100%
            Palette {
                regexp: Regex::new(r#"\s9[89]%|100%\s"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d(K|B)i?\s|\b\d{1,3}\b"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dMi?\s|\b\d{4,6}\b"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[\.,]?\dGi?\s|\b\d{7,9}\b"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dTi?\s|\b\d{10,12}\b"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
        ]
    }
}
