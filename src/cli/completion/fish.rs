use clap::App;
pub struct Fish {}

impl Fish {
    pub fn new() -> App<'static> {
        App::new("fish").about("Generate fish completion")
    }
}
