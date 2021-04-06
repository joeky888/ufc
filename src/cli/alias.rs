use clap::App;

pub struct Alias {}

impl Alias {
    pub fn new() -> App<'static, 'static> {
        App::new("alias")
            .about("alias gen")
    }

    pub fn gen() {
        println!("alias docker='ufc docker'
alias ping='ufc ping'"
        );
    }
}
