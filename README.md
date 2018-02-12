# cargo-mpirun
[![Travis](https://img.shields.io/travis/AndrewGaspar/cargo-mpirun.svg?style=flat-square)](https://travis-ci.org/AndrewGaspar/cargo-mpirun)
[![Crates.io](https://img.shields.io/crates/v/cargo-mpirun.svg?style=flat-square)]()

`cargo-mpirun` is cargo custom command similar to "cargo run" that runs the
target program with mpiexec. It streamlines building and testing MPI programs
written in Rust.

## Requirements
A compliant MPI installation is required with its `mpiexec` command available in
PATH. `cargo mpirun` uses `mpiexec` internally, which is the standardized CLI
for starting MPI jobs.

## Installation
`cargo-mpirun` can be installed from Cargo:

```
cargo install cargo-mpirun
```

## Related Projects
If you're interested in writing MPI applications in Rust, take a look at
[rsmpi](https://github.com/bsteinb/rsmpi). It provides a zero-overhead, safe
abstraction over the C MPI APIs.

## Usage
Invoking `cargo-mpirun` is easy. Since `cargo-mpirun` is a cargo custom command,
it is contextualized to the current cargo project. Therefore, just change
directory to the root of your Rust binary MPI project and run:
```
cargo mpirun
```

This will invoke the binary file with `mpiexec` using the default `mpiexec`
parameters.

If you would like to specify the number of processes to be used, the typical
`mpiexec` parameters apply:
```
cargo mpirun -n <num_processes>
```

`num_processes` is a positive integer. Similarly, `--oversubscribe` can be used
to force MPI to allocate more processes than processing elements.

If your Cargo workspace contains multiple bin targets, specify the target with
`--bin`:
```
cargo mpirun --bin <target>
```

Similarly, `--example` can be used to run an example in the project instead.
```
cargo mpirun --example <example>
```

### Full Usage
```
cargo-mpirun 0.1
Andrew Gaspar <andrew.gaspar@outlook.com>
Run the main binary of the local package (src/main.rs) using mpiexec.

USAGE:
    cargo mpirun [OPTIONS]

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