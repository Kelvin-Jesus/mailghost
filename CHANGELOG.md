# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-07-01

### Added

- Initial Mailghost release.
- Disposable inbox generation, message listing, account display, and local
  account deletion.
- Tagged GitHub releases for macOS, Linux, and Android on x86_64 and ARM64,
  including SHA-256 checksums.
- Branded vector logo and cross-platform CI.

### Changed

- Added descriptive command names while retaining the original short aliases.
- Split account storage, CLI parsing, and application behavior into testable
  modules.
- Report filesystem, input, provider, and launcher failures with a non-zero
  exit status.
- Made account writes atomic and local deletion independent of provider
  availability.
- Set the minimum supported Rust version to 1.88.

[Unreleased]: https://github.com/Kelvin-Jesus/mailghost/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Kelvin-Jesus/mailghost/releases/tag/v1.0.0
