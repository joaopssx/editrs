# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-25

### Added

- Initial release of editrs.
- GitHub Actions CI (format, lint, check, test).
- Smoke tests for `--help`, `--version` and unknown flags.

### Changed

- `--version` now also prints the author.
- The plugin marketplace makes no network requests until editrs has its own server.

### Fixed

- The `British English` language name in `config.jsonc` is now recognized.

[Unreleased]: https://github.com/joaopssx/editrs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/joaopssx/editrs/releases/tag/v0.1.0
