# Gestures (Pressure & Pinch)

**Components:** [`MousePressureEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs), [`PressureStage`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs), [`PinchEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs)

## What is the component and what it does

GPUI supports pressure-sensitive input (Force Touch trackpads) and pinch-to-zoom gestures (multi-touch trackpads). These are primarily macOS features:

- **`MousePressureEvent`** — reports force/pressure level (0.0–1.0+) with stages (Zero, Normal, Force)
- **`PinchEvent`** — reports pinch gesture delta for zoom operations

## Preconditions for use

```rust
use gpui::{PressureStage, Modifiers};
// PinchEvent requires: #[cfg(any(target_os = "linux", target_os = "macos"))]
```

- Pressure events require a Force Touch trackpad (macOS)
- Pinch events require a multi-touch trackpad (macOS/Linux, not Windows)
- `on_pinch` is platform-gated: `#[cfg(any(target_os = "linux", target_os = "macos"))]`
- Events cannot be simulated in the test harness

## Signature for usage

### MousePressureEvent

```rust
pub struct MousePressureEvent {
    pub pressure: f32,            // 0.0 to 1.0+ (can exceed 1.0 on deep press)
    pub stage: PressureStage,     // Current pressure stage
    pub position: Point<Pixels>,  // Mouse position in window
    pub modifiers: Modifiers,     // Held modifier keys
}
```

### PressureStage

```rust
pub enum PressureStage {
    Zero,    // No pressure (default)
    Normal,  // Normal click pressure
    Force,   // High pressure (force click)
}
```

### PinchEvent

```rust
pub struct PinchEvent {
    pub position: Point<Pixels>,  // Pinch center position
    pub delta: f32,               // Zoom delta (positive = zoom in)
    pub modifiers: Modifiers,     // Held modifier keys
    pub phase: TouchPhase,        // Started, Moved, Ended
}
```

### TouchPhase

```rust
pub enum TouchPhase {
    Started,  // Gesture began
    Moved,    // Gesture in progress (default)
    Ended,    // Gesture finished
}
```

### Handler registration

```rust
// On Interactivity (imperative API)
interactivity.on_mouse_pressure(|event: &MousePressureEvent, window, cx| {
    let pressure = event.pressure;
    let stage = event.stage;
});

// Pinch (macOS/Linux only)
#[cfg(any(target_os = "linux", target_os = "macos"))]
interactivity.on_pinch(|event: &PinchEvent, window, cx| {
    let zoom_delta = event.delta;
});
```

## Usage and examples

### Pressure-sensitive drawing

```rust
fn pressure_to_brush_size(pressure: f32, min: f32, max: f32) -> f32 {
    min + pressure * (max - min)
}

// In event handler:
let size = pressure_to_brush_size(event.pressure, 1.0, 10.0);
// Draw with `size` at event.position
```

### Pinch-to-zoom

```rust
fn apply_pinch(zoom: f32, delta: f32, min: f32, max: f32) -> f32 {
    (zoom + delta).clamp(min, max)
}

// In event handler:
self.zoom_level = apply_pinch(self.zoom_level, event.delta, 0.1, 5.0);
```

### Force click detection

```rust
match event.stage {
    PressureStage::Zero => { /* released */ }
    PressureStage::Normal => { /* normal click */ }
    PressureStage::Force => { /* force click — show preview, etc. */ }
}
```

## Post-conditions / destruction requirements

- No cleanup needed — events stop when the element is removed from the tree
- Gesture state is not retained by the framework — track it yourself

## Testing

Gesture events cannot be simulated in the test harness. Test pure logic separately:

```rust
#[test]
fn test_pressure_mapping() {
    assert_eq!(pressure_to_brush_size(0.0, 1.0, 10.0), 1.0);
    assert_eq!(pressure_to_brush_size(0.5, 1.0, 10.0), 5.5);
    assert_eq!(pressure_to_brush_size(1.0, 1.0, 10.0), 10.0);
}

#[test]
fn test_pinch_zoom_clamping() {
    assert_eq!(apply_pinch(1.0, -2.0, 0.1, 5.0), 0.1);  // Clamped to min
    assert_eq!(apply_pinch(4.0, 2.0, 0.1, 5.0), 5.0);    // Clamped to max
}
```

Run tests: `cargo test --test gestures_test`

## Surprises, Anti-patterns, and Bugs

### macOS only (mostly)

Pressure events require Force Touch hardware (MacBook trackpads since 2015). Pinch events require multi-touch. On Windows, pinch is simulated as Ctrl+scroll wheel (not a `PinchEvent`).

### Pressure values can exceed 1.0

A deep force press can produce pressure values above 1.0. Clamp if your logic assumes 0.0–1.0 range.

### `PressureStage` may not map to haptic clicks

The stage transitions don't always correspond to the haptic feedback clicks you feel. The stages indicate pressure thresholds, not haptic events.

### Pinch delta is additive, not multiplicative

`PinchEvent.delta` is a delta value to add to zoom level, not a scale factor. Positive = zoom in, negative = zoom out. To use as a scale factor, compute `1.0 + delta`.

### Pinch and scroll can fire simultaneously

On trackpads, two-finger scroll and pinch gestures can overlap. Your event handlers may receive both `ScrollWheelEvent` and `PinchEvent` for the same gesture.

### Not all trackpads support force touch

External trackpads and older MacBook models may not support pressure sensing. Always provide fallback behavior for systems without Force Touch.

### `on_pinch` is conditionally compiled

The `on_pinch` method is behind `#[cfg(any(target_os = "linux", target_os = "macos"))]`. On Windows, it does not exist.
