#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let cli = load_yaml!("cli.yml");
    let _matches = App::from_yaml(cli).get_matches();
}
