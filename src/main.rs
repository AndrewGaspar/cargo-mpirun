extern crate cargo_metadata;
#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};
use std::path::*;

mod cargo_build;
mod mpirun;

use cargo_metadata::{Metadata, Package};

fn find_target(metadata: &Metadata, args: &ArgMatches) -> PathBuf {
    let packages: Vec<&Package> = {
        if let Some(name) = args.value_of("package") {
            vec![metadata.packages.iter().find(|p| p.name == name).unwrap()]
        } else {
            metadata.packages.iter().collect()
        }
    };

    let target_directory = Path::new(&metadata.target_directory);
    let executable_path = if args.is_present("release") {
        target_directory.join("release")
    } else {
        target_directory.join("debug")
    };

    if let Some(bin) = args.value_of("bin") {
        // Validates bin actually exists
        let matching_bins: Vec<_> = packages
            .iter()
            .flat_map(|p| p.targets.iter())
            .filter(|t| t.kind[0] == "bin" && t.name == bin)
            .collect();

        if matching_bins.len() > 1 {
            println!(
                "error: `cargo mpirun` can run at most one executable, but multiple were specified"
            );
            std::process::exit(101);
        } else if matching_bins.is_empty() {
            println!("error: no bin target named `{}`", bin);
            std::process::exit(101);
        }

        executable_path.join(bin)
    } else if let Some(example) = args.value_of("example") {
        // Validates example actually exists
        let examples: Vec<_> = packages
            .iter()
            .flat_map(|p| p.targets.iter())
            .filter(|t| t.kind[0] == "example" && t.name == example)
            .collect();

        if examples.len() > 1 {
            println!(
                "error: `cargo mpirun` can run at most one executable, but multiple were specified"
            );
            std::process::exit(101);
        } else if examples.is_empty() {
            println!("error: no example target named `{}`", example);
            std::process::exit(101);
        }

        executable_path.join("examples").join(example)
    } else {
        // Get all bins targets in workspace
        let bin_targets: Vec<_> = packages
            .iter()
            .flat_map(|p| p.targets.iter())
            .filter(|t| t.kind[0] == "bin")
            .collect();

        if bin_targets.is_empty() {
            eprintln!("error: a bin target must be available for `cargo mpirun`");
            std::process::exit(101);
        } else if bin_targets.len() > 1 {
            eprintln!(
                "error: `cargo mpirun` requires that a project only have one executable; use \
                 the `--bin` option to specify which one to run \n\
                 available binaries: {}",
                bin_targets
                    .iter()
                    .map(|t| t.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            std::process::exit(101);
        }

        executable_path.join(&bin_targets[0].name)
    }
}

fn main() {
    let cli = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    let matches = matches
        .subcommand_matches("mpirun")
        .expect("Only the 'mpirun' sub-command is implemented.");

    cargo_build::run(&matches);
    let metadata =
        cargo_metadata::metadata(matches.value_of("manifest-path").map(Path::new)).unwrap();

    let target = find_target(&metadata, matches);

    mpirun::run(&target, matches);
}
