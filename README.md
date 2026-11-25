<p align="center">
  <img src="content/banner.svg" alt="editrs" width="600">
</p>

<p align="center"><b>A highly customizable TUI text editor written in Rust.</b></p>

<p align="center">
  <a href="https://github.com/joaopssx/editrs/actions/workflows/ci.yml"><img src="https://github.com/joaopssx/editrs/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <img src="https://img.shields.io/badge/rust-1.96+-orange?logo=rust" alt="Rust 1.96+">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/joaopssx/editrs" alt="License"></a>
  <a href="https://github.com/joaopssx/editrs/commits/main"><img src="https://img.shields.io/github/last-commit/joaopssx/editrs" alt="Last commit"></a>
  <a href="https://github.com/joaopssx/editrs/issues"><img src="https://img.shields.io/github/issues/joaopssx/editrs" alt="Issues"></a>
</p>

<p align="center">English | <a href="README.pt-BR.md">Português (Brasil)</a></p>

<!-- TODO: add screenshot -->

## Features

- Tabbed editing with syntax highlighting (powered by `syntect`)
- Sidebar with a file explorer, project-wide search, Git status with diffs, settings and a plugin marketplace view
- Search and replace inside the current file
- Built-in terminal (Ctrl+T or F5)
- Command palette (Ctrl+P), undo and redo, copy, cut and paste through the system clipboard
- Lua 5.5 plugins, with a debug console for plugin output (Ctrl+E or F12); see LUA_DOCS.md for current limits
- Interface strings available in 15 languages (some translations are partial)

The plugin marketplace has no server yet, so its view shows an "unavailable" message.

## Usage
```
Usage: editrs [OPTIONS] [PATH]
    Open a file or folder in the editrs TUI editor.

Options:
--help, -h       Print this help and exit
--version, -v    Print the version (--version also exits)
--admin, -a      Relaunch editrs with administrator rights
--wait, -w       Wait for Enter before starting
--question, -q   Ask for confirmation before starting
--license=all    Print the bundled font license and exit
```

> The block above is also what `editrs --help` prints: `build.rs` reads it from this file.

`PATH` can be a file or a folder. Without it, editrs opens the current folder.

How each option behaves today:

- `--help` / `-h`: prints the text above and exits.
- `--version`: prints `editrs <version> by <author>` and exits. `-v` prints the same line but then keeps going and opens the editor.
- `--admin` / `-a`: starts a new elevated copy of editrs (UAC through PowerShell on Windows, `sudo` or `pkexec` elsewhere) with the other arguments, then exits.
- `--wait` / `-w`: waits for you to press Enter, then continues.
- `--question` / `-q`: asks `Continue? [Y/n]`; anything other than empty, `y` or `yes` exits.
- `--license=all`: prints the license of the bundled 0xProto Nerd Font and exits. `--license=ofl` prints it and continues.

Each of `--help`, `--version`, `--admin` and their short forms can only be given once.

### Main keys

| Keys | Action |
|---|---|
| F1 / Ctrl+K | Help |
| F2 / Ctrl+F | Search |
| F3 / Ctrl+R | Replace |
| F4 / Ctrl+B | Toggle sidebar |
| F5 / Ctrl+T | Toggle terminal |
| F6, F7 / Ctrl+A, Ctrl+D | Previous / next tab |
| Ctrl+N | New file |
| Ctrl+S | Save |
| Ctrl+W | Close tab |
| Ctrl+Q | Quit |
| Ctrl+G | Refresh Git view |
| Ctrl+P | Command palette |
| Ctrl+Z / Ctrl+Y | Undo / redo |
| F12 / Ctrl+E | Plugin debug console |
| Esc | Switch focus between editor and sidebar |

A [Nerd Font](https://www.nerdfonts.com/) is needed for the icons to render.

## Building from source

Requirements: Rust **1.96 or newer** (some dependencies need it), Git, and a C compiler (Lua is compiled from source). The first build downloads the 0xProto Nerd Font from GitHub, so it needs internet access.

```sh
git clone https://github.com/joaopssx/editrs.git
cd editrs
cargo build --release
./target/release/editrs
```

The binary is `target/release/editrs` (`editrs.exe` on Windows).

### Nix

A `flake.nix` is included (`nix build`, `nix run`), but it is **not validated**. The build script downloads a font, which the Nix sandbox does not allow, so it will likely need changes.

## Platforms

| Platform | Status |
|---|---|
| Windows x86_64 | Builds and runs (tested) |
| Linux x86_64 | Builds in CI; see the known issue below |
| macOS | Does not build yet (caps lock detection is not implemented) |
| FreeBSD | Does not build yet (same reason) |

Known issue: outside Windows, editrs looks for the config folder under `$USERPROFILE`, which is usually not set on Linux, so startup fails unless you set it (for example `USERPROFILE=$HOME editrs`). This is tracked for 0.2.0.

## Configuration

editrs keeps its data in:

- Windows: `%APPDATA%\editrs\`
- Other systems: `$USERPROFILE/.config/editrs/`

On first run it creates `config.jsonc` and a `plugins/` folder there. Today only the first line of `config.jsonc` is read: a `#!` line that picks the interface language. Everything else uses built-in defaults.

```jsonc
#!日本語
```

Accepted values: `English`, `British English`, `한국어`, `日本語`, `简体中文`, `繁體中文`, `français canadien`, `france française`, `polska`, `deutsch`, `nederlands`, `español`, `русский`, `украинский`, `lol cat`. Anything else falls back to English.

Plugins are described in [LUA_DOCS.md](LUA_DOCS.md).

## Roadmap

See [ROADMAP.md](ROADMAP.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Author

João Pedro de Sena Santana (JP, DoctorXD), [@joaopssx](https://github.com/joaopssx).

## License

BSD 2-Clause. See [LICENSE](LICENSE).
