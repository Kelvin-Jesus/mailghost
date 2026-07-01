# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Corrected the minimum supported Rust version to 1.88 to match the locked
  dependency graph.
- Added descriptive command names while retaining the original short aliases.
- Split account storage, CLI parsing, and application behavior into testable
  modules.
- Report filesystem, input, provider, and launcher failures with a non-zero
  exit status.
- Made account writes atomic and local deletion independent of provider
  availability.

### Added

- Added tagged GitHub releases for macOS, Linux, and Android on x86_64 and
  ARM64, including SHA-256 checksums.

## [1.0.0] - 2026-06-30

### Added

- Initial disposable inbox generation, message listing, account display, and
  local account deletion.

[Unreleased]: https://github.com/Kelvin-Jesus/mailsy-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Kelvin-Jesus/mailsy-rs/releases/tag/v1.0.0
