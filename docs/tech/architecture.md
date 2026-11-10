# oXySol architecture — contract

Status: contract v0, written before implementation. Tasks P1-01, P1-02 and P3-01 may refine it.
Any contract change lands in this file in the same PR. Rationale: `decisions.md`.

## Packages and dependency direction

| Package | Path | Responsibility | Allowed dependencies |
|---|---|---|---|
| `oxysol-core` | `crates/core` | cards, decks, shuffling, rules of four games, session (undo/redo, replay) | `serde`, `thiserror` |
| `oxysol-tui` | `crates/tui` | rendering, layout, input, themes, localization, screens, accessibility | `oxysol-core`, `ratatui` (Crossterm via `ratatui::crossterm`), `serde`, `toml`, `thiserror`, `tracing` |
| `oxysol` | `src/` (lib + bin) | entry point, arguments, paths, JSON persistence, audio output, logging, panic hook | both above, `serde_json`, `directories`, `midly`, `cpal`, `tracing`, `tracing-subscriber`, `thiserror` |
| `xtask` | `xtask/` | `check`, `lint`, `sync-metadata`, `fonts`, `midi`, `version` | `syn`, `proc-macro2`, `toml`, `toml_edit`, `thiserror`, `ra-ap-rustc_lexer` |

`oxysol` → `oxysol-tui` → `oxysol-core`. The engine uses no `std::fs`, `std::io`, `std::env`,
`std::time`, `std::net`, `std::process`, `std::thread` and no printing macros.

## Layout

```text
metadata.toml            canonical metadata (name, version, license, URL)
src/                     main.rs lib.rs cli.rs paths.rs storage.rs logging.rs panic_hook.rs error.rs audio/
crates/core/src/         card.rs deck.rs rng.rs pile.rs moves.rs rules.rs game.rs session.rs
crates/core/src/games/   klondike/ spider/ freecell/ pyramid/   (each: mod.rs rules.rs moves.rs tests.rs)
crates/tui/src/          lib.rs app.rs action.rs input.rs mouse.rs caps.rs clock.rs glyphs.rs i18n.rs
                         settings.rs storage.rs audio.rs announce.rs
                         theme/ layout/ widgets/ screens/
locales/<lang>/          common.toml menu.toml board.toml settings.toml help.toml cards.toml cli.toml audio.toml
assets/cards/fronts/<style>/   style.toml glyphs/*.svg
assets/cards/backs/<style>/    style.toml glyphs/*.svg
assets/cards/codepoints.toml   glyph → code point registry
assets/audio/<style>/          style.toml *.mid (optional scores/*.txt)
xtask/src/               main.rs sync_metadata.rs lint/ fonts.rs midi.rs
tests/                   architecture/ integration/ e2e/   (each with main.rs)
docs/                    wiki/ tech/ task_list/
.github/workflows/       pr.yml main.yml deploy.yml
```

## Engine (`oxysol-core`)

The card model separates identity, rank, suit and order. A card's value is not part of the model —
the game that needs one defines it (Pyramid). Appearance and card backs belong to the TUI.

