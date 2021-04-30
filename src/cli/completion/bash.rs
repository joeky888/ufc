use clap::App;
pub struct Bash {}

impl Bash {
    pub fn new() -> App<'static> {
        App::new("bash").about("Generate bash completion")
    }
}
