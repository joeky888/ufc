use clap::App;
pub struct Powershell {}

impl Powershell {
    pub fn new() -> App<'static> {
        App::new("powershell").about("Generate powershell completion")
    }
}
