use clap::App;

pub struct Alias {}

impl Alias {
    pub fn new() -> App<'static, 'static> {
        App::new("alias").about("alias gen")
    }

    pub fn gen() {
        let subcommands = vec!["docker", "ping"];
        let mut alias = String::new();
        for cmd_str in subcommands.iter() {
            alias.push_str(format!("alias {}='ufc {}'\n", cmd_str, cmd_str).as_str());
        }
        alias.pop();
        println!("{}", alias);
    }
}
