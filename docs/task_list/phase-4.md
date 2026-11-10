# Phase 4 — Application: persistence, logging, failures

Goal: the game saves and resumes, and every technical error has exactly one place that handles it.

### P4-01 — JSON persistence

- **Owner:** Haiku
- **Depends on:** P3-01
- **Files:** `src/storage.rs`, `src/paths.rs`, `tests/integration/**`
- **Scope:**
  - `paths.rs`: directories from `directories::ProjectDirs::from("", "oXyFlow", "oXySol")`;
    `OXYSOL_HOME` points everything at one directory (tests, portable use).
  - `JsonStorage` implements `oxysol_tui::Storage`. Files `settings.json` and `savegame.json` in the
    envelope `{ "format": 1, "app_version", "data" }`.
  - Atomic write: temp file in the same directory, then rename. Directories are created on write.
  - Missing file → `Ok(None)`. Syntax error → `StorageError::Corrupt`, and the file is moved to
    `*.corrupt`. Other `format` → `StorageError::Incompatible`. I/O errors keep their `source`.
  - Storage logs nothing — the layer that handles the error logs it.
- **Acceptance:** temp-dir tests: round trip of settings and game, missing file, corrupt file (error
  returned, file moved, next read gives `Ok(None)`), incompatible format, unwritable location (error
  with `source`), `clear_game`.

### P4-02 — Logging, panic hook, entry point

- **Owner:** Haiku
- **Depends on:** P3-01, M-18
- **Files:** `src/{main.rs,lib.rs,logging.rs,panic_hook.rs,error.rs}`, `locales/*/cli.toml`, `tests/architecture/**`
- **Scope:**
  - `main`: parse arguments with `cli::parse` (M-18); `--version` and `--help` print localized text
    and exit; an argument error prints usage and exits with code 2; `--ascii` and `--colors`
    override detected capabilities.
  - `logging.rs`: `tracing-subscriber` writing to `oxysol.log` in the data directory; the file is
    truncated at start when above 1 MB; level from `OXYSOL_LOG` (default `info`). If logging cannot
    start: one warning on stderr before the TUI starts, and the app continues.
  - `panic_hook.rs`, in this order: `oxysol_tui::restore_terminal()`, a short stderr message with
    the panic location and the log path, a diagnostic record through a file handle opened earlier.
    No large allocations; the hook itself must not panic. Ignoring a write error inside the hook is
    the one permitted exception to the ignored-result rule, with a `reason`.
  - `error.rs`: `AppError` aggregating layer errors (`thiserror`). `main` returns `ExitCode`:
    0 success, 1 runtime failure, 2 usage error. A critical error is `tracing::error!` with target
    `oxysol::critical`.
  - No `windows_subsystem`, no `AttachConsole`/`AllocConsole` (decision T-10).
  - Architecture test: `src/main.rs` installs the panic hook before starting the TUI.
- **Acceptance:** a child-process test shows the hook leaving a log record and a non-zero exit code;
  `oxysol --version` prints the version from `metadata.toml`; an unknown argument exits with 2.

### P4-03 — Autosave and resume

- **Owner:** Sonnet
- **Depends on:** P3-03, P4-01
- **Files:** `crates/tui/src/app.rs`, `crates/tui/src/screens/board/**`, `tests/integration/**`, `locales/*/common.toml`
- **Scope:**
  - Save the game (`SessionRecord` + elapsed time) after every session change and when leaving the
    board. A won game removes the save.
  - Startup: load settings (missing or failing → defaults and an alert). Resume rebuilds the session
    with `Session::restore` and restores the clock.
  - A corrupt, incompatible or unreplayable save: alert, remove the save, return to the menu.
  - A write failure during play: one alert per session, one log record, the game continues.
- **Acceptance:** integration tests with `JsonStorage` in a temp dir: play → drop `App` → new `App`
  → Resume gives identical `piles()`, history position and time; a corrupt save leads to an alert
  and the menu; a storage that fails on write neither stops the game nor alerts twice.
