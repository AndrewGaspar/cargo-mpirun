#[macro_use]
extern crate clap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{App, ArgMatches};
use std::path::*;

mod cargo_build;
mod cargo_metadata;

use cargo_metadata::*;

fn find_target(metadata: &Metadata, args: &ArgMatches) -> PathBuf {
    let package: &Package = {
        if let Some(name) = args.value_of("package") {
            metadata.packages.iter().find(|p| p.name == name).unwrap()
        } else {
            &metadata.packages[0]
        }
    };

    let root = Path::new(&package.manifest_path).parent().unwrap();

    let mut executable_path: PathBuf = root.join("target");
    if args.is_present("release") {
        executable_path.push("release");
    } else {
        executable_path.push("debug");
    }

    if let Some(bin) = args.value_of("bin") {
        // Validates bin actually exists
        package
            .targets
            .iter()
            .find(|t| t.kind[0] == "bin" && t.name == bin)
            .unwrap();

        executable_path.push(bin);
    } else if let Some(example) = args.value_of("example") {
        // Validates example actually exists
        package
            .targets
            .iter()
            .find(|t| t.kind[0] == "example" && t.name == example)
            .unwrap();

        executable_path.push("examples");
        executable_path.push(example);
    } else {
        let bin_targets: Vec<_> = package.targets.iter().filter(|t| t.kind[0] == "bin").collect();

        if bin_targets.is_empty() {
            eprintln!("The target package does not contain any bin targets.");
            std::process::exit(-1);
        } else if bin_targets.len() > 1 {
            eprintln!("You must specify --bin if the package contains more than one bin");
            std::process::exit(-1);
        }

        executable_path.push(&bin_targets[0].name);
    }

    executable_path
}

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    let matches = matches
        .subcommand_matches("mpirun")
        .expect("Only the 'mpirun' sub-command is implemented.");

    cargo_build::run(&matches);
    let metadata = cargo_metadata::run(&matches);

    let target = find_target(&metadata, matches);

    println!("{}", target.to_str().unwrap());
}
