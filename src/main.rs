#![forbid(unsafe_code)]

use std::io;

use clap::{App, AppSettings, Shell};
mod cli;
use cli::{alias, completion::Completion, df, dig, docker, du, env, fdisk, findmnt, free, id, ifconfig, ping, top, ualias};

fn build_app() -> App<'static, 'static> {
    App::new("ufc")
        .version("v0.0.1")
        .about("Ultimate Friendly Colorizer")
        .author("The UFC Team <https://github.com/joeky888/ufc>")
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpFlags)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
            ping::Cmd::new(),
            top::Cmd::new(),
        ])
}

fn main() {
    let supported_cmd = &vec![
        "df", "docker", "dig", "du", "env", "fdisk", "free", "id", "ifconfig", "ping", "top",
    ];

    let app_matches = build_app().get_matches();

    match app_matches.subcommand() {
        ("completion", Some(args)) => match args.subcommand_name() {
            Some("bash") => {
                build_app().gen_completions_to("ufc", Shell::Bash, &mut io::stdout());
            }
            Some("zsh") => {
                build_app().gen_completions_to("ufc", Shell::Zsh, &mut io::stdout());
            }
            Some("fish") => {
                build_app().gen_completions_to("ufc", Shell::Fish, &mut io::stdout());
            }
            Some("powershell") => {
                build_app().gen_completions_to("ufc", Shell::PowerShell, &mut io::stdout());
            }
            Some("elvish") => {
                build_app().gen_completions_to("ufc", Shell::Elvish, &mut io::stdout());
            }
            _ => {
                println!("Unsupported completion")
            }
        },
        ("alias", Some(_args)) => alias::Cmd::gen(supported_cmd),
        ("ualias", Some(_args)) => ualias::Cmd::gen(supported_cmd),
        ("df", Some(args)) => df::Cmd::parse(args),
        ("dig", Some(args)) => dig::Cmd::parse(args),
        ("docker", Some(args)) => docker::Cmd::parse(args),
        ("du", Some(args)) => du::Cmd::parse(args),
        ("env", Some(args)) => env::Cmd::parse(args),
        ("fdisk", Some(args)) => fdisk::Cmd::parse(args),
        ("findmnt", Some(args)) => findmnt::Cmd::parse(args),
        ("free", Some(args)) => free::Cmd::parse(args),
        ("id", Some(args)) => id::Cmd::parse(args),
        ("ifconfig", Some(args)) => ifconfig::Cmd::parse(args),
        ("ping", Some(args)) => ping::Cmd::parse(args),
        ("top", Some(args)) => top::Cmd::parse(args),
        _ => println!("Unsupported command"),
    }
}
