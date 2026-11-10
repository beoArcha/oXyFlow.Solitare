# Phase 3 — Terminal interface

Goal: a complete, readable interface driven by keyboard and by mouse. Sonnet builds the foundation
and the board (P3-01…P3-03); Haiku adds the screens around it.

Rules for the whole phase:

- Text through `i18n`, colors through theme tokens, symbols through `Glyphs`.
- A new key is added to both locales in the same PR. In `pl-PL` it may be a copy of the English
  text; micro tasks M-17 and M-34 translate.
- Every screen works by keyboard and by mouse. Focus and selection are visible without color
  (border style, marker).
- Rendering is tested on `TestBackend`.

### P3-01 — TUI foundation

- **Owner:** Sonnet
- **Depends on:** P1-02
- **Files:** `crates/tui/**`, `locales/**`
- **Scope:**
  - `lib.rs`: `run`, `restore_terminal`; raw mode, alternate screen, mouse capture, hidden cursor;
    the terminal is restored on every exit path (a `Drop` guard).
  - `app.rs`: `App`, `Screen { MainMenu, GameSelect, Board, Settings }`, modal layer
    `Modal { Help, Pause, Confirm, Alert, Win }`, `handle`, `draw`, `should_quit`.
  - `action.rs`, `input.rs`: `Action` and the key table — the single source for mapping, help and footer.
  - `settings.rs`: `Settings` with locale, theme, front style, back style, glyph mode
    (`Font`/`Unicode`/`Ascii`), color mode, sound style, cues on/off, music on/off, volume.
    Styles are referenced by folder id.
  - `storage.rs`: trait `Storage`, `SavedGame`, `StorageError`. `audio.rs`: trait `Audio`,
    `SoundCue`, `AudioSettings`, `NullAudio`.
  - `i18n.rs`: `t(key)`, `tf(key, params)`, fallback to `en-US`, then to the key. All locale area
    files from `architecture.md` are created and embedded here; `common.toml` is filled. Tests:
    identical key sets in both locales; every key used in the sources exists.
  - `theme/`: token struct, Dark palette, resolution for (`ThemeId`, `ColorDepth`) using `ansi.rs`
    from M-13. Constructor stubs for Light and High Contrast.
  - `glyphs.rs` (Unicode and ASCII sets), `clock.rs` (`GameClock`), `announce.rs` (stub).
    `caps.rs` and `widgets/logo.rs` come from M-15 and M-16.
  - "Terminal too small" screen below 100 × 30 showing current and required size.
  - Stubs with fixed signatures for later tasks: `screens/{main_menu,game_select,settings}.rs`,
    `widgets/overlay/`, `mouse.rs`.
  - Finalize micro cards M-11, M-12 (token list, pairs to test) and mark them `READY`.
- **Acceptance:** `TestBackend` tests: start, stub menu rendered, quit by key, too-small screen,
  resize; `cargo run` starts and leaves the terminal as it found it (checked by hand, or stated as
  not checked); `architecture.md` matches the code.

### P3-02 — Board geometry and rendering

- **Owner:** Sonnet
- **Depends on:** P3-01, P2-01
- **Files:** `crates/tui/src/layout/**`, `crates/tui/src/widgets/{card.rs,pile.rs}`, `crates/tui/src/screens/board/view.rs`
- **Scope:**
  - `layout::compute` → `BoardGeometry` (a rectangle per pile and per card, `hit(col, row)`).
    Column layout: a top row (stock, waste, cells, foundations) and fanned tableau columns.
    Pyramid layout: overlapping rows, stock and waste below.
  - Card size: the largest step that fits the game (for example 7×5, 9×7, 11×9, 13×11); spacing and
    margins grow with the window.
  - A column always fits vertically: fan compression, and in the extreme case face-down cards
    collapse into one row with a counter.
  - Card widget v1: front (rank and suit in the corners) in the International style, back, empty slot,
    focus and selection states. Style-driven rendering arrives in P5-02.
  - Klondike Draw 3: up to three top waste cards fanned horizontally.
  - Board screen: header (game, variant, time), board, status line, footer with key hints.
- **Acceptance:** for every variant at 100 × 30 and 160 × 50: all rectangles inside the area, top
  cards fully visible, ranks of face-up cards never covered; a larger window yields a larger card
  step; a worst-case Spider column test.

### P3-03 — Board controller

- **Owner:** Sonnet
- **Depends on:** P3-02
- **Files:** `crates/tui/src/screens/board/**` (except `view.rs`), `crates/tui/src/app.rs`, `locales/*/board.toml`
- **Scope:**
  - Cursor: movement between piles by geometry, run-depth selection, pick, drop, cancel. After a
    pick, highlight the targets from `legal_moves()`.
  - Commands: stock action, quick move (first legal target, foundation before tableau), undo, redo,
    hint (highlight source and target from `hint()`, message when there is none), restart and new
    game with confirmation, pause (clock stops, board hidden), back to menu.
  - Every `MoveError` variant has a localized status message. No ERROR logs.
  - Win detection → `Modal::Win`. No legal moves → a message.
  - Emit a `SoundCue` at each semantic event through `Audio`.
  - The clock runs from the start of the game; pause and modals stop it.
  - A `persist()` hook called after every session change (filled by P4-03).
  - Finalize micro card M-17 and mark it `READY`.
