#!/usr/bin/env bash
# Measures editrs startup time with hyperfine.
#
# Usage: scripts/bench-startup.sh [path/to/editrs] [output.md]
#
# Without arguments it builds the release binary first. The TUI itself needs
# an interactive terminal, so this times the paths that exit right away:
# `--version` (argument parsing and process start) and `--help`.
set -euo pipefail

if ! command -v hyperfine >/dev/null 2>&1; then
    echo "error: hyperfine is not installed (https://github.com/sharkdp/hyperfine)" >&2
    exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
bin="${1:-}"
out="${2:-$root/target/bench-startup.md}"

if [ -z "$bin" ]; then
    cargo build --release --manifest-path "$root/Cargo.toml"
    bin="$root/target/release/editrs"
    [ -x "$bin" ] || bin="$bin.exe"
fi

if [ ! -x "$bin" ]; then
    echo "error: binary not found or not executable: $bin" >&2
    exit 1
fi

mkdir -p "$(dirname "$out")"
hyperfine \
    --warmup 5 \
    --runs 50 \
    --shell=none \
    --export-markdown "$out" \
    "$bin --version" \
    "$bin --help"

echo
echo "Results written to $out"
