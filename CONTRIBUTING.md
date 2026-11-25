# Contributing to editrs

Thanks for your interest in improving editrs.

## Reporting bugs and ideas

Open an issue at https://github.com/joaopssx/editrs/issues using the bug report or feature request template. Include your editrs version (`editrs --version`), OS and terminal.

## Development setup

You need Rust 1.96 or newer (the toolchain is pinned to stable in `rust-toolchain.toml`), Git and a C compiler. The first build downloads a font, so it needs internet access.

```sh
git clone https://github.com/joaopssx/editrs.git
cd editrs
cargo build
```

Before opening a pull request, run:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI runs the same checks on Linux, Windows and macOS.

## Pull requests

1. Fork the repository and create a branch from `main`.
2. Keep each pull request focused on one change.
3. Add or update tests when you change behavior.
4. Update `CHANGELOG.md` under `[Unreleased]` for user-facing changes.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add regex search
fix: keep cursor inside the line after paste
docs: document plugin lifecycle
```

Common types: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `ci`, `chore`, `style`.

## Code of conduct

By participating you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
