# Mailghost Branded CLI Home and Self-Directed TDD - Implementation Plan

## Overview

Add a polished, colored home screen when `mailghost` is invoked without a
subcommand, while using the feature as a guided TDD project. The learner writes
every test personally; Codex may review a failing test and implement the
smallest production change only after the learner has demonstrated RED.

This plan intentionally specifies observable behavior, test names, commands,
and hints without supplying completed test functions. That pedagogical
constraint takes precedence over providing copy-paste test solutions.

## Acceptance Criteria

1. Running `mailghost` with no arguments exits with status `0`.
2. The no-argument invocation writes a branded home screen to `stdout` and
   writes nothing to `stderr`.
3. The home screen contains:
   - a compact envelope/terminal ASCII mark;
   - `mailghost` and the package version;
   - a one-line product description;
   - usage text;
   - all four commands, their short aliases, and descriptions;
   - `--help` and `--version`.
4. Interactive terminals receive cyan/blue/violet styling inspired by the
   project logo.
5. Redirected output, captured test output, and `NO_COLOR=1` contain no ANSI
   escape sequences.
6. Rendering the home screen does not inspect the config directory, access
   Mail.tm, or require network connectivity.
7. Existing command behavior, aliases, `mailghost --help`, and
   `mailghost --version` remain functional.
8. Tests verify observable behavior through the binary or a deliberately
   exposed rendering interface. They do not inspect private helpers or mock
   internal modules.

## Learning Contract

- The learner writes every test.
- Work in one vertical slice at a time: one RED test, one minimal GREEN
  implementation, then refactor.
- Do not write the complete test suite before implementation.
- Do not accept a test that passes before the intended production change.
- Before each GREEN step, save the failing output; it is evidence that the test
  can detect the missing behavior.
- Expected values come from this specification, not from reproducing the
  implementation inside the test.
- Codex reviews behavior, test boundaries, and failure evidence but does not
  provide a complete test function unless the learner explicitly abandons the
  self-directed constraint.

## Current State

**Language:** Rust 2021, MSRV 1.88  
**Runtime:** Tokio multi-thread runtime  
**CLI:** Clap derive 4.6  
**Testing:** Rust built-in test harness and `tempfile`  
**CI command:** `cargo test --all-targets --locked`

Relevant code:

- `src/main.rs` parses `Cli` and calls `run`.
- `src/cli.rs` requires a `Command` subcommand.
- `src/app.rs::run` creates an `AccountStore` before dispatching a command.
- `src/lib.rs` exposes `Cli` and `run`.
- `src/cli.rs` contains two parser tests.
- `src/account.rs` contains three storage/domain tests.
- There is no `tests/` integration-test directory.

Observed baseline:

```text
$ mailghost
exit status: 2
stdout: empty
stderr: standard Clap help
```

This behavior is generated before `run` executes because Clap currently
requires a subcommand.

## Desired End State

The precise spacing may be adjusted after viewing it in a real terminal, but
the information hierarchy should resemble:

```text
       .------------------.      mailghost 1.0.0
      / \                /       Disposable inboxes from your terminal
     |   \     >_       /
     '------------------'

Usage
  mailghost <COMMAND>

Commands
  generate  g    Create a new disposable inbox
  messages  m    List messages and optionally open one
  delete    d    Forget the locally stored inbox
  account   me   Show the active inbox

Options
  -h, --help       Print help
  -V, --version    Print version
```

The live terminal version uses restrained color:

- cyan/blue for the ASCII mark;
- violet for the product name and version;
- yellow or orange for command names;
- default foreground for descriptions;
- dim text only where contrast remains accessible.

Tests assert semantic anchors rather than the entire drawing or exact spacing.
This allows visual refinement without breaking behavior-focused tests.

## Out of Scope

- A full-screen TUI.
- Interactive menus or arrow-key navigation.
- Changing the behavior of existing subcommands.
- Network integration tests against Mail.tm.
- Snapshotting the complete ANSI output.
- Automatically setting the GitHub repository avatar or social preview.

## Design Options

### Option A: Static `before_help` or custom Clap help template

**Pros**

- Small change.
- Clap continues to own most rendering.

**Cons**

- Static templates are awkward for adaptive colors and dynamic version data.
- The branded output becomes coupled to Clap template syntax.
- Testing the no-argument behavior separately is less direct.

### Option B: Dedicated home renderer plus optional subcommand

Change `Cli.command` to `Option<Command>`. Dispatch `None` to one small,
write-oriented home module. Use Clap metadata as the source of command names,
aliases, and descriptions where practical.

**Pros**

- The bare invocation becomes a normal successful application behavior.
- Rendering can adapt to TTY, pipes, CI, Windows, and `NO_COLOR`.
- The output can be tested through the binary seam.
- Config and network work can remain outside the home path.
- One renderer hides layout and styling complexity behind a small interface.

