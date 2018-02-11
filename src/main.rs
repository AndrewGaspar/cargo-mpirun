#[macro_use]
extern crate clap;

use clap::App;

use std::process::{self, Command, Stdio};

static CARGO_OPTIONS: &[&str] = &[
    "bin",
    "example",
    "package",
    "jobs",
    "release",
    "target",
    "manifest-path",
    "color",
    "message-format",
    "Z",
];

static CARGO_OPTIONS_MULTIPLE_VALUES: &[&str] = &["features"];

static CARGO_FLAGS: &[&str] = &[
    "all-features",
    "no-default-features",
    "quiet",
    "frozen",
    "locked",
];

static CARGO_FLAGS_MULTIPLE: &[&str] = &["verbose"];

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    if !matches.is_present("mpirun") {
        println!("Only the 'mpirun' sub-command is implemented.");
        process::exit(-1);
    }

    let matches = matches.subcommand_matches("mpirun").unwrap();

    let mut cargo_build = Command::new("cargo");
    cargo_build.arg("build");

    for option in CARGO_OPTIONS {
        if let Some(value) = matches.value_of(option) {
            cargo_build.arg("--".to_owned() + option).arg(value);
        }
    }

    for option in CARGO_OPTIONS_MULTIPLE_VALUES {
        if let Some(values) = matches.values_of(option) {
            cargo_build.arg("--".to_owned() + option).args(values);
        }
    }

    for flag in CARGO_FLAGS {
        if matches.is_present(flag) {
            cargo_build.arg("--".to_owned() + flag);
        }
    }

    for flag in CARGO_FLAGS_MULTIPLE {
        let occurrences = matches.occurrences_of(flag);
        if occurrences > 0 {
            for _ in 0..occurrences {
                cargo_build.arg("--".to_owned() + flag);
            }
        }
    }

    let status = cargo_build
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute cargo build - is cargo in your path?");

    if !status.success() {
        let code = match status.code() {
            Some(x) => x,
            None => 127,
        };
        process::exit(code);
    }
}
