# App

## What is the component and what it does

`App` is the top-level application context in GPUI. It owns all entities (views, models), manages the global keymap, menu bar, action handlers, clipboard, and window lifecycle. Every GPUI application starts by creating an `Application` via `gpui_platform::application()` and calling `.run()` to enter the event loop.

The `App` context is passed as `&mut App` or `&App` to callbacks throughout the framework. It is the central hub through which all state flows.

## Signature for usage

### Application startup

```rust
use gpui_platform::application;

application()
    .run(|cx: &mut App| {
        // App is running — set up state, menus, windows
    });
```

### Builder methods (before `.run()`)

```rust
application()
    .with_assets(asset_source)           // Set asset source for loading resources
    .with_http_client(http_client)       // Set HTTP client
    .with_quit_mode(QuitMode::LastWindowClosed)  // Control quit behavior
    .run(|cx| { ... });
```

### QuitMode

```rust
QuitMode::Default          // Explicit on macOS, LastWindowClosed elsewhere
QuitMode::LastWindowClosed // Quit when last window closes
QuitMode::Explicit         // Only quit via cx.quit()
```

### App activation and control

```rust
cx.activate(true);       // Bring to foreground, ignoring other apps
cx.activate(false);      // Bring to foreground, respecting window order
cx.quit();               // Request graceful quit
cx.hide();               // Hide the application
cx.hide_other_apps();    // Hide other applications
cx.unhide_other_apps();  // Unhide other applications
```

### Window management

```rust
cx.open_window(options, build_root_view) -> Result<WindowHandle<V>>
cx.active_window() -> Option<AnyWindowHandle>
cx.window_stack() -> Option<Vec<AnyWindowHandle>>  // front-to-back order
```

### Global state

```rust
// Type-indexed global singletons
cx.set_global(MyState { ... });
cx.global::<MyState>() -> &MyState             // panics if not set
cx.global_mut::<MyState>() -> &mut MyState     // panics if not set
cx.try_global::<MyState>() -> Option<&MyState>
cx.has_global::<MyState>() -> bool
cx.default_global::<MyState>() -> &mut MyState // creates with Default if missing
cx.remove_global::<MyState>() -> MyState
cx.observe_global::<MyState>(callback) -> Subscription
```

Requires `impl Global for MyState {}`.

### Action handling

```rust
// Register global action handler (bubble phase, after local handlers)
cx.on_action(|action: &MyAction, cx: &mut App| { ... });

// Register key bindings
cx.bind_keys(vec![
    KeyBinding::new("cmd-q", Quit, None),
    KeyBinding::new("cmd-s", Save, Some("Editor")),  // context-scoped
]);
cx.clear_key_bindings();

// Programmatic dispatch
cx.dispatch_action(&MyAction);
cx.is_action_available(&MyAction) -> bool
```

### Observation and subscription

```rust
// Watch for entity changes (called when entity calls cx.notify())
cx.observe(&entity, |entity, cx| { ... }) -> Subscription

// Watch for typed events (called when entity calls cx.emit(event))
cx.subscribe(&entity, |entity, event, cx| { ... }) -> Subscription

// Watch creation of new views of a type
cx.observe_new::<MyView>(|view, window, cx| { ... }) -> Subscription

// Watch entity cleanup
cx.observe_release(&entity, |state, cx| { ... }) -> Subscription
```

### Keystroke observation

```rust
// After action dispatch (cannot prevent action)
cx.observe_keystrokes(|event, window, cx| { ... }) -> Subscription

// Before action dispatch (can call stop_propagation())
cx.intercept_keystrokes(|event, window, cx| { ... }) -> Subscription
```

### Clipboard

```rust
cx.read_from_clipboard() -> Option<ClipboardItem>
cx.write_to_clipboard(ClipboardItem::new_string("text".into()))
```

### Lifecycle hooks

```rust
cx.on_reopen(|cx| { ... });              // macOS dock icon click / app relaunch
cx.on_app_quit(|cx| async { ... });      // Before quit (100ms timeout)
cx.on_app_restart(|cx| { ... });         // Before restart
cx.on_window_closed(|cx| { ... });       // After a window closes
cx.on_open_urls(|urls: Vec<String>| { ... });  // URL scheme handler
```

### Async task execution

