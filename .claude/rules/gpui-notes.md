## GPUI Notes

- GPUI is a retained-mode, reactive UI framework built on Metal (macOS) / Vulkan / DirectX.
- Key concepts: `App`, `Window`, `View`, `Model`, `Element`, `Render` trait, `IntoElement`.
- Views implement the `Render` trait and use GPUI's element tree (divs, text, flex layout via Taffy).
- State management uses `Model<T>` (shared observable state) and `cx: &mut Context<Self>` for view contexts.
- Event handling uses `on_click`, `on_mouse_down`, etc. closures on elements.
- Styling uses a builder pattern: `div().flex().bg(black()).size_full()`.
- GPUI docs are sparse; refer to Zed source code for real-world usage patterns.
- The GPUI test harness uses `#[gpui::test]` macro with `TestAppContext`. Enable via `features = ["test-support"]` in dev-dependencies.
