# Phase 5 — Card graphics

Goal: card fronts and backs drawn as vector art, delivered to the terminal as a font, with a
Unicode and an ASCII rendering for everyone who does not install it. The delivery method is
confirmed by spike X-01 before any production work.

### X-01 — Spike: card font in real terminals

- **Owner:** Sonnet (prototype) + User (testing)
- **Depends on:** P0-01
- **Files:** `docs/tech/spikes/x-01-card-font.md`; the prototype stays on its branch and is not merged
- **Scope:**
  - Build a throwaway font from six hand-written SVGs: four suits, one back tile, and a 2 × 2 tile
    set that should join into one picture. Glyphs go to Private Use Area code points; the advance
    width equals one terminal cell.
  - A small program prints the test card: each glyph alone, a row of glyphs, the 2 × 2 mosaic,
    glyphs next to box-drawing characters and next to wide text.
  - The user installs the font and runs the program in Windows Terminal, the classic console (CMD
    and PowerShell), macOS Terminal, one Linux terminal, PuTTY, and over SSH; screenshots go in the report.
- **Questions the report must answer:**
  1. Where does the terminal pick up the font by itself, and where must the player configure it?
  2. Are glyphs aligned to the cell, correctly scaled, one cell wide?
  3. Do multi-cell tiles join without gaps — are full-card illustrations feasible?
  4. Which code point range avoids collisions with common icon fonts?
  5. Is a patched complete monospace font needed for terminals without font fallback?
  6. Which build tool: a font library called from a script, or pure Rust in `xtask`?
- **Acceptance:** a results table per terminal with screenshots, and a recommendation for each question.

### F-02 — Checkpoint: graphics delivery

- **Owner:** Fable
- **Depends on:** X-01
- **Scope:** decide D-06 with the user from the spike report: the delivery method, the code point
  range, the glyph box and metrics, single-cell versus mosaic scope, the build tool. Record it in
  `decisions.md` (T-16 becomes final), write the artwork style guide into micro cards M-19…M-24 and
  mark them `READY`, adjust P5-01 and P5-02.
- **Acceptance:** decision recorded; cards updated; no production task of this phase starts before it.

### P5-01 — Font builder and preview sheet

- **Owner:** Haiku
- **Depends on:** F-02
- **Files:** `xtask/src/fonts.rs`, `assets/cards/codepoints.toml`, `assets/fonts/**`
- **Scope:**
  - `cargo xtask fonts`: read every `assets/cards/{fronts,backs}/<style>/glyphs/*.svg`, validate
    each against the glyph box from F-02, assign code points, build the font files into `target/fonts/`.
  - `codepoints.toml` is the registry: an assigned code point never changes; new glyphs are
    appended. The TUI reads the same file.
  - `target/fonts/preview.html`: a contact sheet of every glyph with its name, style and code
    point, for review by eye.
- **Acceptance:** a test builds a font from two fixture SVGs and reads it back (glyph count, code
  points, advance width); an invalid SVG fails with a message naming the file and the rule; running
  the command twice yields identical registry contents.

### P5-02 — Style-driven cards and graphics check

- **Owner:** Haiku
- **Depends on:** P5-01, P3-05
- **Files:** `crates/tui/build.rs`, `crates/tui/src/{card_style.rs,widgets/card.rs,screens/graphics_check.rs}`, `assets/cards/**/style.toml`, `locales/*/cards.toml`
- **Scope:**
  - `style.toml` for a front: per suit a name key, the font glyph, the Unicode symbol, the ASCII
    letter and a color token; labels for ace and court ranks. For a back: the tile glyphs, the
    Unicode and ASCII patterns and color tokens.
  - A build script embeds every style folder; the settings screen lists what was found.
  - The card widget draws any front and any back in the three glyph modes at every size step.
  - Write the manifests for fronts `international` (the default), `french`, `german`, `italian` and
    the two backs. A skin may reuse glyphs of another style: `french` uses the International suits
    with its own court labels.
  - Graphics-check screen: shows sample glyphs in Font mode and asks whether they look right; "yes"
    switches glyph mode to Font, "no" keeps Unicode and points to the installation page of the wiki.
- **Acceptance:** snapshot tests front × back × glyph mode × size; the four suits differ by shape in
  every mode; a rank is readable at the smallest size; adding a style folder needs no code change
  (a test style folder proves it); no color outside the tokens.

### U-02 — Artwork review

- **Owner:** User
- **Depends on:** P5-02, M-19…M-24
- **Scope:** open `target/fonts/preview.html`, then play one game in Font mode with each front and
  back. Accept each style or list the glyphs to redraw.
- **Acceptance:** every MVP style accepted, or a redraw list attached to the micro cards.
