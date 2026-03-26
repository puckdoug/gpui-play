## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Run example binary:** `cargo run --bin <name>`
- **Check (fast compile check):** `cargo check`
- **Run tests:** `cargo test`
- **Run a single test:** `cargo test <test_name>`
- **Run integration tests only:** `cargo test --test <test_file>`
- **Clippy lint:** `cargo clippy`
- **Search codebase:** `rg <pattern> [path]` — ripgrep, recurses directories by default
  - `-i` case-insensitive, `-w` whole word, `-F` fixed string (no regex)
  - `-t rust` filter by file type, `-g '*.rs'` filter by glob
  - `-l` list matching files only, `-c` count matches per file
  - `-A N` / `-B N` / `-C N` context lines after/before/around
  - `-n` line numbers (default), `-U` multiline matching
  - `--sort path` sort output by path, `-o` only matching text
  - Respects .gitignore by default; use `-u` to search ignored files
