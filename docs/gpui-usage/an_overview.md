# GPUI Overview

GPUI is a GPU-accelerated, retained-mode UI framework from the [Zed editor](https://github.com/zed-industries/zed). It renders via Metal (macOS), Vulkan, or DirectX — it does **not** use native OS controls. This means you build everything yourself: text inputs, buttons, lists, etc.

## Architecture

A GPUI application has three layers:

1. **App** (`gpui::App`) — the application context. Manages global state, key bindings, menus, action handlers, and window lifecycle.
2. **Window** (`gpui::Window`) — a native OS window. Each window has a root view and its own element tree. Multiple windows are supported.
3. **Views and Elements** — views implement `Render` to produce an element tree. Elements are laid out via Taffy (flexbox/grid) and painted to GPU.

```
App
├── Global state, menus, key bindings, action handlers
├── Window 1
│   └── Root View (impl Render)
│       └── Element tree (div, text, canvas, custom elements...)
├── Window 2
│   └── Root View
│       └── ...
```

## Application Lifecycle

```rust
use gpui_platform::application;

application().run(|cx: &mut App| {
    cx.activate(true);                    // Bring app to foreground
    cx.bind_keys(key_bindings());         // Register keyboard shortcuts
    cx.set_menus(menus());                // Set menu bar (after bind_keys!)
    cx.on_action(|_: &Quit, cx| cx.quit());  // Register action handlers
    cx.open_window(opts, |_, cx| {        // Open window with root view
        cx.new(|cx| MyView::new(cx))
    }).unwrap();
});
```

**Critical ordering:** `bind_keys` must come before `set_menus` for keyboard shortcuts to display in menus.

## Key Concepts

### Actions

Actions are the primary communication mechanism. They are unit structs (or structs with data) that flow through the element tree via keyboard shortcuts, menu items, or programmatic dispatch.

```rust
actions!(my_app, [Quit, Save, Copy, Paste]);
```

Actions are registered as handlers at the app level (`cx.on_action`) or view level (`.on_action(cx.listener(...))`), and bound to keystrokes via `cx.bind_keys`.

### Views and Render

Every interactive component is a view — a struct that implements `Render`. Views are wrapped in `Entity<V>` for shared ownership and reactive updates.

```rust
struct MyView { /* state */ }

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().size_full().child("Hello")
    }
}
```

Call `cx.notify()` after state changes to trigger a re-render.

### Elements and Styling

Elements use a builder pattern inspired by Tailwind CSS:

```rust
div()
    .flex()
    .flex_col()
    .bg(rgb(0xffffff))
    .size_full()
    .p_4()
    .gap_4()
    .text_xl()
    .text_color(rgb(0x000000))
    .child("text content")
    .child(other_element)
```

Layout is powered by Taffy (flexbox and CSS grid).

### Focus

Keyboard input routes to the focused element. Views that accept keyboard input must implement `Focusable` and call `.track_focus()` in their render method.

```rust
impl Focusable for MyView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```

### Models and Shared State

`Model<T>` provides shared, observable state across views. `Global` trait provides app-wide singletons. Both trigger reactive updates when modified.

## Component Index

Detailed documentation for each component:

- **[App](app.md)** — application lifecycle, global state, action handling, async, platform integration
- **[Window](window.md)** — creating and configuring native windows, traffic light buttons, WindowOptions
- **[Menus](menus.md)** — application menu bar, menu items, actions, keyboard shortcuts
- **[Text Input](text-input.md)** — editable text fields, EntityInputHandler, custom elements, IME support, undo/redo
- **[Button](button.md)** — clickable elements, hover/active states, cursor styling (pattern, not a widget)

## Essential Dependencies

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_platform = { git = "https://github.com/zed-industries/zed", features = ["font-kit"] }

[dev-dependencies]
gpui = { git = "https://github.com/zed-industries/zed", features = ["test-support"] }
```

**`font-kit` is required** on `gpui_platform` for text to render. Without it, all text is invisible.

## Testing

GPUI provides `#[gpui::test]` macro and `TestAppContext` for testing. Key limitations:

- **Menu and keymap state** is not accessible on the test platform. Extract data into pure functions and test directly.
- **Windows** can be opened in tests but are headless. Use `VisualTestContext` for action dispatch.
- **Text input state** should be separated from GPUI rendering (as `TextInputState`) to enable pure unit testing of buffer operations.

Pattern: keep testable logic in pure structs, wrap with GPUI rendering in a separate layer.

## What GPUI Does Not Provide

- No built-in text input widget — you implement ~400 lines of `EntityInputHandler` + custom `Element`
- No native undo/redo — NSUndoManager is bypassed since GPUI doesn't use native controls
- No theme system — colors are explicit, no automatic dark/light mode
- No built-in buttons, dropdowns, checkboxes, etc. — build from `div()` with click handlers and styling
- Sparse documentation — refer to Zed source code and examples for patterns