```rust
pub enum Suit { Clubs, Diamonds, Hearts, Spades }
pub enum Color { Red, Black }                       // Suit::color()
pub enum Rank { Ace, Two, /* … */ King }            // Rank::ALL, index() 0..=12, lower(), higher()
pub struct CardId(pub u16);                         // unique within a deal (Spider has duplicates)
pub struct Card { pub id: CardId, pub suit: Suit, pub rank: Rank }

pub enum PileRef {
    Stock, Waste, Foundation(u8), Tableau(u8), Cell(u8),
    Pyramid { row: u8, index: u8 },
}
pub struct TableCard { pub card: Card, pub face_up: bool }
pub struct PileSnapshot { pub pile: PileRef, pub cards: Vec<TableCard> }   // bottom to top

pub enum Move {
    Stock,                                                    // draw / deal a row / recycle
    Transfer { from: PileRef, to: PileRef, count: u8 },       // top `count` cards of `from`
    Remove { first: PileRef, second: Option<PileRef> },       // Pyramid: a pair, or a lone king
}
impl Move { pub fn source(&self) -> Option<PileRef>; pub fn target(&self) -> Option<PileRef>; }

pub enum MoveError { /* rejection reasons: Unsupported, NoSuchPile, EmptySource, FaceDown,
                        InvalidSequence, TargetRejects, StockEmpty, … */ }

pub enum Variant { Klondike(DrawCount), Spider(SuitCount), FreeCell, Pyramid(PyramidRules) }
pub enum DrawCount { One, Three }
pub enum SuitCount { One, Two, Four }
pub enum PyramidRules { Relaxed, Classic }

pub trait Game {
    fn apply(&mut self, mv: Move) -> Result<(), MoveError>;
    fn legal_moves(&self) -> Vec<Move>;
    fn hint(&self) -> Option<Move>;
    fn is_won(&self) -> bool;
    fn piles(&self) -> Vec<PileSnapshot>;
}
pub fn deal(variant: Variant, seed: u64) -> Box<dyn Game>;    // the only place that knows all games
```

Rules of the `Game` contract:

- `apply` validates, executes and performs automatic effects (flip a card, remove a completed run).
  On `Err` the state is unchanged. No input may panic.
- `legal_moves()` returns exactly the set of moves that `apply` accepts.
- `MoveError` is an expected validation result, not a system error.
- Game modules are `pub(crate)`; the outside world sees only `Game`, `Variant`, `deal` and `Session`.

The session implements history by replaying the move log from the seed (decision T-03):

```rust
pub struct SessionRecord { pub variant: Variant, pub seed: u64, pub moves: Vec<Move>, pub cursor: usize }

impl Session {
    pub fn new(variant: Variant, seed: u64) -> Self;
    pub fn restore(record: SessionRecord) -> Result<Self, RestoreError>;
    pub fn record(&self) -> SessionRecord;
    pub fn apply(&mut self, mv: Move) -> Result<(), MoveError>;   // truncates the redo tail
    pub fn undo(&mut self) -> bool;
    pub fn redo(&mut self) -> bool;
    pub fn restart(&mut self);                                    // same deal, empty history
    pub fn game(&self) -> &dyn Game;
    pub fn variant(&self) -> Variant;
    pub fn seed(&self) -> u64;
}
```

Serialized types (`SessionRecord`, `Move`, `PileRef`, `Variant`) have stable serde names.

## TUI (`oxysol-tui`)

```rust
pub struct Deps {
    pub storage: Box<dyn Storage>,
    pub audio: Box<dyn Audio>,
    pub caps: TerminalCaps,
    pub seeds: Box<dyn FnMut() -> u64>,
}
pub enum Input { Key(KeyEvent), Mouse(MouseEvent), Resize(u16, u16), Tick }

impl App {
    pub fn new(deps: Deps) -> Self;
    pub fn handle(&mut self, input: Input, now: Instant);
    pub fn draw(&mut self, frame: &mut Frame);
    pub fn should_quit(&self) -> bool;
}
pub fn run(deps: Deps) -> Result<(), TuiError>;   // terminal setup, event loop, restore
pub fn restore_terminal();                        // best effort; also called from the panic hook

pub trait Storage {
    fn load_settings(&self) -> Result<Option<Settings>, StorageError>;
    fn save_settings(&mut self, settings: &Settings) -> Result<(), StorageError>;
    fn load_game(&self) -> Result<Option<SavedGame>, StorageError>;
    fn save_game(&mut self, game: &SavedGame) -> Result<(), StorageError>;
    fn clear_game(&mut self) -> Result<(), StorageError>;
}
pub struct SavedGame { pub record: SessionRecord, pub elapsed_ms: u64 }

pub enum SoundCue { CardPick, CardPlace, CardFlip, StockDeal, Illegal, Undo, Hint, Win, MenuMove, MenuSelect }
pub trait Audio {
    fn available(&self) -> bool;
    fn configure(&mut self, settings: &AudioSettings);   // style, volume, cues on/off, music on/off
    fn play(&mut self, cue: SoundCue);
}
```

