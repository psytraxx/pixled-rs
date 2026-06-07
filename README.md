# pixled-rs

A Rust application for pixel processing and camera/image handling using `crossterm`, `image`, and `nokhwa`.

## Development

- After making changes, add a changelog entry to `CHANGELOG.md`.
- Run formatting and linting before committing:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`

## Build and Run

```bash
cargo build --release
cargo run --release
```

## Project files

- `Cargo.toml` - Rust package metadata and dependencies
- `src/` - Rust source code
- `CHANGELOG.md` - keep a history of updates and changes
- `CLAUDE.md` - guidance for assistant and workflow notes
