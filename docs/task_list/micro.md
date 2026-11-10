# Micro tasks — Antigravity queue

Small, self-contained cards for Beo, Oxy and Son; Dad may take any `READY` card nobody has started.
Read `AGENTS.md` and your card only. A `DRAFT` card is not workable yet: the task named under
"Finalized by" fills in the exact interface and marks it `READY`. "Needs" must be merged first.

| ID | Card | Lane | Needs | Status |
|---|---|---|---|---|
| M-01 | Lint rule: file length | Beo | P0-01 | READY |
| M-02 | Lint rule: nesting depth | Oxy | P0-01 | READY |
| M-03 | Lint rule: empty `Err` arms | Son | P0-01 | READY |
| M-04 | Lint rule: vague error types | Beo | P0-01 | READY |
| M-05 | Metadata sync | Oxy | P0-01 | READY |
| M-06 | Architecture tests | Son | P0-01 | READY |
| M-07 | Wiki: Klondike rules | Beo | P0-01 | READY |
| M-08 | Wiki: Spider rules | Oxy | P0-01 | READY |
| M-09 | Wiki: FreeCell rules | Son | P0-01 | READY |
| M-10 | Wiki: Pyramid rules | Beo | P0-01 | READY |
| M-11 | Theme: Light palette | Beo | P3-01 | DRAFT |
| M-12 | Theme: High Contrast palette | Oxy | P3-01 | DRAFT |
| M-13 | RGB to 256-color conversion | Oxy | P0-01 | READY |
| M-14 | Contrast ratio helper | Son | P0-01 | READY |
| M-15 | Terminal capability detection | Beo | P0-01 | READY |
| M-16 | Title logo | Oxy | P0-01 | READY |
| M-17 | Polish translation, part 1 | Son | P3-03 | DRAFT |
| M-18 | Command-line parser | Son | P0-01 | READY |
| M-19 | Artwork: International suits | Beo | P5-01 | DRAFT |
| M-20 | Artwork: German suits | Oxy | P5-01 | DRAFT |
| M-21 | Artwork: Italian suits | Son | P5-01 | DRAFT |
| M-22 | Artwork: court emblems | Beo | P5-01 | DRAFT |
| M-23 | Artwork: back style 1 | Oxy | P5-01 | DRAFT |
| M-24 | Artwork: back style 2 | Son | P5-01 | DRAFT |
| M-25 | Sound style 1: cues | Beo | P6-03 | DRAFT |
| M-26 | Sound style 2: cues | Oxy | P6-03 | DRAFT |
| M-27 | Music loops | Son | P6-03 | DRAFT |
| M-28 | E2E: Spider and FreeCell | Beo | P7-01 | DRAFT |
| M-29 | E2E: Pyramid and settings | Oxy | P7-01 | DRAFT |
| M-30 | Wiki: installation and requirements | Son | P3-09 | DRAFT |
| M-31 | Wiki: controls | Beo | P3-09 | DRAFT |
| M-32 | Wiki: decks, themes, sound | Oxy | P5-02, P6-02 | DRAFT |
| M-33 | Wiki: accessibility and FAQ | Son | P3-09 | DRAFT |
| M-34 | Polish translation, part 2 | Beo | P5-02, P6-02 | DRAFT |

## Tooling

### M-01 — Lint rule: file length

- **Branch:** `feat/M-01-lint-loc` · **File:** `xtask/src/lint/loc.rs`
- **Do:** implement the existing `pub fn check(file: &SourceFile) -> Vec<Finding>`. Count lines that
  are neither blank nor comment-only (a line starting with `//` after trimming, or lying wholly
  inside a `/* … */` block). More than 300 → one `Warning`; more than 500 → one `Error` instead.
  Rule name `"loc"`, line 1, message with the count and the limit.
- **Done when:** unit tests in the same file pass for 300 lines (nothing), 301 (warning), 500
  (warning), 501 (error), and show that blank and comment-only lines are not counted.

### M-02 — Lint rule: nesting depth

