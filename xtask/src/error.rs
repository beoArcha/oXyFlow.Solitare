//! Errors of the `xtask` runner.

use std::path::PathBuf;

use crate::sync_metadata::SyncMetadataError;

#[derive(Debug, thiserror::Error)]
pub enum XtaskError {
    #[error("{0}")]
    Usage(String),
    #[error("cannot locate the workspace root")]
    WorkspaceRoot,
    #[error("cannot read `{path}`")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot start `{program}`")]
    Spawn {
        program: String,
        #[source]
        source: std::io::Error,
    },
    #[error("step `{step}` failed (exit code {code:?})")]
    StepFailed {
        step: &'static str,
        code: Option<i32>,
    },
    #[error("`{path}` is not valid TOML")]
    Exceptions {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("lint found {count} error(s)")]
    LintFailed { count: usize },
    #[error(transparent)]
    SyncMetadata(#[from] SyncMetadataError),
}
