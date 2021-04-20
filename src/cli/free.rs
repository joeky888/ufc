use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("free")
            .args(&[
                Arg::with_name("bytes").long("bytes").short("b").help("Display the amount of memory in bytes."),
                Arg::with_name("kibi").long("kibi").short("k").help("Display the amount of memory in kibibytes.  This is the default."),
                Arg::with_name("mebi").long("mebi").short("m").help("Display the amount of memory in mebibytes."),
                Arg::with_name("gibi").long("gibi").short("g").help("Display the amount of memory in gibibytes."),
                Arg::with_name("tebi").long("tebi").help("Display the amount of memory in tebibytes."),
                Arg::with_name("pebi").long("pebi").help("Display the amount of memory in pebibytes."),
                Arg::with_name("kilo").long("kilo").help("Display the amount of memory in kilobytes. Implies --si."),
                Arg::with_name("mega").long("mega").help("Display the amount of memory in megabytes. Implies --si."),
                Arg::with_name("giga").long("giga").help("Display the amount of memory in gigabytes. Implies --si."),
                Arg::with_name("tera").long("tera").help("Display the amount of memory in terabytes. Implies --si."),
                Arg::with_name("peta").long("peta").help("Display the amount of memory in petabytes. Implies --si."),
                Arg::with_name("human").long("human").short("h").help("Show all output fields automatically scaled to shortest three digit unit and display the units of print out."),
                Arg::with_name("wide").long("wide").short("w").help("Switch to the wide mode. The wide mode produces lines longer than 80 characters."),
                Arg::with_name("count").long("count").short("c").takes_value(true).help("Display the result count times. Requires the -s option."),
                Arg::with_name("lohi").long("lohi").short("l").help("Show detailed low and high memory statistics."),
                Arg::with_name("seconds").long("seconds").short("s").takes_value(true).help("Continuously display the result delay seconds apart."),
                Arg::with_name("si").long("si").help("Use kilo, mega, giga etc (power of 1000) instead of kibi, mebi, gibi (power of 1024)."),
                Arg::with_name("total").long("total").short("t").help("Display a line showing the column totals."),
                Arg::with_name("help").long("help").help("Print help."),
                Arg::with_name("version").long("version").short("V").help("Display version information."),
            ])
            .about("free")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Zero
            Palette {
                regexp: Regex::new(r#"\s+0\w?(\s|$)"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Swap
            Palette {
                regexp: Regex::new(r#"^Swap"#).unwrap(),
                colours: vec![&Colours::BMagenta],
            },
            // Mem
            Palette {
                regexp: Regex::new(r#"^Mem"#).unwrap(),
                colours: vec![&Colours::BCyan],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dTi?|\b\d{10,12}\b"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dGi?|\b\d{7,9}\b"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dMi?|\b\d{4,6}\b"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dKi?|\b\d{1,3}\b"#).unwrap(),
                colours: vec![&Colours::Green],
            },
        ]
    }
}
