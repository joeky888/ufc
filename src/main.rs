use std::io;

use clap::{App, AppSettings, Shell};
mod cli;
use cli::{alias::Alias, completion::Completion, df, docker, ping};

fn build_app() -> App<'static, 'static> {
    App::new("ufc")
        .version("v0.0.1")
        .about("Ultimate Fantastic CLI")
        .author("The UFC Team <https://github.com/joeky888/ufc>")
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpFlags)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        // .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // .global_setting(AppSettings::SubcommandRequiredElseHelp)
        // .global_setting(AppSettings::DisableHelpSubcommand)
        .subcommands(vec![
            Alias::new(),
            Completion::new(),
            df::CMD::new(),
            docker::CMD::new(),
            ping::CMD::new(),
        ])
}

fn main() {
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
        ("alias", Some(_args)) => Alias::gen(),
        ("df", Some(args)) => df::CMD::parse(args),
        ("docker", Some(args)) => docker::CMD::parse(args),
        ("ping", Some(args)) => ping::CMD::parse(args),
        _ => println!("Unsupported command"),
    }
}
