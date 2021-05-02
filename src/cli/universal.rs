use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::ArgMatches;
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    // pub fn new() -> App<'static> {
    //     App::new("")
    //         .about("")
    // }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Warning
            Palette {
                regexp: Regex::new(r#"[Ww]arning|[Aa]lert"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // Negative
            Palette {
                regexp: Regex::new(r#"[Dd]isabled?|[Ee]rrors?|[Ss]topped|[Ff]alse|[Nn]one|[Tt]erminated|[Ff]aile?d?"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // Positive
            Palette {
                regexp: Regex::new(r#"[Ee]nabled?|[Oo]k|[Rr]unning|[Tt]rue|[Rr]eady|[Aa]ctive|[Aa]vailable|[Aa]pproved|[Cc]reated|[Cc]ompleted"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // ipv6
            Palette {
                regexp: Regex::new(r#"\b[0-9a-fA-F]{1,4}(\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colors: vec![&Colors::BCyan],
            },
            // ipv4
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colors: vec![&Colors::Cyan],
            },
            // 98-100%
            Palette {
                regexp: Regex::new(r#"9[89]%|100%"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // 90-97%
            Palette {
                regexp: Regex::new(r#"9[0-7]%"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // 70-89%
            Palette {
                regexp: Regex::new(r#"[78][0-9]%"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // Use 0-60%
            Palette {
                regexp: Regex::new(r#"[1-6]?[0-9]%"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dTi?|\b\d{10,12}\b"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dGi?|\b\d{7,9}\b"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dMi?|\b\d{4,6}\b"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dKi?|\b\d{1,3}\b"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // Numbers
            Palette {
                regexp: Regex::new(r#"\d*\.?\d+"#).unwrap(),
                colors: vec![&Colors::BBlue],
            },
        ]
    }
}
