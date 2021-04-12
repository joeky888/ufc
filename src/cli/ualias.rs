use clap::App;

pub struct Ualias {}

impl Ualias {
    pub fn new() -> App<'static, 'static> {
        App::new("ualias").about("ualias gen (like alias but with a 'u' prefixed for each subcommand)")
    }

    pub fn gen() {
        let subcommands = vec!["df", "docker", "ping"];
        let mut alias = String::new();
        for cmd_str in subcommands.iter() {
            alias.push_str(format!("alias u{}='ufc {}'\n", cmd_str, cmd_str).as_str());
        }
        alias.pop();
        println!("{}", alias);
    }
}
