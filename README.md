# statline

> Profile CSV datasets from the terminal and get column stats, nulls, uniques, and type insights without opening a notebook.

## Features

- CSV profiling from a single CLI command
- Parquet scan path for `.parquet` / `.pq` files
- Per-column summary: type, count, null count, null %, mean, min, max
- Full numeric stats: standard deviation, P25, P50, P75
- Unique counts for datasets up to 5,000,000 rows
- Heuristic column insights: `Identifier`, `Numeric`, `Categorical`, `Boolean/Categorical`, `Text`
- JSON output for scripts and CI checks
- Row sampling with `--sample <N>`
- Polars LazyFrame input scanning

## Quick Start

```bash
cargo run -- examples/Employee.csv
```

```bash
cargo run -- examples/Employee.csv --full --no-color
```

## Installation

Install from the Git repository:

```bash
cargo install --git https://github.com/a6ir/statline
```

Build from source:

```bash
git clone https://github.com/a6ir/statline
cd statline
./scripts/build.sh
./target/release/statline examples/Employee.csv
```

Build with a clean target directory:

```bash
./scripts/build.sh --clean
```

The build script runs a release build and prints:

- The binary output path
- Binary size
- Total build time

Install to your system (Linux helper script):

```bash
./scripts/install.sh
```

## Example Output

Command:

```bash
statline examples/Employee.csv --full --no-color
```

Output excerpt from the checked-in `examples/Employee.csv` fixture:

```text
Rows: 1000
Columns: 10

Column Summary:
╭───────────────────┬───────┬───────┬───────┬───────┬──────────┬──────────┬──────────┬──────────┬──────────┬───────────┬───────────┬────────╮
│ Column            ┆ DType ┆ Count ┆ Nulls ┆ Null% ┆ Mean     ┆ Std      ┆ Min      ┆ P25      ┆ P50      ┆ P75       ┆ Max       ┆ Unique │
╞═══════════════════╪═══════╪═══════╪═══════╪═══════╪══════════╪══════════╪══════════╪══════════╪══════════╪═══════════╪═══════════╪════════╡
│ Employee_ID       ┆ i64   ┆ 1000  ┆ 0     ┆ 0.00  ┆ 500.50   ┆ 288.82   ┆ 1.00     ┆ 250.75   ┆ 500.50   ┆ 750.25    ┆ 1000.00   ┆ 1000   │
│ Age               ┆ i64   ┆ 1000  ┆ 0     ┆ 0.00  ┆ 39.97    ┆ 11.03    ┆ 22.00    ┆ 30.00    ┆ 40.00    ┆ 49.00     ┆ 59.00     ┆ 38     │
│ Salary_USD        ┆ i64   ┆ 1000  ┆ 0     ┆ 0.00  ┆ 91975.44 ┆ 24843.98 ┆ 45912.00 ┆ 72453.00 ┆ 88191.00 ┆ 111767.75 ┆ 151281.00 ┆ 996    │
│ Performance_Score ┆ f64   ┆ 1000  ┆ 0     ┆ 0.00  ┆ 8.27     ┆ 1.01     ┆ 6.50     ┆ 7.41     ┆ 8.25     ┆ 9.16      ┆ 10.00     ┆ 335    │
╰───────────────────┴───────┴───────┴───────┴───────┴──────────┴──────────┴──────────┴──────────┴──────────┴───────────┴───────────┴────────╯

Column Insights:
  • Employee_ID: Identifier
  • Name: Text
  • Age: Numeric
  • Gender: Boolean/Categorical
  • Department: Categorical
  • Experience_Years: Numeric
  • Salary_USD: Numeric
  • Remote_Work: Boolean/Categorical
  • Performance_Score: Numeric
  • City: Categorical
```

JSON output is available with:

```bash
statline examples/Employee.csv --sample 5 --json
```

## Performance

Criterion artifacts in this repository benchmark CSV profiling on generated data with columns `id,value,group`.

| Rows | Mean |
|---:|---:|
| 10,000 | 4.67 ms |
| 50,000 | 7.36 ms |
| 100,000 | 10.64 ms |
| 1,000,000 | ~70 ms extrapolated, not measured |

The benchmark includes dataset scanning and profiling. The 1M value is a linear extrapolation from the committed 10k-100k Criterion results, not a committed benchmark artifact.

Run benchmarks locally:

```bash
cargo bench
```

## How It Works

- `statline` validates the input path and detects format by extension.
- CSV files are scanned with `LazyCsvReader`.
- Parquet files are scanned with `LazyFrame::scan_parquet`.
- `--sample <N>` applies a lazy `limit(N)` before profiling.
- Row count is computed with a lazy `len()` projection.
- Column profiles are built from a collected DataFrame.
- Numeric aggregations are planned through Polars expressions.
- Output is rendered as a terminal table or serialized as pretty JSON.

Data flow:

```text
scan CSV/Parquet -> optional sample -> row count -> column profiles -> insights -> table/JSON
```

Why use this instead of `pandas.describe()` or `polars.describe()`?

- It is CLI-first: useful in shells, CI jobs, Makefiles, and data handoff workflows.
- It needs no notebook, Python script, or dataframe setup.
- It includes nulls, uniques, and simple semantic column labels in one pass.
- It is built on Polars, so startup-to-summary time stays low for everyday datasets.

## CLI Options

```text
Usage: statline [OPTIONS] <INPUT>
```

| Option | Description |
|---|---|
| `<INPUT>` | Input dataset path: `.csv`, `.parquet`, or `.pq` |
| `--full` | Show std, percentiles, unique counts, and insights |
| `--json` | Emit pretty JSON |
| `--sample <N>` | Profile only the first `N` rows |
| `--no-color` | Disable ANSI colors in table output |
| `--no-insights` | Disable heuristic insights |
| `--corr` | Accepted by the CLI; correlation is currently wired but not implemented |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

## Insights

Insights are simple labels derived from dtype and unique count:

| Insight | Meaning |
|---|---|
| `Identifier` | Numeric column with unique values for every row |
| `Numeric` | Numeric column that is not classified as an identifier |
| `Boolean/Categorical` | String column with exactly 2 unique values |
| `Categorical` | String column with fewer than 20 unique values |
| `Text` | String column with 20 or more unique values |
| `Unknown` | Any dtype without a current rule |

## Roadmap

- Implement real numeric correlation output for `--corr`
- Fix the current Parquet profiling issue exposed by `examples/dataset.parquet`
- Add tests for CLI output, JSON schema, sampling, and unsupported formats
- Add committed 1M-row benchmark artifacts
- Improve insight rules for semantic IDs, dates, and numeric-looking strings

## Contributing

Contributions are welcome.

Useful checks before opening a PR:

```bash
cargo fmt
cargo test
cargo bench
```

Keep changes focused and include an example or test when behavior changes.

## License

Apache-2.0
