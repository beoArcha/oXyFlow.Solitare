# Phase 1 — Engine contract and quality tooling

Goal: a stable `oxysol-core` contract that the games build on, and working quality gates. Sonnet
writes the contract; micro tasks M-01…M-06 fill in the lint rules, metadata sync and architecture tests.

### P1-01 — Engine domain model

- **Owner:** Sonnet
- **Depends on:** P0-01
- **Files:** `crates/core/src/{lib.rs,card.rs,deck.rs,rng.rs,pile.rs,moves.rs,rules.rs}`
- **Scope:**
  - `card.rs`: `Suit`, `Color`, `Rank`, `CardId`, `Card` as in `architecture.md`. `Rank::ALL`,
    `index()`, `lower()`, `higher()`. No value method — a game defines value when it needs one.
  - `deck.rs`: decks with unique `CardId`s: 52 cards, and `build(suits, copies)` for Spider.
  - `rng.rs`: SplitMix64, unbiased `below(n)` (rejection sampling), Fisher–Yates shuffle.
  - `pile.rs`, `moves.rs`: `PileRef`, `TableCard`, `PileSnapshot`, `Move` with `source()`/`target()`,
    `MoveError` (`thiserror`) with the rejection reasons the four games need.
  - `rules.rs`: shared predicates — descending alternating colors, descending same suit, descending
    by rank, building a foundation up from the ace.
  - Stable serde names on serialized types.
- **Acceptance:** unit tests per module; a golden-shuffle test (fixed seed → hard-coded order of 52
  cards); `oxysol-core` depends only on `serde` and `thiserror`.

### P1-02 — Game contract and session

- **Owner:** Sonnet
- **Depends on:** P1-01
- **Files:** `crates/core/src/{game.rs,session.rs}`, `crates/core/src/games/**` (stubs), `crates/core/tests/playout.rs`
- **Scope:**
  - `game.rs`: trait `Game`, `Variant` (with `Variant::ALL`), `DrawCount`, `SuitCount`, `deal(variant, seed)`.
  - Stubs for the four games with final type names and `deal` constructors, compiling and lint-clean,
    so phase 2 fills them in without touching `game.rs`.
  - `session.rs`: `Session`, `SessionRecord`, `RestoreError`; undo, redo, `restart` and `restore` by
    replaying the log from the seed.
  - `tests/playout.rs`: one shared test over every `Variant` and seeds 0..50 — a random playout
    drawn from `legal_moves()` checking:
    1. every move from `legal_moves()` is accepted by `apply`;
    2. the set of `CardId`s across all piles never changes;
    3. undoing to the start reproduces the dealt `piles()`;
    4. `Session::restore(record())` reproduces `piles()`;
    5. every k-th step: a sample of moves outside `legal_moves()` is rejected.
- **Acceptance:** session tests on a `#[cfg(test)]` toy game cover undo/redo, redo truncation after a
  new move, `restart`, and `restore` failing on a bad log; the playout passes on the stubs;
  `architecture.md` matches the code.

### F-01 — Checkpoint: engine contract

- **Owner:** Fable
- **Depends on:** P1-02
- **Scope:** read `game.rs`, `session.rs`, `moves.rs`, `pile.rs` and the playout test — not the whole
  diff. Confirm the contract can express all four games and the shared TUI interaction, or change it
  now. Update `architecture.md`, and adjust phase 2 cards if the contract moved.
- **Acceptance:** a short verdict (go, or the list of required changes) recorded in the PR; phase 2
  starts only after "go".

### P1-03 — Comment lint and phase 1 gate

- **Owner:** Haiku
- **Depends on:** P1-02, M-01…M-06
- **Files:** `xtask/src/lint/comments.rs`, fixes in files of M-01…M-06
- **Scope:**
  - `comments`: tokenize with `ra_ap_rustc_lexer`; take function-body ranges from `syn`
    (`proc-macro2` with `span-locations`). Error for a `//` or `/* */` comment inside a function or
    method body unless it starts with `// SAFETY:` or is a doc comment. Error anywhere for a
    decorative separator (a comment made only of repeated punctuation).
  - Gate: run `cargo xtask check` on the real engine code and remove false positives. Prove each
    gate fires: temporarily add a forbidden import to the engine, a comment inside a function, an
    `unwrap()`, an empty `Err` arm and a 501-line file; confirm the failure; revert.
  - Update the Status column in `README.md` and `micro.md` for phase 0 and 1.
- **Acceptance:** tests for `comments` including the traps — `//` inside a string and a raw string,
  a doc comment on an item nested in a function, `// SAFETY:` before `unsafe`; a report listing each
  injected violation and what caught it.
