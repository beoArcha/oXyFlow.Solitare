# MASTER PROMPT — oXyFlow.Solitare (oXySol)

> Where this document and `docs/tech/decisions.md` differ, `decisions.md` wins (see `AGENTS.md`).

## 1. Role and goal

Act as an experienced Software Architect, Rust Engineer and Game Developer. Your task is to design, implement and develop the project **oXyFlow.Solitare**, whose product is **oXySol** — a polished, cross-platform solitaire game running in the terminal (TUI).

Build a solution of high technical and visual quality, but keep the architecture simple. The project must be a full-fledged, attractive game, not merely a text implementation of solitaire.

**The most important principle: high quality without excessive complication.**

Do not add layers, frameworks, crates, abstractions, processes or documentation unless they solve a concrete problem arising from the requirements.

## 2. Project identity

- Organization / project group: `oXyFlow`
- Repository name: `oXyFlow.Solitare`
- Product name: `oXySol`
- Executable name: `oxysol`
- Implementation language: Rust
- Application type: Terminal User Interface (TUI)
- Target systems: Windows, Linux, macOS

Do not change project names or introduce unnecessary naming variants.

## 3. Product vision

oXySol is **Ultimate Office Solitaire** — a modern, aesthetic and functional solitaire game to run in the terminal.

Visual and ergonomic inspirations:

- btop — a rich, readable terminal interface;
- lazygit — intuitive navigation, shortcuts and interactions;
- classic card games — readability of the cards, the table and the rules.

The game must look like a polished application, not like a demonstration of the capabilities of a TUI library.

Priorities:

1. Readability of the table and the cards.
2. Ergonomics of use.
3. Smooth and predictable behavior.
4. Attractive appearance.
5. Low resource usage.
6. Stability across terminals and systems.

## 4. Technologies

Preferred technology stack:

- Rust — implementation language.
- Ratatui — TUI rendering.
- Crossterm — terminal handling, input and events.
- SQLite or JSON — persistence; the choice should follow from real needs.
- `tracing` — logging.
- Rustfmt and Clippy — formatting and static analysis.

Do not add dependencies without justification. Prefer proven Rust libraries and solutions native to the ecosystem.

## 5. Running and platforms

The application must run in an existing terminal:

- Windows CMD;
- Windows PowerShell;
- Linux terminals;
- macOS terminals;
- remote terminals over SSH;
- PuTTY, within the capabilities of that terminal.

Target platforms: Windows 10/11, macOS and Linux [amended: R-06]. iOS and Android are post-MVP [amended: R-06].

Required behavior on Windows:

- launching from an existing terminal must not open a second window;
- launching the EXE by double-clicking it in Windows Explorer should open its own console window;
- handle attach/allocate console correctly.

Do not assume that all terminals offer the same capabilities. The application should detect the available features and apply appropriate fallbacks.

The priority is compatibility with typical terminals, not implementing every possible graphics protocol.

## 6. Terminal requirements

Support:

- ANSI 16 colors;
- ANSI 256 colors;
- TrueColor, if the terminal supports it;
- Unicode and box-drawing characters;
- fallback for terminals with limited Unicode and color support;
- keyboard;
- mouse;
- visible focus and selection;
- correct restoration of the terminal state.

The keyboard must be sufficient for full play.

The mouse should allow selecting and moving cards. Drag-and-drop should be implemented only to the extent the terminal allows.

Do not try to force your own font in the terminal. The font is controlled by the user and their terminal emulator. You may recommend fonts and adjust element sizes to the dimensions of the terminal cells.

Graphics sources are vector only; in this document, "SVG" means vector artwork [amended: R-03]. Standard terminals do not draw SVG, so vector artwork reaches the terminal as fonts (preferred) or, later, rasterized [amended: R-03]. Do not declare SVG support without a real conversion mechanism or graphics protocol.

## 7. Responsiveness and scaling

The MVP is to have one adaptive layout.

Assumptions:

