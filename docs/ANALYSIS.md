# editrs 0.1.0 codebase analysis

Written while preparing 0.1.0, as the starting point for 0.2.0. Line numbers refer to the 0.1.0 tree and will drift.

## 1. Architecture

Single binary crate, no library target. Edition 2024, global allocator `mimalloc`.

| Module | Responsibility |
|---|---|
| `main.rs` | Argument parsing, privilege escalation (`--admin`), config folder creation, plugin loading, terminal setup/teardown |
| `app/mod.rs` | `App` state (one large struct), main loop `App::run`, key handling, `dispatch(Action)` for every action, search, Git refresh, saving |
| `app/ui.rs` | Top-level layout and drawing (editor, sidebar, tabs, status bar, terminal, debug console) |
| `app/langs.rs` | Interface languages and translated strings (`WorldStrings`), `get_trans!` macro |
| `app/marketplace.rs` | Marketplace plugin record (`title`, `desc`) |
| `components/` | ratatui drawing: `editor/view.rs`, `layout/{sidebar,status,tabs}.rs`, `modals/view.rs` |
| `config/` | Default settings (`config::default()`), unused key-combo decoder (`keybind.rs`), translated setting names (`langs.rs`), `defaults.jsonc` (not loaded anywhere) |
| `core/editor.rs` | Text buffer (`Vec<CompactString>`, one entry per line), cursor, selection, clipboard, file loading and language detection by extension |
| `core/terminal.rs` | Built-in terminal on `portable-pty` with its own small ANSI parser |
| `core/git.rs` | Git status and diffs by running the `git` CLI |
| `core/tree.rs` | File tree for the explorer (uses `ignore` to respect `.gitignore`) |
| `input/` | `Action` enum, key to action mapping (`keymap.rs`), caps lock detection per OS |
| `plugin/` | Lua runtime: loader (`plugin_main.rs`), sandbox policy (`policy.rs`), globals (`global_set.rs`, `lua_integrate/*`), file handle (`lua_io.rs`), messages to the app (`action.rs`) |
| `windows/modal.rs` | Modal dialog state (search, replace, prompts, help, palette, settings) |
| `markdown/` | Not compiled (no `mod markdown` in `main.rs`); placeholder |
| `build.rs` | Generates `--help` text from the README "Usage" block; downloads and embeds the 0xProto Nerd Font license |

Data flow:

1. `main` parses flags, creates the config folder, starts Lua workers (rayon pool) if plugins exist, then `ratatui::init()`.
2. `App::run` loops until `should_quit`: drains plugin messages from a crossbeam channel (`PluginAction`), redraws, then polls input for 10 ms.
3. Key events go through `keymap::map_key` to an `Action`, which `App::dispatch` applies to state.
4. Plugin calls that read state send a message with a `Condvar` responder and block their worker thread until the main loop answers.

## 2. Dependencies

Versions are the ones resolved in `Cargo.lock`.

