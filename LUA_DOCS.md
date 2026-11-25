# Lua plugins

editrs runs plugins written in Lua 5.5 (vendored through the `mlua` crate). Each plugin runs on its own worker thread and talks to the editor by sending messages, so calls that read editor state wait until the editor answers.

> Status: the API below is read from the source code (`src/plugin/`). The examples have not yet been run end to end in an interactive session, and several functions have known bugs listed at the end. Treat this API as experimental.

## Where plugins live

```
<config folder>/
  config            <- must exist, otherwise no plugin is loaded
  config.jsonc
  plugins/
    my-plugin/
      config        <- required (its content is passed as the plugin policy)
      src/
        main.lua    <- the plugin code
```

The config folder is `%APPDATA%\editrs\` on Windows and `$USERPROFILE/.config/editrs/` elsewhere. Every folder inside `plugins/` is one plugin. A plugin whose `config` file or `src/main.lua` is missing is skipped. An empty `main.lua` is skipped too.

## Lifecycle

1. The script is executed once (top-level code runs).
2. If it defines a global `init()`, it is called once.
3. If it defines a global `run()`, it is called again and again in a loop for as long as editrs is open. Keep it short.

Each plugin has a memory limit of 10 MB.

## Removed globals

The loader tries to set these standard globals to `nil` before the plugin runs (see the known issues below, this does not fully work yet): `package`, `debug`, `loadfile`, `collectgarbage`, `_VERSION`, `require`, `warn`, `assert`, `rawset`, `getmetatable`, `setmetatable`, `arg`.

## API

### `print(...)`

Sends the arguments, joined by spaces, to the debug console (open it with F12 or Ctrl+E). Nothing is written to the terminal.

### `key()`

Returns the last key pressed in the editor as a string, or `nil` if editrs is not focused or no key was pressed since the last call.

### `open(path)`

Opens a file for reading and returns a handle with these methods:

| Method | Returns |
|---|---|
| `f:read(n)` | Up to `n` bytes as a string; without `n`, the rest of the file |
| `f:read_to_end()` | The rest of the file |
| `f:read_line()` | The next line, including its newline |
| `f:seek(pos)` | Moves to byte `pos` from the start |
| `f:tell()` | Current byte position |
| `f:close()` | Closes the handle |

### `settings`

| Function | Description |
|---|---|
| `settings.add(name, value)` | Adds a setting |
| `settings.set(name, value)` | Changes a setting |
| `settings.get(name)` | Returns the value, or `nil` |

Values can be any Lua value that converts to JSON (numbers, strings, booleans, tables).

### `cursor`

| Function | Description |
|---|---|
| `cursor.x.get()` / `cursor.x.set(x)` | Column of the cursor |
| `cursor.y.get()` / `cursor.y.set(y)` | Line of the cursor |
| `cursor.pos.get()` / `cursor.pos.set({x, y})` | Both at once |

### `editor`

Positions are tables. Unless noted, indexes start at 0.

| Function | Description |
|---|---|
| `editor.get.line(n)` | Text of line `n` |
| `editor.get.str(from, to)` | Text between two positions |
| `editor.get.char(pos)` | Character at `pos` |
| `editor.get.cursor()` | Character under the cursor |
| `editor.set.char(pos, c)` | Replaces the character at `pos` |
| `editor.set.str(from, to, text)` | Replaces the text between two positions |
| `editor.set.cursor(c)` | Replaces the character under the cursor |
| `editor.insert.cursor(text)` | Inserts text at the cursor |
| `editor.insert.str(text)` | Same as `editor.insert.cursor` |
| `editor.insert.char(pos, text)` | Inserts text at `pos` |
| `editor.insert.line(n, text)` | Replaces line `n` with `text` (despite the name) |

## Example

```lua
-- plugins/hello/src/main.lua
function init()
  print("hello plugin loaded")
  settings.add("hello.greeting", "Hi from editrs")
end

function run()
  local k = key()
  if k == "F9" then
    editor.insert.cursor(settings.get("hello.greeting"))
  end
end
```

## Known issues

These are bugs in the current code, tracked for 0.2.0:

- The plugin `config` file is passed as a plain string, so none of the policy options are actually read.
- Some `io.*` removals use keys such as `"io.open"` as global names, which does not remove `io.open`. The standard libraries are also loaded after the removals, which can bring removed globals back.
- `editor.get.char`, `editor.get.str` and `editor.get.cursor` use inverted or byte-based bounds checks and can return wrong results or crash editrs.
- `editor.insert.cursor` uses the cursor column as the line index and the line as the column.
- `run()` is called in a busy loop, so an idle plugin still uses CPU.
