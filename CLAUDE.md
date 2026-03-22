# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust playground/experimentation project using [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), the GPU-accelerated UI framework from the Zed editor. Uses Rust edition 2024. GPUI is pulled directly from the Zed monorepo via git dependency.

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Check (fast compile check):** `cargo check`
- **Run tests:** `cargo test`
- **Run a single test:** `cargo test <test_name>`
- **Clippy lint:** `cargo clippy`

## GPUI Notes

- GPUI is a retained-mode, reactive UI framework built on Metal (macOS) / Vulkan / DirectX.
- Key concepts: `App`, `Window`, `View`, `Model`, `Element`, `Render` trait, `IntoElement`.
- Views implement the `Render` trait and use GPUI's element tree (divs, text, flex layout via Taffy).
- State management uses `Model<T>` (shared observable state) and `cx: &mut Context<Self>` for view contexts.
- Event handling uses `on_click`, `on_mouse_down`, etc. closures on elements.
- Styling uses a builder pattern: `div().flex().bg(black()).size_full()`.
- GPUI docs are sparse; refer to Zed source code for real-world usage patterns.