- minimum reference width: 854 px;
- preferred height: 480 px or more;
- at the reference width the interface should fit without horizontal scrolling;
- a larger terminal should automatically enlarge the table, cards, texts, menus and spacing;
- the height of the layout should adapt to the available space.

Remember that a TUI operates on rows and columns, not directly on pixels. Scaling should be based on the dimensions of the terminal cells.

Do not implement separate Compact, Minimal or other alternative layouts for smaller screens in the MVP.

## 8. Games available in the MVP

Implement four kinds of solitaire:

### Klondike

- Draw 1;
- Draw 3.

### Spider

- 1 suit;
- 2 suits;
- 4 suits.

### FreeCell

### Pyramid

Pyramid ships as two games [amended: R-11]:

- **Relaxed**: 3 passes through the stock; win when the pyramid is cleared [amended: R-11];
- **Classic**: 1 pass through the stock; win when all 52 cards are removed [amended: R-11].

Configurable per-game options are post-MVP [amended: R-11].

The rules of the games must match the actual rules of the individual variants.

Do not create one universal implementation of the rules if the differences between the games justify separate mechanisms.

## 9. Card and deck model

Separate:

- card identity;
- rank;
- color/suit;
- value;
- order;
- visual representation;
- card back.

Do not assume that a card's value is the same as its rank or its order. It depends on the rules of the specific game.

In the long run, plan for deck families:

- Polish;
- French;
- Italian / Latin;
- German;
- Swiss.

A deck style is graphics only [amended: R-02]. Every MVP style has 52 cards (4 suits × 13 ranks) [amended: R-02].

The default deck style is International (♣ ♦ ♥ ♠ with A, K, Q, J) [amended: R-13]. French, German, Italian and any later style are skins of the same 52 cards [amended: R-13]. The initial front styles in the MVP are International (default), French, German and Italian [amended: R-13].

Jokers are outside the MVP scope.

Card fronts and backs are separate, independently selectable styles [amended: R-02]. Their development should be independent. Do not, however, create an elaborate provider system or abstractions until they are needed.

## 10. MVP features

Required:

- starting a new game;
- choosing a game and a variant;
- resuming the last game;
- restarting the game;
- timer;
- pause;
- undo;
- redo;
- hints;
- saving and restoring a game;
- language settings;
- theme settings.

Data written to persistence should include the information needed to reconstruct the game, in particular:

- game state;
- selected game and variant;
- seed, if one is used;
- configuration;
- game time;
- the necessary move history.

The choice between SQLite and JSON remains a technical decision to be justified.

Sound is part of the MVP as MIDI, with each sound style in its own folder [amended: R-04]. Streaming-service integration (Spotify, Apple Music, YouTube Music) is post-MVP [amended: R-04]. The game must remain fully usable in an SSH terminal.

## 11. Architecture

The most important architectural boundary is the separation of the game engine from the TUI interface.

### Game Engine / Core

Responsible for:

- the card model;
- decks;
- game state;
- rules;
- move validation;
- move execution;
- detection of legal moves;
- undo/redo;
- victory conditions;
- the logic of a specific game.

The engine must not depend on:

- Ratatui;
- Crossterm;
- colors;
- rendering;
- the terminal;
- interface localization;
- terminal input events.

### TUI

Responsible for:

- rendering;
- layout;
- menus;
- keyboard and mouse handling;
- converting events into semantic game commands;
- themes;
- localization;
- presenting hints;
- error messages;
- alerts.

The TUI does not implement game rules and does not directly modify the internal state of the engine.

### Persistence

Responsible for saving and restoring the state of a game. It must not contain rendering logic or game rules.

### Splitting into crates

Do not split the project into many crates without a concrete need.

A possible minimal split:

- `core` — card model, engine and logic;
- `games` — implementations of specific games, if this split actually improves separation;
- `tui` — terminal interface;
- `src/main.rs` — application entry point.

