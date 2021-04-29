use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("journalctl")
            .args(&[
                Arg::with_name("all").long("all").short("a").help(r#"Show all fields in full, even if they include unprintable characters or are very long."#),
                Arg::with_name("follow").long("follow").short("f").help(r#"Show only the most recent journal entries, and continuously print new entries as they are appended to the journal."#),
                Arg::with_name("pager-end").long("pager-end").short("e").help(r#"Immediately jump to the end of the journal inside the implied pager tool."#),
                Arg::with_name("lines").long("lines").short("n").takes_value(true).help(r#"Show the most recent journal events and limit the number of events shown. If --follow is used, this option is implied. The argument is a positive integer or "all" to disable line limiting. The default value is 10 if no argument is given."#),
                Arg::with_name("no-tail").long("no-tail").help(r#"Show all stored output lines, even in follow mode. Undoes the effect of --lines=."#),
                Arg::with_name("reverse").long("reverse").short("r").help(r#"Reverse output so that the newest entries are displayed first."#),
                Arg::with_name("output").long("output").short("o").takes_value(true).help(r#"Controls the formatting of the journal entries that are shown."#),
                Arg::with_name("utc").long("utc").help(r#"Express time in Coordinated Universal Time (UTC)."#),
                Arg::with_name("no-hostname").long("no-hostname").help(r#"Don't show the hostname field of log messages originating from the local host."#),
                Arg::with_name("catalog").long("catalog").short("x").help(r#"Augment log lines with explanation texts from the message catalog. This will add explanatory help texts to log messages in the output where this is available."#),
                Arg::with_name("quiet").long("quiet").short("q").help(r#"Suppresses all informational messages (i.e. "-- Journal begins at ...", "-- Reboot --"), any warning messages regarding inaccessible system journals when run as a normal user."#),
                Arg::with_name("merge").long("merge").short("m").help(r#"Show entries interleaved from all available journals, including remote ones."#),
                Arg::with_name("[[ID][±offset]|all]").long("boot").short("b").takes_value(true).help(r#"Show messages from a specific boot. This will add a match for "_BOOT_ID="."#),
                Arg::with_name("list-boots").long("list-boots").help(r#"Show a tabular list of boot numbers (relative to the current boot), their IDs, and the timestamps of the first and last message pertaining to the boot."#),
                Arg::with_name("dmesg").long("dmesg").short("k").help(r#"Show only kernel messages. This implies -b and adds the match "_TRANSPORT=kernel"."#),
                Arg::with_name("SYSLOG_IDENTIFIER").long("identifier").short("t").takes_value(true).help(r#"Show messages for the specified syslog identifier SYSLOG_IDENTIFIER."#),
                Arg::with_name("UNIT|PATTERN").long("unit").short("u").takes_value(true).help(r#"Show messages for the specified systemd unit UNIT (such as a service unit), or for any of the units matched by PATTERN."#),
                Arg::with_name("user-unit").long("user-unit").takes_value(true).help(r#"Show messages for the specified user session unit."#),
                Arg::with_name("facility").long("facility").takes_value(true).help(r#"Filter output by syslog facility. Takes a comma-separated list of numbers or facility names."#),
                Arg::with_name("grep").long("grep").short("g").takes_value(true).help(r#"Filter output to entries where the MESSAGE= field matches the specified regular expression. PERL-compatible regular expressions are used."#),
                Arg::with_name("case-sensitive").long("case-sensitive").takes_value(true).help(r#"A boolean value to make pattern matching case sensitive or case insensitive."#),
                Arg::with_name("cursor").long("cursor").short("c").takes_value(true).help(r#"Start showing entries from the location in the journal specified by the passed cursor."#),
                Arg::with_name("cursor-file").long("cursor-file").takes_value(true).help(r#"If FILE exists and contains a cursor, start showing entries after this location. Otherwise the show entries according the other given options."#),
                Arg::with_name("after-cursor").long("after-cursor").takes_value(true).help(r#"Start showing entries from the location in the journal after the location specified by the passed cursor."#),
                Arg::with_name("show-cursor").long("show-cursor").help(r#"The cursor is shown after the last entry after two dashes."#),
                Arg::with_name("since").long("since").short("S").takes_value(true).help(r#"Start showing entries on or newer than the specified date, or on or older than the specified date, respectively. Date specifications should be of the format "2012-10-30 18:17:16". If the time part is omitted, "00:00:00" is assumed."#),
                Arg::with_name("until").long("until").short("U").takes_value(true).help(r#"Start showing entries on or newer than the specified date, or on or older than the specified date, respectively. Date specifications should be of the format "2012-10-30 18:17:16". If the time part is omitted, "00:00:00" is assumed."#),
                Arg::with_name("field").long("field").short("F").takes_value(true).help(r#"Print all possible data values the specified field can take in all entries of the journal."#),
                Arg::with_name("fields").long("fields").short("N").help(r#"Print all field names currently used in all entries of the journal."#),
                Arg::with_name("system").long("system").help(r#"Show messages from system services and the kernel."#),
                Arg::with_name("user").long("user").help(r#"Show messages from service of current user."#),
                Arg::with_name("machine").long("machine").short("M").takes_value(true).help(r#"Show messages from a running, local container. Specify a container name to connect to."#),
                Arg::with_name("directory").long("directory").short("D").takes_value(true).help(r#"Takes a directory path as argument. If specified, journalctl will operate on the specified journal directory DIR instead of the default runtime and system journal paths."#),
                Arg::with_name("GLOB").long("file").takes_value(true).help(r#"Takes a file glob as an argument. If specified, journalctl will operate on the specified journal files matching GLOB."#),
                Arg::with_name("ROOT").long("root").takes_value(true).help(r#"Takes a directory path as an argument. If specified, journalctl will operate on journal directories and catalog file hierarchy underneath the specified  directory instead of the root directory."#),
                Arg::with_name("IMAGE").long("image").takes_value(true).help(r#"Takes a path to a disk image file or block device node. If specified, journalctl will operate on the file system in the indicated disk image."#),
                Arg::with_name("NAMESPACE").long("namespace").takes_value(true).help(r#"Takes a journal namespace identifier string as argument. If not specified the data collected by the default namespace is shown."#),
                Arg::with_name("header").long("header").help(r#"Instead of showing journal contents, show internal header information of the journal fields accessed."#),
                Arg::with_name("disk-usage").long("disk-usage").help(r#"Shows the current disk usage of all journal files. This shows the sum of the disk usage of all archived and active journal files."#),
                Arg::with_name("vacuum-size").long("vacuum-size").takes_value(true).help(r#"Removes the oldest archived journal files until the disk space they use falls below the specified size (specified with the usual "K", "M", "G" and "T" suffixes)"#),
                Arg::with_name("vacuum-time").long("vacuum-time").takes_value(true).help(r#"Removes all archived journal files contain no data older than the specified timespan (specified with the usual "s", "m", "h", "days", "months", "weeks" and "years" suffixes)"#),
                Arg::with_name("list-catalog").long("list-catalog").takes_value(true).help(r#"List the contents of the message catalog as a table of message IDs, plus their short description strings."#),
                Arg::with_name("dump-catalog").long("dump-catalog").takes_value(true).help(r#"Show the contents of the message catalog, with entries separated by a line consisting of two dashes and the ID (the format is the same as .catalog files)."#),
                Arg::with_name("update-catalog").long("update-catalog").help(r#"Update the message catalog index. This command needs to be executed each time new catalog files are installed, removed, or updated to rebuild the binary catalog index."#),
                Arg::with_name("setup-keys").long("setup-keys").help(r#"Instead of showing journal contents, generate a new key pair for Forward Secure Sealing (FSS). This will generate a sealing key and a verification key."#),
                Arg::with_name("force").long("force").help(r#"When --setup-keys is passed and Forward Secure Sealing (FSS) has already been configured, recreate FSS keys."#),
                Arg::with_name("interval").long("interval").takes_value(true).help(r#"Specifies the change interval for the sealing key when generating an FSS key pair with --setup-keys."#),
                Arg::with_name("verify").long("verify").help(r#"Check the journal file for internal consistency."#),
                Arg::with_name("verify-key").long("verify-key").takes_value(true).help(r#"Specifies the FSS verification key to use for the --verify operation."#),
                Arg::with_name("sync").long("sync").help(r#"Asks the journal daemon to write all yet unwritten journal data to the backing file system and synchronize all journals."#),
                Arg::with_name("flush").long("flush").help(r#"Asks the journal daemon to flush any log data stored in /run/log/journal/ into /var/log/journal/, if persistent storage is enabled."#),
                Arg::with_name("relinquish-var").long("relinquish-var").help(r#"Asks the journal daemon for the reverse operation to --flush: if requested the daemon will write further log data to /run/log/journal/ and stops writing to /var/log/journal/."#),
                Arg::with_name("smart-relinquish-var").long("smart-relinquish-var").help(r#"Similar to --relinquish-var but executes no operation if the root file system and /var/lib/journal/ reside on the same mount point."#),
                Arg::with_name("rotate").long("rotate").help(r#"Asks the journal daemon to rotate journal files. This call does not return until the rotation operation is complete."#),
                Arg::with_name("help").long("help").short("h").help(r#"Print a short help text and exit."#),
                Arg::with_name("version").long("version").help(r#"Print a short version string and exit."#),
                Arg::with_name("no-pager").long("no-pager").help(r#"Do not pipe output into a pager."#),
            ])
            .about("journalctl")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Connect requires special attention
            Palette {
                regexp: Regex::new(r#"connect"#).unwrap(),
                colours: vec![&Colours::OnRed],
            },
            // Status deferred
            Palette {
                regexp: Regex::new(r#"status\=deferred|Connection refused"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // 5xx status
            Palette {
                regexp: Regex::new(r#"\s\b5\d{2}\b\s"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // 4xx status
            Palette {
                regexp: Regex::new(r#"\s\b4\d{2}\b\s"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // 3xx status
            Palette {
                regexp: Regex::new(r#"\s\b3\d{2}\b\s"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // 2xx status
            Palette {
                regexp: Regex::new(r#"\s\b2\d{2}\b\s"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // HTTP verbs
            Palette {
                regexp: Regex::new(r#"GET|POST|PUT|DELETE|PATCH|HEAD"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Email address
            Palette {
                regexp: Regex::new(r#"[a-zA-z0-9\.\-\+]+\@[\w\-\.]+"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Date and hostname
            Palette {
                regexp: Regex::new(r#"^... (\d| )\d \d\d:\d\d:\d\d(\s[-.\w\d]+?\s)"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::Green, &Colours::Yellow],
            },
            // IPv6
            Palette {
                regexp: Regex::new(r#"\b[0-9a-fA-F]{1,4}(\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colours: vec![&Colours::BYellow],
            },
            // IPv4 or IPv4:Port
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(:\d{1,5})?"#).unwrap(),
                colours: vec![&Colours::BYellow],
            },
            // Name of process and pid
            Palette {
                regexp: Regex::new(r#"([\w/\.\-]+)(\[\d+?\])"#).unwrap(),
                colours: vec![&Colours::BBlue, &Colours::BRed],
            },
            // Everything in <>
            Palette {
                regexp: Regex::new(r#"\<.*?\>"#).unwrap(),
                colours: vec![&Colours::Blue],
            },
            // This is probably a pathname
            Palette {
                regexp: Regex::new(r#"\s/[a-zA-Z_/\.\-\?\d\=\&]+"#).unwrap(),
                colours: vec![&Colours::Blue],
            },
            // Everything in "
            Palette {
                regexp: Regex::new(r#"\".*?\""#).unwrap(),
                colours: vec![&Colours::Blue],
            },
            // Everything in `'
            Palette {
                regexp: Regex::new(r#"\`.+?\'"#).unwrap(),
                colours: vec![&Colours::BYellow],
            },
            // Everything in parentheses
            Palette {
                regexp: Regex::new(r#"\(.*?\)"#).unwrap(),
                colours: vec![&Colours::Blue],
            },
            // Display this line in yellow and stop further processing
            Palette {
                regexp: Regex::new(r#".*last message repeated \d+ times$"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
        ]
    }
}
