use std::path::Path;
use std::process::Command;

/// Build command to run MPI job
pub fn run_command(executable: &Path, args: &crate::Args) -> Command {
    let mut mpirun = Command::new("mpiexec");

    if let Some(n) = args.num_processes {
        mpirun.arg("-n").arg(n.to_string());
    }
    if let Some(n) = args.num_processes_per_node {
        mpirun.arg("--npernode").arg(n.to_string());
    }
    if args.oversubscribe {
        mpirun.arg("--oversubscribe");
    }

    mpirun.arg(executable);
    mpirun.args(&args.args);

    if args.verbose > 0 {
        eprintln!("Running: {:?}", mpirun);
    }
    mpirun
}
