use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("journalctl")
            .args(&[
                Arg::new("all").long("all").short('a').about(r#"Show all fields in full, even if they include unprintable characters or are very long."#),
                Arg::new("follow").long("follow").short('f').about(r#"Show only the most recent journal entries, and continuously print new entries as they are appended to the journal."#),
                Arg::new("pager-end").long("pager-end").short('e').about(r#"Immediately jump to the end of the journal inside the implied pager tool."#),
                Arg::new("lines").long("lines").short('n').takes_value(true).about(r#"Show the most recent journal events and limit the number of events shown. If --follow is used, this option is implied. The argument is a positive integer or "all" to disable line limiting. The default value is 10 if no argument is given."#),
                Arg::new("no-tail").long("no-tail").about(r#"Show all stored output lines, even in follow mode. Undoes the effect of --lines=."#),
                Arg::new("reverse").long("reverse").short('r').about(r#"Reverse output so that the newest entries are displayed first."#),
                Arg::new("output").long("output").short('o').takes_value(true).about(r#"Controls the formatting of the journal entries that are shown."#),
                Arg::new("utc").long("utc").about(r#"Express time in Coordinated Universal Time (UTC)."#),
                Arg::new("no-hostname").long("no-hostname").about(r#"Don't show the hostname field of log messages originating from the local host."#),
                Arg::new("catalog").long("catalog").short('x').about(r#"Augment log lines with explanation texts from the message catalog. This will add explanatory help texts to log messages in the output where this is available."#),
                Arg::new("quiet").long("quiet").short('q').about(r#"Suppresses all informational messages (i.e. "-- Journal begins at ...", "-- Reboot --"), any warning messages regarding inaccessible system journals when run as a normal user."#),
                Arg::new("merge").long("merge").short('m').about(r#"Show entries interleaved from all available journals, including remote ones."#),
                Arg::new("[[ID][±offset]|all]").long("boot").short('b').takes_value(true).about(r#"Show messages from a specific boot. This will add a match for "_BOOT_ID="."#),
                Arg::new("list-boots").long("list-boots").about(r#"Show a tabular list of boot numbers (relative to the current boot), their IDs, and the timestamps of the first and last message pertaining to the boot."#),
                Arg::new("dmesg").long("dmesg").short('k').about(r#"Show only kernel messages. This implies -b and adds the match "_TRANSPORT=kernel"."#),
                Arg::new("SYSLOG_IDENTIFIER").long("identifier").short('t').takes_value(true).about(r#"Show messages for the specified syslog identifier SYSLOG_IDENTIFIER."#),
                Arg::new("UNIT|PATTERN").long("unit").short('u').takes_value(true).about(r#"Show messages for the specified systemd unit UNIT (such as a service unit), or for any of the units matched by PATTERN."#),
                Arg::new("user-unit").long("user-unit").takes_value(true).about(r#"Show messages for the specified user session unit."#),
                Arg::new("facility").long("facility").takes_value(true).about(r#"Filter output by syslog facility. Takes a comma-separated list of numbers or facility names."#),
                Arg::new("grep").long("grep").short('g').takes_value(true).about(r#"Filter output to entries where the MESSAGE= field matches the specified regular expression. PERL-compatible regular expressions are used."#),
                Arg::new("case-sensitive").long("case-sensitive").takes_value(true).about(r#"A boolean value to make pattern matching case sensitive or case insensitive."#),
                Arg::new("cursor").long("cursor").short('c').takes_value(true).about(r#"Start showing entries from the location in the journal specified by the passed cursor."#),
                Arg::new("cursor-file").long("cursor-file").takes_value(true).about(r#"If FILE exists and contains a cursor, start showing entries after this location. Otherwise the show entries according the other given options."#),
                Arg::new("after-cursor").long("after-cursor").takes_value(true).about(r#"Start showing entries from the location in the journal after the location specified by the passed cursor."#),
                Arg::new("show-cursor").long("show-cursor").about(r#"The cursor is shown after the last entry after two dashes."#),
                Arg::new("since").long("since").short('S').takes_value(true).about(r#"Start showing entries on or newer than the specified date, or on or older than the specified date, respectively. Date specifications should be of the format "2012-10-30 18:17:16". If the time part is omitted, "00:00:00" is assumed."#),
                Arg::new("until").long("until").short('U').takes_value(true).about(r#"Start showing entries on or newer than the specified date, or on or older than the specified date, respectively. Date specifications should be of the format "2012-10-30 18:17:16". If the time part is omitted, "00:00:00" is assumed."#),
                Arg::new("field").long("field").short('F').takes_value(true).about(r#"Print all possible data values the specified field can take in all entries of the journal."#),
                Arg::new("fields").long("fields").short('N').about(r#"Print all field names currently used in all entries of the journal."#),
                Arg::new("system").long("system").about(r#"Show messages from system services and the kernel."#),
                Arg::new("user").long("user").about(r#"Show messages from service of current user."#),
                Arg::new("machine").long("machine").short('M').takes_value(true).about(r#"Show messages from a running, local container. Specify a container name to connect to."#),
                Arg::new("directory").long("directory").short('D').takes_value(true).about(r#"Takes a directory path as argument. If specified, journalctl will operate on the specified journal directory DIR instead of the default runtime and system journal paths."#),
                Arg::new("GLOB").long("file").takes_value(true).about(r#"Takes a file glob as an argument. If specified, journalctl will operate on the specified journal files matching GLOB."#),
                Arg::new("ROOT").long("root").takes_value(true).about(r#"Takes a directory path as an argument. If specified, journalctl will operate on journal directories and catalog file hierarchy underneath the specified  directory instead of the root directory."#),
                Arg::new("IMAGE").long("image").takes_value(true).about(r#"Takes a path to a disk image file or block device node. If specified, journalctl will operate on the file system in the indicated disk image."#),
                Arg::new("NAMESPACE").long("namespace").takes_value(true).about(r#"Takes a journal namespace identifier string as argument. If not specified the data collected by the default namespace is shown."#),
                Arg::new("header").long("header").about(r#"Instead of showing journal contents, show internal header information of the journal fields accessed."#),
                Arg::new("disk-usage").long("disk-usage").about(r#"Shows the current disk usage of all journal files. This shows the sum of the disk usage of all archived and active journal files."#),
                Arg::new("vacuum-size").long("vacuum-size").takes_value(true).about(r#"Removes the oldest archived journal files until the disk space they use falls below the specified size (specified with the usual "K", "M", "G" and "T" suffixes)"#),
                Arg::new("vacuum-time").long("vacuum-time").takes_value(true).about(r#"Removes all archived journal files contain no data older than the specified timespan (specified with the usual "s", "m", "h", "days", "months", "weeks" and "years" suffixes)"#),
                Arg::new("list-catalog").long("list-catalog").takes_value(true).about(r#"List the contents of the message catalog as a table of message IDs, plus their short description strings."#),
                Arg::new("dump-catalog").long("dump-catalog").takes_value(true).about(r#"Show the contents of the message catalog, with entries separated by a line consisting of two dashes and the ID (the format is the same as .catalog files)."#),
                Arg::new("update-catalog").long("update-catalog").about(r#"Update the message catalog index. This command needs to be executed each time new catalog files are installed, removed, or updated to rebuild the binary catalog index."#),
                Arg::new("setup-keys").long("setup-keys").about(r#"Instead of showing journal contents, generate a new key pair for Forward Secure Sealing (FSS). This will generate a sealing key and a verification key."#),
                Arg::new("force").long("force").about(r#"When --setup-keys is passed and Forward Secure Sealing (FSS) has already been configured, recreate FSS keys."#),
                Arg::new("interval").long("interval").takes_value(true).about(r#"Specifies the change interval for the sealing key when generating an FSS key pair with --setup-keys."#),
                Arg::new("verify").long("verify").about(r#"Check the journal file for internal consistency."#),
                Arg::new("verify-key").long("verify-key").takes_value(true).about(r#"Specifies the FSS verification key to use for the --verify operation."#),
                Arg::new("sync").long("sync").about(r#"Asks the journal daemon to write all yet unwritten journal data to the backing file system and synchronize all journals."#),
                Arg::new("flush").long("flush").about(r#"Asks the journal daemon to flush any log data stored in /run/log/journal/ into /var/log/journal/, if persistent storage is enabled."#),
                Arg::new("relinquish-var").long("relinquish-var").about(r#"Asks the journal daemon for the reverse operation to --flush: if requested the daemon will write further log data to /run/log/journal/ and stops writing to /var/log/journal/."#),
                Arg::new("smart-relinquish-var").long("smart-relinquish-var").about(r#"Similar to --relinquish-var but executes no operation if the root file system and /var/lib/journal/ reside on the same mount point."#),
                Arg::new("rotate").long("rotate").about(r#"Asks the journal daemon to rotate journal files. This call does not return until the rotation operation is complete."#),
                Arg::new("help").long("help").short('h').about(r#"Print a short help text and exit."#),
                Arg::new("version").long("version").about(r#"Print a short version string and exit."#),
                Arg::new("no-pager").long("no-pager").about(r#"Do not pipe output into a pager."#),
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