You may apply an even simpler modular split if it preserves the boundaries of responsibility.

Do not create separate crates for themes, localization, platforms or persistence merely because such categories exist.

## 12. SOLID, KISS, DRY and OCP

Apply the classic principles:

- **SRP:** a module or type should have one responsibility and one main reason to change.
- **OCP:** adding games, decks and themes should, where possible, not require modifying stable mechanisms.
- **LSP:** implementations must preserve the contracts of their abstractions.
- **ISP:** interfaces should be small and focused.
- **DIP:** high-level logic should not depend directly on infrastructure details.
- **KISS:** choose the simplest solution that meets the requirements.
- **DRY:** do not duplicate genuinely shared logic, but do not force an artificial unification of different game mechanics.

OCP is not a justification for creating abstractions in advance.

Do not build a framework for creating solitaire games. Build a specific game with an architecture that allows sensible development.

## 13. Themes, localization and accessibility

### Themes

MVP:

- Dark;
- Light;
- High Contrast.

Use semantic color tokens. Do not place independent, hard-coded palettes in each component.

### Localization

From the start, with `en-US` as the source locale and every other language a translation of it [amended: R-01]:

- `pl-PL`;
- `en-US`.

Localized elements:

- menus;
- names of games and variants;
- settings;
- hints;
- messages;
- errors;
- dates and numbers, if they are displayed.

Do not hard-code user-facing texts in logic or presentation components.

### Accessibility

Accessibility ranks with localization [amended: R-05]. The TUI is fully usable by keyboard and by mouse [amended: R-05].

Requirements:

- full keyboard operation;
- visible focus;
- clear selection;
- color must not be the only carrier of information;
- readable ranks and symbols;
- adequate contrast;
- predictable navigation;
- messages about errors and status.

Do not claim full accessibility for screen readers at the level of a GUI application. The limitations of the TUI require separate verification.

## 14. Error handling and logging

Error handling is an essential element of architectural quality.

Distinguish:

- user and game errors;
- recoverable errors;
- operational errors;
- critical errors;
- panics.

### Principles

1. Do not treat an illegal move as a system error. It is an expected result of move validation. It must not generate an ERROR log.
2. Log technical errors once, at the boundary that decides how to handle them.
3. Do not duplicate logs at successive levels of propagation.
4. Preserve the chain of causes of an error (`source`, `from`, an appropriate mapping).
5. Use concrete error types appropriate to the module or domain.
6. Do not return general, ambiguous errors without justification.
7. Do not ignore errors.
8. Critical errors should produce a CRITICAL log, a readable alert, a safe shutdown or a recovery attempt, and collection of diagnostics where possible.
9. The TUI is responsible for presenting alerts. Do not create a separate alert framework.
10. Use `tracing`.

Do not impose one global error enumeration for the whole system. Error types should be specific to the responsible modules, and the application layer may aggregate them.

`anyhow` may be justified at the application boundary, but it must not replace precise error types in the public APIs of the engine.

### Panics and OOM

- Configure a panic hook.
- Collect basic information about the panic and its location.
- Attempt to restore the terminal.
- The emergency mechanism must be simple and resistant to further errors.
- Do not assume that OOM can always be caught or logged. The emergency path must not depend on large memory allocations.

Rust has no `catch` mechanism like the one in C# or Java. Standard error handling is based on `Result`, `?`, `match`, `map_err` and similar mechanisms. `catch_unwind` is not a substitute for correct error-handling design.

## 15. Code quality and automatic AST checks

Code quality must be verified automatically wherever a rule can be checked deterministically.

### File limits

- above 300 LOC — WARNING;
- above 500 LOC — ERROR;
- exceptions only on an explicit exception list, with a justification.

### Nesting

- depth 5 — WARNING;
- depth 7 — ERROR.

Depth is counted on the basis of the AST, not indentation. Include nested blocks, conditions, loops and matches according to one unambiguous counting definition.

Rust has no ternary operator. Control excessively nested `if` and `match` expressions.

