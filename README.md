# mailsy

[![CI](https://github.com/Kelvin-Jesus/mailsy-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Kelvin-Jesus/mailsy-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mailsy.svg)](https://crates.io/crates/mailsy)
[![License](https://img.shields.io/crates/l/mailsy.svg)](LICENSE)

`mailsy` creates and checks a disposable [Mail.tm](https://mail.tm/) inbox
without leaving the terminal.

> Disposable inboxes are public-facing, temporary infrastructure. Do not use
> them for sensitive data, permanent accounts, or account recovery.

## Features

- Create and persist one disposable inbox.
- Show its address and creation time.
- List received messages and open a selected message.
- Remove the locally stored inbox data.

## Installation

After a release is published, install it from crates.io:

```sh
cargo install mailsy --locked
```

Or install the current repository version:

```sh
cargo install --git https://github.com/Kelvin-Jesus/mailsy-rs
```

To build from a local checkout:

```sh
git clone https://github.com/Kelvin-Jesus/mailsy-rs.git
cd mailsy-rs
cargo build --release
```

The project requires Rust 1.88 or newer.

### Prebuilt releases

Version tags publish archives for:

- macOS on Intel (`x86_64`) and Apple Silicon (`aarch64`)
- Linux on `x86_64` and `aarch64`
- Android on `x86_64` and `aarch64`

Each GitHub release includes a `SHA256SUMS` file. The Android artifacts are
command-line binaries for Termux or ADB, not APK applications. On Android,
selected message bodies are printed in the terminal instead of being opened by
a desktop application.

## Usage

```sh
mailsy generate
mailsy account
mailsy messages
mailsy delete
```

The original `g`, `me`, `m`, and `d` forms remain available as aliases. Run
`mailsy --help` or `mailsy <command> --help` for command-line help.

## Local data

The active account is stored as `mailsy/account.json` inside the operating
system's user configuration directory:

- macOS: `~/Library/Application Support/mailsy/account.json`
- Linux: `${XDG_CONFIG_HOME:-~/.config}/mailsy/account.json`

When you select a message, `mailsy messages` writes its text body to a
temporary file and asks the operating system to open it.

`mailsy delete` removes local data only. Inbox availability and retention are
controlled by Mail.tm.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow.

## Limitations

- Only one local account is supported at a time.
- Mail.tm is the only provider.
- The CLI is not a full terminal mail reader.
- Provider-side inbox retention and domain availability can change.
- Disposable email should not be used for sensitive, permanent, or
  account-recovery communication.

## License

[MIT](LICENSE)
