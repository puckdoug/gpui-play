# Shadows

**Components:** [`BoxShadow`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`window.paint_shadows()`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs)

## What is the component and what it does

GPUI provides drop shadows on elements via style presets (`.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`) and custom `BoxShadow` configuration. Shadows are rendered as Gaussian blurs around element bounds.

For custom painting (canvas elements), `window.paint_shadows()` provides direct shadow rendering during the paint phase.

## Preconditions for use

```rust
use gpui::{BoxShadow, Styled};       // For styled shadow methods
use gpui::{hsla, point, px};          // For custom BoxShadow construction
```

- `Styled` trait provides all shadow preset methods and `.shadow()`
- `window.paint_shadows()` is only available during the paint phase of canvas/custom elements

## Signature for usage

### Shadow presets (Styled trait)

```rust
div().shadow_2xs()   // Smallest shadow
div().shadow_xs()
div().shadow_sm()
div().shadow_md()
div().shadow_lg()
div().shadow_xl()
div().shadow_2xl()   // Largest shadow
```

### Custom shadows

```rust
div().shadow(vec![BoxShadow {
    color: hsla(0.0, 0.0, 0.0, 0.3),    // Shadow color with alpha
    offset: point(px(0.0), px(4.0)),      // Horizontal, vertical offset
    blur_radius: px(8.0),                 // Gaussian blur radius
    spread_radius: px(0.0),               // Expand/shrink shadow
}])
```

### BoxShadow struct

```rust
pub struct BoxShadow {
    pub color: Hsla,               // Shadow color (use alpha for intensity)
    pub offset: Point<Pixels>,     // (x, y) offset from element
    pub blur_radius: Pixels,       // Blur amount (0 = hard shadow)
    pub spread_radius: Pixels,     // Positive = larger, negative = smaller
}
```

### Multiple shadows

```rust
div().shadow(vec![
    BoxShadow { color: hsla(0., 0., 0., 0.1), offset: point(px(0.), px(1.)), blur_radius: px(3.), spread_radius: px(0.) },
    BoxShadow { color: hsla(0., 0., 0., 0.2), offset: point(px(0.), px(4.)), blur_radius: px(8.), spread_radius: px(0.) },
])
```

### Canvas painting

```rust
window.paint_shadows(
    bounds: Bounds<Pixels>,           // Element bounds
    corner_radii: Corners<Pixels>,    // Rounded corner radii
    shadows: &[BoxShadow],           // Shadow array
)
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Styled` | Provides `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`, `.shadow()` |

## Usage and examples

### Preset shadow sizes

```rust
impl Render for ShadowView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().gap_4()
            .child(div().size_20().bg(gpui::white()).shadow_sm().child("sm"))
            .child(div().size_20().bg(gpui::white()).shadow_md().child("md"))
            .child(div().size_20().bg(gpui::white()).shadow_lg().child("lg"))
    }
}
```

### Custom colored shadow

```rust
div()
    .size_20()
    .bg(gpui::white())
    .shadow(vec![BoxShadow {
        color: hsla(0.6, 0.8, 0.5, 0.5), // Blue-purple colored shadow
        offset: point(px(0.0), px(4.0)),
        blur_radius: px(12.0),
        spread_radius: px(2.0),
    }])
```

### Hard shadow (zero blur)

```rust
BoxShadow {
    color: hsla(0.0, 0.0, 0.0, 0.3),
    offset: point(px(4.0), px(4.0)),
    blur_radius: px(0.0),     // No blur = sharp edge
    spread_radius: px(0.0),
}
```

## Post-conditions / destruction requirements

- No cleanup needed — shadows are repainted each frame
- Shadows render outside element bounds (parent must have `overflow: visible`)

## Testing

```rust
#[gpui::test]
fn test_shadow(cx: &mut TestAppContext) {
    struct ShadowView;
    impl Render for ShadowView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_20().bg(gpui::white()).shadow_md()
        }
    }
    let _window = cx.add_window(|_w, _cx| ShadowView);
}
```

Run tests: `cargo test --test shadows_test`

## Surprises, Anti-patterns, and Bugs

### Shadows render outside element bounds

Shadows extend beyond the element's layout bounds. If a parent has `overflow: hidden` (or scroll), shadows may be clipped. Ensure parent containers allow overflow for shadows to be visible.

### Multiple shadows stack

When providing multiple `BoxShadow` entries, they are all rendered. Order matters — earlier shadows are painted first (behind later ones).

### Shadow color alpha controls intensity

Use the alpha channel of `Hsla` to control shadow intensity. `hsla(0., 0., 0., 0.1)` is a very subtle shadow; `hsla(0., 0., 0., 0.5)` is strong.

### No inset shadows

GPUI's `BoxShadow` does not support inset/inner shadows. All shadows are outer (drop) shadows.

### No text shadows

Shadows can only be applied to elements (boxes), not to text. There is no `text-shadow` equivalent.

### `paint_shadows()` requires bounds and corner radii

When using `window.paint_shadows()` in canvas painting, you must provide the element bounds and corner radii manually. The shadow shape matches the rounded rectangle defined by these parameters.
