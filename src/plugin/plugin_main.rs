use crate::plugin::action::PluginAction;
use crossbeam_channel::Sender;
use mlua::{Lua, Result};
use rayon;
use std::{fs, path::PathBuf};

/// Reads a plugin's `config` file as its JSON permission policy.
///
/// An empty file means "no permissions". A file that is not a JSON object is
/// reported to the debug console and also treated as "no permissions".
fn parse_policy(text: &str, tx: &Sender<PluginAction>) -> serde_json::Value {
    let empty = || serde_json::Value::Object(Default::default());
    if text.trim().is_empty() {
        return empty();
    }
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(policy) if policy.is_object() => policy,
        Ok(_) | Err(_) => {
            let _ = tx.send(PluginAction::DebugLog {
                message: String::from(
                    "Plugin config is not a JSON object; running with no permissions",
                ),
            });
            empty()
        }
    }
}

/// Creates a Lua state with the plugin sandbox and editor API applied.
fn build_runtime(tx: &Sender<PluginAction>, policy: &serde_json::Value) -> Result<Lua> {
    // `Lua::new` already loads the safe standard libraries. They must not be
    // loaded again after `apply_restrictions`, or the removed globals return.
    let lua = Lua::new();
    crate::plugin::policy::apply_restrictions(&lua, tx.clone(), policy)?;
    crate::plugin::global_set::apply_globals(&lua, tx.clone())?;
    Ok(lua)
}

fn spawn_lua_worker(
    plugin_path: PathBuf,
    action_tx: crossbeam_channel::Sender<PluginAction>,
) -> Result<()> {
    let script = fs::read_to_string(PathBuf::from(&plugin_path).join("src").join("main.lua"))?;
    let plugin_policy = parse_policy(
        &fs::read_to_string(PathBuf::from(&plugin_path).join("config"))?,
        &action_tx,
    );
    if !script.is_empty() {
        rayon::spawn(move || {
            // Since an empty script is wasted work, we just do nothing with it
            let lua = match build_runtime(&action_tx, &plugin_policy) {
                Ok(lua) => lua,
                Err(e) => {
                    let _ = action_tx.send(PluginAction::DebugLog {
                        message: format!("Plugin sandbox error: {}", e),
                    });
                    return;
                }
            };

            // Clean up, to free anything leftover from setting the security policy,
            // Such as the following scenario...
            // `Table` is a pre-defined global that was included. Then we set it to Nil.
            // The contents of Table still exist, taking up memory.

            // Called twice as suggested by documentation
            let _ = lua.gc_collect();
            let _ = lua.gc_collect();

            // Set a 10MB limit
            // It's here, so that during startup, we don't somehow hit it, triggering an unnecesarry collection
            // Especially in the future
            if let Err(e) = lua.set_memory_limit(10 * 1024 * 1024) {
                let _ = action_tx.send(PluginAction::DebugLog {
                    message: format!("Memory limit error: {}", e),
                });
            }

            // Compile the script
            if let Err(e) = lua.load(&script).exec() {
                let _ = action_tx.send(PluginAction::DebugLog {
                    message: format!("Script Load Error: {}", e),
                });
                return; // Can't continue if the script is broken
            }

            // After compiling the script, the source is useless. clearing it frees a variable amount of memory
            drop(script);
            drop(plugin_policy);

            // Run init (if it exists)
            let plugin_init: Option<mlua::Function> = lua.globals().get("init").unwrap();
            if let Some(f) = plugin_init {
                let _ = f.call::<()>(());
            }

            let run_fn: Option<mlua::Function> = lua.globals().get("run").ok();
            loop {
                if let Some(ref f) = run_fn
                    && let Err(e) = f.call::<()>(())
                {
                    let _ = action_tx.send(PluginAction::DebugLog {
                        message: e.to_string(),
                    });
                }

                // Check if it did anything
                if action_tx.is_empty() {
                    // If it didn't, we have less reason to keep moving instantly
                    std::thread::sleep(std::time::Duration::from_micros(500));
                } else {
                    rayon::yield_now();
                }
            }
        });
    }
    Ok(())
}

pub fn load_plugins(
    plugin_path: PathBuf,
    tx: crossbeam_channel::Sender<PluginAction>,
) -> Result<()> {
    // We don't have to worry about it not existing, as we create it if it doesn't, and don't run this if it doesn't!
    let entries = std::fs::read_dir(&plugin_path).unwrap();

    for entry in entries.flatten() {
        let path = entry.path();

        // We only care about directories (each directory is a plugin)
        if path.is_dir() {
            let _ = spawn_lua_worker(path, tx.clone());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_runtime, parse_policy};

    #[test]
    fn policy_is_parsed_as_json() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let policy = parse_policy(r#"{ "make threads": true }"#, &tx);
        assert_eq!(policy["make threads"], serde_json::json!(true));
        assert!(rx.try_recv().is_err(), "valid policy logs nothing");
    }

    #[test]
    fn empty_or_invalid_policy_grants_nothing() {
        let (tx, rx) = crossbeam_channel::unbounded();
        assert_eq!(
            parse_policy(
                "  
", &tx
            ),
            serde_json::json!({})
        );
        assert!(rx.try_recv().is_err());
        for bad in ["not json", "[1, 2]", "true"] {
            assert_eq!(parse_policy(bad, &tx), serde_json::json!({}));
            assert!(rx.try_recv().is_ok(), "{bad} should be reported");
        }
    }

    #[test]
    fn runtime_keeps_restrictions_and_adds_editor_api() {
        let (tx, _rx) = crossbeam_channel::unbounded();
        let lua = build_runtime(&tx, &serde_json::json!({})).expect("runtime builds");
        let check = |expr: &str| -> bool {
            lua.load(format!("return {expr} == nil"))
                .eval()
                .expect("expression runs")
        };
        assert!(check("io.open"), "io.open must stay removed");
        assert!(check("package"), "package must stay removed");
        assert!(!check("editor"), "editor API is available");
        assert!(!check("open"), "open() replacement is available");
    }
}