**Cons**

- Adds one production module and direct styling dependencies.
- Requires care not to duplicate command metadata.

### Recommendation

Use Option B. It creates the deeper module: callers ask to write the home
screen; the module owns layout, styling, and command-catalog formatting.

## Public Test Seam

The primary seam is the compiled `mailghost` executable:

```text
arguments + environment
          |
          v
      mailghost binary
          |
          +----> exit status
          +----> stdout
          `----> stderr
```

Use `assert_cmd` only as a process-driving tool. Do not mock `Cli`, the home
renderer, Clap, or `AccountStore` for these tests.

For later storage tests, use `AccountStore` with a temporary directory. For
future provider tests, introduce a provider seam only when a concrete behavior
requires it; mock Mail.tm, not Mailghost's internal modules.

## Rollout & Rollback

**Reversibility mechanism:** neither.

- No shared schema, API, or multi-consumer contract changes.
- The behavior should ship to every CLI user immediately.
- The blast radius is the zero-argument path only.
- Rollback is a normal Git revert and release patch.
- A feature flag would add more branches and test states than risk reduction.

## TDD Execution Plan

## Phase 0: Establish the Baseline

### What This Accomplishes

Proves the repository is green and records the behavior that the tracer test
must change.

### Learner Actions

1. Create a branch such as `feat/branded-home`.
2. Run:

   ```bash
   cargo test --all-targets --locked
   cargo run --quiet --
   cargo run --quiet -- --help
   cargo run --quiet -- --version
   ```

3. Record:
   - the exit status for bare `mailghost`;
   - whether output uses stdout or stderr;
   - the strings that must remain stable after the change.
4. Without reading the answer, predict why `run` is never reached for the bare
   invocation. Then verify the prediction by tracing `main` → `Cli::parse`.

### Phase Checks

- [ ] Five existing tests pass.
- [ ] Bare invocation is observed exiting `2`.
- [ ] The learner can explain that required-subcommand validation occurs
      during Clap parsing.

## Phase 1: Prepare the Binary Test Harness

### What This Accomplishes

Creates a public-interface test location without changing production behavior.

### Changes

**File:** `Cargo.toml`

- Add `assert_cmd = "2.2.2"` under `[dev-dependencies]`.
- Keep `tempfile`; it is used by existing account tests.

**File:** `tests/home.rs`

- Create the integration-test file.
- Import only what is needed to launch the compiled binary and inspect its
  process result.
- Do not import private Mailghost modules.

### Learner Exercise

Before writing an assertion, answer:

> Which three outputs of a CLI process are part of its public contract?

Expected concepts to retrieve: status, stdout, stderr.

### Phase Checks

- [ ] `cargo test --test home --locked` compiles with zero tests.
- [ ] Existing unit tests remain green.

## Phase 2: Tracer Bullet — Bare Invocation

### What This Accomplishes

Proves the entire path from process launch to observable output.

### RED

Write exactly one integration test:

```text
bare_command_shows_branded_home
```

It should launch `mailghost` with:

- no arguments;
- `NO_COLOR=1` for deterministic plain output.

It should verify one coherent behavior:

- success status;
- stdout contains `mailghost`, `Usage`, and `generate`;
- stderr is empty.

Do not assert the complete output. Do not add tests for all commands yet.

Run:

```bash
cargo test --test home bare_command_shows_branded_home --locked -- --exact --nocapture
```

Save the failure. A valid RED failure should report the current exit code `2`,
empty stdout, or help on stderr.

### GREEN

Only after RED is demonstrated:

**File:** `src/cli.rs`

- Make the parsed subcommand optional.
- Preserve all current variants and aliases.

**File:** `src/home.rs`

- Add the smallest renderer capable of writing a plain branded home screen to
  a supplied writer.
- Keep the interface write-oriented so layout does not leak into `main`.
- Initially render only enough content to satisfy the tracer test.

**File:** `src/lib.rs`

- Register the home module.
- Export only the minimum interface required by the application entry point.

**File:** `src/app.rs`

- Dispatch the missing-command case to the home renderer before constructing
  `AccountStore`.
- Preserve current dispatch for every real command.

### Phase Checks

- [ ] The new test transitions RED → GREEN.
- [ ] Bare output is on stdout.
- [ ] No config directory or network access occurs.
- [ ] Existing five tests remain green.

## Phase 3: Discoverable Command Catalog

### What This Accomplishes

Makes the home screen a useful replacement for the accidental Clap error.

Repeat these as separate RED→GREEN cycles. Never write both tests first.

### Cycle 3.1 — Commands and aliases

Add one behavior test:

```text
home_lists_commands_and_short_aliases
```

The independent expected literals are:

```text
generate / g
messages / m
delete   / d
account  / me
```

The test should tolerate spacing and visual-layout changes.

Minimal GREEN:

- render all commands and aliases;
- prefer Clap's command metadata over a second manually maintained catalog;
- keep descriptions sourced from the existing command definitions where
  practical.

### Cycle 3.2 — Version and options

Add one behavior test:

```text
home_shows_version_and_global_options
```

Verify the package version, `--help`, and `--version`.

Minimal GREEN:

- obtain version from compile-time package metadata or Clap metadata;
- do not duplicate a hard-coded `1.0.0` in production.

### Phase Checks

- [ ] Each test is observed failing before its matching production change.
- [ ] Adding or renaming an internal helper would not break these tests.
- [ ] `cargo run --quiet --` shows the complete plain home screen.

## Phase 4: Adaptive Color Without Brittle Tests

### What This Accomplishes

Adds the visual identity while preserving clean redirected output.

### Dependency Decision

Use direct dependencies:

```toml
anstream = "1.0"
anstyle = "1.0"
```

`anstream` adapts ANSI styling to terminal capabilities and respects
`NO_COLOR`; `anstyle` represents styles without hand-assembling escape codes.
Both support Rust versions older than the project's MSRV.

### Cycle 4.1 — `NO_COLOR`

Write:

```text
no_color_disables_ansi_sequences
```

With `NO_COLOR=1`, assert that stdout does not contain the ANSI control-sequence
prefix.

Minimal GREEN:

- route styled output through an adaptive stream;
- do not put raw ANSI escapes directly into the ASCII-art constant.

### Cycle 4.2 — Piped output

Decide whether this needs a separate automated test after inspecting
`anstream` behavior. Captured integration-test output is already non-TTY, so
the tracer test may provide sufficient evidence.

Recommended resolution: avoid a duplicate test unless a regression proves the
environment behavior is unclear.

### Manual Visual Check

```bash
cargo run --quiet --
NO_COLOR=1 cargo run --quiet --
cargo run --quiet -- | sed -n '1,40p'
```

Check legibility on both light and dark terminal themes.

### Phase Checks

- [ ] Interactive output is colored.
- [ ] `NO_COLOR=1` output is plain.
- [ ] Piped output contains no escape sequences.
- [ ] Semantic integration tests do not assert exact colors or full spacing.

## Phase 5: Regression Behaviors

### What This Accomplishes

Protects existing CLI contracts affected by making the subcommand optional.

Add one test and complete its RED→GREEN or characterization cycle before
starting the next:

1. `explicit_help_still_succeeds`
   - `mailghost --help` succeeds;
   - output contains the command list.
2. `version_flag_still_succeeds`
   - `mailghost --version` succeeds;
   - output contains the package version.
3. `unknown_command_is_rejected`
   - an invalid subcommand fails;
   - stderr explains the invalid command.

If a characterization test passes immediately, it is not a TDD RED cycle.
Keep it only if it protects an important regression caused by the interface
change, and label it as characterization in the commit message or notes.

### Phase Checks

- [ ] Existing aliases still parse.
- [ ] Existing subcommands still dispatch.
- [ ] Unknown commands do not fall through to the home screen.

## Phase 6: Refactor While Green

### What This Accomplishes

Improves the design only after all required behavior exists.

Review in this order:

1. Is command metadata duplicated between Clap and the home renderer?
2. Does the home renderer hide layout and style decisions behind a small
   write-oriented interface?
3. Does the no-command branch avoid config and provider construction?
4. Are tests coupled only to observable output and status?
5. Would changing the ASCII art preserve all semantic tests?

Potential refactors:

- derive command rows from `Cli::command()` metadata;
- separate semantic rows from presentation styling inside `home.rs`;
- extract repeated integration-test process setup only after the third real
  duplication;
- keep private helpers untested directly.

Run tests after each individual refactor.

### Phase Checks

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --all-targets --locked -- -D warnings`
- [ ] `cargo test --all-targets --locked`
- [ ] `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.12 .github/workflows/*.yml`

