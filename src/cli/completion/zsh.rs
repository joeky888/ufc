use clap::App;
pub struct Zsh {}

impl Zsh {
    pub fn new() -> App<'static> {
        App::new("zsh").about("gen zsh completion")
    }
}
