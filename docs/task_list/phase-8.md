# Phase 8 — Stabilization and release

Goal: the game checked on real terminals, polished, and released as 0.1.0.

### U-03 — Terminal matrix

- **Owner:** User
- **Depends on:** P7-01
- **Scope:** run the game in each environment and record the result (OK, or the problem with a
  screenshot). Agents cannot verify this.

  | Environment | Start and exit | Symbols and colors | Keyboard | Mouse | Resize | Sound |
  |---|---|---|---|---|---|---|
  | Windows 10/11 CMD | | | | | | |
  | Windows PowerShell | | | | | | |
  | Windows Terminal | | | | | | |
  | EXE by double click (own window, no second window) | | | | | | |
  | Linux graphical terminal | | | | | | |
  | macOS Terminal | | | | | | |
  | SSH to Linux (musl build) | | | | | | n/a |
  | PuTTY | | | | | | n/a |

  In one environment also: every theme, both locales, every front and back style in each glyph
  mode, 16-color mode, a 100 × 30 window and a maximized one, both sound styles, keyboard-only and
  mouse-only play of one full game, terminal state after exit and after a forced panic.
- **Acceptance:** the table filled in here; problems described well enough to reproduce.

### P8-01 — Fixes and visual polish

- **Owner:** Sonnet
- **Depends on:** U-03
- **Files:** as required by the U-03 findings
- **Scope:** fix the matrix findings, each with a reproducing test where possible. Then a visual
  pass against the product priorities: readability of board and cards, consistent borders and
  spacing, hierarchy of header and footer, visibility of focus, the look at 100 × 30 and on a large window.
- **Acceptance:** every U-03 finding closed or documented as a known limitation in the wiki FAQ;
  `cargo xtask check` passes.
- **Out of scope:** new features.

### P8-02 — Final review

- **Owner:** Sonnet
- **Depends on:** P8-01
- **Files:** fixes across the repository
- **Scope:** review the whole project against `master_prompt.md` and `decisions.md`: architecture
  boundaries, error handling and logging (one log at the boundary, `source` kept), justification of
  every exception (`#[allow]`, `unsafe`, `lint-exceptions.toml`), comments, naming, needless
  abstractions and dependencies, accessibility rules, test coverage of MVP requirements. Every
  finding names a rule and a location; style preferences are not findings.
- **Acceptance:** a report: each MVP requirement with a status (met / partial / missing) and
  evidence (a test or a file); a list of known limitations; `cargo xtask check` passes.

### F-03 — Checkpoint: release sign-off

- **Owner:** Fable
- **Depends on:** P8-02
- **Scope:** read the P8-02 report and the U-03 table, not the code. Decide: release, or the list
  of blocking items. Record post-MVP items (configurable per-game options, streaming music, raster
  graphics, mobile, more deck styles) as a short backlog in `decisions.md`.
- **Acceptance:** a go / no-go verdict with reasons.

### P8-03 — Release 0.1.0

- **Owner:** Haiku + User
- **Depends on:** F-03, P7-02
- **Files:** `metadata.toml`, `Cargo.toml`, `Cargo.lock`
- **Scope:** Haiku: a `chore` PR that sets the version in `metadata.toml` and runs
  `cargo xtask sync-metadata`. User: merge it, wait for `main.yml` to promote to `release`, run the
  deploy workflow, download the artifacts and start the binary on at least one system.
- **Acceptance:** tag `v0.1.0`; a GitHub Release with binaries for Windows, Linux and macOS and the
  card fonts; the web page updated.
