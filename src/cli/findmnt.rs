use crate::cli::cli::{pre_exec, Colors, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("findmnt")
            .args(&[
                Arg::new("device|mountpoint").about("device|mountpoint"),
                Arg::new("all").long("all").short('A').about("Disable all built-in filters and print all filesystems."),
                Arg::new("ascii").long("ascii").short('a').about("Use ascii characters for tree formatting."),
                Arg::new("bytes").long("bytes").short('b').about("Print the SIZE, USED and AVAIL columns in bytes rather than in a human-readable format."),
                Arg::new("nocanonicalize").long("nocanonicalize").short('C').about("Do not canonicalize paths at all. This option affects the comparing of paths and the evaluation of tags (LABEL, UUID, etc.)."),
                Arg::new("canonicalize").long("canonicalize").short('c').about("Canonicalize all printed paths."),
                Arg::new("df").long("df").short('D').about("Imitate the output of df."),
                Arg::new("direction").long("direction").short('d').takes_value(true).about("The search direction, either forward or backward."),
                Arg::new("evaluate").long("evaluate").short('e').about("Convert all tags (LABEL, UUID, PARTUUID or PARTLABEL) to the corresponding device names."),
                Arg::new("tab-file").long("tab-file").short('F').takes_value(true).about("Search in an alternative file.  If used with --fstab, --mtab or --kernel, then it overrides the default paths."),
                Arg::new("first-only").long("first-only").short('f').about("Print the first matching filesystem only."),
                Arg::new("help").long("help").short('h').about("Display help text and exit."),
                Arg::new("invert").long("invert").short('i').about("Invert the sense of matching."),
                Arg::new("json").long("json").short('J').about("Use JSON output format."),
                Arg::new("kernel").long("kernel").short('k').about("Search in /proc/self/mountinfo. The output is in the tree-like format."),
                Arg::new("list").long("list").short('l').about("Use the list output format."),
                Arg::new("mountpoint").long("mountpoint").short('M').takes_value(true).about("Explicitly define the mountpoint file or directory. See also --target."),
                Arg::new("mtab").long("mtab").short('m').about("Search in /etc/mtab. The output is in the list format by default (see --tree)."),
                Arg::new("task").long("task").short('N').takes_value(true).about("Use alternative namespace /proc/<tid>/mountinfo rather than the default /proc/self/mountinfo."),
                Arg::new("noheadings").long("noheadings").short('n').about("Do not print a header line."),
                Arg::new("options").long("options").short('O').takes_value(true).about("Limit the set of printed filesystems."),
                Arg::new("output").long("output").short('o').takes_value(true).about("Define output columns. See the --help output to get a list of the currently supported columns."),
                Arg::new("output-all").long("output-all").about("Output almost all available columns."),
                Arg::new("pairs").long("pairs").short('P').about(r#"Use key="value" output format."#),
                Arg::new("poll").long("poll").short('p').takes_value(true).about("Monitor changes in the /proc/self/mountinfo file."),
                Arg::new("pseudo").long("pseudo").about("Print only pseudo filesystems."),
                Arg::new("submounts").long("submounts").short('R').about("Print recursively all submounts for the selected filesystems."),
                Arg::new("raw").long("raw").short('r').about("Use raw output format."),
                Arg::new("real").long("real").about("Print only real filesystems."),
                Arg::new("source").long("source").short('S').takes_value(true).about("Explicitly define the mount source."),
                Arg::new("fstab").long("fstab").short('s').about("Search in /etc/fstab."),
                Arg::new("target").long("target").short('T').takes_value(true).about("Define the mount target."),
                Arg::new("types").long("types").short('t').takes_value(true).about("Limit the set of printed filesystems."),
                Arg::new("tree").long("tree").about("Enable tree-like output if possible."),
                Arg::new("uniq").long("uniq").short('U').about("Ignore filesystems with duplicate mount targets, thus effectively skipping over-mounted mount points."),
                Arg::new("notruncate").long("notruncate").short('u').about("Do not truncate text in columns."),
                Arg::new("nofsroot").long("nofsroot").short('v').about("Do not print a [/dir] in the SOURCE column for bind mounts or btrfs subvolumes."),
                Arg::new("timeout").long("timeout").short('w').takes_value(true).about("Specify an upper limit on the time for which --poll will block, in milliseconds."),
                Arg::new("verify").long("verify").short('x').about("Check mount table content. The default is to verify /etc/fstab parsability and usability."),
                Arg::new("verbose").long("verbose").about("Force findmnt to print more information (--verify only for now)."),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("findmnt")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // MS Types
            Palette {
                regexp: Regex::new(r#"\b(fat|vfat|ntfs|msdos)\b"#).unwrap(),
                colors: vec![&Colors::OnCyan],
            },
            // Common Types
            Palette {
                regexp: Regex::new(r#"\b(ext\d|xfs|btrfs|nfs)\b"#).unwrap(),
                colors: vec![&Colors::Cyan],
            },
            // Like comment, leave at end always
            Palette {
                regexp: Regex::new(r#"^.*(?=cgroup|tmpfs).*$"#).unwrap(),
                colors: vec![&Colors::BBlack],
            },
            // RO
            Palette {
                regexp: Regex::new(r#"(?:\s)ro"#).unwrap(),
                colors: vec![&Colors::BGreen],
            },
            // RW
            Palette {
                regexp: Regex::new(r#"(?:\s)rw"#).unwrap(),
                colors: vec![&Colors::BRed],
            },
            // Mount Path
            Palette {
                regexp: Regex::new(r#"(?<=─|-)(?:\/([^\/ ]+))+"#).unwrap(),
                colors: vec![&Colors::UnChanged, &Colors::BYellow],
            },
            // Devices
            Palette {
                regexp: Regex::new(r#"\s\/dev(?:\/([^\/ ]+))+"#).unwrap(),
                colors: vec![&Colors::Green, &Colors::BGreen],
            },
        ]
    }
}
