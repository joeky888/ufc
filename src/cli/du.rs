use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("du")
            .args(&[
                Arg::new("FILE").about("[FILE]"),
                Arg::new("null").long("null").short('0').about("end each output line with NUL, not newline"),
                Arg::new("all").long("all").short('a').about("write counts for all files, not just directories"),
                Arg::new("apparent-size").long("apparent-size").about("print apparent sizes, rather than disk usage"),
                Arg::new("block-size").long("block-size").short('B').takes_value(true).about("scale sizes by SIZE before printing them"),
                Arg::new("bytes").long("bytes").short('b').about("equivalent to '--apparent-size --block-size=1'"),
                Arg::new("total").long("total").short('c').about("produce a grand total"),
                Arg::new("dereference-args").long("dereference-args").short('D').short('H').about("dereference only symlinks that are listed on the command line"),
                Arg::new("max-depth").long("max-depth").short('d').takes_value(true).about("print the total for a directory"),
                Arg::new("files0-from").long("files0-from").short('F').takes_value(true).about("summarize disk usage of the NUL-terminated file names specified in file F; if F is -, then read names from standard input"),
                Arg::new("human-readable").long("human-readable").short('h').about("print sizes in human readable format (e.g., 1K 234M 2G)"),
                Arg::new("inodes").long("inodes").about("list inode usage information instead of block usage"),
                Arg::new("k").short('k').about("like --block-size=1K"),
                Arg::new("dereference").long("dereference").short('L').about("dereference all symbolic links"),
                Arg::new("count-links").long("count-links").short('l').about("count sizes many times if hard linked"),
                Arg::new("m").short('m').about("like --block-size=1M"),
                Arg::new("no-dereference").long("no-dereference").short('P').about("don't follow any symbolic links (this is the default)"),
                Arg::new("separate-dirs").long("separate-dirs").short('S').about("for directories do not include size of subdirectories"),
                Arg::new("si").long("si").about("like -h, but use powers of 1000 not 1024"),
                Arg::new("summarize").long("summarize").short('s').about("display only a total for each argument"),
                Arg::new("threshold").long("threshold").short('t').takes_value(true).about("exclude entries smaller than SIZE if positive, or entries greater than SIZE if negative"),
                Arg::new("time").long("time").about("show time of the last modification of any file in the directory, or any of its subdirectories"),
                Arg::new("time-style").long("time-style").takes_value(true).about("show times using STYLE, which can be: full-iso, long-iso, iso, or +FORMAT; FORMAT is interpreted like in 'date'"),
                Arg::new("exclude-from").long("exclude-from").short('X').takes_value(true).about("exclude files that match any pattern in FILE"),
                Arg::new("exclude").long("exclude").takes_value(true).about("exclude files that match PATTERN"),
                Arg::new("one-file-system").long("one-file-system").short('x').takes_value(true).about("skip directories on different file systems"),
                Arg::new("help").long("help").about("display this help and exit"),
                Arg::new("version").long("version").about("output version information and exit"),
            ])
            .about("du")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
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
