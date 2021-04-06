use clap::{App,AppSettings};
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
            .subcommands(vec![
                Bash::new(),
                Fish::new(),
                Zsh::new(),
                Powershell::new(),
                Elvish::new(),
            ])
            .setting(AppSettings::NeedsSubcommandHelp)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("completion gen")
    }
}
