use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, AppSettings, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("fdisk")
            .args(&[
                Arg::new("FILE").about(r#"[FILE]"#),
                Arg::new("sector-size").long("sector-size").short('b').takes_value(true).about(r#"Specify the sector size of the disk.  Valid values are 512, 1024, 2048, and 4096."#),
                Arg::new("protect-boot").long("protect-boot").short('B').about(r#"Don't erase the beginning of the first disk sector when creating a new disk label.  This feature is supported for GPT and MBR."#),
                Arg::new("compatibility").long("compatibility").short('c').takes_value(true).about(r#"Specify the compatibility mode, 'dos' or 'nondos'.  The default is non-DOS mode.  For backward compatibility,"#),
                Arg::new("help").long("help").short('h').about(r#"Display a help text and exit."#),
                Arg::new("color").long("color").short('L').takes_value(true).about(r#"Colorize the output. The optional argument when can be auto, never or always."#),
                Arg::new("list").long("list").short('l').about(r#"List the partition tables for the specified devices and then exit. If no devices are given, those mentioned in /proc/partitions (if that file exists) are used."#),
                Arg::new("list-details").long("list-details").short('x').about(r#"Like --list, but provides more details."#),
                Arg::new("lock").long("lock").takes_value(true).about(r#"Use exclusive BSD lock for device or file it operates. The optional argument mode can be yes, no (or 1 and 0) or nonblock."#),
                Arg::new("noauto-pt").long("noauto-pt").short('n').about(r#"Don't automatically create a default partition table on empty device."#),
                Arg::new("output").long("output").short('o').takes_value(true).about(r#"Specify which output columns to print."#),
                Arg::new("getsz").long("getsz").short('s').about(r#"Print the size in 512-byte sectors of each given block device. This option is DEPRECATED in favour of "blockdev"."#),
                Arg::new("type").long("type").short('t').takes_value(true).about(r#"Enable support only for disklabels of the specified type, and disable support for all other types."#),
                Arg::new("units").long("units").short('u').takes_value(true).about(r#"When listing partition tables, show sizes in 'sectors' or in 'cylinders'. The default is to show sizes in sectors."#),
                Arg::new("cylinders").long("cylinders").short('C').takes_value(true).about(r#"Specify the number of cylinders of the disk."#),
                Arg::new("heads").long("heads").short('H').takes_value(true).about(r#"Specify  the  number  of heads of the disk.  (Not the physical number, of course, but the number used for partition tables.)  Reasonable values are 255 and 16."#),
                Arg::new("sectors").long("sectors").short('S').takes_value(true).about(r#"Specify the number of sectors per track of the disk.  (Not the physical number, of course, but the number used for partition tables.) A reasonable value is 63."#),
                Arg::new("wipe").long("wipe").short('w').takes_value(true).about(r#"Wipe filesystem, RAID and partition-table signatures from the device, in order to avoid possible collisions."#),
                Arg::new("wipe-partitions").long("wipe-partitions").short('W').takes_value(true).about(r#"Wipe  filesystem, RAID and partition-table signatures from a newly created partitions, in order to avoid possible collisions."#),
                Arg::new("version").long("version").short('V').about(r#"Display version information and exit."#),
            ])
            .setting(AppSettings::ArgRequiredElseHelp)
            .about(r#"fdisk"#)
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // Size 'G'
            Palette {
                regexp: Regex::new(r#"\s\d+[.,]?\d*\s?Gi?B?"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // Size 'M'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d*\s?Mi?B?"#).unwrap(),
                colours: vec![&Colours::Yellow],
            },
            // Size 'K'
            Palette {
                regexp: Regex::new(r#"\s\d*[.,]?\d*\s?Ki?B?"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // ID
            Palette {
                regexp: Regex::new(r#"identifier: (.*)$"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::Cyan],
            },
            // Type
            Palette {
                regexp: Regex::new(r#"type: (.*)$"#).unwrap(),
                colours: vec![&Colours::UnChanged, &Colours::BCyan],
            },
            // Partitions
            Palette {
                regexp: Regex::new(r#"^(?:\/([^\/: ]+))+"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
            // Boot?
            Palette {
                regexp: Regex::new(r#"\*\s\s\s"#).unwrap(),
                colours: vec![&Colours::OnRed, &Colours::BWhite],
            },
            // Disk
            Palette {
                regexp: Regex::new(r#"^(Disk) (?:\/([^\/: ]+))+"#).unwrap(),
                colours: vec![
                    &Colours::Yellow,
                    &Colours::OnYellow,
                    &Colours::BYellow,
                    &Colours::BYellow,
                ],
            },
            // Error
            Palette {
                regexp: Regex::new(r#"fdisk: cannot open ([^:]+).*$"#).unwrap(),
                colours: vec![&Colours::Red, &Colours::BRed],
            },
        ]
    }
}
