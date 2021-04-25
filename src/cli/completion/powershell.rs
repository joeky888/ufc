use clap::App;
pub struct Powershell {}

impl Powershell {
    pub fn new() -> App<'static, 'static> {
        App::new("powershell").about("gen powershell completion")
    }
}
