use clap::ArgMatches;
use std::path::Path;
use std::process::{self, Command};

pub fn run(executable: &Path, matches: &ArgMatches) -> ! {
    let mut mpirun = Command::new("mpiexec");

    if let Some(np) = matches.value_of("num_processes") {
        mpirun.arg("-n").arg(np);
    }

    if let Some(n) = matches.value_of("num_processes_per_node") {
        mpirun.arg("--npernode").arg(n);
    }

    if matches.is_present("oversubscribe") {
        mpirun.arg("--oversubscribe");
    }

    mpirun.arg(executable);

    if let Some(args) = matches.values_of("args") {
        mpirun.args(args);
    }

    if matches.is_present("verbose") {
        eprintln!("Running: {:?}", mpirun);
    }

    let code = match mpirun.status() {
        Ok(status) => status.code().unwrap_or(-1),
        Err(_) => {
            eprintln!("Could not find mpiexec - is it in your user path?");
            -1
        }
    };

    process::exit(code)
}