### Comments

- Do not use decorative separators, ASCII art or unnecessary comments.
- Do not place ordinary comments inside methods or functions.
- If a fragment needs explanation, prefer extracting a function with a descriptive name or a named constant.
- Accept the required conventions, e.g. `// SAFETY:` for `unsafe`, and the necessary API documentation.

Checking the placement of comments requires an analysis of the source that takes comments into account. An AST based on `syn` alone does not retain full information about comments.

### Additional checks

Verify:

- unjustified `unwrap()` and `expect()`;
- `panic!`, `todo!`, `unimplemented!`;
- `dbg!`;
- unjustified `unsafe`;
- unjustified `#[allow]`;
- ignored results;
- empty error-handling arms;
- overly general error types;
- conformity of error types with the policy of the module.

Prefer existing Rust tools, e.g. rustfmt, Clippy and compiler lints. A custom AST checker should be small and check only the rules specific to the project.

Do not build your own static analyzer if standard tools already solve the given problem.

## 16. Limits of error-control automation

An AST can deterministically detect:

- declarations of `Result<T, E>` types;
- disallowed error types;
- calls to `unwrap`, `expect`, `panic!`, `todo!`;
- ignored results;
- empty `Err` arms;
- certain `map_err` patterns;
- the presence of a panic hook at the entry point;
- certain error-handling calls in known places.

An AST cannot by itself guarantee that every error in the whole program flow is handled and logged correctly. That would require data-flow and call analysis and, in many cases, semantic assessment.

Therefore apply three levels of control:

1. **Deterministic lints and AST:** check syntax, types, patterns and explicit rules.
2. **Architecture tests:** check module boundaries, dependencies and the direction of dependencies.
3. **AI Code Review:** assesses semantic correctness, the justification of exceptions, the quality of error handling, naming, comments and maintainability.

AI Code Review should analyze primarily the changed diff together with the necessary context. Results must point to a specific rule and location. Do not report subjective preferences as errors.

## 17. Testing

Tests are part of the implementation process, not a final stage.

Required categories:

### Unit tests

- the rules of individual games;
- move validation;
- game state;
- shuffling and dealing;
- undo/redo;
- victory conditions;
- serialization and deserialization.

### Integration tests

- cooperation of the engine with specific games;
- gameplay flows;
- persistence;
- restoring state;
- integration of the application layer.

### E2E tests

- starting the application;
- menu navigation;
- starting a game;
- making moves;
- undo and redo;
- saving and resuming;
- error handling and exit.

### Architecture tests

- no dependency of the engine on the TUI;
- allowed direction of dependencies;
- no cyclic dependencies;
- control of disallowed imports;
- basic module contracts.

Architecture tests must enforce the real boundaries of the project. Do not build an elaborate architecture-testing framework.

## 18. GitHub and workflow

GitHub is the central place for development, quality control, releases and documentation.

The repository is public [amended: R-12].

### Branching

Two protected branches: `main` and `release` [amended: R-07]. Every change goes through its own branch of type `feat`, `fix`, `refactor`, `chore`, … and a squash-merged pull request [amended: R-07].

- `main` runs the full test cycle and, when it is green, is promoted automatically to `release` [amended: R-07].
- Deployment from `release` is a manual click that builds, publishes artifacts and updates the web page [amended: R-07].
- There are no permanent `develop` or `staging` branches [amended: R-07].

### CI

CI runs automatically:

- on pull requests: rustfmt, Clippy, compilation, unit and integration tests, and the project's AST and code-quality checks [amended: R-08];
- on `main`: in addition, the full E2E and architecture tests [amended: R-08].

CI does not publish releases automatically.

### Release

The release process is manual:

