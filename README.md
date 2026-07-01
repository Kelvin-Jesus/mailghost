<p align="center">
  <img src="assets/mailghost-logo.svg" alt="Mailghost logo" width="220">
</p>

<h1 align="center">mailghost</h1>

<p align="center">
  <a href="https://github.com/Kelvin-Jesus/mailghost/actions/workflows/ci.yml"><img src="https://github.com/Kelvin-Jesus/mailghost/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/mailghost"><img src="https://img.shields.io/crates/v/mailghost.svg" alt="Crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/Kelvin-Jesus/mailghost" alt="License"></a>
</p>

`mailghost` is a friendly command-line client for creating and checking a
disposable [Mail.tm](https://mail.tm/) inbox without leaving the terminal.

> Disposable inboxes are public-facing, temporary infrastructure. Do not use
> them for sensitive data, permanent accounts, or account recovery.

## Features

- Create and persist one disposable inbox.
- Show its address and creation time.
- List received messages and open a selected message.
- Remove the locally stored inbox data.

## Installation

Install the prebuilt binary with Homebrew on macOS or Linux:

```sh
brew install Kelvin-Jesus/tap/mailghost
```

Arch Linux packaging is prepared, but its initial AUR publication is pending
while new AUR account registrations are unavailable. Until then, use the
prebuilt Linux archive from the [latest release](https://github.com/Kelvin-Jesus/mailghost/releases/latest).

You can also install it from crates.io:

```sh
cargo install mailghost --locked
```

Or install the current repository version:

```sh
cargo install --git https://github.com/Kelvin-Jesus/mailghost
```

To build from a local checkout:

```sh
git clone https://github.com/Kelvin-Jesus/mailghost.git
cd mailghost
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
mailghost generate
mailghost account
mailghost messages
mailghost delete
```

The `g`, `me`, `m`, and `d` forms are available as short aliases. Run
`mailghost --help` or `mailghost <command> --help` for command-line help.

## Local data

The active account is stored as `mailghost/account.json` inside the operating
system's user configuration directory:

- macOS: `~/Library/Application Support/mailghost/account.json`
- Linux: `${XDG_CONFIG_HOME:-~/.config}/mailghost/account.json`

When you select a message, `mailghost messages` writes its text body to a
temporary file and asks the operating system to open it.

`mailghost delete` removes local data only. Inbox availability and retention are
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
