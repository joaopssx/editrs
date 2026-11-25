pub mod keybind;
pub mod langs;

use indexmap::IndexMap;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::path::PathBuf;

pub type Config = IndexMap<String, Value>;

/// Folder that holds `config.jsonc` and `plugins/`.
///
/// Windows: `%APPDATA%\editrs`. Elsewhere: `$XDG_CONFIG_HOME/editrs`, or
/// `$HOME/.config/editrs` when `XDG_CONFIG_HOME` is unset or not absolute.
/// Returns `None` when the needed environment variables are missing.
pub fn config_dir() -> Option<PathBuf> {
    resolve_config_dir(
        cfg!(windows),
        std::env::var_os("APPDATA"),
        std::env::var_os("XDG_CONFIG_HOME"),
        std::env::var_os("HOME"),
    )
}

fn resolve_config_dir(
    windows: bool,
    appdata: Option<OsString>,
    xdg_config_home: Option<OsString>,
    home: Option<OsString>,
) -> Option<PathBuf> {
    let non_empty = |v: Option<OsString>| v.filter(|s| !s.is_empty()).map(PathBuf::from);

    let base = if windows {
        non_empty(appdata)?
    } else {
        match non_empty(xdg_config_home).filter(|p| p.is_absolute()) {
            Some(xdg) => xdg,
            None => non_empty(home)?.join(".config"),
        }
    };
    Some(base.join(crate::APP_NAME))
}

pub fn default() -> Config {
    let default_json = json!({
        // The comments are the explanations to use on hover
        "Tab Size": 4, // Number of spaces to insert when pressing tab
        "Auto Save": false, // Whether to autosave
        "Time To Auto Save": 30_000, // How long between autosaves
        "Tab BG Color": "#A9A9A9", // The color of the tabs
        "Tab FG Color": "#FFFFFF", // Color of tab foreground
        "Active Tab FG Color": "#FFFFFF", // Color of the text of the current tab
        "Active Tab BG Color": "#EE7101", // Color of tab currently in
        "Inactive Sidebar BG Color": "#808080", // Color of sidebar when not active
        "Active Sidebar BG Color": "#EE7101", // Color of sidebar BG when active
        "Inactive Sidebar Text Color": "#FFFFFF", // Color of sidebar FG when inactive
        "Active Sidebar Text Color": "#EE7101", // Color of sidebar FG when active
        "Selected Text Font Color": "#000000", // Color of selected text
        "Highlighted Text BG": "#0084FF", // Color of selected text BG
        "Editor BG Color": "#323232", // Background color of entire editor
        "Highlighting Theme": "base16-ocean.dark", // Highlighting theme for languages
        "Search": "Ctrl+F", // Shortcut to search
        "Search Highlight Selected Color BG": "#FFFF00", // BG of current chosen item when searching (modal, !sidebar)
        "Search Highlight Unselected Color BG": "#00FFFF", // BG of any item not chosen when searching (modal, !sidebar)
        "Search Highlight Selected Font Color": "#000000", // Font color of selected search item
        "Search Highlight Unselected Font Color": "#FFFFFF", // Font color of unselected items when searching
        "Help": "Ctrl+K", // Shortcut to open help modal
        "Sidebar Toggle": "Ctrl+B", // Shortcut for opening or closing sidebar
        "Previous Sidebar Tab": "Ctrl+A", // Shortcut to go to the previous tab
        "Next Sidebar Tab": "Ctrl+D", // Shortcut to go to the next tab
        "Refresh Git": "Ctrl+G", // Shortcut to refresh git view
        "Close Tab": "Ctrl+A", // Shortcut to close the current tab
        "Quit": "Ctrl+W", // Shortcut to quit editrs
        "New File": "Ctrl+N", // Shortcut to open a new file
        "Save File": "Ctrl+S", // Shortcut to save current open file
        "Open Terminal": "Ctrl+T", // Shortcut to open the builtin terminal
        "Open Debug Console": "Ctrl+E", // Shortcut to open the plugin debug console
        "Debug Character": "λ", // Character put next to plugin print outputs
        "Debug Console": true, // Whether the debug console "exists" or not
    });

    // Convert the JSON into an object for IndexMap
    default_json
        .as_object()
        .unwrap()
        .clone()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::resolve_config_dir;
    use std::path::PathBuf;

    fn os(s: &str) -> Option<std::ffi::OsString> {
        Some(s.into())
    }

    #[test]
    fn windows_uses_appdata() {
        let dir = resolve_config_dir(true, os(r"C:\Users\me\AppData\Roaming"), None, None);
        assert_eq!(
            dir,
            Some(PathBuf::from(r"C:\Users\me\AppData\Roaming").join("editrs"))
        );
        assert_eq!(resolve_config_dir(true, None, None, os("/home/me")), None);
    }

    // "/home/me/cfg" is only an absolute path on Unix.
    #[cfg(unix)]
    #[test]
    fn unix_prefers_absolute_xdg_config_home() {
        let dir = resolve_config_dir(false, None, os("/home/me/cfg"), os("/home/me"));
        assert_eq!(dir, Some(PathBuf::from("/home/me/cfg/editrs")));
    }

    #[test]
    fn unix_falls_back_to_home_dot_config() {
        for xdg in [None, os(""), os("relative/dir")] {
            let dir = resolve_config_dir(false, None, xdg, os("/home/me"));
            assert_eq!(dir, Some(PathBuf::from("/home/me/.config/editrs")));
        }
        assert_eq!(resolve_config_dir(false, None, None, None), None);
    }
}