- **Flow:** terminal event → `input` maps it to a semantic `Action` → `App` updates UI state or calls
  `Session`. The key table in `input.rs` is the single source for mapping, the help overlay and the footer.
- **Board interaction is shared by all games:** after a source is picked, the TUI filters
  `legal_moves()` by `Move::source()`, highlights targets and applies the matching move. A move with
  no target (a king in Pyramid) applies immediately.
- **Geometry:** `layout::compute(...) -> BoardGeometry` returns rectangles for piles and cards. The
  same result drives rendering and mouse hit-testing. Two layouts: columns (Klondike, Spider,
  FreeCell) and pyramid. Card size is the largest step that fits the window.
- **Themes:** `Theme` is a set of semantic tokens resolved for (`ThemeId`, `ColorDepth`). Components
  never use colors directly.
- **Card rendering:** front style and back style are selected independently. Each is drawn in one of
  three glyph modes — `Font` (Private Use Area glyphs from the oXySol card font), `Unicode`, `Ascii` —
  described by the style's `style.toml`. `Font` is opt-in through the graphics-check screen.
- **Terminal capabilities:** `TerminalCaps { color, unicode, remote }` from detection, overridden by
  settings and arguments. `remote` (an SSH session) switches audio off.
- **Accessibility:** every `Action` has a key binding and a mouse path. `announce` produces a
  one-line plain-text description of the focus and the last event ("Tableau 3: seven of hearts
  selected, 2 targets") shown in the status area. Sound cues duplicate, never replace, visible feedback.
- **Clock:** `GameClock` computes elapsed time from a passed-in `Instant`; pause and modals stop it.

## Application (`oxysol`)

- **Persistence:** `JsonStorage` implements `Storage`. `settings.json` (config dir) and
  `savegame.json` (data dir) use the envelope `{ "format": 1, "app_version": "…", "data": … }`.
  A missing file is `Ok(None)`; a corrupt or incompatible file is a concrete `StorageError`. The TUI
  decides how to react (alert, defaults) and logs there.
- **Audio:** `src/audio/` implements `Audio`: MIDI sequencing and a built-in synthesizer as pure
  code (`render` is testable without a device), plus `cpal` output. Sound styles are embedded from
  `assets/audio/` at build time. Feature `audio` can be disabled; a `NullAudio` is used when the
  feature is off, no device exists, or the session is remote.

## Errors and logging

| Kind | Example | Handling |
|---|---|---|
| Gameplay result | illegal move | `MoveError` → status-bar message and cue, no log |
| Recoverable | corrupt save, read-only directory, no audio device | alert or silent fallback, logged once at the boundary, game continues |
| Operational | terminal refuses raw mode | `TuiError` → `main` logs, message on stderr, exit code 1 |
| Critical / panic | broken invariant | panic hook: restore terminal, write diagnostics, message |

Logs go only to `oxysol.log`; nothing is written to stdout or stderr while the TUI runs.

## Quality gates

| Command | Steps | Where |
|---|---|---|
| `cargo xtask check --quick` | fmt check, Clippy `-D warnings`, `xtask lint`, unit and integration tests | locally before a PR; `pr.yml` |
| `cargo xtask check` | the above plus architecture and E2E tests and `sync-metadata --check` | `main.yml` on three systems |

Definitions used by `xtask lint`:

- **LOC:** lines that are neither blank nor comment-only.
- **Nesting depth:** the number of nested `if`, `match`, `for`, `while`, `loop`, block (`{}`,
  `unsafe`, `async`) and block-bodied closure constructs inside a function body. The function body
  is level 0. `else if` stays at the level of its `if`. A `match` arm adds no level.