- **Acceptance:** key-sequence tests on fixed seeds: legal move, illegal move with message, undo/redo,
  hint, pause, quick move, a pair and a king in Pyramid, a deal in Spider; a recording `Audio` double
  proves the cues fire.

### P3-04 — Menus and game selection

- **Owner:** Haiku
- **Depends on:** P3-01
- **Files:** `crates/tui/src/screens/{main_menu.rs,game_select.rs}`, `locales/*/menu.toml`
- **Scope:** main menu: Resume (only when a save exists), New game, Settings, Help, Quit. Game and
  variant selection: Klondike (Draw 1, Draw 3), Spider (1, 2, 4 suits), FreeCell, Pyramid (Relaxed,
  Classic). The logo
  from `widgets/logo.rs`. Arrows and `j`/`k`, Enter, Esc; click selects, wheel scrolls. Centered
  layout that scales with the window.
- **Acceptance:** tests: visiting every item by keyboard and by mouse, Resume hidden without a
  save, every variant creates a session with the right `Variant`, rendering in both locales and in ASCII.

### P3-05 — Settings screen

- **Owner:** Haiku
- **Depends on:** P3-01
- **Files:** `crates/tui/src/screens/settings.rs`, `locales/*/settings.toml`
- **Scope:** options: language, theme, front style, back style, glyph mode, color mode, sound style,
  cues on/off, music on/off, volume. A change applies immediately and is saved through `Storage`.
  When `Audio::available()` is false the sound options are shown as unavailable with the reason.
  Storage failure: an alert; the setting holds for the session.
- **Acceptance:** tests: changing each option changes app state and reaches a test storage, by
  keyboard and by mouse; language switches text at once; a failing storage shows an alert and does
  not interrupt.

### P3-06 — Mouse on the board

- **Owner:** Haiku
- **Depends on:** P3-03
- **Files:** `crates/tui/src/mouse.rs`
- **Scope:** translate mouse events into the same `Action`s the keyboard produces, through
  `BoardGeometry::hit`. Click a card — pick; click a target — drop; click the stock — stock action;
  double click (same card within 400 ms) — quick move; right button — cancel. Drag: press on a
  card, move, release over the target; where the terminal reports no drag events, click–click works.
- **Acceptance:** tests with synthetic events on known geometry: each gesture above, a click on
  empty space, a release outside the board, a click on a face-down card.

### P3-07 — Overlays: help, pause, win, dialogs

- **Owner:** Haiku
- **Depends on:** P3-03
- **Files:** `crates/tui/src/widgets/overlay/**`, `locales/*/help.toml`
- **Scope:** help generated from the key table, grouped by topic, including the mouse gestures;
  a pause overlay that covers the board; a win screen with the elapsed time and "new game" / "menu";
  confirmation and alert dialogs with a clearly focused button, operable by keyboard and by click.
  Centered and scaled with the window.
- **Acceptance:** render tests for every overlay in both locales, Unicode and ASCII; help lists
  every binding in the table; pause shows no card.

### P3-08 — Accessibility: announcer and tests

- **Owner:** Haiku
- **Depends on:** P3-03
- **Files:** `crates/tui/src/announce.rs`, `crates/tui/tests/accessibility.rs`, `locales/*/board.toml`
- **Scope:**
  - `announce`: a one-line plain-text description of the focus and of the last event, built from
    localized card and pile names ("Tableau 3: seven of hearts selected, 2 targets"; "Illegal move:
    needs a black eight"). Shown in the status line of the board.
  - Tests that keep accessibility from regressing:
    1. every `Action` has a key binding and appears in help;
    2. with all colors removed, the rendered board still distinguishes focus, selection, legal
       targets, face-down cards and the four suits;
    3. every interactive element of every screen is reachable by keyboard alone;
    4. every theme meets its contrast target for the token pairs used together.
- **Acceptance:** the four tests pass; announcements exist for every `MoveError` and every action
  that changes the board, in both locales.

### P3-09 — Gate: TUI

- **Owner:** Sonnet
- **Depends on:** P3-04…P3-08, P4-03
- **Files:** fixes in `crates/tui/**`, `locales/**`
- **Scope:** review the merged diff of phases 3 and 4 against `AGENTS.md`: hard-coded text or
  colors, consistency of keys, mouse gestures and focus across screens, completeness of both
  locales, storage error handling. Findings name the rule and location; fix small ones, open cards
  for larger ones. Finalize micro cards M-30…M-34. Update the Status columns.
- **Acceptance:** review report; the full path menu → game → settings → quit exercised for every
  game, by test or by hand (say which); `cargo xtask check` passes.
