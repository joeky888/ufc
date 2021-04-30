use clap::App;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("ualias")
            .about("ualias gen (like alias but with a 'u' prefixed for each subcommand)")
    }

    pub fn gen(subcommands: &Vec<&str>) {
        // let subcommands = vec!["df", "docker", "dig", "du", "env", "fdisk", "free", "ping", "top"];
        let mut alias = String::new();
        for cmd_str in subcommands.iter() {
            alias.push_str(format!("alias u{}='ufc {}'\n", cmd_str, cmd_str).as_str());
        }
        alias.pop();
        println!("{}", alias);
    }
}
