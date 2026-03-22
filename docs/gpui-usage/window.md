# window

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

### Getting the package version at compile time

Use `env!("CARGO_PKG_VERSION")` to embed the version from Cargo.toml:

```rust
let version = format!("MyApp: {}", env!("CARGO_PKG_VERSION"));
```

## Surprises, Anti-patterns

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

### macOS app menu name comes from the binary name

The bold application name in the macOS menu bar is the process/binary name, not anything set via `WindowOptions` or `TitlebarOptions`. Control it with `[[bin]] name = "MyApp"` in Cargo.toml.

### Test platform `open_window` works but is headless

Windows can be opened in tests via `TestAppContext`, but rendering is not visual. Use `VisualTestContext::from_window()` to dispatch actions and simulate interaction.
