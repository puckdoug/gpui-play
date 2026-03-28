# Transforms

**Components:** [`TransformationMatrix`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/scene.rs), [`Transformation`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/svg.rs) (SVG-specific)

## What is the component and what it does

`TransformationMatrix` is GPUI's low-level 2D affine transformation type. It represents a 2x2 rotation/scale matrix plus a translation vector, supporting rotate, scale, translate, and arbitrary composition.

**Important:** There is no `.transform()` style method on divs or general elements. Transforms are available in two contexts:
1. **SVG elements** — via `Transformation` (see [svg.md](svg.md))
2. **Canvas/custom painting** — via `TransformationMatrix` applied to sprites during paint
3. **Pure math** — `TransformationMatrix` can be used standalone for coordinate transforms

## Preconditions for use

```rust
use gpui::{TransformationMatrix, Radians, ScaledPixels};
use gpui::{point, px, size};
```

- Methods are on instances (`self`), not associated functions — start with `TransformationMatrix::unit()`
- Rotation uses `Radians`, not degrees or `Percentage`
- Translation uses `ScaledPixels`, not `Pixels`

## Signature for usage

### TransformationMatrix struct

```rust
pub struct TransformationMatrix {
    pub rotation_scale: [[f32; 2]; 2],  // 2x2 rotation/scale matrix (row-major)
    pub translation: [f32; 2],           // Translation vector
}
```

### Construction and chaining

```rust
// Identity (no-op)
TransformationMatrix::unit()

// Chain operations (all methods take self and return Self)
TransformationMatrix::unit()
    .translate(point(ScaledPixels(10.0), ScaledPixels(5.0)))
    .rotate(Radians(std::f32::consts::FRAC_PI_4))  // 45°
    .scale(size(2.0, 2.0))
```

### Individual operations

```rust
.translate(point: Point<ScaledPixels>) -> Self
.rotate(angle: Radians) -> Self
.scale(size: Size<f32>) -> Self
```

### Composition

```rust
// Combine two matrices: applies `other` first, then `self`
let composed = matrix_a.compose(matrix_b);
// Equivalent to: matrix_a(matrix_b(point))
```

### Apply to a point

```rust
let result: Point<Pixels> = matrix.apply(point(px(10.0), px(20.0)));
```

## Relevant Traits

None — `TransformationMatrix` is a standalone struct.

## Usage and examples

### Identity matrix

```rust
let m = TransformationMatrix::unit();
let p = point(px(10.0), px(20.0));
assert_eq!(m.apply(p), p); // No change
```

### Translation

```rust
let m = TransformationMatrix::unit()
    .translate(point(ScaledPixels(5.0), ScaledPixels(10.0)));
let result = m.apply(point(px(0.0), px(0.0)));
assert_eq!(result, point(px(5.0), px(10.0)));
```

### Scaling

```rust
let m = TransformationMatrix::unit().scale(size(2.0, 3.0));
let result = m.apply(point(px(10.0), px(10.0)));
assert_eq!(result, point(px(20.0), px(30.0)));
```

### Rotation (90° clockwise)

```rust
use std::f32::consts::FRAC_PI_2;
let m = TransformationMatrix::unit().rotate(Radians(FRAC_PI_2));
let result = m.apply(point(px(10.0), px(0.0)));
// ≈ (0, 10) — rotated 90° clockwise
```

### Composition order matters

```rust
let translate = TransformationMatrix::unit()
    .translate(point(ScaledPixels(10.0), ScaledPixels(0.0)));
let scale = TransformationMatrix::unit().scale(size(2.0, 2.0));

// compose(other) applies other first, then self
let a = translate.compose(scale);  // scale then translate
let b = scale.compose(translate);  // translate then scale

// Different results for the same input point
assert_ne!(a.apply(point(px(5.0), px(5.0))), b.apply(point(px(5.0), px(5.0))));
```

### SVG transforms (high-level API)

For SVG elements, use the `Transformation` type instead (see [svg.md](svg.md)):

```rust
use gpui::{svg, Transformation, percentage};

svg().path("icon.svg")
    .with_transformation(Transformation::rotate(percentage(0.25)))  // 90°
```

## Post-conditions / destruction requirements

- `TransformationMatrix` is `Copy` — no cleanup needed
- Transforms are visual only; they do not affect layout

## Testing

```rust
#[test]
fn test_scale() {
    let m = TransformationMatrix::unit().scale(size(2.0, 3.0));
    let result = m.apply(point(px(10.0), px(10.0)));
    assert_eq!(result.x, px(20.0));
    assert_eq!(result.y, px(30.0));
}
```

Run tests: `cargo test --test transforms_test`

## Surprises, Anti-patterns, and Bugs

### No `.transform()` style method on divs

Unlike CSS, you cannot apply transforms to div elements via a style method. Transforms are only available for SVG elements (via `Transformation`) and during canvas painting. This is a significant limitation compared to web CSS.

### Methods are instance methods, not associated functions

You must start with `TransformationMatrix::unit()` and chain:

```rust
// WRONG — these are not static methods
TransformationMatrix::translate(point(...))  // ERROR

// RIGHT — chain from unit()
TransformationMatrix::unit().translate(point(...))
```

### `compose(other)` applies `other` first

`a.compose(b)` means "apply b, then apply a". This is standard matrix multiplication order (right-to-left) but can be unintuitive.

### Translation uses `ScaledPixels`, not `Pixels`

The `translate()` method takes `Point<ScaledPixels>`, not `Point<Pixels>`. Use `ScaledPixels(value)` to construct.

### Rotation uses `Radians`, not degrees

`rotate()` takes `Radians`, not degrees or `Percentage`. Use `Radians(std::f32::consts::FRAC_PI_2)` for 90°. This differs from SVG's `Transformation::rotate()` which takes `Percentage`.

### Transforms do not affect layout

Transforms are visual-only. The element still occupies its original layout space. Hit testing may or may not account for transforms depending on the context.

### `Pixels` fields are private

You cannot access `Pixels.0` directly. Use arithmetic operations (`px_a - px_b`) and comparison (`assert_eq!(result.x, px(5.0))`) instead.