| Crate | Version | License | Used for | Used in code |
|---|---|---|---|---|
| aho-corasick | 1.1.5 | Unlicense OR MIT | Project search | yes |
| arboard | 3.7.0 | MIT OR Apache-2.0 | Clipboard (Linux, Windows, macOS) | yes |
| arc-swap | 1.9.5 | MIT OR Apache-2.0 | Highlight cache swapping | yes |
| bumpalo | 3.20.4 | MIT OR Apache-2.0 | Arena allocation | **no** |
| compact_str | 0.9.1 | MIT | Small-string optimized lines | yes |
| crossbeam-channel | 0.5.17 | MIT OR Apache-2.0 | Plugin to app messages | yes |
| crossterm | 0.29.0 | MIT | Terminal input | yes |
| evdev | 0.13.2 | Apache-2.0 OR MIT | Linux input (caps lock) | **no** |
| git2 | 0.20.4 | MIT OR Apache-2.0 | Git (BSDs only) | **no** |
| gix | 0.83.0 | MIT OR Apache-2.0 | Git (Linux, Windows, macOS) | **no** (git CLI is used) |
| glob | 0.3.4 | MIT OR Apache-2.0 | Caps lock LED lookup (Linux) | yes |
| hashbrown | 0.17.2 | MIT OR Apache-2.0 | Hash maps | yes |
| ignore | 0.4.35 | Unlicense OR MIT | `.gitignore`-aware file walking | yes |
| indexmap | 2.14.2 | Apache-2.0 OR MIT | Ordered config map | yes |
| mimalloc | 0.1.52 | MIT | Global allocator | yes |
| mlua | 0.11.6 | MIT | Lua 5.5 (vendored) | yes |
| modular-bitfield | 0.13.1 | MIT OR Apache-2.0 | Key combo bitfield | yes (dead code) |
| online | 4.0.2 | MIT | Connectivity check | yes |
| parking_lot | 0.12.5 | MIT OR Apache-2.0 | Mutex, RwLock, Condvar | yes |
| pgp | 0.19.0 | MIT OR Apache-2.0 | Marketplace signature verification | **no** |
| portable-pty | 0.9.0 | MIT | Built-in terminal | yes |
| pulldown-cmark | 0.13.6 | MIT | Markdown parsing | **no** |
| ratatui | 0.30.2 | MIT | Rendering | yes |
| rayon | 1.12.0 | MIT OR Apache-2.0 | Thread pool (plugins, marketplace, search) | yes |
| reqwest | 0.13.5 | MIT OR Apache-2.0 | Marketplace HTTP | yes |
| serde / serde_json | 1.0.229 / 1.0.154 | MIT OR Apache-2.0 | Config values, Lua bridge | yes |
| smol_str | 0.2.2 | MIT OR Apache-2.0 | Small strings | **no** |
| syntect | 5.3.0 | MIT | Syntax highlighting | yes |
| termimad | 0.34.1 | MIT | Markdown rendering | only mentioned, not called |
| tokio | 1.54.1 | MIT | Async runtime | **no** direct use |
| triomphe | 0.1.17 | MIT OR Apache-2.0 | `Arc` without weak counts | yes |
| vt100 | 0.16.2 | MIT | Terminal emulation | **no** (own parser) |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | Caps lock (Windows) | yes |
| ureq, tar, xz2 (build) | 3.5.2, 0.4.46, 0.1.7 | MIT OR Apache-2.0 | Font download in `build.rs` | yes |

Removing the unused crates (especially `gix`, `pgp`, `tokio`, `git2`) should cut build time noticeably.

Third-party assets: the 0xProto Nerd Font license (SIL OFL 1.1) is downloaded at build time and embedded; `--license=ofl|all` prints it. Only the license text is embedded today (`FONT_BYTES` in `font.rs` is generated but never included). The MIT/Apache dependencies require their notices in binary distributions; nothing collects them yet (consider `cargo about` or `cargo-bundle-licenses` before shipping release binaries).

## 3. User-facing strings

Translated (through `get_trans!`):

- `app/langs.rs`: 18 `WorldStrings` keys (commands, marketplace errors, sidebar titles) in 15 languages. German and some others are partial.
- `config/langs.rs`: 21 `SettingsStrings` keys for setting names; most languages only fill 12 to 16 of them, and `build_settings_map` is not used for rendering yet.

Hardcoded English (about 60 literals):

| File | Approx. count | Content |
|---|---|---|
| `components/modals/view.rs` | 42 | Help screen, prompts (unsaved changes, quit, delete), button labels |
| `components/editor/view.rs` | 11 | Empty state, line info |
| `components/layout/sidebar.rs` | 5 | Section labels |
| `app/ui.rs` | 2 | Terminal and debug console titles |
| `components/layout/tabs.rs` | 1 | Tab label |
| `main.rs` | 6 | CLI errors and `Continue? [Y/n]` |
| `README.md` (via `build.rs`) | 1 block | `--help` text |

Recommendation for 0.2.0: replace the per-language `HashMap` built at startup with resource files (for example Fluent `.ftl` via `fluent-bundle`, or a compile-time `phf` map per locale), one key per string, English as fallback. Move the hardcoded literals above to keys first; that is mechanical. Language selection via the `#!` first line of `config.jsonc` can stay, but should move into the parsed config. Brazilian Portuguese is not present yet.

## 4. Likely performance bottlenecks

From reading the code (not profiled yet):

1. **Main loop spawns a task every iteration** (`App::run`): the marketplace fetch was scheduled roughly every 10 ms, each doing a connectivity check and an HTTP request. 0.1.0 disables it (no URL), but the design must change to a one-shot fetch.
2. **Full redraw almost every frame**: `if self.dirty || self.focused` redraws whenever the window is focused, so the whole UI is drawn about 100 times per second even when idle.
3. **`self.translations.clone()` every iteration** (a `HashMap` clone per loop, now inside the disabled branch).
4. **Caps lock detection per key press** (`keymap::map_key`): on Linux this globs `/sys/class/leds/*` and reads files for every key.
5. **Text buffer is `Vec<CompactString>`**: inserting or deleting lines is O(n) in the number of lines, and saving does `lines.join("\n")`, allocating the whole file. A rope (`ropey`) or piece table fits large files better.
6. **Synchronous file loading** with `read_to_string` and a per-line copy into `CompactString`.
7. **Git view runs `git diff` once per changed file** on every refresh, synchronously.
8. **Plugin `run()` busy loop** with a 500 µs sleep, one thread per plugin.
9. **Syntax and theme sets are loaded twice** (`App::new` and again in `Editor::new` via `ThemeSet::load_defaults()` per editor).
10. **Release profile**: `lto = "fat"` and `codegen-units = 1` are set; `panic = "abort"` and `strip = true` are not.

