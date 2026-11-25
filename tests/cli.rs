//! Smoke tests for the command-line flags that exit before the TUI starts.

use std::process::Command;

fn editrs(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_editrs"))
        .args(args)
        .output()
        .expect("failed to run the editrs binary")
}

#[test]
fn version_shows_name_version_and_author() {
    let out = editrs(&["--version"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        stdout.trim(),
        concat!(
            "editrs ",
            env!("CARGO_PKG_VERSION"),
            " by ",
            env!("CARGO_PKG_AUTHORS")
        )
    );
}

#[test]
fn help_lists_every_flag() {
    let out = editrs(&["--help"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Usage: editrs [OPTIONS]"));
    for flag in [
        "--help, -h",
        "--version, -v",
        "--admin, -a",
        "--wait, -w",
        "--question, -q",
    ] {
        assert!(stdout.contains(flag), "missing {flag} in --help output");
    }
}

#[test]
fn unknown_short_flag_is_rejected() {
    let out = editrs(&["-x"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Unknown flag 'x'"));
}
