# Platform Utilities

**Components:** Various APIs on [`App`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app.rs) and the [`Platform`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs) trait

## What is the component and what it does

A collection of platform integration APIs for system-level operations: keychain access, URL handling, keyboard layout detection, thermal state monitoring, dock menu customization, and file system operations.

## Preconditions for use

```rust
use gpui::ThermalState;
```

- Most APIs are on `App` (accessed via `cx`)
- Many are macOS-specific or have platform-dependent behavior
- Some require app entitlements for distribution (keychain, URL schemes)

## Signature for usage

### Keychain

```rust
cx.write_credentials(url: &str, username: &str, password: &[u8]) -> Task<Result<()>>
cx.read_credentials(url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>
cx.delete_credentials(url: &str) -> Task<Result<()>>
```

### File system

```rust
cx.open_with_system(path: &Path)    // Open file with default app
cx.reveal_path(path: &Path)          // Show in Finder/Explorer
```

### Keyboard layout

```rust
cx.keyboard_layout() -> &dyn PlatformKeyboardLayout
```

### Thermal state

```rust
cx.thermal_state() -> ThermalState

pub enum ThermalState {
    Nominal,    // Normal operation
    Fair,       // Slightly elevated temperature
    Serious,    // High temperature, may throttle
    Critical,   // Critical temperature, heavy throttling
}
```

### Dock menu (macOS)

```rust
cx.set_dock_menu(menus: Vec<MenuItem>)
```

### URL schemes

```rust
cx.register_url_scheme(scheme: &str) -> Task<anyhow::Result<()>>
cx.on_open_urls(callback: impl Fn(Vec<String>, &mut App) + 'static)
```

### Recent documents

```rust
cx.add_recent_document(path: &Path)
```

## Usage and examples

### Read/write keychain

```rust
// Store credentials
cx.write_credentials("https://api.example.com", "user@example.com", b"secret_token")
    .detach();

// Read credentials (in spawned task)
let task = cx.spawn(async move |async_cx| {
    let creds = async_cx.read_credentials("https://api.example.com").await;
    if let Ok(Some((username, password))) = creds {
        println!("User: {}, Pass: {} bytes", username, password.len());
    }
});
```

### Open file with system

```rust
cx.open_with_system(Path::new("/tmp/document.pdf"));  // Opens in Preview
cx.reveal_path(Path::new("/tmp/document.pdf"));        // Shows in Finder
```

### Monitor thermal state

```rust
let state = cx.thermal_state();
match state {
    ThermalState::Nominal | ThermalState::Fair => { /* normal operation */ }
    ThermalState::Serious | ThermalState::Critical => {
        // Reduce work — disable animations, lower frame rate
    }
}
```

## Post-conditions / destruction requirements

- Keychain entries persist across app launches
- URL scheme registration persists until another app claims it
- Dock menu persists until changed or app exits
- No explicit cleanup needed for most APIs

## Testing

Most platform APIs have limited testability. Test type existence and basic construction:

```rust
#[gpui::test]
fn test_thermal_state(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let state = cx.thermal_state();
        match state {
            ThermalState::Nominal | ThermalState::Fair
            | ThermalState::Serious | ThermalState::Critical => {}
        }
    });
}
```

Run tests: `cargo test --test platform_utils_test`

## Surprises, Anti-patterns, and Bugs

### Keychain may prompt for permission

First access to the keychain on macOS may show a system permission dialog. For signed/notarized apps, this requires entitlements.

### `open_with_system` is fire-and-forget

There is no return value or callback for `open_with_system()`. You cannot know if the file was opened successfully.

### `reveal_path` is macOS Finder-specific

On macOS, this opens Finder and selects the file. On other platforms, behavior may differ or be a no-op.

### `thermal_state()` is macOS-specific

On non-macOS platforms, this may always return `Nominal`. Use it for adaptive performance (disable animations under thermal pressure) but don't depend on it for correctness.

### URL scheme registration may conflict

If another app has registered the same URL scheme, `register_url_scheme` may fail silently. Only one app can own a scheme.

### `on_open_urls` only fires for already-running apps

If the app is launched via a URL, the URLs are handled differently (via the app launch mechanism). `on_open_urls` fires when the app is already running and receives a URL activation.

### `set_dock_menu` replaces the menu entirely

Each call to `set_dock_menu` replaces the previous menu. There is no append API.
