#[derive(Debug, Default, clap::Args)]
#[command(
    author,
    version,
    about = "Run a binary or example of the local package using mpiexec.",
    after_help = "If neither `--bin` nor `--example` are given, then if the project only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All of the trailing arguments are passed to mpiexec. If you're passing arguments
to both Cargo and the binary, the ones after `--` go to mpiexec, the
ones before go to Cargo."
)]
pub struct Args {
    /// Number of processes
    #[arg(
        short = 'n',
        long = "np",
        // Open MPI's mpiexec uses number of physical cores by default; MPICH
        // uses 1, srun uses number configured in sbatch.
        value_parser = clap::value_parser!(u32).range(1..)
    )]
    pub num_processes: Option<u32>,
    #[arg(
        short = 'N',
        long = "npernode",
        help = "Number of processes per node on all allocated nodes",
        value_parser = clap::value_parser!(u32).range(1..)
    )]
    pub num_processes_per_node: Option<u32>,
    /// Allow nodes to be oversubscribed (may cause severe performance degradation)
    #[arg(long)]
    pub oversubscribe: bool,
    /// Name of bin target to run
    #[arg(long, group = "exec-args")]
    pub bin: Option<String>,
    /// Name of example target to run
    #[arg(long, group = "exec-args")]
    pub example: Option<String>,
    /// Package with the target to run
    #[arg(short = 'p', long)]
    pub package: Option<String>,
    /// Number of parallel build jobs; default to # of CPUs
    #[arg(short = 'j', long, value_parser = clap::value_parser!(u32).range(1..))]
    pub jobs: Option<u32>,
    /// Build artifacts in release mode, with optimizations
    #[arg(long)]
    pub release: bool,
    /// Build without accessing the network
    #[arg(long)]
    pub offline: bool,
    /// Space or comma separated list of features to activate
    #[arg(long)]
    pub features: Option<String>,
    /// Build all available features
    #[arg(long)]
    pub all_features: bool,
    /// Do not build the `default` feature
    #[arg(long)]
    pub no_default_features: bool,
    /// Build for the target triple
    #[arg(long, value_name = "TRIPLE")]
    pub target: Option<String>,
    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<String>,
    /// Override a configuration value
    #[arg(long, value_name = "KEY=VAL")]
    pub config: Vec<String>,
    /// Path to the manifest to execute
    #[arg(long)]
    pub manifest_path: Option<std::path::PathBuf>,
    /// Use verbose output (`-vv` very verbose/build.rs output)
    #[arg(
        short = 'v',
        long,
        action = clap::ArgAction::Count,
    )]
    pub verbose: u8,
    /// No output printed to stdout
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(long, value_name = "WHEN")]
    pub color: Option<clap::ColorChoice>,
    /// Require Cargo.lock and cache are up to date
    #[arg(long)]
    pub frozen: bool,
    /// Require Cargo.lock is up to date
    #[arg(long)]
    pub locked: bool,
    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG")]
    pub unstable_flags: Vec<String>,
    /// Arguments for mpiexec
    #[arg(last = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}
