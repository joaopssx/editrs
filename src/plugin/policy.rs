use crate::plugin::action::PluginAction;
use mlua::{
    Lua, Result,
    Value::{self, Nil},
};

pub fn apply_restrictions(
    lua: &Lua,
    tx: crossbeam_channel::Sender<crate::plugin::action::PluginAction>,
    policy: &serde_json::Value,
) -> Result<()> {
    // "networking": false,
    // "read stdout": false,
    // "io operations": true,
    // "limited io operations": true,
    // "unlimited io operations": false,
    // "stdin access": false,
    // "stdout access": false,
    // "limited io locations": [],
    // "make threads": false,

    let globals = lua.globals();

    // Each permission is a boolean in the plugin's policy file.
    // A missing or non-boolean entry means the permission is not granted.
    let allowed =
        |key: &str| -> bool { policy.get(key).and_then(|v| v.as_bool()).unwrap_or(false) };

    // Included in ALL_SAFE; This is something unsafe for us
    globals.set("package", Nil)?;
    globals.set("debug", Nil)?;
    globals.set("loadfile", Nil)?;
    globals.set("collectgarbage", Nil)?;
    globals.set("_VERSION", Nil)?;
    globals.set("require", Nil)?; // TODO: Make this so we can only load some lua files, excluding luac files
    globals.set("warn", Nil)?;

    // File, process and stream access. Plugins read files through our own
    // `open` global instead (see `global_set.rs`).
    if !allowed("unlimited io access") {
        globals.set("dofile", Nil)?;

        if let Ok(io) = globals.get::<mlua::Table>("io") {
            for name in [
                "tmpfile", "stderr", "flush", "input", "output", "read", "write", "open", "lines",
                "close", "type", "popen",
            ] {
                io.set(name, Nil)?;
            }
            if !allowed("stdout access") {
                io.set("stdout", Nil)?;
            }
            if !allowed("stdin access") {
                io.set("stdin", Nil)?;
            }
        }

        if let Ok(os) = globals.get::<mlua::Table>("os") {
            for name in [
                "execute",
                "remove",
                "rename",
                "tmpname",
                "exit",
                "setlocale",
            ] {
                os.set(name, Nil)?;
            }
        }
    }
    globals.set("assert", Nil)?;
    globals.set("rawset", Nil)?;
    globals.set("getmetatable", Nil)?;
    globals.set("setmetatable", Nil)?;
    globals.set("arg", Nil)?;

    if !allowed("make threads") {
        globals.set("coroutine", Nil)?;
    }

    // Consider: rawlen, rawequals
    // arg     table
    // xpcall  function
    // pcall   function
    // error   function
    // select  function
    // ipairs  function

    // Printing shifts up the screen, which we *DON'T* want
    // Instead, we offer printing, but contained :)

    // Redirects Lua `print()` function to a debug console
    // This is apart of the policy, since print must go to the debug console, and going elsewhere is not accepted.
    let debug_print_fn = lua.create_function(move |_, args: mlua::Variadic<mlua::Value>| {
        let msg = args
            .iter()
            .map(|v| match v {
                Value::String(s) => s.to_str().ok().as_deref().unwrap_or("").to_string(),
                Value::Nil => "nil".to_string(),
                Value::Boolean(b) => b.to_string(),
                Value::Integer(i) => i.to_string(),
                Value::Number(n) => n.to_string(),
                any => format!("{:?}", any),
            })
            .collect::<Vec<_>>()
            .join(" ");

        let _ = tx.send(PluginAction::DebugLog { message: msg });

        Ok(())
    })?;

    globals.set("print", debug_print_fn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::apply_restrictions;
    use crate::plugin::action::PluginAction;
    use mlua::Lua;

    fn sandbox(policy: serde_json::Value) -> (Lua, crossbeam_channel::Receiver<PluginAction>) {
        let (tx, rx) = crossbeam_channel::unbounded();
        let lua = Lua::new();
        apply_restrictions(&lua, tx, &policy).expect("restrictions apply");
        (lua, rx)
    }

    fn is_nil(lua: &Lua, expr: &str) -> bool {
        lua.load(format!("return {expr} == nil"))
            .eval()
            .expect("expression runs")
    }

    #[test]
    fn empty_policy_removes_file_process_and_thread_access() {
        let (lua, _rx) = sandbox(serde_json::json!({}));
        for expr in [
            "package",
            "debug",
            "require",
            "dofile",
            "io.open",
            "io.popen",
            "io.write",
            "io.stdout",
            "io.stdin",
            "os.execute",
            "os.remove",
            "os.exit",
            "coroutine",
        ] {
            assert!(is_nil(&lua, expr), "{expr} should be removed");
        }
        // Harmless parts of the standard library stay.
        assert!(!is_nil(&lua, "string.format"));
        assert!(!is_nil(&lua, "os.time"));
    }

    #[test]
    fn granted_permissions_keep_their_functions() {
        let (lua, _rx) = sandbox(serde_json::json!({
            "unlimited io access": true,
            "make threads": true,
        }));
        for expr in ["io.open", "os.execute", "dofile", "coroutine"] {
            assert!(!is_nil(&lua, expr), "{expr} should be kept");
        }
    }

    #[test]
    fn stdout_permission_is_separate_from_full_io() {
        let (lua, _rx) = sandbox(serde_json::json!({ "stdout access": true }));
        assert!(!is_nil(&lua, "io.stdout"));
        assert!(is_nil(&lua, "io.stdin"));
        assert!(is_nil(&lua, "io.open"));
    }

    #[test]
    fn print_goes_to_the_debug_console() {
        let (lua, rx) = sandbox(serde_json::json!({}));
        lua.load(r#"print("hi", 1, true, nil)"#)
            .exec()
            .expect("print runs");
        match rx.try_recv() {
            Ok(PluginAction::DebugLog { message }) => assert_eq!(message, "hi 1 true nil"),
            _ => panic!("expected a DebugLog message"),
        }
    }
}
