#[macro_use]
extern crate clap;

use clap::App;

mod cargo_build;

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    let matches = matches
        .subcommand_matches("mpirun")
        .expect("Only the 'mpirun' sub-command is implemented.");

    cargo_build::run(&matches);
}
