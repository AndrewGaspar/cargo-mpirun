#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let cli = load_yaml!("cli.yml");
    let _matches = App::from_yaml(cli)
        .arg(
            Arg::with_name("args") // args specified manually because of clap issue #1173
                .last(true)
                .multiple(true)
                .allow_hyphen_values(true)
                .help("Arguments passed to the mpirun command"),
        )
        .get_matches();
}
