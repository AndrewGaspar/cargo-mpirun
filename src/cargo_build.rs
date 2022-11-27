use crate::MpirunError;

use cargo_metadata::{Artifact, Message};
use std::process::{Command, Output, Stdio};

/// Add Cargo options such as (`--bin`, `--features`, `--target') to `command`.
pub fn add_cargo_options(command: &mut Command, args: &crate::Args) {
    if let Some(ref val) = args.bin {
        command.arg("--bin").arg(val);
    }
    if let Some(ref val) = args.example {
        command.arg("--example").arg(val);
    }
    if let Some(ref val) = args.package {
        command.arg("--package").arg(val);
    }
    if let Some(ref val) = args.jobs {
        command.arg("--jobs").arg(val.to_string());
    }
    if args.release {
        command.arg("--release");
    }
    if args.offline {
        command.arg("--offline");
    }
    if let Some(ref val) = args.features {
        command.arg("--features").arg(val);
    }
    if args.all_features {
        command.arg("--all-features");
    }
    if args.no_default_features {
        command.arg("--no-default-features");
    }
    if let Some(ref val) = args.target {
        command.arg("--target").arg(val);
    }
    if let Some(ref val) = args.manifest_path {
        command.arg("--manifest-path").arg(val);
    }
    for _ in 1..args.verbose {
        command.arg("--verbose");
    }
    if args.quiet {
        command.arg("--quiet");
    }
    if let Some(ref val) = args.color {
        command.arg("--color").arg::<String>(val.to_string());
    }
    if args.frozen {
        command.arg("--frozen");
    }
    if args.locked {
        command.arg("--locked");
    }
    for flag in args.unstable_flags.iter() {
        command.arg("-Z").arg(flag);
    }
}

/// Invoke `cargo build` with suitable options and parse the output to return
/// executable artifacts.
pub fn build(args: &crate::Args) -> crate::Result<Vec<Artifact>> {
    let mut cargo_build = Command::new("cargo");
    cargo_build.arg("build");
    add_cargo_options(&mut cargo_build, args);
    cargo_build.arg("--message-format=json-render-diagnostics");
    if args.verbose > 1 {
        println!("Build: {:?}", cargo_build);
    }
    let Output { status, stdout, .. } = cargo_build.stderr(Stdio::inherit()).output()?;
    if !status.success() {
        return Err(MpirunError::BuildStatus(format!(
            "cargo build return {status}"
        )));
    }
    Message::parse_stream(&*stdout)
        .filter_map(|m| match m {
            Ok(Message::CompilerArtifact(a)) if a.executable.is_some() => Some(Ok(a)),
            _ => None,
        })
        .collect()
}

/// Like `build()`, but ensure that exactly one artifact matches
pub fn build_unique(args: &crate::Args) -> crate::Result<Artifact> {
    let artifacts = build(&args)?;
    if artifacts.len() == 0 {
        return Err(MpirunError::Incompatible(
            "No target found, perhaps --example is needed".to_owned(),
        ));
    }
    if artifacts.len() > 1 {
        return Err(MpirunError::Incompatible(
            "Multiple targets found".to_owned(),
        ));
    }
    Ok(artifacts[0].clone())
}