## 5. Configuration and Lua

Configuration:

- Folder: `%APPDATA%\editrs` on Windows, `$XDG_CONFIG_HOME/editrs` or `$HOME/.config/editrs` elsewhere (`config::config_dir`). Before 0.2.0 it used `$USERPROFILE` outside Windows, which broke startup on Linux and macOS.
- `config.jsonc` is created empty. Only its first line is read, as `#!<language name>`.
- Every other setting comes from `config::default()` in code. `config/defaults.jsonc` describes a richer nested format but is never loaded.
- Plugins can add and change settings at runtime (`settings.add/set`), stored in the same `IndexMap`.

Lua (details in `LUA_DOCS.md`):

- Plugins load only if `<config>/config` exists and `<config>/plugins/` is not empty. Each plugin folder needs `config` and `src/main.lua`.
- Globals exposed: `print` (to the debug console), `key`, `open`, `settings`, `cursor`, `editor`.
- Lifecycle: run script, call `init()` once, call `run()` forever.
- Sandbox: fixed in 0.2.0 (policy parsed as JSON, permissions checked the right way, `io`/`os` entries removed from their tables, standard libraries loaded once before the restrictions). Still open: the `open()` global ignores the policy.

## 6. Known limitations and technical debt

Bugs noted, not fixed in 0.1.0 (0.1.0 is identity only):

- `input/caps_lock.rs` only detects caps lock on Linux and Windows; elsewhere it reports "off". `App::new` hits `unreachable!` on OSes other than Windows, macOS, Linux and FreeBSD.
- `-v` prints the version but does not exit (`--version` does).
- A path argument that does not exist panics (`canonicalize().unwrap()`).
- Lua API bugs: inverted bounds in `GetCharAt` and `GetStrAt`, byte indexing in `GetCharAtCursor` (panics on non-ASCII), swapped x/y in `InsertStrAtCursor`, `editor.insert.str` ignores its position, `editor.insert.line` replaces instead of inserting, responders are reused across calls without being reset.
- Saving drops CRLF line endings and the trailing newline (`lines()` then `join("\n")`).
- `build.rs` assumes the README "Usage" header is followed by exactly 13 bytes (`## Usage\n` + backticks + newline); with CRLF checkouts the help gains a leading blank line. It also panics without internet on a clean build.
- Help screen lists `Ctrl+W` as close tab while `config::default()` labels `Ctrl+W` as quit; keybinds in `config` are not used (hardcoded in `keymap.rs`).
- `tree.rs::delete_file` deletes from disk with errors ignored.
- `App` is a single struct with about 50 public fields and a 900-line `dispatch`.
- No unit tests; only CLI smoke tests in `tests/cli.rs`.

Changes made in 0.1.0 that touch behavior (all small):

- Marketplace fetch disabled (`MARKETPLACE_URL = None`), because the URL belonged to the previous project.
- `--version` output includes the author.
- `british english` and `english` match arms lowercased (they could never match; clippy deny lint).
- Clippy machine fixes and a few manual equivalents; no intended behavior change.

## 7. Performance baseline

Measured on Windows 11, AMD Ryzen 9 7950X (32 threads), Rust 1.99.1, on the unmodified upstream tree before any change. `hyperfine` was not available; startup was measured with PowerShell `Measure-Command` over 20 runs.

| Metric | Value |
|---|---|
| `cargo check` (clean, deps included) | 28.3 s |
| `cargo build` (debug, after check) | 34.0 s |
| `cargo build --release` (fat LTO, 1 CGU) | 1 min 30 s |
| Debug binary size | 20,593,152 bytes |
| Release binary size | 9,276,416 bytes |
| `--version` startup, release | min 8.3 ms, median 9.0 ms |
| Compiler warnings (rustc) | 7 |
| Clippy | 1 error (deny lint), about 95 warnings |
| Tests | 0 |

Build times include downloading the font on the first build.
