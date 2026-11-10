//! Source lint runner. Rule modules return findings; this module collects the sources, applies
//! `lint-exceptions.toml` and prints the result.

#![allow(
    dead_code,
    reason = "the lint types are populated by the rule modules, which are stubs for now"
)]

pub mod comments;
pub mod err_arms;
pub mod err_types;
pub mod loc;
pub mod nesting;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::error::XtaskError;

const EXCEPTIONS_FILE: &str = "lint-exceptions.toml";

const RULES: [fn(&SourceFile) -> Vec<Finding>; 5] = [
    loc::check,
    nesting::check,
    comments::check,
    err_arms::check,
    err_types::check,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub rule: &'static str,
    pub severity: Severity,
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub text: String,
}

struct Exception {
    rule: String,
    path: String,
}

pub fn run(root: &Path) -> Result<(), XtaskError> {
    let (exceptions, mut findings) = load_exceptions(root)?;
    for file in collect_sources(root)? {
        for rule in RULES {
            findings.extend(rule(&file));
        }
    }
    findings.retain(|finding| !is_excepted(finding, &exceptions));
    for finding in &findings {
        print_finding(finding);
    }
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    if errors == 0 {
        Ok(())
    } else {
        Err(XtaskError::LintFailed { count: errors })
    }
}

fn print_finding(finding: &Finding) {
    let level = match finding.severity {
        Severity::Warning => "WARNING",
        Severity::Error => "ERROR",
    };
    println!(
        "{level} {} {}:{} {}",
        finding.rule,
        display_path(&finding.file),
        finding.line,
        finding.message
    );
}

fn load_exceptions(root: &Path) -> Result<(Vec<Exception>, Vec<Finding>), XtaskError> {
    let path = root.join(EXCEPTIONS_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            return Ok((Vec::new(), Vec::new()));
        }
        Err(source) => return Err(XtaskError::Io { path, source }),
    };
    let table: toml::Table = text.parse().map_err(|source| XtaskError::Exceptions {
        path: path.clone(),
        source,
    })?;
    let entries = table
        .get("exception")
        .and_then(toml::Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut exceptions = Vec::new();
    let mut findings = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let fields = entry.as_table();
        let field = |key: &str| {
            fields
                .and_then(|table| table.get(key))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
        };
        let reason = field("reason").filter(|reason| !reason.trim().is_empty());
        match (field("rule"), field("path"), reason) {
            (Some(rule), Some(path), Some(_)) => exceptions.push(Exception { rule, path }),
            (Some(_), Some(_), None) => {
                findings.push(exception_finding(index, "has no reason"));
            }
            _ => findings.push(exception_finding(index, "needs a rule and a path")),
        }
    }
    Ok((exceptions, findings))
}

fn exception_finding(index: usize, problem: &str) -> Finding {
    Finding {
        rule: "exceptions",
        severity: Severity::Error,
        file: PathBuf::from(EXCEPTIONS_FILE),
        line: 1,
        message: format!("exception #{} {problem}", index + 1),
    }
}

fn is_excepted(finding: &Finding, exceptions: &[Exception]) -> bool {
    let path = display_path(&finding.file);
    exceptions
        .iter()
        .any(|exception| exception.rule == finding.rule && exception.path == path)
}

fn collect_sources(root: &Path) -> Result<Vec<SourceFile>, XtaskError> {
    let mut paths = Vec::new();
    for dir in source_roots(root)? {
        collect_rust_files(&dir, &mut paths)?;
    }
    paths.sort();
    paths.iter().map(|path| read_source(root, path)).collect()
}

fn source_roots(root: &Path) -> Result<Vec<PathBuf>, XtaskError> {
    let mut roots = vec![
        root.join("src"),
        root.join("tests"),
        root.join("xtask").join("src"),
    ];
    for crate_dir in list_dir(&root.join("crates"))? {
        roots.push(crate_dir.join("src"));
        roots.push(crate_dir.join("tests"));
    }
    Ok(roots.into_iter().filter(|dir| dir.is_dir()).collect())
}

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), XtaskError> {
    for path in list_dir(dir)? {
        if path.is_dir() {
            collect_rust_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

fn list_dir(dir: &Path) -> Result<Vec<PathBuf>, XtaskError> {
    let io_error = |source: io::Error| XtaskError::Io {
        path: dir.to_path_buf(),
        source,
    };
    fs::read_dir(dir)
        .map_err(io_error)?
        .map(|entry| entry.map(|entry| entry.path()).map_err(io_error))
        .collect()
}

fn read_source(root: &Path, path: &Path) -> Result<SourceFile, XtaskError> {
    let text = fs::read_to_string(path).map_err(|source| XtaskError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let relative = path.strip_prefix(root).unwrap_or(path).to_path_buf();
    Ok(SourceFile {
        path: relative,
        text,
    })
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