- **Branch:** `feat/M-02-lint-nesting` · **File:** `xtask/src/lint/nesting.rs`
- **Do:** implement `check` with a `syn` visitor. For every function and method compute the deepest
  nesting of `if`, `match`, `for`, `while`, `loop`, plain/`unsafe`/`async` blocks and closures with
  a block body. The function body is level 0. `else if` stays at the level of its `if`. A `match`
  arm adds no level. A nested function is measured on its own. Depth 5 or 6 → `Warning`; 7 or more
  → `Error`. Rule name `"nesting"`, the line of the deepest point, message with the function name.
- **Done when:** tests pass for: `match` in `for` in `if` = 3; an `else if` chain = 1; a
  block-bodied closure adds one; a nested function does not add to its parent; depth 4 (nothing),
  5 (warning), 7 (error).

### M-03 — Lint rule: empty `Err` arms

- **Branch:** `feat/M-03-lint-err-arms` · **File:** `xtask/src/lint/err_arms.rs`
- **Do:** implement `check` with `syn`. `Error` for a `match` arm whose pattern is `Err(..)` (any
  inner pattern) and whose body is `{}` or `()`, and for `if let Err(..) = … {}` with an empty
  block. Rule name `"err-arms"`.
- **Done when:** tests pass for both forms, and show no finding for `Err(e) => return Err(e.into())`,
  `Err(e) => log(e)`, and `Ok(_) => {}`.

### M-04 — Lint rule: vague error types

- **Branch:** `feat/M-04-lint-err-types` · **File:** `xtask/src/lint/err_types.rs`
- **Do:** implement `check` with `syn`. `Error` for every `pub fn` (free function or method) whose
  return type is `Result<_, String>`, `Result<_, &str>`, `Result<_, ()>`, `Result<_, Box<dyn …>>`,
  or any type whose path starts with `anyhow`. Rule name `"err-types"`.
- **Done when:** tests pass for each forbidden form, and show no finding for a private function
  with `Result<_, String>` or a public one with `Result<_, MoveError>`.

### M-05 — Metadata sync

- **Branch:** `feat/M-05-sync-metadata` · **File:** `xtask/src/sync_metadata.rs`
- **Do:** add a pure function that takes the text of `Cargo.toml` and of `metadata.toml` and
  returns the new `Cargo.toml` text with `[workspace.package]` `version`, `description`, `license`
  and `repository` set from `[project]` `version`, `description`, `license`, `repository_url`, using
  `toml_edit` so everything else is preserved byte for byte. Keep the signature of the existing
  entry function: without `--check` it rewrites the file; with `--check` it changes nothing and
  fails when the result would differ.
- **Done when:** tests on strings pass: values are copied, comments and ordering survive, already
  synced input is returned unchanged, a missing `[project]` key is an error.

### M-06 — Architecture tests

- **Branch:** `test/M-06-architecture` · **File:** `tests/architecture/main.rs`
- **Do:** four tests reading files relative to `env!("CARGO_MANIFEST_DIR")`:
  1. `[dependencies]` of `crates/core/Cargo.toml` contains nothing but `serde` and `thiserror`;
  2. `[dependencies]` of `crates/tui/Cargo.toml` contains none of `oxysol`, `serde_json`,
     `directories`, `cpal`, `midly`;
  3. no `.rs` file under `crates/core/src` contains `std::fs`, `std::io`, `std::env`, `std::time`,
     `std::net`, `std::process`, `std::thread`, `println!` or `eprintln!`;
  4. `crates/core/src/lib.rs` does not contain `pub mod games`.
- **Done when:** `cargo test --test architecture` passes (this target is not part of `--quick`,
  run it explicitly), and each test fails when you temporarily break its rule — say so in the report.

### M-18 — Command-line parser

- **Branch:** `feat/M-18-cli-parser` · **File:** `src/cli.rs`
- **Do:** no dependencies, no printing, no text for users:

  ```rust
  pub enum ColorArg { Ansi16, Ansi256, TrueColor }
  pub struct Cli { pub show_version: bool, pub show_help: bool, pub ascii: bool, pub colors: Option<ColorArg> }
  pub enum CliError { UnknownArgument(String), MissingValue(&'static str), InvalidValue { flag: &'static str, value: String } }
  pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Cli, CliError>;
  ```

  Flags: `--version`, `--help`, `--ascii`, `--colors <16|256|truecolor>`. `CliError` derives
  `thiserror::Error`. `args` excludes the program name.
