# AGENTS.md

Guidance for AI coding agents (GitHub Copilot CLI, Claude Code, Cursor, Aider,
OpenAI Codex / ChatGPT agents, Continue, Windsurf, etc.) contributing to the
`embedded-sensors` repository. This file complements
[`.github/copilot-instructions.md`](.github/copilot-instructions.md) and the
human-facing [`CONTRIBUTING.md`](CONTRIBUTING.md). If anything here conflicts
with `CONTRIBUTING.md`, the human contribution rules win.

---

## 1. Repository at a glance

`embedded-sensors` is a small Rust Cargo **workspace** that defines a Hardware
Abstraction Layer (HAL) for sensors used in embedded systems. It is published
on crates.io as two sibling crates:

| Crate                       | Path                       | Description                                | Notes                          |
| --------------------------- | -------------------------- | ------------------------------------------ | ------------------------------ |
| `embedded-sensors-hal`      | `embedded-sensors/`        | Blocking sensor traits                     | `#![no_std]`, MSRV `1.79`      |
| `embedded-sensors-hal-async`| `embedded-sensors-async/`  | Async sensor traits, depends on blocking   | `#![no_std]`, MSRV `1.79`      |

Both crates:

- are `#![no_std]`
- `#![forbid(missing_docs)]` and `#![forbid(unsafe_code)]`
- expose `humidity`, `sensor`, and `temperature` modules
- gate `defmt` support behind a `defmt` Cargo feature
- use the `paste` crate to drive a shared `decl_threshold_traits!` macro that
  generates blocking and async threshold trait families from one source

The workspace root `Cargo.toml` declares both members and includes:

```toml
[patch.crates-io]
embedded-sensors-hal = { path = "embedded-sensors" }
```

so the async crate always resolves to the in-tree blocking crate. **Do not
remove this patch** without a deliberate, reviewed reason.

### Source layout

```
embedded-sensors/
├── Cargo.toml                       # workspace root
├── README.md
├── CONTRIBUTING.md
├── CODEOWNERS
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE                          # MIT
├── .github/
│   ├── copilot-instructions.md      # commit / attribution rules (READ FIRST)
│   ├── DOCS.md                      # rationale for the CI layout
│   ├── codecov.yml
│   ├── dependabot.yml
│   └── workflows/
│       ├── check.yml                # fmt, clippy, semver, doc, hack, msrv
│       └── nostd.yml                # no-std target builds
├── embedded-sensors/                # blocking crate
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs
│       ├── sensor.rs                # Error, ErrorKind, ErrorType, macro
│       ├── temperature.rs           # TemperatureSensor + threshold traits
│       └── humidity.rs              # RelativeHumiditySensor + threshold traits
├── embedded-sensors-async/          # async crate
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs                   # adds #![allow(async_fn_in_trait)]
│       ├── sensor.rs                # re-exports from blocking crate
│       ├── temperature.rs           # async TemperatureSensor + ThresholdWait
│       └── humidity.rs              # async RelativeHumiditySensor + ThresholdWait
└── supply-chain/                    # cargo-vet config (ODP consolidated audits)
```

---

## 2. Ground rules for every agent

These are **non-negotiable** and apply to every pilot, every session.

1. **Read [`.github/copilot-instructions.md`](.github/copilot-instructions.md)
   before composing any commit.** The commit-message and AI-attribution
   conventions there are authoritative.
2. **Every commit you author or co-author must include an `Assisted-by:`
   trailer** of the form:
   ```
   Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
   ```
   Verify your own agent name and model version at the start of the session
   — do not copy a value from a previous run.
3. **AI agents MUST NOT add `Signed-off-by` trailers.** DCO sign-off can only
   be done by humans.
4. **Subject lines:** capitalized, imperative mood, ≤ 50 characters. Body
   wrapped at 72 characters, separated from the subject by a blank line.
   Explain *what* and *why*, not *how*.
5. **Never commit secrets, tokens, or personal data.** This repo has no
   secret-handling code; if you find any leaked, stop and surface it.
