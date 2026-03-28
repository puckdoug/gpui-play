# Display & Appearance

**Components:** [`WindowAppearance`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs), [`PlatformDisplay`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs)

## What is the component and what it does

GPUI provides APIs to query connected displays (resolution, scale factor, bounds) and the system appearance (light/dark mode). These enable adaptive UI — positioning windows on specific displays and theming based on system preferences.

## Preconditions for use

```rust
use gpui::WindowAppearance;
```

- Display and appearance queries are on `App` (via `cx.displays()`, `cx.window_appearance()`)
- Display info is a snapshot — no callback for display connect/disconnect
- Appearance may not update immediately when the user toggles system dark mode

## Signature for usage

### Display queries

```rust
cx.displays() -> Vec<Rc<dyn PlatformDisplay>>   // All connected displays
cx.primary_display() -> Option<Rc<dyn PlatformDisplay>>  // Main display
```

### WindowAppearance

```rust
pub enum WindowAppearance {
    Light,          // Standard light mode
    VibrantLight,   // Light with vibrancy effects
    Dark,           // Standard dark mode
    VibrantDark,    // Dark with vibrancy effects
}

cx.window_appearance() -> WindowAppearance
```

## Usage and examples

### Adaptive theming

```rust
let appearance = cx.window_appearance();
let (bg, text) = match appearance {
    WindowAppearance::Dark | WindowAppearance::VibrantDark => {
        (gpui::rgb(0x1e1e1e), gpui::white())
    }
    WindowAppearance::Light | WindowAppearance::VibrantLight => {
        (gpui::white(), gpui::rgb(0x1e1e1e))
    }
};

div().bg(bg).text_color(text).child("Adaptive content")
```

### Listing displays

```rust
let displays = cx.displays();
for display in &displays {
    // PlatformDisplay provides bounds, scale factor, etc.
}

if let Some(primary) = cx.primary_display() {
    // Position window on primary display
}
```

## Post-conditions / destruction requirements

- Display info is a snapshot — query again for updated info
- No cleanup needed

## Testing

```rust
#[gpui::test]
fn test_appearance(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let appearance = cx.window_appearance();
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight
            | WindowAppearance::Dark | WindowAppearance::VibrantDark => {}
        }
    });
}
```

Run tests: `cargo test --test display_appearance_test`

## Surprises, Anti-patterns, and Bugs

### No callback for display changes

There is no `on_display_change()` listener. If monitors are connected/disconnected, you must poll `cx.displays()`.

### VibrantLight/VibrantDark may not differ in practice

The vibrant variants indicate the OS supports vibrancy effects, but your GPUI app may not visually differ between vibrant and standard modes unless you explicitly use vibrancy.

### Scale factor matters for pixel-perfect rendering

Display scale factor (1x, 2x Retina, etc.) affects pixel density. Use logical pixels (`Pixels`) throughout — GPUI handles the physical pixel conversion.

### `window_appearance()` may not update immediately

When the user toggles system dark mode, the appearance value may lag. There is no built-in callback for appearance changes — Zed handles this through platform-specific observers.