- **Done when:** tests pass for no arguments, each flag, all three color values, `--colors` without
  a value, `--colors 8`, and an unknown argument.

## Wiki: rules

### M-07, M-08, M-09, M-10 — Rules of Klondike, Spider, FreeCell, Pyramid

- **Branch:** `docs/M-0X-wiki-<game>` · **File:** `docs/wiki/rules/<game>.md` (one game per card)
- **Read also:** the card of that game in `docs/task_list/phase-2.md` — it is the only source.
- **Do:** explain the game to a player in plain English under the headings Goal, Layout, Moves,
  Stock, Winning, Variants. Use card-table words, never code names such as `Move::Stock`. Do not add
  a rule that is not in the source card and do not omit one.
- **Done when:** every rule of the source card appears once; nothing else is claimed.

## Terminal and theme helpers

### M-13 — RGB to 256-color conversion

- **Branch:** `feat/M-13-ansi256` · **File:** `crates/tui/src/theme/ansi.rs`
- **Do:** `pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8` returning the nearest color by squared
  distance among the 6 × 6 × 6 cube (indexes 16–231, channel levels 0, 95, 135, 175, 215, 255) and
  the gray ramp (indexes 232–255, value 8 + 10·i). Never return 0–15.
- **Done when:** tests pass: (0,0,0) → 16; (255,255,255) → 231; (255,0,0) → 196; (128,128,128) → 244;
  (95,135,175) → 67.

### M-14 — Contrast ratio helper

- **Branch:** `feat/M-14-contrast` · **File:** `crates/tui/src/theme/contrast.rs`
- **Do:** `pub fn contrast_ratio(a: [u8; 3], b: [u8; 3]) -> f32` using the WCAG 2 relative
  luminance formula for sRGB.
- **Done when:** tests pass: black on white = 21.0 (± 0.01); identical colors = 1.0; the result is
  symmetric; (119,119,119) on white is 4.48 (± 0.01).

### M-15 — Terminal capability detection

- **Branch:** `feat/M-15-caps` · **File:** `crates/tui/src/caps.rs`
- **Do:**

  ```rust
  pub enum ColorDepth { Ansi16, Ansi256, TrueColor }
  pub enum Platform { Windows, MacOs, Linux }
  pub struct TerminalCaps { pub color: ColorDepth, pub unicode: bool, pub remote: bool }
  pub fn detect(env: &dyn Fn(&str) -> Option<String>, platform: Platform) -> TerminalCaps;
  ```

  Color: `COLORTERM` equal to `truecolor` or `24bit` → `TrueColor`; otherwise Windows → `TrueColor`;
  otherwise `TERM` containing `256color` → `Ansi256`; otherwise `Ansi16`. Unicode: Windows → true;
  otherwise true when the first non-empty of `LC_ALL`, `LC_CTYPE`, `LANG` contains `utf-8` or `utf8`
  in any case. Remote: `SSH_CONNECTION` or `SSH_TTY` is set. The function reads nothing but `env`.
- **Done when:** a table test passes for: Windows with no variables; Linux `xterm-256color` with
  `LANG=en_US.UTF-8`; Linux `TERM=xterm` with `LANG=C` (16 colors, no Unicode); macOS
  `xterm-256color` UTF-8; `COLORTERM=truecolor`; `LC_ALL` overriding `LANG`; an SSH session; no
  variables at all on Linux.

### M-16 — Title logo

- **Branch:** `feat/M-16-logo` · **File:** `crates/tui/src/widgets/logo.rs`
- **Do:** `pub fn logo(unicode: bool) -> &'static [&'static str]` returning the word `oXySol` drawn
  in block letters, 5 or 6 rows, at most 60 columns, every row the same width. The Unicode version
  uses only `█`, `▀`, `▄` and spaces; the ASCII version only `#` and spaces.
- **Done when:** tests pass for equal row widths, the width limit and the allowed characters of
  both versions; the six letters are legible when printed.

### M-11, M-12 — Theme palettes: Light, High Contrast

- **Status:** DRAFT · **Finalized by:** P3-01 (token list, constructor signature, pairs to test)
- **Branch:** `feat/M-1X-theme-<name>` · **File:** `crates/tui/src/theme/<name>.rs`
- **Do:** fill every token of the theme for true color, and its 16-color variant using only the
  named ANSI colors. Light: dark text on a light surface. High Contrast: maximum separation, no
  mid-tones for text.
