use clap::App;
pub struct Zsh {}

impl Zsh {
    pub fn new() -> App<'static, 'static> {
        App::new("zsh").about("gen zsh completion")
    }
}
