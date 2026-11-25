# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `scripts/bench-startup.sh` to measure startup time with hyperfine, also run on pull requests.

### Changed

- Release builds abort on panic and strip symbols: the Windows binary is about 16% smaller.
- Nine unused dependencies were removed (611 -> 390 packages in `Cargo.lock`), so builds are faster.

### Security

- The Lua plugin sandbox now applies the plugin's permissions. Before, file and process functions (`io.open`, `os.execute` and others) stayed available to every plugin.

### Fixed

- editrs now starts on Linux and macOS: the config folder comes from `$XDG_CONFIG_HOME` or `$HOME/.config` instead of `$USERPROFILE`.
- editrs builds on macOS and FreeBSD (caps lock is reported as off there).
- The Linux build, broken by a missing import in 0.1.0.

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
