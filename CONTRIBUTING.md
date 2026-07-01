# Contributing

## Development setup

Install a Rust toolchain compatible with the `rust-version` in `Cargo.toml`,
then run:

```sh
cargo test --all-targets
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

Keep changes focused and add tests for new behavior. Commit `Cargo.lock`
changes because `mailghost` is an application.

## Pull requests

Describe the user-visible change, note any provider or platform assumptions,
and update `CHANGELOG.md` when appropriate. CI must pass before merge.