```rust
// Foreground (main thread)
cx.spawn(|cx: &mut AsyncApp| async move {
    let result = some_async_work().await;
    cx.update(|app| { /* modify state */ });
}) -> Task<R>

// Background thread
cx.background_spawn(async { heavy_computation() }) -> Task<R>

// Deferred (end of current effect cycle)
cx.defer(|cx| { ... });
```

### Platform queries

```rust
cx.displays() -> Vec<Rc<dyn PlatformDisplay>>
cx.primary_display() -> Option<Rc<dyn PlatformDisplay>>
cx.window_appearance() -> WindowAppearance    // light/dark mode
cx.keyboard_layout() -> &dyn PlatformKeyboardLayout
cx.on_keyboard_layout_change(|cx| { ... }) -> Subscription
```

### System integration

```rust
cx.open_url("https://example.com");
cx.reveal_path(&path);                        // Show in Finder
cx.prompt_for_paths(options) -> Receiver<...>  // File picker dialog
cx.prompt_for_new_path(dir, name) -> Receiver<...>  // Save dialog
cx.write_credentials(url, user, pw) -> Task<Result<()>>  // Keychain
cx.read_credentials(url) -> Task<Result<Option<(String, Vec<u8>)>>>
```

## Relevant Macros

### `actions!()`

Defines action types dispatched through the app's action system:

```rust
actions!(my_app, [Quit, Save, Open]);
```

## Relevant Traits

### `Global`

Marker trait for types stored as app-level singletons:

```rust
struct AppState { mode: Mode }
impl Global for AppState {}
```

### `EventEmitter<E>`

Enables entities to emit typed events that subscribers receive:

```rust
impl EventEmitter<MyEvent> for MyModel {}
```

### `Action`

All actions implement this trait. Generated by `actions!()` macro or derived manually for actions with data fields.

## Usage and examples

### Minimal application

```rust
use gpui::App;
use gpui_platform::application;

application().run(|cx: &mut App| {
    cx.activate(true);
    cx.open_window(Default::default(), |_, cx| cx.new(|_| MyView))
        .unwrap();
});
```

### Application with global state

```rust
use gpui::{App, Global};

struct AppConfig { dark_mode: bool }
impl Global for AppConfig {}

application().run(|cx: &mut App| {
    cx.set_global(AppConfig { dark_mode: false });
    cx.observe_global::<AppConfig>(|cx| {
        // React to config changes — e.g., refresh menus
    }).detach();
    // ...
});
```

### Auto-quit when last window closes

```rust
application()
    .with_quit_mode(gpui::QuitMode::LastWindowClosed)
    .run(|cx| { ... });
```

### AsyncApp for async operations

```rust
cx.spawn(|cx: &mut AsyncApp| async move {
    let data = fetch_data().await;
    cx.update(|app| {
        app.global_mut::<MyState>().data = data;
    });
});
```

## Surprises, Anti-patterns

### macOS app menu name comes from the binary name

The bold application name in the macOS menu bar is the **process/binary name**, not anything set via `WindowOptions`, `TitlebarOptions`, or `Menu::new()`. To control it, set the binary name in `Cargo.toml`:

```toml
[[bin]]
name = "MenuTest"
path = "src/bin/menu_test.rs"
```

### `bind_keys` must come before `set_menus`

Keyboard shortcuts display in menus only if the bindings are registered before `set_menus()` is called. See [menus.md](menus.md) for details.

### `on_app_quit` callbacks have a 100ms timeout

Quit callbacks must complete within 100ms or the app force-exits. Don't do heavy work in quit handlers.

### `activate(true)` vs `activate(false)`

`activate(true)` forcefully brings the app to the foreground even if another app has focus. `activate(false)` is polite — it respects the current window ordering. Use `true` at startup, `false` in most other cases.

### Globals are type-indexed

There can only be one global per type. If you need multiple instances of the same data, wrap them in distinct newtype structs.

### Effect batching

State changes are batched and flushed at effect cycle boundaries. `cx.notify()` schedules a re-render but doesn't happen immediately. `cx.defer()` runs at the next flush. This means reading state right after a mutation in the same callback sees the new state, but observers fire later.

### No native NSUndoManager integration

macOS `NSUndoManager` is bypassed because GPUI doesn't use native text controls. Undo/redo must be implemented in userspace. See [text-input.md](text-input.md).

### `QuitMode::Default` differs by platform

On macOS, default is `Explicit` (app stays running with no windows). On Linux/Windows, default is `LastWindowClosed`. Set it explicitly if you want consistent behavior.
