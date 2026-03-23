# Window

**Components:** [`Window`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs#L910), [`WindowOptions`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1215), [`WindowBounds`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1315), [`WindowKind`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1389), [`TitlebarOptions`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1375), [`WindowHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs#L5228), [`WindowBackgroundAppearance`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1441), [`FocusHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs#L334), [`Focusable`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs#L510), [`TabStopMap`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/tab_stop.rs#L11)

## What is the component and what it does

GPUI windows are native OS windows created via `cx.open_window()`. Each window hosts a root view (implementing `Render`) that defines its content. Windows are configured through `WindowOptions` which controls size, position, titlebar, traffic light buttons (close/minimize/zoom), resizability, and window kind.

On macOS, the traffic light buttons (red/yellow/green) are controlled by:

- **Close (red):** Always enabled when a titlebar is present. Cannot be disabled.
- **Minimize (yellow):** Controlled by `is_minimizable`.
- **Zoom/fullscreen (green):** Disabled when `is_resizable` is false.

## Signature for usage

### Opening a window

```rust
cx.open_window(
    options: WindowOptions,
    build_root_view: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
) -> anyhow::Result<WindowHandle<V>>
```

### WindowOptions

```rust
WindowOptions {
    window_bounds: Option<WindowBounds>,             // Size and position
    titlebar: Option<TitlebarOptions>,               // Title, transparency, traffic light pos
    focus: bool,                                     // Focus on create (default: true)
    show: bool,                                      // Show on create (default: true)
    kind: WindowKind,                                // Normal, PopUp, Floating, Dialog
    is_movable: bool,                                // User can drag (default: true)
    is_resizable: bool,                              // User can resize (default: true)
    is_minimizable: bool,                            // User can minimize (default: true)
    display_id: Option<DisplayId>,                   // Which display
    window_background: WindowBackgroundAppearance,   // Background style
    app_id: Option<String>,                          // App identifier
    window_min_size: Option<Size<Pixels>>,            // Minimum size
    ..
}
```

### TitlebarOptions

```rust
TitlebarOptions {
    title: Option<SharedString>,                     // Window title
    appears_transparent: bool,                       // Hide system titlebar chrome
    traffic_light_position: Option<Point<Pixels>>,   // Custom traffic light position
}
```

### WindowBounds

```rust
// Explicit position and size
WindowBounds::Windowed(Bounds<Pixels>)

// Centered on a display
Bounds::centered(display_id: Option<DisplayId>, size: Size<Pixels>, cx: &App) -> Bounds<Pixels>
```

### WindowKind

```rust
WindowKind::Normal    // Standard window
WindowKind::PopUp     // Above other windows
WindowKind::Floating  // Float on top of parent
WindowKind::Dialog    // Modal dialog
```

### Window control methods

```rust
window.remove_window()       // Close the window
window.minimize_window()     // Minimize
window.zoom_window()         // Toggle zoom (green button)
window.toggle_fullscreen()   // Toggle fullscreen
window.is_fullscreen() -> bool
window.set_title(title: &str)
window.focus_next(cx)        // Move focus to next tab stop
window.focus_prev(cx)        // Move focus to previous tab stop
window.focus(&handle, cx)    // Focus a specific element
```

### FocusHandle (tab navigation)

```rust
// Create a focus handle that participates in tab navigation
cx.focus_handle().tab_stop(true)

// Set tab order (lower index = earlier in tab order)
cx.focus_handle().tab_stop(true).tab_index(0)
```

## Relevant Macros

None specific to windows. Views use `actions!()` for action handling and `#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]` for custom actions.

## Relevant Traits

### `Render`

Every window root view must implement `Render`:

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // ... element tree
    }
}
```

### `Focusable`

Views that participate in focus management (including tab navigation) must implement `Focusable`:

```rust
impl Focusable for MyView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```

The root view and any child views that should be focusable must implement this trait and call `.track_focus()` in their render method.

## Usage and examples

### Opening a centered window

```rust
let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        ..Default::default()
    },
    |_, cx| cx.new(|_| MyView),
)
.unwrap();
```

### Opening multiple windows via menu action

GPUI supports multiple windows. Each call to `cx.open_window()` creates an independent native window with its own root view and focus state. A common pattern is to extract window creation into a reusable function and trigger it from a menu action:

```rust
actions!(my_app, [NewWindow]);