## Project-Wide Test Learning Roadmap

Complete the branded-home feature first. Then grow project confidence in this
order. Each row is a separate learning milestone, not a batch of tests to write
up front.

| Priority | Behavior | Highest practical seam | Boundary policy |
|---|---|---|---|
| P0 | Bare CLI home | compiled binary | no mocks |
| P0 | CLI help/version/errors | compiled binary | no mocks |
| P0 | Account missing/save/load/delete | `AccountStore` with temp dir | real filesystem fixture |
| P0 | Corrupt account JSON reports context | `AccountStore::load` | real temp file |
| P1 | Generate refuses a second account | application command seam | fake Mail.tm boundary only |
| P1 | Generate saves and prints new address | application command seam | fake provider + temp store |
| P1 | Messages without account fail offline | application command seam | provider must not be called |
| P1 | Delete is idempotent | application command seam | real temp store |
| P2 | Empty and populated inbox output | application command seam | fake provider response |
| P2 | Message selection validation | application I/O seam | supplied input/output |
| P2 | Desktop opener invocation | opener boundary | fake external opener |
| P3 | Live Mail.tm smoke test | opt-in external test | no mock, never default CI |

### Rule for P1 and P2

Do not introduce one giant generic mock object. When a test reaches the
Mail.tm, clock, terminal input, or desktop opener boundary, define the smallest
capability-specific interface required by that behavior. Keep Mailghost's own
modules real.

