use std::path::Path;
use std::process::Command;

/// Build command to run MPI job
pub fn run_command(executable: &Path, args: &crate::Args) -> Command {
    let mpiexec = if let Some(ref c) = args.mpiexec {
        c.clone()
    } else if let Ok(c) = std::env::var("MPIEXEC") {
        c
    } else if let Ok(prefix) = std::env::var("MPI_HOME") {
        format!("{prefix}/bin/mpiexec")
    } else {
        // assume mpiexec is in PATH
        "mpiexec".to_string()
    };
    let mut mpirun = Command::new(mpiexec);

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
