use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct CMD {}

impl CMD {
    pub fn new() -> App<'static, 'static> {
        App::new("df")
            .args(&[
                Arg::with_name("FILE").help("URL destination"),
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help("include pseudo, duplicate, inaccessible file systems"),
                Arg::with_name("block-size")
                    .long("block-size")
                    .short("B")
                    .takes_value(true)
                    .help("scale sizes by SIZE before printing them; e.g., '-BM' prints sizes in units of 1,048,576 bytes; see SIZE format below"),
                Arg::with_name("human-readable")
                    .long("human-readable")
                    .short("h")
                    .help("print sizes in powers of 1024 (e.g., 1023M)"),
                Arg::with_name("inodes")
                    .long("inodes")
                    .short("i")
                    .help("list inode information instead of block usage"),
                Arg::with_name("block-size-k")
                    .short("k")
                    .help("like --block-size=1K"),
                Arg::with_name("local")
                    .long("local")
                    .short("l")
                    .help("limit listing to local file systems"),
                Arg::with_name("no-sync")
                    .long("no-sync")
                    .help("do not invoke sync before getting usage info (default)"),
                Arg::with_name("output")
                    .long("output")
                    .takes_value(true)
                    .help("use the output format defined by FIELD_LIST, or print all fields if FIELD_LIST is omitted."),
                Arg::with_name("portability")
                    .long("portability")
                    .short("P")
                    .help("use the POSIX output format"),
                Arg::with_name("total")
                    .long("total")
                    .help("elide all entries insignificant to available space, and produce a grand total"),
                Arg::with_name("type")
                    .long("type")
                    .short("t")
                    .takes_value(true)
                    .help("limit listing to file systems of type TYPE"),
                Arg::with_name("print-type")
                    .long("print-type")
                    .short("T")
                    .help("print file system type"),
                Arg::with_name("exclude-type")
                    .long("exclude-type")
                    .short("x")
                    .takes_value(true)
                    .help("limit listing to file systems not of type TYPE"),
                Arg::with_name("help")
                    .long("help")
                    .help("display this help and exit"),
                Arg::with_name("version")
                    .long("version")
                    .help("output version information and exit"),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("df")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(CMD::palette());
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
            // Size 'T'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dTi?\s|\b\d{10,12}\b"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dGi?\s|\b\d{7,9}\b"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\dMi?\s|\b\d{4,6}\b"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d(K|B)i?\s|\b\d{1,3}\b"#).unwrap(),
                colours: vec![&Colours::Green],
            },
        ]
    }
}
