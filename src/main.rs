use std::io;

use clap::{App, AppSettings, Shell};
mod cli;
use cli::{completion::Completion, docker::Docker, ping::Ping};

fn build_app() -> App<'static, 'static> {
    App::new("ufc")
        .version("v0.0.1")
        .about("Ultimate Fantastic CLI")
        .author("Joeky <https://github.com/joeky888>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpFlags)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .subcommands(vec![Completion::new(), Docker::new(), Ping::new()])
}

fn main() {
    let app_matches = build_app().get_matches();

    match app_matches.subcommand() {
        ("completion", Some(args)) => {
            match args.subcommand_name() {
                Some("bash") => {
                    build_app().gen_completions_to("ufc", Shell::Bash, &mut io::stdout());
                } // clone was used
                Some("zsh") => {
                    build_app().gen_completions_to("ufc", Shell::Zsh, &mut io::stdout());
                } // commit was used
                Some("fish") => {
                    build_app().gen_completions_to("ufc", Shell::Fish, &mut io::stdout());
                } // push was used
                Some("powershell") => {
                    build_app().gen_completions_to("ufc", Shell::PowerShell, &mut io::stdout());
                } // commit was used
                Some("elvish") => {
                    build_app().gen_completions_to("ufc", Shell::Elvish, &mut io::stdout());
                } // commit was used
                None => {
                    println!("Please specify a completion: \ne.g: ufc completion bash")
                }
                _ => {
                    println!("Unsupported completion")
                } //
            }
        }
        ("docker", Some(args)) => Docker::parse(args),
        ("ping", Some(args)) => Ping::parse(args),
        _ => println!("Unsupported command"),
    }
}