6. **Do not change `unsafe_code` or `missing_docs` lints from `forbid` to
   anything weaker.** Both crates intentionally forbid unsafe code and
   undocumented public items.
7. **Do not lower the MSRV (`1.79`)** without an explicit human request and a
   matching update to `.github/workflows/check.yml`.
8. **Do not edit `LICENSE`, `CODEOWNERS`, `CODE_OF_CONDUCT.md`, or
   `SECURITY.md`** unless the task is explicitly about them.
9. **No force-pushes.** Use additional commits to fix issues, never rewrite
   shared history.
10. **Stay inside the working directory** when searching the filesystem. Do
    not write outside the repo or into temp directories.
11. **Prefer small, surgical diffs.** If a task balloons, stop and report
    instead of making sweeping unrelated changes.

---

## 3. Conventions you must follow

### 3.1 Rust style

- **Edition** `2021`, **MSRV** `1.79`. Use only features stable in `1.79`.
- Format with `cargo fmt` (no project-specific `rustfmt.toml`; use defaults).
- Keep clippy clean on both `stable` and `beta` (CI runs both).
- All public items need rustdoc — `missing_docs` is a hard error.
- No `unsafe` blocks. If you genuinely need one, stop and ask a human.
- Prefer `#[inline]` on trivial `&mut T` blanket-impl forwarders, matching
  the style already used in `embedded-sensors/src/temperature.rs` and
  `humidity.rs`.

### 3.2 Trait design pattern

