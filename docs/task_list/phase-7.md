# Phase 7 — E2E tests, documentation, deployment

Goal: the full player journey covered by tests on `main`, documentation for players and
developers, and a one-click deployment from `release`.

### P7-01 — E2E harness and core scenarios

- **Owner:** Haiku
- **Depends on:** P3-09, P4-02
- **Files:** `tests/e2e/**`
- **Scope:** in-process tests (decision T-12): a `Harness` combining `App`, a 120 × 40
  `TestBackend`, `JsonStorage` in a temp dir, a recording `Audio` double and a fixed seed, with
  helpers to press keys, click cells and read the screen. Core scenarios:
  1. start → menu → quit;
  2. Klondike: new game → legal moves → undo → redo → hint;
  3. an illegal move shows a message and leaves the board unchanged;
  4. save → new instance → resume the same game;
  5. a corrupt save → alert → menu;
  6. pause stops the clock;
  7. binary smoke test: `--version`, `--help`, an unknown argument (exit code 2).
  Finalize micro cards M-28 and M-29 with a worked example and mark them `READY`.
- **Acceptance:** all scenarios pass in `main.yml` on the three systems; tests depend neither on
  real time nor on the home directory.

### P7-02 — Deploy workflow and web page

- **Owner:** Haiku
- **Depends on:** P0-03, P5-01
- **Files:** `.github/workflows/deploy.yml`, `docs/book.toml`, `xtask/src/main.rs` (subcommand `version`), `docs/tech/workflow.md`
- **Scope:**
  - `cargo xtask version` prints the version from `metadata.toml`.
  - `deploy.yml`: `workflow_dispatch` only, refuses to run on any ref but `release`. Steps: read the
    version; stop if tag `v<version>` exists; build `x86_64-pc-windows-msvc`,
    `x86_64-unknown-linux-gnu` (with audio), `x86_64-unknown-linux-musl` (without audio),
    `x86_64-apple-darwin`, `aarch64-apple-darwin`; build the card fonts; package archives with the
    binary, `README` and `LICENSE`; create the tag and a GitHub Release with binaries and fonts;
    build the site from `docs/wiki` (mdBook) with download links and publish it to GitHub Pages.
  - Describe the release procedure in `workflow.md`: version bump PR → `main` → automatic promotion
    → manual deploy.
- **Acceptance:** workflow is valid; `cargo xtask version` has a test. The workflow can only be
  proven on GitHub — state plainly whether it was run and what was observed.
- **Out of scope:** automatic deployment, a browser version of the game.

### P7-03 — Technical documentation and README

- **Owner:** Haiku
- **Depends on:** P3-09
- **Files:** `docs/tech/**` (except `master_prompt.md` and `spikes/`), `README.md`
- **Scope:** reconcile `architecture.md` with the code (signatures, layout). Add `conventions.md`
  (code rules, error handling, localization, themes, accessibility rules, lint exceptions) and
  `testing.md` (test categories, what runs on a PR and what on `main`, how to run each locally).
  Keep `decisions.md` current. `README`: description, installation, running, key summary, links to
  the wiki and releases.
- **Acceptance:** every signature and path in the documents exists in the code; nothing is stated
  twice across files; `README` promises nothing outside MVP.
