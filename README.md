# cargo-mpirun
[![Travis](https://img.shields.io/travis/AndrewGaspar/cargo-mpirun.svg?style=flat-square)](https://travis-ci.org/AndrewGaspar/cargo-mpirun)
[![Crates.io](https://img.shields.io/crates/v/cargo-mpirun.svg?style=flat-square)](https://crates.io/crates/cargo-mpirun)

`cargo-mpirun` is a cargo custom command similar to `cargo run` that runs the
target with `mpiexec`. It streamlines building and testing MPI programs written
in Rust.

## Requirements
A compliant MPI installation is required with its `mpiexec` command available in
PATH. `cargo mpirun` uses `mpiexec` internally, which is the standardized CLI
for starting MPI jobs.

## Installation
`cargo-mpirun` can be installed from Cargo:

```
cargo install cargo-mpirun
```

### Library
To use `cargo-mpirun` as a library, skip the install command above and edit your `Cargo.toml` to include

``` toml
[dependencies]
cargo-mpirun = "0.2.0"
```

## Related Projects
If you're interested in writing MPI applications in Rust, take a look at
[rsmpi](https://github.com/rsmpi/rsmpi). It provides a zero-overhead, safe
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
Run a binary or example of the local package using mpiexec.

Usage: cargo mpirun [OPTIONS] [-- <ARGS>...]

Arguments:
  [ARGS]...
          Arguments for mpiexec

Options:
  -n, --np <NUM_PROCESSES>
          Number of processes

  -N, --npernode <NUM_PROCESSES_PER_NODE>
          Number of processes per node on all allocated nodes

      --mpiexec <MPIEXEC>
          Command to execute in place of `mpiexec` (see also MPIEXEC and MPI_HOME)

      --oversubscribe
          Allow nodes to be oversubscribed (may cause severe performance degradation)

      --bin <BIN>
          Name of bin target to run

      --example <EXAMPLE>
          Name of example target to run

  -p, --package <PACKAGE>
          Package with the target to run

  -j, --jobs <JOBS>
          Number of parallel build jobs; default to # of CPUs

      --release
          Build artifacts in release mode, with optimizations

      --offline
          Build without accessing the network

      --features <FEATURES>
          Space or comma separated list of features to activate

      --all-features
          Build all available features

      --no-default-features
          Do not build the `default` feature

      --target <TRIPLE>
          Build for the target triple

      --target-dir <DIRECTORY>
          Directory for all generated artifacts

      --config <KEY=VAL>
          Override a configuration value

      --manifest-path <MANIFEST_PATH>
          Path to the manifest to execute

  -v, --verbose...
          Use verbose output (`-vv` very verbose/build.rs output)

  -q, --quiet
          No output printed to stdout

      --color <WHEN>
          Possible values:
          - auto:   Use colored output if writing to a terminal/TTY
          - always: Always use colored output
          - never:  Never use colored output

      --frozen
          Require Cargo.lock and cache are up to date

      --locked
          Require Cargo.lock is up to date

  -Z <FLAG>
          Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information

If neither `--bin` nor `--example` are given, then if the project only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All of the trailing arguments are passed to mpiexec. If you're passing arguments
to both Cargo and the binary, the ones after `--` go to mpiexec, the
ones before go to Cargo.

Environment variables:

  MPIEXEC  - Command to use for mpiexec
  MPI_HOME - Find mpiexec in $MPI_HOME/bin/mpiexec
```
