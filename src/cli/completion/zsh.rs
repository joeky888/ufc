use clap::App;
pub struct Zsh {}

impl Zsh {
    pub fn new() -> App<'static> {
        App::new("zsh").about("Generate zsh completion")
    }
}