### Suggested Next Unit-Test Exercises

After finishing the home feature:

1. Add one corrupt-JSON `AccountStore::load` test.
2. Add one replacement-save test and decide whether replacement is part of the
   store contract.
3. Trace `generate` and list every hard-coded boundary that prevents a
   deterministic test.
4. Propose two provider interfaces, compare their caller complexity, and choose
   the deeper one before writing the first provider-driven test.

## Test Review Rubric

For every test the learner submits, review in this order:

1. **Requirement:** Does the name describe user-visible behavior?
2. **Seam:** Does it use the highest public interface practical?
3. **Boundary:** Are only external/system boundaries faked?
4. **Expectation:** Is the expected value an independent literal or specified
   fact?
5. **Evidence:** Was the intended RED failure observed?
6. **Minimality:** Did GREEN add only what this test required?
7. **Durability:** Would an internal refactor leave the test unchanged?

If a test fails the rubric, repair the earliest failed item rather than adding
more assertions.

## Commit Cadence

Recommended commits preserve the learning history:

```text
test: specify branded home for bare command
feat: render home when no command is provided
test: specify command catalog on home screen
feat: render command catalog from CLI metadata
test: specify plain output when color is disabled
feat: adapt home colors to terminal capabilities
test: protect explicit help and version behavior
refactor: deepen branded home renderer
```

Do not commit a permanently failing test to `main`; keep RED and GREEN close
enough that each pushed commit is buildable unless working on a dedicated
learning branch.

## File Summary

Expected production and test shape after the learning plan is complete:

```text
mailghost/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── app.rs                 # None dispatches home before account setup
│   ├── cli.rs                 # Optional subcommand; command metadata
│   ├── home.rs                # Layout, metadata rows, adaptive styling
│   ├── lib.rs                 # Registers the home module
│   └── main.rs                # Remains a thin process entry point
├── tests/
│   └── home.rs                # Public binary behavior tests
└── thoughts/
    └── plans/
        └── 2026-07-01-branded-home-and-tdd.md
```

**Expected new production files:** 1  
**Expected new integration-test files:** 1  
**Plan files:** 1

## Verification Strategy

### Automated

```bash
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo publish --dry-run --allow-dirty --locked
```

### Manual

```bash
cargo run --quiet --
cargo run --quiet -- --help
cargo run --quiet -- --version
NO_COLOR=1 cargo run --quiet --
cargo run --quiet -- | sed -n '1,40p'
```

### Expected Evidence

- Bare invocation succeeds.
- Interactive output visually resembles the approved branded layout.
- Captured and piped output is plain.
- Existing command paths remain unchanged.
- Every new behavior test has recorded RED evidence before GREEN.

## Related Research

- Existing implementation: `src/main.rs`, `src/cli.rs`, `src/app.rs`
- Existing tests: `src/cli.rs`, `src/account.rs`
- [Clap `Command` and `CommandFactory`](https://docs.rs/clap/latest/clap/struct.Command.html)
- [`assert_cmd` CLI integration testing](https://docs.rs/assert_cmd/)
- [`anstream` adaptive output](https://docs.rs/anstream/latest/anstream/struct.AutoStream.html)
- [Rust Cargo testing guide](https://doc.rust-lang.org/cargo/guide/tests.html)
- Matt Pocock TDD skill: behavior-focused tests, public seams, vertical slices

## Open Questions

- **Should bare `mailghost` and `mailghost --help` render identically?**

  Recommend: no. Bare invocation gets the branded landing screen;
  `--help` remains Clap's conventional reference output. This preserves
  scripting expectations and gives users both a friendly entry point and a
  standard help surface.

- **Should tests snapshot the full home screen?**

  Recommend: no initially. Assert semantic anchors and status streams. A full
  snapshot would make harmless spacing and color refinements expensive.

- **Should color be forced in CI to test exact ANSI codes?**

  Recommend: no. Test the stable requirement that `NO_COLOR=1` strips ANSI.
  Validate the exact palette manually because it is presentation, not command
  behavior.

- **Should provider abstractions be added during this feature?**

  Recommend: no. They do not help the zero-argument path. Add provider seams
  later in response to the first application-command behavior test.
