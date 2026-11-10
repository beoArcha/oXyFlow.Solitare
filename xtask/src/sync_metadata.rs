//! `sync-metadata`: keeps `metadata.toml` and the Cargo manifests in agreement. Stub for now.

#![allow(
    dead_code,
    reason = "stub: the error variants are constructed once sync-metadata does real work"
)]

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SyncMetadataError {
    #[error("cannot read `{path}`")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn run(_check: bool) -> Result<(), SyncMetadataError> {
    Ok(())
}
