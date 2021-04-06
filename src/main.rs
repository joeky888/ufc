use clap::{App, AppSettings};
mod cli;
use cli::{docker::Docker, ping::Ping};

fn main() {
    let app = App::new("ufc")
        .version("v0.0.1")
        .about("Ultimate Fantastic CLI")
        .author("Joeky <https://github.com/joeky888>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpFlags)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .subcommands(vec![Docker::new(), Ping::new()])
        .get_matches();

    match app.subcommand() {
        ("completion", Some(args)) => Docker::parse(args),
        ("docker", Some(args)) => Docker::parse(args),
        ("ping", Some(args)) => Ping::parse(args),
        _ => println!("Unsupported command"),
        // Some(("completion", _args)) => cli::cli::completion(&app),
        // Some(("docker", args)) => Docker::parse(args),
        // Some(("ping", args)) => Ping::parse(args),
        // None => println!("No subcommand was used"),
        // _ => println!("Unsupported command"),
    }
}
