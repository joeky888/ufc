#![forbid(unsafe_code)]

use std::io;

use clap::{App, AppSettings, Arg, Clap};
mod cli;
use cli::{
    alias,
    cli::{Opts, SETTINGS},
    completion::Completion,
    df, dig, docker, du, env, fdisk, findmnt, free, id, ifconfig, journalctl, ping, top, ualias,
    universal,
};

use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use termcolor::{BufferWriter, ColorChoice};
use std::io::Write;

fn build_app() -> App<'static> {
    App::new("ufc")
        .version("v0.8.5")
        .about("Ultimate Friendly Colorizer")
        .author("The UFC Team <https://github.com/ufc-cli/ufc>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableVersionForSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .global_setting(AppSettings::DisableHelpFlag)
        .subcommands(vec![
            alias::Cmd::new(),
            ualias::Cmd::new(),
            Completion::new(),
            df::Cmd::new(),
            dig::Cmd::new(),
            docker::Cmd::new(),
            du::Cmd::new(),
            env::Cmd::new(),
            fdisk::Cmd::new(),
            findmnt::Cmd::new(),
            free::Cmd::new(),
            id::Cmd::new(),
            ifconfig::Cmd::new(),
            journalctl::Cmd::new(),
            ping::Cmd::new(),
            top::Cmd::new(),
        ])
        .args(&[
            Arg::new("watch")
                .long("watch")
                .short('w')
                .default_value("0")
                .about(r#"Optional watch mode, Duration of waiting for executing subcommand periodically. Values can be "1.5h", "2m", "5s", "5" or "1.5h2m5s", set to "0" to disable it."#),
            Arg::new("time")
                .long("time")
                .short('t')
                .about("Optional time mode, timing statistics when the subprogram exits"),
            Arg::new("boost")
                .long("boost")
                .short('b')
                .about("Optional boost mode, make mass stdout/stderr print faster by using BufferedStandardStream"),
            Arg::new("nocolor")
                .long("nocolor")
                .short('n')
                .about("Disable colorizer"),
            Arg::new("universal")
                .long("universal")
                .short('u')
                .about("Universal subcommand, this option will try to colorize unsupported subcommands"),
        ])
}

fn main() {
    let supported_cmd = &vec![
        "df",
        "docker",
        "dig",
        "du",
        "env",
        "fdisk",
        "free",
        "id",
        "ifconfig",
        "journalctl",
        "ping",
        "top",
    ];

    let app_matches = build_app().get_matches();
    SETTINGS.write().unwrap().clap_args = Opts::parse();

    match app_matches.subcommand_name() {
        Some(value) => {
            SETTINGS.write().unwrap().subcommand_name = value.to_string();
        }
        _ => {}
    }

    match app_matches.subcommand() {
        Some(("completion", args)) => match args.subcommand_name() {
            Some("bash") => {
                generate::<Bash, _>(&mut build_app(), "ufc", &mut io::stdout());
            }
            Some("zsh") => {
                let stdout = BufferWriter::stdout(ColorChoice::Never);
                let mut buf = stdout.buffer();
                writeln!(buf, "compdef _ufc ufc").unwrap();

                generate::<Zsh, _>(&mut build_app(), "ufc", &mut buf);
                let mut completion = String::from_utf8(buf.as_slice().to_vec()).unwrap();
                completion = completion.strip_suffix(r#"_ufc "$@""#).unwrap().to_string();
                println!("{}", completion);
            }
            Some("fish") => {
                let stdout = BufferWriter::stdout(ColorChoice::Never);
                let mut buf = stdout.buffer();
                generate::<Fish, _>(&mut build_app(), "ufc", &mut buf);
                let mut completion = String::from_utf8(buf.as_slice().to_vec()).unwrap();
                completion = completion.replace("\n", ";");
                println!("{}", completion);
            }
            Some("powershell") => {
                generate::<PowerShell, _>(&mut build_app(), "ufc", &mut io::stdout());
            }
            Some("elvish") => {
                generate::<Elvish, _>(&mut build_app(), "ufc", &mut io::stdout());
            }
            _ => {
                println!("Unsupported completion")
            }
        },
        Some(("alias", _args)) => alias::Cmd::gen(supported_cmd),
        Some(("ualias", _args)) => ualias::Cmd::gen(supported_cmd),
        Some(("df", args)) => df::Cmd::parse(args),
        Some(("dig", args)) => dig::Cmd::parse(args),
        Some(("docker", args)) => docker::Cmd::parse(args),
        Some(("du", args)) => du::Cmd::parse(args),
        Some(("env", args)) => env::Cmd::parse(args),
        Some(("fdisk", args)) => fdisk::Cmd::parse(args),
        Some(("findmnt", args)) => findmnt::Cmd::parse(args),
        Some(("free", args)) => free::Cmd::parse(args),
        Some(("id", args)) => id::Cmd::parse(args),
        Some(("ifconfig", args)) => ifconfig::Cmd::parse(args),
        Some(("journalctl", args)) => journalctl::Cmd::parse(args),
        Some(("ping", args)) => ping::Cmd::parse(args),
        Some(("top", args)) => top::Cmd::parse(args),
        _ => {
            if SETTINGS.read().unwrap().clap_args.universal {
                match app_matches.subcommand() {
                    Some((_, args)) => universal::Cmd::parse(args),
                    _ => {
                        println!("Please try -h or --help to get the full usages");
                    }
                }
            } else {
                println!("Unsupported subcommand, please use -u or --universal to enable universal mode.\nThis option will try to colorize unsupported subcommands");
            }
        }
    }
}