fn open_main_window(cx: &mut App) {
    let window = cx
        .open_window(WindowOptions::default(), |_, cx| {
            let input1 = cx.new(|cx| TextInput::new(cx, "", "Field 1..."));
            let input2 = cx.new(|cx| TextInput::new(cx, "", "Field 2..."));
            cx.new(|cx| MyView {
                focus_handle: cx.focus_handle(),
                input1,
                input2,
            })
        })
        .unwrap();

    // Focus the first input in the new window
    window
        .update(cx, |view, window, cx| {
            window.focus(&view.input1.focus_handle(cx), cx);
        })
        .unwrap();
}

// In main:
cx.on_action(|_: &NewWindow, cx: &mut App| {
    open_main_window(cx);
});
```

Each window is fully independent — its own view tree, focus state, and tab stops. The menu bar is shared across all windows (it's app-level, not window-level). Closing one window does not affect others.

With `QuitMode::Default` on macOS (`QuitMode::Explicit`), the app keeps running even when all windows are closed. Use ⌘N to open a new window from the menu bar.

### Closing the active window programmatically

To close a window, call `window.remove_window()`. This must be done at the **view level** (inside the window's own update cycle), not at the app level. The `remove_window()` call sets a `removed` flag that is checked when the window's update cycle completes — if called from a nested `window.update()` at the app level, the flag is set on a different borrow and the removal trail never fires.

The working pattern is to handle the `CloseWindow` action in the root view:

```rust
actions!(my_app, [CloseWindow]);

impl MyView {
    fn close_window(&mut self, _: &CloseWindow, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::close_window))
            // ...
    }
}

// Key binding at app level:
cx.bind_keys([KeyBinding::new("cmd-w", CloseWindow, None)]);
```

The action and key binding are defined in the library (for menu display), but the handler is on the view so it runs within the window's update cycle.

**Every window that should respond to ⌘W must have its root view handle `CloseWindow`.** This includes dialog windows like About. The view must implement `Focusable`, call `.track_focus()`, register the `.on_action()` handler, and — critically — the focus handle must be explicitly focused after the window opens:

```rust
if let Ok(window) = cx.open_window(opts, |_, cx| {
    cx.new(|cx| AboutView {
        focus_handle: cx.focus_handle(),
        version,
    })
}) {
    window
        .update(cx, |view, window, cx| {
            window.focus(&view.focus_handle, cx);
        })
        .ok();
}
```

Without the explicit `window.focus()` call, the view's element tree never receives keyboard actions — even though the window itself has platform-level focus.

### About dialog: close-only window (minimize and zoom disabled)

```rust
let bounds = Bounds::centered(None, size(px(300.), px(150.)), cx);
cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        is_minimizable: false,
        is_resizable: false,
        ..Default::default()
    },
    |_, cx| cx.new(|_| AboutView { version }),
)
.ok();
```

### Opening a window from an action handler

```rust
cx.on_action(|_: &ShowAbout, cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(300.), px(150.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            is_minimizable: false,
            is_resizable: false,
            ..Default::default()
        },
        |_, cx| cx.new(|_| AboutView { version: "1.0".into() }),
    )
    .ok();
});
```

### Rendering text in a window

Text must be placed in a child element with an explicit text size. A bare `.child("text")` on a container div will not render visible text.

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(gpui::white())
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_xl()
                    .text_color(gpui::black())
                    .child("Hello, world!"),
            )
    }
}
```

### Tab navigation between focusable elements

GPUI has a built-in tab stop system, but Tab/Shift-Tab navigation is **not automatic**. You must:

1. Mark focusable elements with `.tab_stop(true)` on their `FocusHandle`
2. Define `FocusNext`/`FocusPrev` actions
3. Bind Tab and Shift-Tab to those actions
4. Handle the actions by calling `window.focus_next(cx)` / `window.focus_prev(cx)`

```rust
actions!(my_app, [FocusNext, FocusPrev]);

struct MyView {
    focus_handle: FocusHandle,
    input1: Entity<TextInput>,
    input2: Entity<TextInput>,
}

impl MyView {
    fn focus_next(&mut self, _: &FocusNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn focus_prev(&mut self, _: &FocusPrev, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
    }
}

impl Focusable for MyView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_prev))
            .child(self.input1.clone())
            .child(self.input2.clone())
    }
}

// In main:
cx.bind_keys([
    KeyBinding::new("tab", FocusNext, None),
    KeyBinding::new("shift-tab", FocusPrev, None),
]);
```

Child elements (like `TextInput`) must create their focus handle with `.tab_stop(true)`:

```rust
let focus_handle = cx.focus_handle().tab_stop(true);
```

The `TabStopMap` determines navigation order based on `tab_index` values and insertion order. Elements with lower `tab_index` come first; elements with equal `tab_index` follow insertion order.

### Getting the package version at compile time

Use `env!("CARGO_PKG_VERSION")` to embed the version from Cargo.toml:

```rust
let version = format!("MyApp: {}", env!("CARGO_PKG_VERSION"));
```

## Surprises, Anti-patterns, and Bugs

### `gpui_platform` requires `font-kit` feature for text rendering

By default, `gpui_platform` does **not** enable font loading. Without the `font-kit` feature, all text is invisible — windows render but show no text at all. This is the most critical gotcha.

```toml
# Cargo.toml — font-kit is REQUIRED for visible text
gpui_platform = { git = "https://github.com/zed-industries/zed", features = ["font-kit"] }
```

The `gpui` crate has `font-kit` in its default features, but `gpui_platform` does not. You must enable it explicitly.

### Close button cannot be disabled

On macOS, `WindowOptions` has no field to disable the close (red) button. It is always present when a titlebar exists. `is_minimizable` and `is_resizable` control the yellow and green buttons respectively.

### Zoom button follows resizable

The green (zoom/fullscreen) button is disabled when `is_resizable: false`. There is no separate flag for it.

### `Bounds::centered` requires `&App` context

`Bounds::centered()` needs access to the App context to query display dimensions. This means window options that include centered bounds cannot be constructed as pure functions — the bounds must be set at open time.

### Tab navigation is not automatic

Despite having `FocusHandle::tab_stop(true)` and a `TabStopMap`, GPUI does **not** automatically handle Tab/Shift-Tab key events for focus navigation. You must explicitly bind Tab to an action that calls `window.focus_next(cx)` and Shift-Tab to `window.focus_prev(cx)`. The parent view must implement `Focusable`, call `.track_focus()`, and register the action handlers. Without all of these pieces, Tab does nothing.

### `remove_window()` must be called from within the window's own update cycle

Calling `window.remove_window()` from an app-level `cx.on_action()` handler via `active_window().update()` does **not** work — the window stays open. The `removed` flag is checked in the window update trail function, but a nested `update()` from the app level doesn't trigger the correct trail.

The fix: handle the close action at the **view level** using `.on_action(cx.listener(Self::close_window))` so that `window.remove_window()` is called on the `&mut Window` directly within the window's own update cycle. The red close button works because macOS calls the platform's close handler which runs in the correct context.

### New windows don't focus their view elements automatically

`WindowOptions { focus: true, .. }` brings the window to the platform foreground, but does **not** focus any element within the window's view tree. Without an explicit `window.focus(&handle, cx)` call after opening, keyboard actions (including ⌘W for close) will not reach the view's action handlers. This affects all windows — main windows, dialogs, About windows. Always focus the root view's handle after `open_window()`.

### Test platform `open_window` works but is headless

Windows can be opened in tests via `TestAppContext`, but rendering is not visual. Use `VisualTestContext::from_window()` to dispatch actions and simulate interaction.
