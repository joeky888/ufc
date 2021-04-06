use clap::{App};
mod bash;
use bash::Bash;
mod fish;
use fish::Fish;
mod powershell;
use powershell::Powershell;
mod zsh;
use zsh::Zsh;
mod elvish;
use elvish::Elvish;

pub struct Completion {}

impl Completion {
    pub fn new() -> App<'static, 'static> {
        App::new("completion")
            // .args(&[
            //     Arg::with_name("count").short("c").help("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
            // ])
            .subcommands(vec![
                Bash::new(),
                Fish::new(),
                Zsh::new(),
                Powershell::new(),
                Elvish::new(),
            ])
            // .global_setting(AppSettings::AllowExternalSubcommands)
            // .global_setting(AppSettings::TrailingValues)
            .about("completion gen")
    }
}
