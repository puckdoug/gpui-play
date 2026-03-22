## Project Structure

```
gpui-play/
  src/
    lib.rs              Library root, exports modules
    main.rs             Default binary
    bin/                Example binaries (cargo run --bin <name>)
  tests/                Integration tests for binaries
  docs/
    plans/              Implementation plans
    gpui-usage/         GPUI component usage documentation
```

- Production logic lives in `src/<module>.rs`, exported via `src/lib.rs`.
- Binary entry points in `src/bin/` import from the library crate.
- Integration tests in `tests/` test the complete binary behavior.
- Unit tests may live in the module itself (`#[cfg(test)] mod tests`).
