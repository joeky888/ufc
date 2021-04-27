#![forbid(unsafe_code)]

use fancy_regex::Regex;
use std::io;

use clap::{App, AppSettings, Arg, Shell};
mod cli;
use cli::{
    alias, cli::SETTINGS, completion::Completion, df, dig, docker, du, env, fdisk, findmnt, free,
    id, ifconfig, ping, top, ualias,
};

fn build_app() -> App<'static, 'static> {
    App::new("ufc")
        .version("v0.8.0")
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
        .args(&[
            Arg::with_name("watch_duration")
                .long("watch")
                .short("w")
                .takes_value(true)
                .help(r#"Optional watch mode, Duration of waiting for executing subcommand periodically. Values can be "1.5h", "2m", "5s" or "1h2m5s", set to "0" to disable. Default: "0""#),
            Arg::with_name("time")
                .long("time")
                .short("t")
                .help("Optional time mode, timing statistics when the subprogram exits"),
        ])
}

fn main() {
    let supported_cmd = &vec![
        "df", "docker", "dig", "du", "env", "fdisk", "free", "id", "ifconfig", "ping", "top",
    ];

    let app_matches = build_app().get_matches();

    // Watch mode
    match app_matches.value_of("watch_duration") {
        Some(value) => {
            let time_re =
                Regex::new(r#"((\d*\.?\d*)[h|H])?((\d*\.?\d*)[m|M])?((\d*\.?\d*)[s|S])?"#).unwrap();
            let captures = time_re.captures(value).unwrap().unwrap();
            // println!("cap: {:?}", captures);
            // println!("cap1: {:?}", captures.get(1));
            // println!("cap2: {:?}", captures.get(2));
            // println!("cap3: {:?}", captures.get(3));
            // println!("cap4: {:?}", captures.get(4));
            // println!("cap5: {:?}", captures.get(5));
            // println!("cap6: {:?}", captures.get(6));
            // println!("cap7: {:?}", captures.get(7));
            // println!("cap8: {:?}", captures.get(8));
            // println!("cap9: {:?}", captures.get(9));
            let h = captures
                .get(2)
                .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
            let m = captures
                .get(4)
                .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
            let s = captures
                .get(6)
                .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
            // println!("h:{} m:{} s:{}", h, m, s);
            let duration = h * 3600.0 + m * 60.0 + s;
            SETTINGS.write().unwrap().watch_duration = if duration != 0.0 {
                duration // hhmmss format
            } else {
                value.parse().unwrap() // ss format
            }
        }
        _ => {}
    }
    // Time mode
    SETTINGS.write().unwrap().time = app_matches.is_present("time");

    match app_matches.subcommand_name() {
        Some(value) => {
            SETTINGS.write().unwrap().subcommand_name = value.to_string();
        }
        _ => {}
    }

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
