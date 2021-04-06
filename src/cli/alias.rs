use clap::App;

pub struct Alias {}

impl Alias {
    pub fn new() -> App<'static, 'static> {
        App::new("alias")
            // .args(&[
            //     Arg::with_name("count").short("c").help("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
            // ])
            // .subcommands(vec![
            //     Bash::new(),
            //     Fish::new(),
            //     Zsh::new(),
            //     Powershell::new(),
            //     Elvish::new(),
            // ])
            // .global_setting(AppSettings::AllowExternalSubcommands)
            // .global_setting(AppSettings::TrailingValues)
            .about("alias gen")
    }

    pub fn gen() {
        println!("alias docker='ufc docker'
alias ping='ufc ping'"
        );
    }
}
