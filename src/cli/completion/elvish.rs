use clap::{App};
pub struct Elvish {}

impl Elvish {
    pub fn new() -> App<'static, 'static> {
        App::new("elvish").about("gen elvish completion")
    }
}