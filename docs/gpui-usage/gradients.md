# Gradients

**Components:** [`linear_gradient`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/color.rs), [`LinearColorStop`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/color.rs), [`ColorSpace`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/color.rs)

## What is the component and what it does

GPUI supports linear gradient backgrounds on any styled element. Gradients are applied through the `.bg()` method and interpolate between two color stops at a specified angle. Color interpolation can be done in sRGB (default) or Oklab color space — Oklab produces more perceptually uniform transitions.

## Preconditions for use

```rust
use gpui::{linear_gradient, linear_color_stop, ColorSpace, Styled};
```

- Element must have defined bounds (size) for the gradient to fill
- `linear_gradient()` returns a `Background` which is accepted by `.bg()`
- Color stops use the `Hsla` color type

## Signature for usage

### Creating a gradient

```rust
linear_gradient(
    angle: f32,                           // Degrees: 0=bottom-to-top, 90=left-to-right, 180=top-to-bottom
    from: impl Into<LinearColorStop>,     // Start color + position
    to: impl Into<LinearColorStop>,       // End color + position
) -> Background
```

### Color stops

```rust
linear_color_stop(color: impl Into<Hsla>, percentage: f32) -> LinearColorStop

// percentage: 0.0 = start, 1.0 = end
```

### Color space

```rust
linear_gradient(angle, from, to)
    .color_space(ColorSpace::Oklab)  // Perceptually uniform interpolation
```

### ColorSpace variants

```rust
ColorSpace::Srgb   // Default — standard RGB interpolation
ColorSpace::Oklab   // Perceptually uniform — smoother transitions for most color pairs
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Styled` | Provides `.bg()` which accepts gradients |

## Usage and examples

### Vertical gradient (top to bottom)

```rust
div().size_full().bg(linear_gradient(
    180.0,
    linear_color_stop(gpui::red(), 0.0),
    linear_color_stop(gpui::blue(), 1.0),
))
```

### Horizontal gradient (left to right)

```rust
div().size_full().bg(linear_gradient(
    90.0,
    linear_color_stop(gpui::red(), 0.0),
    linear_color_stop(gpui::blue(), 1.0),
))
```

### Diagonal gradient

```rust
div().size_full().bg(linear_gradient(
    45.0,
    linear_color_stop(gpui::green(), 0.0),
    linear_color_stop(gpui::blue(), 1.0),
))
```

### sRGB vs Oklab comparison

```rust
// sRGB (default) — may show muddy middle tones for complementary colors
div().bg(linear_gradient(180.0,
    linear_color_stop(gpui::red(), 0.0),
    linear_color_stop(gpui::blue(), 1.0),
))

// Oklab — smoother perceptual transition
div().bg(linear_gradient(180.0,
    linear_color_stop(gpui::red(), 0.0),
    linear_color_stop(gpui::blue(), 1.0),
).color_space(ColorSpace::Oklab))
```

## Post-conditions / destruction requirements

- No cleanup needed — gradients are recomputed on resize
- Gradient fills the element bounds completely

## Testing

```rust
#[test]
fn test_color_stop() {
    let stop = linear_color_stop(gpui::red(), 0.5);
    assert_eq!(stop.percentage, 0.5);
}
```

Run tests: `cargo test --test gradients_test`

## Surprises, Anti-patterns, and Bugs

### Only two color stops

`linear_gradient()` takes exactly two stops (from, to). There is no built-in multi-stop gradient function. For multi-stop gradients, you would need to layer multiple gradient elements.

### Angle follows CSS convention

0° = bottom-to-top, 90° = left-to-right, 180° = top-to-bottom, 270° = right-to-left. This matches CSS `linear-gradient()` angle behavior.

### No radial or conic gradients

GPUI only supports linear gradients. Radial, conic, and repeating gradients are not available.

### Oklab produces better transitions for most color pairs

For complementary colors (red↔blue, green↔purple), sRGB interpolation passes through muddy middle tones. Oklab interpolation stays vibrant. Default to Oklab unless you specifically need sRGB behavior.

### Gradient may look banded at narrow widths

Very narrow gradient regions or subtle color differences may show visible banding due to 8-bit color quantization.
