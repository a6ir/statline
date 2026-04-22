# Contributing to statline

Thanks for your interest in improving statline.

## Development Setup

1. Install Rust (stable toolchain):
   - https://rustup.rs
2. Clone the repository and move into it.
3. Build the project:

```bash
cargo build
```

4. Run on a local dataset:

```bash
cargo run -- data.csv
```

## Common Commands

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo bench
```

## How to Add Features

1. Add or update CLI flags in `src/cli/mod.rs`.
2. Extend loading behavior in `src/io/mod.rs`.
3. Add profiling logic in `src/profiler/mod.rs`.
4. Add presentation changes in `src/output/mod.rs`.
5. Add tests/benchmarks that cover the change.
6. Update README examples when user-visible behavior changes.

## Code Style Expectations

- Follow idiomatic Rust and keep functions focused.
- Avoid panics in user-facing paths.
- Prefer `Result`-based error propagation with contextual errors.
- Keep startup and memory overhead low.
- Use `cargo fmt` and `cargo clippy` before opening a PR.

## Pull Request Guidelines

- Keep PRs focused and small enough to review.
- Include a clear problem statement and change summary.
- Attach benchmark notes for performance-sensitive changes.
- Ensure CI is green.