The crate follows the [`embedded-hal`](https://crates.io/crates/embedded-hal)
style: a small `Error` / `ErrorKind` / `ErrorType` triple plus per-quantity
sensor traits. New sensor types should mirror this pattern:

1. Add a module (e.g. `pressure.rs`) in **both** `embedded-sensors/src/` and
   `embedded-sensors-async/src/`.
2. Define a unit alias (e.g. `pub type Pascals = f32;`) in the blocking
   crate, and `pub use` it from the async crate to keep types in sync.
3. Define the blocking `XSensor` trait extending `ErrorType` with a single
   measurement method.
4. Define the async equivalent with `async fn`, extending the re-exported
   `ErrorType`.
5. Provide `impl<T: XSensor + ?Sized> XSensor for &mut T` blanket impls.
6. Call `decl_threshold_traits!(blocking, …)` / `decl_threshold_traits!(async, …)`
   to get `XThresholdSet`, `XHysteresis`, and (async only) `XThresholdWait`
   traits for free.
7. Register the new module in `lib.rs` (both crates).
8. Add unit tests using the `MockX` pattern visible in `temperature.rs`
   (`assert_approx_eq` for `f32` comparisons; `tokio` `#[tokio::test]` with
   `features = ["macros", "rt"]` for async tests).

### 3.3 Feature flags

- The only feature today is `defmt`. The async crate forwards it via
  `defmt = ["dep:defmt", "embedded-sensors-hal/defmt"]`. If you add a new
  feature, keep it **additive** — `cargo hack --feature-powerset check` runs
  in CI and will fail otherwise.
- Optional dependencies must use `dep:` syntax (namespaced features, MSRV
  `1.60+`).

### 3.4 Documentation

- `lib.rs` in each crate uses `#![doc = include_str!("../README.md")]`. Edits
  to the per-crate README change the rendered crate docs — keep them
  doctest-clean. `cargo doc --no-deps --all-features --locked` runs in CI.
- Public modules begin with a `//!` overview. The `temperature` and
  `humidity` modules also include a worked HAL-implementor example as a
  doctest — keep new modules consistent.

### 3.5 Commit hygiene

From `CONTRIBUTING.md`:

- Each commit must build **without warnings**.
- Squash typo / formatting fixups into the commit they belong to before
  pushing.
- The project disables squash-merging; the commit history you push *is* the
  history that lands.
- Open PRs as **draft** first and let CI go green before requesting review.

---

## 4. CI — what runs and how to mirror it locally

`.github/workflows/check.yml` jobs:

| Job      | Command                                              |
| -------- | ---------------------------------------------------- |
| `fmt`    | `cargo fmt --check`                                  |
| `clippy` | `cargo clippy` (stable **and** beta toolchains)      |
| `semver` | `cargo semver-checks` (via `cargo-semver-checks-action`) |
| `doc`    | `cargo doc --no-deps --all-features --locked` (nightly, `RUSTDOCFLAGS=--cfg docsrs`) |
| `hack`   | `cargo hack --feature-powerset check --locked`       |
| `msrv`   | `cargo +1.79 check --locked`                         |

`.github/workflows/nostd.yml`:

- `cargo check --target thumbv7m-none-eabi --no-default-features --locked`
- `cargo check --target aarch64-unknown-none --no-default-features --locked`

### 4.1 Minimum local validation before pushing

Run these from the repo root and make sure they all pass:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo build --workspace --all-features --locked
cargo test  --workspace --all-features --locked
cargo doc   --workspace --no-deps --all-features --locked
```

If you have `cargo-hack` installed, also run:

```sh
cargo hack --feature-powerset check --locked
```

If you have the cross targets installed:

```sh
cargo check --target thumbv7m-none-eabi --no-default-features --locked
cargo check --target aarch64-unknown-none --no-default-features --locked
```

If a CI command cannot run in your sandbox (e.g. nightly toolchain or cross
targets unavailable), **say so explicitly in the final report** — do not
silently skip a check.

### 4.2 Supply-chain (`cargo-vet`)

`supply-chain/` holds `cargo-vet` configuration that points at the ODP
consolidated audits repo. If you add or upgrade a runtime dependency, expect
`cargo vet` failures and let a human resolve the audit.

---

## 5. Branching, remotes, and pushing

- Default branch is **`main`**. Never push directly to `main`.
- Work on a topic branch off `upstream/main`:
  ```sh
  git fetch upstream
  git checkout -B <topic> upstream/main
  ```
- Push to your **fork's `origin`**, not upstream:
  ```sh
  git push -u origin <topic>
  ```
- Open a **draft PR** against `OpenDevicePartnership/embedded-sensors` `main`
  only after CI on the topic branch is green.
- **Do not force-push** to shared branches. If history needs to change, add a
  new commit instead and let a human decide.

### 5.1 Author identity

When asked to author commits as a specific human, use per-invocation
identity rather than mutating global git config:

```sh
git -c user.name="Felipe Balbi" \
    -c user.email="felipe.balbi@microsoft.com" \
    commit -m "<subject>"
```

Never run `git config --global user.email …` to impersonate a contributor.

### 5.2 Line endings

This repo stores files with **LF** endings. On Windows, configure the local
clone with:

```sh
git config --local core.autocrlf false
```

Do not introduce CRLF (`^M`) into committed files. Verify with `git diff` or
`Select-String "`r"` before committing.

---

## 6. What "done" looks like for a typical task

For any change you author:

1. Code compiles cleanly (`cargo build --workspace --all-features`).
2. Tests pass (`cargo test --workspace --all-features`).
3. Clippy is clean on stable (best-effort on beta).
4. `cargo fmt --check` is clean.
5. `cargo doc` builds without warnings.
6. New public items have rustdoc.
7. New sensor traits exist in **both** the blocking and async crates and
   share type aliases.
8. New features are additive and pass `cargo hack --feature-powerset check`.
9. Commit messages follow the rules in
   [`.github/copilot-instructions.md`](.github/copilot-instructions.md),
   including the `Assisted-by:` trailer.
10. The diff is scoped to the task; no opportunistic refactors.

---

## 7. Per-pilot notes

The rules above apply to all agents. The subsections below highlight
pilot-specific quirks.

### 7.1 GitHub Copilot CLI (`copilot` / Copilot in the terminal)

- Verify your model (e.g. `claude-opus-4.7`, `gpt-5.x`, etc.) before
  composing the `Assisted-by` trailer. Example trailer for this agent:
  ```
  Assisted-by: GitHub Copilot:claude-opus-4.7
  ```
- Use the local `gh` CLI for repo operations (`gh repo fork`,
  `gh repo sync`, `gh pr create --draft`). Do **not** open the PR
  automatically unless the task explicitly asks for it.
- When running long commands (builds, tests, `cargo hack`), give them ample
  time — workspace builds from cold can take a few minutes.
- Batch independent shell calls in a single turn when possible; prefer
  ripgrep / `grep` over ad-hoc `Get-ChildItem` recursion for code search.

### 7.2 GitHub Copilot Chat / Copilot in IDE

- Pick up `.github/copilot-instructions.md` automatically — re-read it if
  the session is long-lived.
- Prefer inline-edit suggestions over whole-file rewrites for `.rs` files
  so reviewers can see surgical diffs.
- Doctest snippets inside `//!` comments are compiled; keep them runnable
  and `no_std`-friendly (don't reach for `std`-only types).

### 7.3 Claude Code / Anthropic agents

- Honor this `AGENTS.md` and the `copilot-instructions.md` pointer.
- When composing `Assisted-by`, use the actual model id you are running on
  (`claude-opus-4.x`, `claude-sonnet-4.x`, …). Do not invent a version.
- Avoid speculative refactors; the codebase is small and stable, and review
  bandwidth is limited.

### 7.4 Cursor / Windsurf

- Treat `AGENTS.md` as the project-level "rules" file. If the IDE supports
  per-repo rules, mirror the ground rules from §2 into it.
- Keep generated edits inside the workspace — do not propose changes to
  files outside `embedded-sensors/`, `embedded-sensors-async/`, `.github/`,
  or the root metadata files unless the task asks for it.

### 7.5 Aider

- Add `AGENTS.md`, `.github/copilot-instructions.md`, `CONTRIBUTING.md`, and
  the affected crate's `Cargo.toml` to the chat context for any non-trivial
  change.
- Use `--no-auto-commits` for exploratory sessions so you can craft a single
  conventional commit at the end with the correct `Assisted-by:` trailer.

### 7.6 OpenAI Codex / ChatGPT code agents

- Identify the model in the trailer, e.g.
  `Assisted-by: OpenAI Codex:gpt-5.x`.
- Do not enable network access in sandboxes unless required — the CI
  commands listed in §4.1 do not need network once dependencies are
  fetched.

### 7.7 Continue / other open-source assistants

- Same ground rules: `#![forbid(unsafe_code)]`, `#![forbid(missing_docs)]`,
  MSRV `1.79`, `Assisted-by:` trailer with the actual model name.

---

## 8. Common pitfalls

- **Forgetting to update both crates.** The blocking and async crates mirror
  each other; a new trait, type alias, or feature in one almost always
  needs a counterpart in the other.
- **Breaking `decl_threshold_traits!`.** The macro is shared via
  `pub use embedded_sensors_hal::decl_threshold_traits;` in the async crate.
  Changing the macro signature affects both crates and both modes
  (`blocking` and `async`).
- **Bumping `embedded-sensors-hal` without considering semver.** The
  `semver` CI job runs `cargo-semver-checks`; breaking changes need a major
  version bump and coordinated updates in `embedded-sensors-async/Cargo.toml`.
- **Touching the workspace `[patch.crates-io]` table.** Removing it makes
  the async crate resolve against the published version, which silently
  hides local breakage.
- **Adding `std` dependencies.** Both crates are `#![no_std]` and CI checks
  cross-compilation to `thumbv7m-none-eabi` and `aarch64-unknown-none`.
- **Adding non-additive features.** `cargo hack --feature-powerset` will
  fail; design features so any combination compiles.
- **CRLF line endings on Windows clones.** See §5.2.

---

## 9. When in doubt

- Re-read [`.github/copilot-instructions.md`](.github/copilot-instructions.md)
  and [`CONTRIBUTING.md`](CONTRIBUTING.md).
- Prefer asking a human over guessing on: MSRV changes, new public API,
  feature-flag design, dependency additions, anything touching
  `supply-chain/`, and anything that would require a force-push.
- If a CI command cannot be reproduced locally, document the gap in your PR
  description and let reviewers decide.

---

_Last reviewed for repository layout: workspace with `embedded-sensors`
(blocking) and `embedded-sensors-async` (async) crates, MSRV `1.79`, MIT
licensed._
