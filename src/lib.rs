use thiserror::Error;

#[derive(Error, Debug)]
pub enum MpirunError {
    #[error("cargo build failed")]
    Build(#[from] std::io::Error),
    #[error("cargo build status")]
    BuildStatus(String),
    #[error("no unique artifact")]
    Incompatible(String),
    #[error("mpiexec failed")]
    Run(String),
}

pub type Result<T> = std::result::Result<T, MpirunError>;

mod args;
mod cargo_build;
mod mpirun;

pub use self::{
    args::Args,
    cargo_build::{add_cargo_options, build, build_unique},
    mpirun::run_command,
};
