use clap::App;
pub struct Elvish {}

impl Elvish {
    pub fn new() -> App<'static> {
        App::new("elvish").about("Generate elvish completion")
    }
}