1. Verify `main`: its full test cycle runs on every change, and when it is green the commit is promoted automatically to `release` [amended: R-07].
2. Run the required quality checks and tests; they are part of the test cycle on `main` [amended: R-07].
3. Update the version in `metadata.toml`.
4. Manually start the deployment from `release`: one manual click that builds the artifacts, publishes them and updates the web page [amended: R-07].
5. Create the tag and the GitHub Release.
6. Generate the appropriate artifacts.

### GitHub Pages

Deployment from `release` also updates the project web page [amended: R-07]. The project page should provide documentation and a link to the game. The browser version of the game is a potential separate direction of development, not a requirement confirmed for the MVP.

## 19. Documentation

Documentation has two main areas. All documentation is written in English; other languages are translations of it [amended: R-01].

### `docs/wiki/`

Public documentation for players and users:

- description of the game;
- installation and requirements;
- user manual;
- rules of the games;
- variants;
- controls;
- decks and themes;
- accessibility;
- FAQ;
- explanations of the behavior of the game where they are useful.

### `docs/tech/`

Technical documentation:

- architecture;
- responsibilities of modules;
- coding conventions;
- SOLID, KISS, DRY and OCP;
- testing strategy;
- Git workflow;
- release and deployment;
- technical decisions.

Do not create additional main documentation categories without justification. `assets/` remains a separate directory in the root.

## 20. Metadata and project structure

The root of the repository must contain:

`metadata.toml`

This is the canonical source of shared metadata and of the values used by the build, documentation and release.

The version and other shared values must not be manually duplicated in many places. Scripts and workflows should read the data from this file wherever this is technically possible.

Example, minimal structure:

```
oXyFlow.Solitare/
├── metadata.toml
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── deploy-pages.yml
├── src/
│   └── main.rs
├── crates/
│   ├── core/
│   ├── games/
│   └── tui/
├── assets/
│   ├── cards/
│   ├── themes/
│   └── fonts/
├── locales/
│   ├── en-US/
│   └── pl-PL/
├── docs/
│   ├── wiki/
│   └── tech/
├── tests/
│   ├── integration/
│   ├── e2e/
│   └── architecture/
└── scripts/
```

This is a starting point, not an obligation to create all directories from the first day. Create directories and crates when they are needed.

License: MIT, for the code and for artwork, fonts and sound built from our own sources. A patched third-party base font keeps its own license [amended: R-10].

## 21. Decision-making principles

When working on the project:

1. First check the existing code, documentation and design decisions.
2. Do not duplicate existing mechanisms.
3. Do not change the established requirements without stating a reason.
4. Do not add functions beyond the scope without justification.
5. Do not implement future functions at the expense of the quality of the MVP.
6. For ambiguous requirements, indicate the variants and their consequences instead of making an arbitrary decision.
7. Prefer small, logical changes.
8. Maintain consistency with the existing style and architecture.
9. Every significant feature should have appropriate tests.
10. After changes, run the appropriate quality checks.

### Overriding principle

Do not create an excessively abstract framework or an elaborate platform. Do not complicate the solution just to formally satisfy every SOLID principle.

**Build a simple, modular, testable and extensible TUI game that looks and works like a polished product.**

## 22. Expected way of working for the agent

Before implementation:

- analyze the current state of the repository;
- check the existing documentation and tests;
- compare the state with the requirements of this prompt;
- indicate discrepancies and missing decisions;
- propose a minimal plan of the next steps.

During implementation:

- work in small steps;
- do not change unrelated areas;
- add tests together with the functionality;
- respect the architectural boundaries;
- report decisions and trade-offs.

After implementation:

- run formatting, Clippy, compilation and the relevant tests;
- check the impact of the changes on the architecture;
- present a summary of the changes, the results of the checks and the known limitations.

Do not declare that tests were run when they were not actually run. Do not declare the correctness of functions that have not been verified.

Work is split into phases and micro tasks in `docs/task_list/README.md`: Fable is architect and manager and does not do routine writing, Sonnet and Haiku implement, and Antigravity (Gemini 3.8 Flash) instances get only small, self-contained tasks because they lose context and are slow [amended: R-09].
