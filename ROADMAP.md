# Roadmap

## 0.1.0 "Foundation" (done)

Project identity, license, documentation, CI and code quality gates.

## 0.2.0 "Evolution" (planned)

### Fixes first

- [x] Find the config folder correctly on Linux, macOS and FreeBSD (`$HOME`, XDG).
- Build on macOS and FreeBSD (caps lock detection).
- Make the Lua sandbox policy actually apply, and fix the plugin API bugs listed in `LUA_DOCS.md`.
- Read `config.jsonc` instead of only its first line.

### Speed

- Redraw only when something changed instead of every frame.
- Avoid per-key and per-frame work that is not needed (caps lock polling, cloning state, spawning tasks).
- Faster loading of large files and a better text buffer structure.
- Release profile tuning and benchmarks (`criterion`, `hyperfine`, `cargo flamegraph`).

### Features

- Regex search and replace, split views, multiple cursors.
- Command palette and keybinds driven by the config file.
- Auto-indent, bracket pairing, relative line numbers.
- Session restore, Git gutter, LSP support.
- A larger Lua API.

### Languages

- Interface: complete the existing translations and add Brazilian Portuguese, Italian and more, with strings moved out of the code.
- Syntax highlighting for more programming languages.

### Polish

- Better error messages, more tests, themes (including high contrast).
- Packages: AUR, Homebrew, Nix, `.deb`, `.rpm`, winget, release binaries.
