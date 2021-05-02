use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("free")
            .args(&[
                Arg::new("bytes").long("bytes").short('b').about("Display the amount of memory in bytes."),
                Arg::new("kibi").long("kibi").short('k').about("Display the amount of memory in kibibytes.  This is the default."),
                Arg::new("mebi").long("mebi").short('m').about("Display the amount of memory in mebibytes."),
                Arg::new("gibi").long("gibi").short('g').about("Display the amount of memory in gibibytes."),
                Arg::new("tebi").long("tebi").about("Display the amount of memory in tebibytes."),
                Arg::new("pebi").long("pebi").about("Display the amount of memory in pebibytes."),
                Arg::new("kilo").long("kilo").about("Display the amount of memory in kilobytes. Implies --si."),
                Arg::new("mega").long("mega").about("Display the amount of memory in megabytes. Implies --si."),
                Arg::new("giga").long("giga").about("Display the amount of memory in gigabytes. Implies --si."),
                Arg::new("tera").long("tera").about("Display the amount of memory in terabytes. Implies --si."),
                Arg::new("peta").long("peta").about("Display the amount of memory in petabytes. Implies --si."),
                Arg::new("human").long("human").short('h').about("Show all output fields automatically scaled to shortest three digit unit and display the units of print out."),
                Arg::new("wide").long("wide").short('w').about("Switch to the wide mode. The wide mode produces lines longer than 80 characters."),
                Arg::new("count").long("count").short('c').takes_value(true).about("Display the result count times. Requires the -s option."),
                Arg::new("lohi").long("lohi").short('l').about("Show detailed low and high memory statistics."),
                Arg::new("seconds").long("seconds").short('s').takes_value(true).about("Continuously display the result delay seconds apart."),
                Arg::new("si").long("si").about("Use kilo, mega, giga etc (power of 1000) instead of kibi, mebi, gibi (power of 1024)."),
                Arg::new("total").long("total").short('t').about("Display a line showing the column totals."),
                Arg::new("help").long("help").about("Print help."),
                Arg::new("version").long("version").short('V').about("Display version information."),
            ])
            .about("free")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Zero
            Palette {
                regexp: Regex::new(r#"\s+0\w?(\s|$)"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // Swap
            Palette {
                regexp: Regex::new(r#"^Swap"#).unwrap(),
                colors: vec![&Colors::BMagenta],
            },
            // Mem
            Palette {
                regexp: Regex::new(r#"^Mem"#).unwrap(),
                colors: vec![&Colors::BCyan],
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
        ]
    }
}
