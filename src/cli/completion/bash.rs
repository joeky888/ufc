use clap::{App};
pub struct Bash {}

impl Bash {
    pub fn new() -> App<'static, 'static> {
        App::new("bash").about("gen bash completion")
    }
}
