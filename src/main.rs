use cargo_mpirun::{build_unique, run_command, Args, MpirunError, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Cargo {
    Mpirun(Args),
}

fn main() -> Result<()> {
    let Cargo::Mpirun(args) = Cargo::parse();

    let artifact = build_unique(&args)?;
    if let Some(ref target) = artifact.executable {
        let mut mpirun = run_command(target.as_std_path(), &args);
        let code = match mpirun.status() {
            Ok(status) => status.code().unwrap_or(-1),
            Err(_) => {
                eprintln!("Could not find mpiexec - is it in your user path?");
                -1
            }
        };
        std::process::exit(code);
    } else {
        return Err(MpirunError::Incompatible(format!(
            "Target {} does not have executable",
            artifact.target.name
        )));
    }
}
