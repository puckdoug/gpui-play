# Gradients

## Goal

Document `linear_gradient()`, `LinearColorStop`, and `ColorSpace` (sRGB, Oklab).

## Design

A view showing gradient swatches: horizontal, vertical, diagonal, multi-stop, and color space comparison (same stops in sRGB vs Oklab).

### Key Concepts

- Gradients are applied as background fills on styled elements
- `LinearColorStop` defines color + position (0.0 to 1.0)
- `ColorSpace` affects interpolation — Oklab produces more perceptually uniform transitions
- Gradient angle controls direction

## View Layer (src/bin/gradient_test.rs)

- Row 1: horizontal gradients (left-to-right) with 2, 3, 5 stops
- Row 2: vertical and diagonal gradients
- Row 3: sRGB vs Oklab comparison — same red→blue gradient in both color spaces
- Each swatch labeled with configuration

## TDD Tests

### Gradient construction (3)
1. Linear gradient with two stops creates valid gradient
2. Multi-stop gradient accepts arbitrary number of stops
3. Gradient with angle creates correct direction

### Color space (2)
4. sRGB color space produces valid gradient
5. Oklab color space produces valid gradient

## Documentation (docs/gpui-usage/gradients.md)

### Sections
1. **What it is** — linear gradient backgrounds for styled elements
2. **Preconditions** — `use gpui::{linear_gradient, LinearColorStop, ColorSpace}`; element must have defined size (gradient fills the element bounds)
3. **Signatures** — `.background(linear_gradient(angle, stops, color_space))`, `LinearColorStop { color, position }`, `ColorSpace::Srgb`, `ColorSpace::Oklab`
4. **Relevant types** — `LinearColorStop`, `ColorSpace`
5. **Usage examples** — two-stop gradient, multi-stop, different angles, color space comparison
6. **Post-conditions** — no cleanup; gradient recomputed on resize
7. **Testing** — construction testable; visual correctness requires manual verification
8. **Surprises** — Oklab produces smoother transitions for most color pairs (especially complementary); stop positions must be 0.0–1.0; no radial or conic gradients; gradient may look banded at low resolution or narrow color ranges; angle 0 = bottom-to-top (CSS convention)

**Note:** Reference Zed's `crates/gpui/examples/gradient.rs`.
