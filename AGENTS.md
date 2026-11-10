# AGENTS.md — rules for all agents

Repository `oXyFlow.Solitare`, product **oXySol**, binary `oxysol`: a terminal (TUI) solitaire in Rust
with Klondike, Spider, FreeCell and Pyramid. Do not rename anything.

**Language:** code, comments, documentation, commit messages, PR titles and reports are in English.
`en-US` is the source locale. Every other language is a translation of it.

## What to read

- **Micro task** (Antigravity instances Beo, Oxy, Son, Dad): this file and your card in
  `docs/task_list/micro.md`. Nothing else unless the card names it.
- **Phase task** (Claude Sonnet or Haiku): this file, `docs/tech/architecture.md`, and your card in
  `docs/task_list/phase-N.md`.
- If sources disagree: `docs/tech/decisions.md` overrides `docs/tech/master_prompt.md`.

## Architecture boundaries

- `crates/core` (engine and games) must not depend on Ratatui, Crossterm, the terminal, colors,
  localization, clocks, files, audio or the network.
- `crates/tui` renders and turns input into commands. It never implements game rules and changes
  engine state only through `Session`.
- `src/` (package `oxysol`) wires the application: persistence, audio output, logging, panic hook, arguments.
- Dependency direction: `oxysol` → `oxysol-tui` → `oxysol-core`. Never the other way.

## Code rules

- File size: above 300 LOC is a warning, above 500 LOC an error. Nesting depth: 5 warning, 7 error.
- No ordinary comments inside function bodies and no decorative separators. Allowed: `///`, `//!`,
  `// SAFETY:`. Prefer a well-named function or constant over a comment.
- Forbidden outside tests: `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`, `dbg!`.
- `unsafe` and `#[allow]` only with a justification (`reason = "..."`, `// SAFETY:`).
- Never ignore a `Result`. No empty `Err` arms.
- Errors: concrete types per module (`thiserror`) that keep their `source`. No `anyhow`,
  `Box<dyn Error>` or `String` as an error type in a public API.
- An illegal move is a `MoveError` — a normal validation result, never an ERROR log.
- Log a technical error once, at the place that decides how to handle it (`tracing`).
- Choose the simplest solution that satisfies the card. No speculative abstractions.

## Product rules

- User-facing text only through localization keys in `locales/`. Colors only through theme tokens.
- Graphics sources are vector (SVG) only, under `assets/`. Sound sources are MIDI, under `assets/audio/`.
- Accessibility is a requirement, not polish: every action works by keyboard and by mouse, focus
  and selection are visible without color, color is never the only carrier of information.

## Workflow

- One task = one branch from current `main`, named `<type>/<ID>-<slug>` with type `feat`, `fix`,
  `refactor`, `chore`, `docs`, `test` or `ci`. Example: `feat/M-02-lint-nesting`.
- Open a pull request to `main`. Title: `<type>(<scope>): <summary> [<ID>]`. PRs are squash-merged.
  Never push to `main` or `release`.
- Change only the files listed in your card. If you need anything else — including a new
  dependency in `Cargo.toml` — stop and say so in the report.
- If you change a contract in `architecture.md`, update that file in the same PR and say so.
- Tests are written together with the feature.
- Before opening the PR run `cargo xtask check --quick`. It must pass.
- Do not edit the task list; status is updated by the phase gate.
- Never claim a test you did not run or behavior you did not verify.

## Final report

State: what was done, files changed, the actual result of `cargo xtask check --quick`, deviations
from the card, known limitations, open questions.
