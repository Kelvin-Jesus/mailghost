# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.3] - 2026-07-02

### Changed

- Restored the original compact terminal envelope from v1.1.0.

## [1.1.2] - 2026-07-01

### Changed

- Enlarged the terminal envelope for a deeper, more precise symmetrical fold.

## [1.1.1] - 2026-07-01

### Fixed

- Replaced the uneven terminal envelope with a symmetrical Unicode drawing.

## [1.1.0] - 2026-07-01

### Added

- Added a branded, colored home screen for bare `mailghost` invocations.
- Added adaptive terminal styling that respects redirected output and
  `NO_COLOR`.

### Changed

- Bare invocation now exits successfully without accessing local account data
  or the Mail.tm provider.

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

[Unreleased]: https://github.com/Kelvin-Jesus/mailghost/compare/v1.1.2...HEAD
[1.1.2]: https://github.com/Kelvin-Jesus/mailghost/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/Kelvin-Jesus/mailghost/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/Kelvin-Jesus/mailghost/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/Kelvin-Jesus/mailghost/releases/tag/v1.0.0
