use clap::ArgMatches;
use serde_json;
use std::process::{self, Command, Stdio};
use std::str;

static CARGO_OPTIONS: &[&str] = &["manifest-path", "color", "Z"];

static CARGO_OPTIONS_MULTIPLE_VALUES: &[&str] = &["features"];

static CARGO_FLAGS: &[&str] = &[
    "all-features",
    "no-default-features",
    "quiet",
    "frozen",
    "locked",
];

static CARGO_FLAGS_MULTIPLE: &[&str] = &["verbose"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub kind: Vec<String>,
    pub crate_types: Vec<String>,
    pub name: String,
    pub src_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub targets: Vec<Target>,
    pub manifest_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub packages: Vec<Package>,
}

pub fn run<'a>(matches: &ArgMatches<'a>) -> Metadata {
    let mut cargo_metadata = Command::new("cargo");
    cargo_metadata
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps");

    for option in CARGO_OPTIONS {
        if let Some(value) = matches.value_of(option) {
            cargo_metadata.arg("--".to_owned() + option).arg(value);
        }
    }

    for option in CARGO_OPTIONS_MULTIPLE_VALUES {
        if let Some(values) = matches.values_of(option) {
            cargo_metadata.arg("--".to_owned() + option).args(values);
        }
    }

    for flag in CARGO_FLAGS {
        if matches.is_present(flag) {
            cargo_metadata.arg("--".to_owned() + flag);
        }
    }

    for flag in CARGO_FLAGS_MULTIPLE {
        for _ in 0..matches.occurrences_of(flag) {
            cargo_metadata.arg("--".to_owned() + flag);
        }
    }

    let output = cargo_metadata
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute cargo metadata - is cargo in your path?");

    if !output.status.success() {
        let code = match output.status.code() {
            Some(x) => x,
            None => 127,
        };
        process::exit(code);
    }

    serde_json::from_str(str::from_utf8(&output.stdout).unwrap()).unwrap()
}
