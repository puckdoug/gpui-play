# Gestures (Mouse Pressure & Pinch)

## Goal

Document `MousePressureEvent`, `PressureStage` (force touch), and `PinchEvent` (macOS pinch-to-zoom).

## Design

A canvas that responds to pressure and pinch gestures. Force touch varies the brush size/opacity. Pinch-to-zoom scales the canvas content.

### Data Model

```rust
struct GestureDemo {
    zoom_level: f32,
    pressure_log: Vec<PressureEntry>,
    brush_marks: Vec<BrushMark>,
}

struct PressureEntry {
    stage: PressureStage,
    pressure: f32,
}

struct BrushMark {
    position: (f32, f32),
    size: f32,     // derived from pressure
    opacity: f32,  // derived from pressure
}
```

### Key Concepts

- `MousePressureEvent` fires on force-touch trackpads with pressure value (0.0–1.0+)
- `PressureStage` indicates force touch level (increase, decrease, stages)
- `PinchEvent` fires on two-finger pinch with scale delta
- Both are macOS-specific (may be no-ops on other platforms)

## View Layer (src/bin/gesture_test.rs)

- Drawing canvas: click to place dots, pressure affects dot size
- Pressure gauge: visual bar showing current force touch pressure
- Zoom area: content that scales on pinch gesture
- Pressure stage indicator: shows current PressureStage
- Log area: recent pressure and pinch events

## TDD Tests

### Pressure (3)
1. MousePressureEvent contains pressure value in valid range
2. PressureStage transitions are reported
3. Pressure value maps to brush size correctly (pure logic)

### Pinch (3)
4. PinchEvent contains scale delta
5. Zoom level updates correctly from pinch delta
6. Zoom clamps to min/max bounds

### Note on testability
Pressure and pinch events are difficult to simulate programmatically. Pure state logic (pressure→size mapping, pinch→zoom calculation) should be tested. Event handling is best verified manually.

## Documentation (docs/gpui-usage/gestures.md)

### Sections
1. **What it is** — pressure-sensitive and multi-touch gesture events (macOS)
2. **Preconditions** — `use gpui::{MousePressureEvent, PressureStage, PinchEvent}`; macOS with Force Touch trackpad for pressure; macOS trackpad for pinch; events are on div/canvas elements
3. **Signatures** — `.on_mouse_pressure(|event, cx| {})`, `MousePressureEvent { pressure, stage }`, `.on_pinch(|event, cx| {})`, `PinchEvent { delta }`
4. **Relevant types** — `MousePressureEvent`, `PressureStage`, `PinchEvent`
5. **Usage examples** — pressure-sensitive drawing, pinch-to-zoom
6. **Post-conditions** — no cleanup; events stop when element is removed from tree
7. **Testing** — pure logic testable; gesture simulation not available in test harness
8. **Surprises** — macOS only (no-op or absent on Linux/Windows); pressure values can exceed 1.0 on deep press; PressureStage may not map to discrete haptic clicks; pinch delta is multiplicative not additive; pinch and scroll can fire simultaneously; not all trackpads support force touch
