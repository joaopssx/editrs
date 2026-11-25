# Benchmarks

## Startup time

`scripts/bench-startup.sh` times how long editrs takes to start and exit, using [hyperfine](https://github.com/sharkdp/hyperfine). It measures `editrs --version` and `editrs --help`, which exit before the TUI opens; the TUI itself needs an interactive terminal and is not measured here.

### Running it

Install hyperfine (`cargo install hyperfine`, or your package manager), then from the repository root:

```sh
scripts/bench-startup.sh                          # builds the release binary, then measures it
scripts/bench-startup.sh path/to/editrs out.md    # measures an existing binary
```

On Windows, run it from Git Bash or WSL. Results are printed and saved as a Markdown table (default `target/bench-startup.md`).

### Reading the output

For each command hyperfine prints the mean time with its standard deviation (σ) and the range (min … max). Depending on the hyperfine version it also shows user/system CPU time (1.x) or peak memory (2.x).

- A large σ means a noisy machine; close other programs and run again.
- The minimum is the most stable number to compare between builds.

Compare two builds on the same machine, right after each other. Differences under about 5% are usually noise.

The CI runs this script on every pull request (`.github/workflows/bench.yml`) and shows the table in the job summary. GitHub runners are shared machines, so use those numbers to spot large regressions, not small changes.

## Baseline

| Date | Build | Machine | `--version` mean | Release binary |
|---|---|---|---|---|
| 2026-11-25 | 0.1.0 upstream tree, Windows, `Measure-Command` | Ryzen 9 7950X | 9.0 ms (median) | 9,276,416 bytes |
| 2026-11-25 | 0.2.0-dev (`perf/001`), Linux (WSL2), hyperfine 2.0, 50 runs | Ryzen 9 7950X | 1.1 ms ± 0.1 ms | 10,694,456 bytes |
