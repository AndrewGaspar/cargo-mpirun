use clap::ArgMatches;
use std::path::Path;
use std::process::{self, Command};

pub fn run(executable: &Path, matches: &ArgMatches) -> ! {
    let mut mpirun = Command::new("mpirun");

    if let Some(np) = matches.value_of("num_processes") {
        mpirun.arg("--np").arg(np);
    }

    if let Some(n) = matches.value_of("num_processes_per_node") {
        mpirun.arg("--npernode").arg(n);
    }

    if matches.is_present("oversubscribe") {
        mpirun.arg("--oversubscribe");
    }

    let code = match mpirun.arg(executable).status() {
        Ok(status) => status.code().unwrap_or(-1),
        Err(_) => {
            eprintln!("Could not find mpirun - is it in your user path?");
            -1
        },
    };

    process::exit(code)
}