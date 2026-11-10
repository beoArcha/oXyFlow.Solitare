//! `cargo xtask`: development tooling for the oXySol workspace.

mod check;
mod error;
mod lint;
mod sync_metadata;

use std::env;
use std::error::Error;
use std::process::ExitCode;

use error::XtaskError;

const USAGE: &str =
    "usage: cargo xtask <check [--quick] | lint | sync-metadata [--check] | version>";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match dispatch(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            let mut cause = err.source();
            while let Some(source) = cause {
                eprintln!("  caused by: {source}");
                cause = source.source();
            }
            ExitCode::FAILURE
        }
    }
}

fn dispatch(args: &[String]) -> Result<(), XtaskError> {
    let Some((command, flags)) = args.split_first() else {
        return Err(usage());
    };
    match command.as_str() {
        "check" => check::run(parse_flag(flags, "--quick")?),
        "lint" if flags.is_empty() => lint::run(&check::workspace_root()?),
        "sync-metadata" => Ok(sync_metadata::run(parse_flag(flags, "--check")?)?),
        "version" if flags.is_empty() => {
            println!("version: not implemented yet");
            Ok(())
        }
        _ => Err(usage()),
    }
}

fn parse_flag(flags: &[String], allowed: &str) -> Result<bool, XtaskError> {
    let mut present = false;
    for flag in flags {
        if flag != allowed {
            return Err(usage());
        }
        present = true;
    }
    Ok(present)
}

fn usage() -> XtaskError {
    XtaskError::Usage(USAGE.to_owned())
}
