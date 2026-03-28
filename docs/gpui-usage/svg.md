# SVG Rendering

**Components:** [`Svg`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/svg.rs), [`Transformation`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/svg.rs)

## What is the component and what it does

The `Svg` element renders SVG files as GPU-accelerated vector paths. SVGs are tinted to a single color (inherited from `text_color`) and can be transformed with rotation, scaling, and translation. This is primarily designed for **icons** — simple, single-color SVG paths.

GPUI does not render SVGs like a browser. It parses the SVG paths and renders them as filled vector geometry with a single tint color. Complex SVG features (gradients, filters, embedded images, CSS, text elements) are not supported.

## Preconditions for use

```rust
use gpui::{svg, Styled, Transformation, percentage};
```

- SVG files must be simple path-based SVGs (single color, no CSS, no embedded raster)
- `Styled` trait must be in scope for sizing (`.size_8()`, etc.)
- SVG files are loaded via the asset system (`.path()`) or file system (`.external_path()`)
- `Transformation::rotate()` takes a `Percentage` (0.0–1.0 = 0–360°), not radians directly

## Signature for usage

### Creating an SVG element

```rust
svg()
    .path("icons/arrow.svg")        // From embedded assets
    .size_8()                        // 32px (8 * 4px spacing unit)
```

### External file path

```rust
svg()
    .external_path("/absolute/path/to/icon.svg")
    .size_8()
```

### Color tinting

```rust
svg()
    .path("icons/arrow.svg")
    .size_8()
    .text_color(gpui::red())  // SVG inherits text_color
```

### Transformation

```rust
use gpui::{Transformation, percentage, size};

// Rotation: percentage(0.25) = 90°, percentage(0.5) = 180°, percentage(1.0) = 360°
Transformation::rotate(percentage(0.25))

// Scale
Transformation::scale(size(2.0, 2.0))

// Translation
Transformation::translate(point(px(10.0), px(5.0)))

// Compose: chain methods
Transformation::rotate(percentage(0.125))
    .with_scaling(size(1.5, 1.5))
    .with_translation(point(px(0.0), px(0.0)))
```

### Applying transformation

```rust
svg()
    .path("icons/arrow.svg")
    .size_8()
    .with_transformation(Transformation::rotate(percentage(0.25)))
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Styled` | Provides sizing, color, and layout methods |
| `InteractiveElement` | SVG supports click handlers and hover events |
| `IntoElement` | Svg implements this — usable as a child element |

## Usage and examples

### Basic SVG icon

```rust
impl Render for IconView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(svg().path("icons/settings.svg").size_8())
    }
}
```

### Rotated SVG

```rust
svg()
    .path("icons/arrow.svg")
    .size_8()
    .with_transformation(Transformation::rotate(percentage(0.25)))  // 90°
```

### Scaled and rotated SVG

```rust
svg()
    .path("icons/star.svg")
    .size_8()
    .with_transformation(
        Transformation::rotate(percentage(0.125))  // 45°
            .with_scaling(gpui::size(1.5, 1.5)),
    )
```

### Colored SVG icons

```rust
// SVGs inherit text_color for tinting
div()
    .child(svg().path("icons/check.svg").size_8().text_color(gpui::green()))
    .child(svg().path("icons/x.svg").size_8().text_color(gpui::red()))
```

## Post-conditions / destruction requirements

- SVGs are loaded and cached by the asset system — no cleanup needed
- Transformations are applied per-render — no persistent state
- Dropping the element releases the GPU resources

## Testing

SVG rendering is visual — tests verify element construction:

```rust
#[gpui::test]
fn test_svg_with_transform(cx: &mut TestAppContext) {
    struct SvgView;
    impl Render for SvgView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                svg().path("icons/test.svg").size_8()
                    .with_transformation(Transformation::rotate(percentage(0.25)))
            )
        }
    }
    let _window = cx.add_window(|_w, _cx| SvgView);
}
```

Run tests: `cargo test --test svg_test`

## Surprises, Anti-patterns, and Bugs

### SVGs are single-color only

GPUI renders SVGs as monochromatic paths tinted by `text_color`. The original fill/stroke colors in the SVG file are **ignored**. If you need multi-color SVGs, use the `Img` element instead (which rasterizes the SVG).

### `Transformation::rotate()` takes `Percentage`, not radians

`percentage(0.25)` = 90°. The value must be between 0.0 and 1.0 — values outside this range **panic** at runtime. Use `percentage()` from `gpui` to create the value.

### Complex SVGs render incorrectly or not at all

SVGs with gradients, filters, masks, clipPaths, text elements, or CSS styles will not render correctly. Stick to simple `<path>` and `<circle>`/`<rect>` elements.

### Color is controlled via `text_color`, not fill

This is unintuitive — you set the icon color using `.text_color()`, not a fill or background property. This follows the CSS pattern where icon fonts inherit text color.

### `.path()` vs `.external_path()`

`.path()` loads from the embedded asset system (requires `AssetSource` configured on the application). `.external_path()` loads from the file system. If your SVG isn't rendering, check which method you're using.

### Transformation origin is the element center

Rotation and scaling are applied around the center of the element bounds, not the top-left corner. This is usually what you want for icons.
