use clap::{App};
pub struct Fish {}

impl Fish {
    pub fn new() -> App<'static, 'static> {
        App::new("fish").about("gen fish completion")
    }
}