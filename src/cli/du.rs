use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("du")
            .args(&[
                Arg::with_name("FILE").help("[FILE]"),
                Arg::with_name("null").long("null").short("0").help("end each output line with NUL, not newline"),
                Arg::with_name("all").long("all").short("a").help("write counts for all files, not just directories"),
                Arg::with_name("apparent-size").long("apparent-size").help("print apparent sizes, rather than disk usage"),
                Arg::with_name("block-size").long("block-size").short("B").takes_value(true).help("scale sizes by SIZE before printing them"),
                Arg::with_name("bytes").long("bytes").short("b").help("equivalent to '--apparent-size --block-size=1'"),
                Arg::with_name("total").long("total").short("c").help("produce a grand total"),
                Arg::with_name("dereference-args").long("dereference-args").short("D").short("H").help("dereference only symlinks that are listed on the command line"),
                Arg::with_name("max-depth").long("max-depth").short("d").takes_value(true).help("print the total for a directory"),
                Arg::with_name("files0-from").long("files0-from").short("F").takes_value(true).help("summarize disk usage of the NUL-terminated file names specified in file F; if F is -, then read names from standard input"),
                Arg::with_name("human-readable").long("human-readable").short("h").help("print sizes in human readable format (e.g., 1K 234M 2G)"),
                Arg::with_name("inodes").long("inodes").help("list inode usage information instead of block usage"),
                Arg::with_name("k").short("k").help("like --block-size=1K"),
                Arg::with_name("dereference").long("dereference").short("L").help("dereference all symbolic links"),
                Arg::with_name("count-links").long("count-links").short("l").help("count sizes many times if hard linked"),
                Arg::with_name("m").short("m").help("like --block-size=1M"),
                Arg::with_name("no-dereference").long("no-dereference").short("P").help("don't follow any symbolic links (this is the default)"),
                Arg::with_name("separate-dirs").long("separate-dirs").short("S").help("for directories do not include size of subdirectories"),
                Arg::with_name("si").long("si").help("like -h, but use powers of 1000 not 1024"),
                Arg::with_name("summarize").long("summarize").short("s").help("display only a total for each argument"),
                Arg::with_name("threshold").long("threshold").short("t").takes_value(true).help("exclude entries smaller than SIZE if positive, or entries greater than SIZE if negative"),
                Arg::with_name("time").long("time").help("show time of the last modification of any file in the directory, or any of its subdirectories"),
                Arg::with_name("time-style").long("time-style").takes_value(true).help("show times using STYLE, which can be: full-iso, long-iso, iso, or +FORMAT; FORMAT is interpreted like in 'date'"),
                Arg::with_name("exclude-from").long("exclude-from").short("X").takes_value(true).help("exclude files that match any pattern in FILE"),
                Arg::with_name("exclude").long("exclude").takes_value(true).help("exclude files that match PATTERN"),
                Arg::with_name("one-file-system").long("one-file-system").short("x").takes_value(true).help("skip directories on different file systems"),
                Arg::with_name("help").long("help").help("display this help and exit"),
                Arg::with_name("version").long("version").help("output version information and exit"),
            ])
            .about("du")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Path
            Palette {
                regexp: Regex::new(r#"\s+[\./]+([\w\s\-\_\.]+)(/.*)?$"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::BBlue, &Colours::Blue],
            },
            // Total
            Palette {
                regexp: Regex::new(r#"(.*)\s+(total)$"#).unwrap(),
                colours: vec![&Colours::BYellow],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"^ ?\d*[.,]?\dTi?"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"^ ?\d*[.,]?\dGi?"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            Palette {
                regexp: Regex::new(r#"^\d{7,9}"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"^ ?\d*[.,]?\dMi?"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            Palette {
                regexp: Regex::new(r#"^\d{4,6}"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"^ ?\d*[.,]?\dKi?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            Palette {
                regexp: Regex::new(r#"^\d{1,3}"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Cannot read error
            Palette {
                regexp: Regex::new(r#"^du.*"#).unwrap(),
                colours: vec![&Colours::Red],
            },
        ]
    }
}
