//! The `check` command: runs the quality gates in order and stops at the first failing step.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::XtaskError;
use crate::{lint, sync_metadata};

pub fn workspace_root() -> Result<PathBuf, XtaskError> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or(XtaskError::WorkspaceRoot)
}

pub fn run(quick: bool) -> Result<(), XtaskError> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    run_step(&cargo, &root, "fmt", &["fmt", "--all", "--", "--check"])?;
    run_step(
        &cargo,
        &root,
        "clippy",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    println!("==> lint");
    lint::run(&root)?;
    let tests: &[&str] = if quick {
        &[
            "test",
            "--workspace",
            "--lib",
            "--bins",
            "--test",
            "integration",
        ]
    } else {
        &["test", "--workspace"]
    };
    run_step(&cargo, &root, "test", tests)?;
    if !quick {
        println!("==> sync-metadata --check");
        sync_metadata::run(true)?;
    }
    println!("check passed");
    Ok(())
}

fn run_step(cargo: &str, root: &Path, step: &'static str, args: &[&str]) -> Result<(), XtaskError> {
    println!("==> {step}: cargo {}", args.join(" "));
    let status = Command::new(cargo)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|source| XtaskError::Spawn {
            program: cargo.to_owned(),
            source,
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(XtaskError::StepFailed {
            step,
            code: status.code(),
        })
    }
}
