# Phase 0 — Bootstrap

Goal: a GitHub repository where `cargo xtask check` passes, the PR flow works end to end, and every
shared file already has its final shape so later tasks do not collide.

### U-01 — Decisions, master prompt, repository

- **Owner:** User
- **Scope:**
  1. Done: the original master prompt is saved as `docs/tech/master_prompt.md` (Polish). P0-02
     turns it into the English reference.
  2. Done: license (MIT), visibility (public), Pyramid (two games), default deck style
     (International) — recorded as R-10…R-13 in `docs/tech/decisions.md`.
  3. Done: the repository is `github.com/beoArcha/oXyFlow.Solitare` (R-12).
  4. Still open: review the assumed defaults in `decisions.md`.
- **Blocks:** nothing.

### P0-01 — Repository skeleton and workspace

- **Owner:** Haiku
- **Depends on:** U-01
- **Files:** the whole repository except `docs/`, `AGENTS.md`, `CLAUDE.md`
- **Scope:**
  1. `git init -b main`, `.gitignore` (`target/`, `*.log`, editor folders). Push only once the user
     has named the remote.
  2. `metadata.toml` with `[project]`: `organization = "oXyFlow"`, `repository = "oXyFlow.Solitare"`,
     `product = "oXySol"`, `binary = "oxysol"`, `version = "0.1.0"`, `description`, `license`, `repository_url`.
  3. `rust-toolchain.toml`: the installed stable channel (1.99.1) with `rustfmt` and `clippy`.
  4. Root `Cargo.toml`: workspace (`.`, `crates/core`, `crates/tui`, `xtask`), edition 2024,
     `[workspace.package]`, `[workspace.dependencies]` for what phases 0–4 need (see the package table
     in `architecture.md`; leave `midly` and `cpal` to phase 6) plus dev-dependencies `tempfile` and `toml`.
     Root package `oxysol` has `src/lib.rs` and `src/main.rs`.
  5. `[workspace.lints]`, inherited by every crate:
     - rust: `unsafe_code = "deny"`, `unused_must_use = "deny"`;
     - clippy, all `deny`: `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`,
       `let_underscore_must_use`, `allow_attributes_without_reason`, `undocumented_unsafe_blocks`.
  6. `clippy.toml`: `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests`,
     `allow-dbg-in-tests` set to `true`.
  7. Compiling crates `crates/core` (`oxysol-core`) and `crates/tui` (`oxysol-tui`). `src/main.rs`
     prints `oxysol <version>` from `CARGO_PKG_VERSION`.
  8. `.cargo/config.toml`: alias `xtask = "run -p xtask --"`.
  9. `xtask` subcommands:
     - `check [--quick]` — the steps listed under "Quality gates" in `architecture.md`; `--quick`
       skips the `architecture` and `e2e` test targets and `sync-metadata --check`;
     - `lint` — collects `.rs` files from `src/`, `crates/*/src`, `crates/*/tests`, `xtask/src`,
       `tests/`, runs every rule, prints `LEVEL rule path:line message`, exits 1 on any error.
       Types: `Severity { Warning, Error }`, `Finding { rule, severity, file, line, message }`,
       `SourceFile { path, text }`. Exceptions come from `lint-exceptions.toml`
       (`[[exception]] rule, path, reason`); an entry without `reason` is itself an error;
     - `sync-metadata [--check]` and `version` — stubs that succeed.
  10. Declared, empty modules that micro tasks fill in without touching shared files:
      `xtask/src/lint/{loc,nesting,comments,err_arms,err_types}.rs` (each
      `pub fn check(file: &SourceFile) -> Vec<Finding>` returning an empty vector),
      `xtask/src/sync_metadata.rs`, `tests/architecture/main.rs`, `tests/integration/main.rs`,
      `tests/e2e/main.rs`, `crates/tui/src/caps.rs`, `crates/tui/src/theme/{mod,ansi,contrast}.rs`,
      `crates/tui/src/widgets/{mod,logo}.rs`, `src/cli.rs`.
  11. `README.md` — name, one-line description, how to run the check.
  12. `LICENSE`: MIT (R-10), copyright holder `oXyFlow`.
- **Acceptance:** `cargo xtask check` and `cargo xtask check --quick` pass; `cargo run` prints
  `oxysol 0.1.0`; first commit on `main`; `release` branch created from it.

### P0-02 — Master prompt in English with amendments

- **Owner:** Haiku
- **Depends on:** P0-01 (the Polish original must be in git history first)
- **Files:** `docs/tech/master_prompt.md`
- **Scope:** translate the file to English in place, faithfully, keeping section numbers. Then
  apply amendments R-01…R-09 from `decisions.md` in the sections they name, marking each changed
  passage with its amendment ID. Do not add requirements of your own.
- **Acceptance:** every original section present; every amendment applied exactly once and
  traceable; no statement contradicts `decisions.md`.

### P0-03 — CI, branch protection, promotion

- **Owner:** Haiku
- **Depends on:** P0-01
- **Files:** `.github/workflows/{pr.yml,main.yml}`, `docs/tech/workflow.md`
- **Scope:**
  - `pr.yml`: on pull requests to `main`; one Ubuntu job; toolchain from `rust-toolchain.toml`,
    cache, `cargo xtask check --quick`.
  - `main.yml`: on push to `main`; matrix Ubuntu, Windows, macOS running `cargo xtask check`; then a
    `promote` job that fast-forwards `release` to the tested commit. A stale run must not move
    `release` backwards (fast-forward only, one run at a time).
  - Branch rules: `main` — pull request required, `pr.yml` check required, squash merge only, no
    force-push or deletion. `release` — updated only by the promotion job, no force-push or deletion.
    Work out which credential the promotion job needs to pass the `release` rule and document it.
  - `workflow.md`: branch naming, PR title format, the three workflows, the secrets and settings the
    user must create by hand.
- **Acceptance:** a throwaway PR shows `pr.yml` running and blocking merge when a test fails; after
  merge `main.yml` goes green and `release` points at the same commit; a direct push to `main` is
  rejected. Report exactly which of these were observed on GitHub and which were not.
- **Out of scope:** deployment (`deploy.yml` is P7-02).
