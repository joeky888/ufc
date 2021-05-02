use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("df")
            .args(&[
                Arg::new("FILE").about("URL destination"),
                Arg::new("all")
                    .long("all")
                    .short('a')
                    .about("include pseudo, duplicate, inaccessible file systems"),
                Arg::new("block-size")
                    .long("block-size")
                    .short('B')
                    .takes_value(true)
                    .about("scale sizes by SIZE before printing them; e.g., '-BM' prints sizes in units of 1,048,576 bytes; see SIZE format below"),
                Arg::new("human-readable")
                    .long("human-readable")
                    .short('h')
                    .about("print sizes in powers of 1024 (e.g., 1023M)"),
                Arg::new("inodes")
                    .long("inodes")
                    .short('i')
                    .about("list inode information instead of block usage"),
                Arg::new("block-size-k")
                    .short('k')
                    .about("like --block-size=1K"),
                Arg::new("local")
                    .long("local")
                    .short('l')
                    .about("limit listing to local file systems"),
                Arg::new("no-sync")
                    .long("no-sync")
                    .about("do not invoke sync before getting usage info (default)"),
                Arg::new("output")
                    .long("output")
                    .takes_value(true)
                    .about("use the output format defined by FIELD_LIST, or print all fields if FIELD_LIST is omitted."),
                Arg::new("portability")
                    .long("portability")
                    .short('P')
                    .about("use the POSIX output format"),
                Arg::new("total")
                    .long("total")
                    .about("elide all entries insignificant to available space, and produce a grand total"),
                Arg::new("type")
                    .long("type")
                    .short('t')
                    .takes_value(true)
                    .about("limit listing to file systems of type TYPE"),
                Arg::new("print-type")
                    .long("print-type")
                    .short('T')
                    .about("print file system type"),
                Arg::new("exclude-type")
                    .long("exclude-type")
                    .short('x')
                    .takes_value(true)
                    .about("limit listing to file systems not of type TYPE"),
                Arg::new("help")
                    .long("help")
                    .about("display this help and exit"),
                Arg::new("version")
                    .long("version")
                    .about("output version information and exit"),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("df")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // FS
            Palette {
                regexp: Regex::new(r#"^(?!Filesystem)(\/[-\w\d.]+)+\s"#).unwrap(),
                colors: vec![&Colors::Blue, &Colors::BBlue],
            },
            // tmpfs lines
            Palette {
                regexp: Regex::new(r#"^tmpfs.*"#).unwrap(),
                colors: vec![&Colors::BBlack],
            },
            // Mounted on
            Palette {
                regexp: Regex::new(r#"\/$|(\/[-\w\d. ]+)+$"#).unwrap(),
                colors: vec![&Colors::Green, &Colors::BGreen],
            },
            // Use 0-60%
            Palette {
                regexp: Regex::new(r#"\s[1-6]?[0-9]%\s"#).unwrap(),
                colors: vec![&Colors::Green],
            },
            // 70-89%
            Palette {
                regexp: Regex::new(r#"\s[78][0-9]%\s"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // 90-97%
            Palette {
                regexp: Regex::new(r#"\s9[0-7]%\s"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // Use 98-100%
            Palette {
                regexp: Regex::new(r#"\s9[89]%|100%\s"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dTi?\s|\b\d{10,12}\b"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dGi?\s|\b\d{7,9}\b"#).unwrap(),
                colors: vec![&Colors::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dMi?\s|\b\d{4,6}\b"#).unwrap(),
                colors: vec![&Colors::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d(K|B)i?\s|\b\d{1,3}\b"#).unwrap(),
                colors: vec![&Colors::Green],
            },
        ]
    }
}
