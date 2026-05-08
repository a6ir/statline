# statline

Fast CLI-first dataset profiling for CSV and Parquet, built with Rust + Polars.

`statline` gives you table summaries, insights, correlation, JSON output, and charting (terminal + PNG + HTML) without opening a notebook.

## Features

- CSV and Parquet (`.parquet`, `.pq`) support
- Column profiling: dtype, count, nulls, null %, mean/std/min/max, percentiles
- Correlation matrix (`--corr`) for numeric columns
- Heuristic column insights (`Identifier`, `Numeric`, `Categorical`, etc.)
- JSON output (`--json`) for scripts/CI
- Sampling (`--sample <N>`)
- Static chart export with Plotters (PNG assets)
- Minimal standalone HTML report export
- Inline terminal charts (text/Unicode bars, histograms, distributions)

## Quick Start

```bash
cargo run -- examples/Employee.csv --full --corr
```

With chart export + HTML:

```bash
cargo run -- examples/Employee.csv --full --corr --charts --html report/index.html
```

## Visuals

### 1) Inline terminal charts

Terminal charts are shown in full output mode (default enabled via `--terminal-charts`):

```bash
statline examples/Employee.csv --full --corr --terminal-charts
```

Example excerpt:

```text
Terminal Charts:
Numeric Distribution: Salary_USD
trend: ▁▃▆█▆▅▂▁
45912-59083   ███████
59083-72254   █████████████
72254-85425   █████████████████
...

Department
Management      ███████████████ 18.6%
Product         ██████████████  17.4%
Software        ██████████████  17.2%
```

### 2) Exported PNG charts

Generated with:

```bash
statline examples/Employee.csv --full --corr --charts
```

Assets are written to `report/assets/` by default:

- `*_histogram.png`
- `*_distribution.png`
- `nulls.png`
- `correlation_heatmap.png`

### 3) Standalone HTML report

```bash
statline examples/Employee.csv --full --corr --charts --html report/index.html
```

Default output layout:

```text
report/
├── index.html
└── assets/
    ├── Age_histogram.png
    ├── Department_distribution.png
    ├── nulls.png
    └── correlation_heatmap.png
```

## Installation

Install from Git:

```bash
cargo install --git https://github.com/a6ir/statline
```

Build locally:

```bash
git clone https://github.com/a6ir/statline
cd statline
./scripts/build.sh
./target/release/statline examples/Employee.csv --full
```

## CLI Options

```text
Usage: statline [OPTIONS] <INPUT>
```

| Option | Description |
|---|---|
| `<INPUT>` | Input dataset path (`.csv`, `.parquet`, `.pq`) |
| `--full` | Show full profile table + insights (+ terminal charts) |
| `--json` | Emit pretty JSON |
| `--sample <N>` | Profile first `N` rows |
| `--no-color` | Disable ANSI table colors |
| `--no-insights` | Disable heuristic insights |
| `--corr` | Compute numeric correlation matrix |
| `--charts` | Generate PNG chart assets |
| `--chart-dir <DIR>` | Directory for chart assets (default: `report/assets`) |
| `--html <FILE>` | Generate standalone HTML report file |
| `--terminal-charts` | Enable inline terminal charts |
| `--no-terminal-charts` | Disable inline terminal charts |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

## Example Commands

Basic table profile:

```bash
statline examples/Employee.csv
```

Full profile + correlation:

```bash
statline examples/Employee.csv --full --corr
```

JSON for automation:

```bash
statline examples/Employee.csv --sample 1000 --json
```

PNG charts only:

```bash
statline examples/Employee.csv --full --corr --charts
```

Disable terminal charts:

```bash
statline examples/Employee.csv --full --no-terminal-charts
```

## How It Works

Data flow:

```text
Input scan (CSV/Parquet)
  -> optional sampling
  -> profiler engine
  -> ProfileReport
  -> output table/json
  -> optional visualization export (PNG / HTML)
  -> optional terminal chart rendering (full mode)
```

Design principles:

- CLI-first, no GUI/web runtime
- Uses Polars lazy scanning for performance
- Keeps profiling and visualization layers separate

## Insights

Insights are heuristic labels derived from dtype + simple cardinality patterns:

- `Identifier`
- `Numeric`
- `Boolean/Categorical`
- `Categorical`
- `Text`
- `Unknown`

## Dev

Useful checks:

```bash
cargo fmt
cargo check
cargo test
cargo bench
```

## License

Apache-2.0