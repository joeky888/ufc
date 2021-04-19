use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static, 'static> {
        App::new("findmnt")
            .args(&[
                Arg::with_name("device|mountpoint").help("device|mountpoint"),
                Arg::with_name("all").long("all").short("A").help("Disable all built-in filters and print all filesystems."),
                Arg::with_name("ascii").long("ascii").short("a").help("Use ascii characters for tree formatting."),
                Arg::with_name("bytes").long("bytes").short("b").help("Print the SIZE, USED and AVAIL columns in bytes rather than in a human-readable format."),
                Arg::with_name("nocanonicalize").long("nocanonicalize").short("C").help("Do not canonicalize paths at all. This option affects the comparing of paths and the evaluation of tags (LABEL, UUID, etc.)."),
                Arg::with_name("canonicalize").long("canonicalize").short("c").help("Canonicalize all printed paths."),
                Arg::with_name("df").long("df").short("D").help("Imitate the output of df."),
                Arg::with_name("direction").long("direction").short("d").takes_value(true).help("The search direction, either forward or backward."),
                Arg::with_name("evaluate").long("evaluate").short("e").help("Convert all tags (LABEL, UUID, PARTUUID or PARTLABEL) to the corresponding device names."),
                Arg::with_name("tab-file").long("tab-file").short("F").takes_value(true).help("Search in an alternative file.  If used with --fstab, --mtab or --kernel, then it overrides the default paths."),
                Arg::with_name("first-only").long("first-only").short("f").help("Print the first matching filesystem only."),
                Arg::with_name("help").long("help").short("h").help("Display help text and exit."),
                Arg::with_name("invert").long("invert").short("i").help("Invert the sense of matching."),
                Arg::with_name("json").long("json").short("J").help("Use JSON output format."),
                Arg::with_name("kernel").long("kernel").short("k").help("Search in /proc/self/mountinfo. The output is in the tree-like format."),
                Arg::with_name("list").long("list").short("l").help("Use the list output format."),
                Arg::with_name("mountpoint").long("mountpoint").short("M").takes_value(true).help("Explicitly define the mountpoint file or directory. See also --target."),
                Arg::with_name("mtab").long("mtab").short("m").help("Search in /etc/mtab. The output is in the list format by default (see --tree)."),
                Arg::with_name("task").long("task").short("N").takes_value(true).help("Use alternative namespace /proc/<tid>/mountinfo rather than the default /proc/self/mountinfo."),
                Arg::with_name("noheadings").long("noheadings").short("n").help("Do not print a header line."),
                Arg::with_name("options").long("options").short("O").takes_value(true).help("Limit the set of printed filesystems."),
                Arg::with_name("output").long("output").short("o").takes_value(true).help("Define output columns. See the --help output to get a list of the currently supported columns."),
                Arg::with_name("output-all").long("output-all").help("Output almost all available columns."),
                Arg::with_name("pairs").long("pairs").short("P").help(r#"Use key="value" output format."#),
                Arg::with_name("poll").long("poll").short("p").takes_value(true).help("Monitor changes in the /proc/self/mountinfo file."),
                Arg::with_name("pseudo").long("pseudo").help("Print only pseudo filesystems."),
                Arg::with_name("submounts").long("submounts").short("R").help("Print recursively all submounts for the selected filesystems."),
                Arg::with_name("raw").long("raw").short("r").help("Use raw output format."),
                Arg::with_name("real").long("real").help("Print only real filesystems."),
                Arg::with_name("source").long("source").short("S").takes_value(true).help("Explicitly define the mount source."),
                Arg::with_name("fstab").long("fstab").short("s").help("Search in /etc/fstab."),
                Arg::with_name("target").long("target").short("T").takes_value(true).help("Define the mount target."),
                Arg::with_name("types").long("types").short("t").takes_value(true).help("Limit the set of printed filesystems."),
                Arg::with_name("tree").long("tree").help("Enable tree-like output if possible."),
                Arg::with_name("uniq").long("uniq").short("U").help("Ignore filesystems with duplicate mount targets, thus effectively skipping over-mounted mount points."),
                Arg::with_name("notruncate").long("notruncate").short("u").help("Do not truncate text in columns."),
                Arg::with_name("nofsroot").long("nofsroot").short("v").help("Do not print a [/dir] in the SOURCE column for bind mounts or btrfs subvolumes."),
                Arg::with_name("timeout").long("timeout").short("w").takes_value(true).help("Specify an upper limit on the time for which --poll will block, in milliseconds."),
                Arg::with_name("verify").long("verify").short("x").help("Check mount table content. The default is to verify /etc/fstab parsability and usability."),
                Arg::with_name("verbose").long("verbose").help("Force findmnt to print more information (--verify only for now)."),
            ])
            // .setting(AppSettings::ArgRequiredElseHelp)
            .about("findmnt")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // MS Types
            Palette {
                regexp: Regex::new(r#"\b(fat|vfat|ntfs|msdos)\b"#).unwrap(),
                colours: vec![&Colours::OnCyan],
            },
            // Common Types
            Palette {
                regexp: Regex::new(r#"\b(ext\d|xfs|btrfs|nfs)\b"#).unwrap(),
                colours: vec![&Colours::Cyan],
            },
            // Like comment, leave at end always
            Palette {
                regexp: Regex::new(r#"^.*(?=cgroup|tmpfs).*$"#).unwrap(),
                colours: vec![&Colours::BBlack],
            },
            // RO
            Palette {
                regexp: Regex::new(r#"(?:\s)ro"#).unwrap(),
                colours: vec![&Colours::BGreen],
            },
            // RW
            Palette {
                regexp: Regex::new(r#"(?:\s)rw"#).unwrap(),
                colours: vec![&Colours::BRed],
            },
            // Mount Path
            Palette {
                regexp: Regex::new(r#"(?<=─|-)(?:\/([^\/ ]+))+"#).unwrap(),
                colours: vec![&Colours::UnChanged,&Colours::BYellow],
            },
            // Devices
            Palette {
                regexp: Regex::new(r#"\s\/dev(?:\/([^\/ ]+))+"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
        ]
    }
}
