# cargo-mpirun
A cargo extension similar to "cargo run" that runs the target program with mpirun.

# Usage
```
cargo-mpirun-mpirun 0.1
Andrew Gaspar <andrew.gaspar@outlook.com>
Run the main binary of the local package (src/main.rs) using mpiexec.

USAGE:
    cargo-mpirun mpirun [OPTIONS]

OPTIONS:
    -n, --np <num_processes>          Number of processes to run
    -N, --npernode <num_processes>
            Launch num_processes per node on all allocated nodes

        --oversubscribe
            Nodes are allowed to be oversubscribed, even on a managed system,
            and overloading of processing elements
        --bin <NAME>                  Name of the bin target to run
        --example <NAME>              Name of the example target to run
    -p, --package <NAME>              Package with the target to run
    -j, --jobs <N>
            Number of parallel jobs, defaults to # of CPUs

        --release
            Build artifacts in release mode, with optimizations

        --features <FEATURE>...
            Space-separated list of features to also build

        --all-features                Build all available features
        --no-default-features         Do not build the `default` feature
        --target <TRIPLE>             Build for the target triple
        --manifest-path <PATH>        Path to the manifest to execute
    -v, --verbose
            Use verbose output (-vv very verbose/build.rs output)

    -q, --quiet                       No output printed to stdout
        --color <WHEN>                Coloring [values: auto, always, never]
        --message-format <FMT>
            Error format [default: human]  [values: human, json]

        --frozen
            Require Cargo.lock and cache are up to date

        --locked                      Require Cargo.lock is up to date
    -Z <FLAG>...                      Unstable (nightly-only) flags to Cargo
    -h, --help                        Prints help information
    -V, --version                     Prints version information

If neither `--bin` nor `--example` are given, then if the project only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All of the trailing arguments are passed to mpiexec. If you're passing arguments
to both Cargo and the binary, the ones after `--` go to mpiexec, the ones before
go to Cargo.


```