- **Done when:** the test listed by P3-01 passes: `contrast_ratio` of every text/background pair is
  at least 4.5 (Light) or 7.0 (High Contrast); red and black card colors differ in both variants.

## Translation

### M-17, M-34 — Polish translation

- **Status:** DRAFT · **Finalized by:** P3-03 (part 1), P3-09 (part 2)
- **Branch:** `docs/M-XX-pl-translation` · **Files:** part 1 — `locales/pl-PL/{common,menu,board}.toml`;
  part 2 — the remaining `locales/pl-PL/*.toml`
- **Do:** translate every value from the matching `locales/en-US/` file into natural Polish. Keep
  keys, placeholders such as `{name}` and key order unchanged. Avoid wording that needs a plural
  form of a number.
- **Done when:** the locale parity test passes; no value is identical to the English one unless it
  is a proper name.

## Artwork

### M-19 … M-24 — Vector glyphs

- **Status:** DRAFT · **Finalized by:** F-02 (glyph box, stroke and fill rules, naming, style guide)
- **Branch:** `feat/M-XX-art-<style>` · **Files:** `assets/cards/fronts/<style>/glyphs/*.svg` or
  `assets/cards/backs/<style>/glyphs/*.svg`
- **Cards:** M-19 International suits (clubs, diamonds, hearts, spades; the French skin reuses
  them); M-20 German suits (acorns,
  leaves, hearts, bells); M-21 Italian suits (batons, swords, cups, coins); M-22 court emblems
  (jack, queen, king); M-23 and M-24 one back style each (repeatable tile set).
- **Do:** draw each glyph as one hand-written SVG that follows the style guide. Shapes must stay
  recognizable at one terminal cell and differ from each other by silhouette, not by color.
- **Done when:** `cargo xtask fonts` accepts every file and the glyphs appear in
  `target/fonts/preview.html`; attach a screenshot of the sheet to the PR.

## Sound

### M-25, M-26, M-27 — Sound styles and music

- **Status:** DRAFT · **Finalized by:** P6-03 (score notation, cue list, length limits)
- **Branch:** `feat/M-XX-sound-<style>` · **Files:** `assets/audio/<style>/{style.toml,scores/*.txt,*.mid}`
- **Cards:** M-25 and M-26 — one sound style each: a score for every cue (pick, place, flip, deal,
  illegal, undo, hint, win, menu move, menu select) and the timbres in `style.toml`. M-27 — one
  looping music track of at most 60 seconds for each style.
- **Do:** write the scores, compile them with `cargo xtask midi <style>`, commit scores and `.mid`
  files. Cues are short (under one second, the win cue under three) and distinguishable from each
  other by contour, not only by pitch.
- **Done when:** the command compiles without errors and the style loads in the game's settings.

## Tests

### M-28, M-29 — Additional E2E scenarios

- **Status:** DRAFT · **Finalized by:** P7-01 (harness API and one worked example)
- **Branch:** `test/M-XX-e2e-<topic>` · **File:** one new file under `tests/e2e/`
- **Cards:** M-28 — Spider (deal a row, move a run, undo) and FreeCell (cell, foundation, a
  multi-card move, a rejected oversized move). M-29 — Pyramid (a pair, a king, stock passes) and
  settings (language and theme persist across a restart).
- **Do:** write the scenarios with the harness, on fixed seeds, by copying the worked example.
- **Done when:** `cargo test --test e2e` passes.

## Wiki: player pages

### M-30 … M-33 — Player documentation

- **Status:** DRAFT · **Finalized by:** P3-09 (final behavior, sources to read for each page)
- **Branch:** `docs/M-XX-wiki-<page>` · **File:** one page under `docs/wiki/`
- **Cards:** M-30 installation and requirements (terminal size, installing the card font);
  M-31 controls (keyboard and mouse, generated from the key table); M-32 decks, themes and sound;
  M-33 accessibility (what is supported and the limits of a TUI — no claim of screen-reader
  support) and FAQ.
- **Do:** describe only what the named sources show. No feature that is not in the game.
- **Done when:** every statement can be traced to a source named in the card.
