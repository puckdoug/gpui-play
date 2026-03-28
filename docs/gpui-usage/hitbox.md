# Hitbox

**Components:** [`Hitbox`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs), [`HitboxId`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs), [`HitboxBehavior`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs)

## What is the component and what it does

Hitboxes define clickable/hoverable rectangular regions for custom elements (canvas, custom painting). When div-based click handlers don't apply — e.g., you're painting shapes on a canvas — hitboxes provide the hit-testing layer.

Hitboxes are registered during the prepaint phase and checked during mouse event dispatch. They support blocking behavior (preventing clicks from reaching elements behind them) and hover detection.

## Preconditions for use

```rust
use gpui::{canvas, Hitbox, HitboxBehavior, Bounds};
```

- Hitboxes must be created during the **prepaint** phase of a canvas element
- Created via `window.insert_hitbox(bounds, behavior)`
- Bounds are in **window coordinates**, not element-local
- Hitboxes are recreated every prepaint (not retained across frames)

## Signature for usage

### Creating a hitbox

```rust
let hitbox = window.insert_hitbox(
    Bounds::new(point(px(10.0), px(10.0)), size(px(100.0), px(100.0))),
    HitboxBehavior::Normal,
);
```

### Hitbox struct

```rust
pub struct Hitbox {
    pub id: HitboxId,                       // Unique identifier
    pub bounds: Bounds<Pixels>,             // Rectangular bounds (Deref target)
    pub content_mask: ContentMask<Pixels>,  // Clipping mask at insertion time
    pub behavior: HitboxBehavior,           // Mouse interaction behavior
}
```

### HitboxBehavior

```rust
HitboxBehavior::Normal              // Standard — receives events, doesn't block others
HitboxBehavior::BlockMouse          // Blocks ALL mouse events to hitboxes behind
HitboxBehavior::BlockMouseExceptScroll  // Blocks mouse except scroll events behind
```

### Checking hover

```rust
hitbox.is_hovered(window)  // true if mouse is over this hitbox and not blocked
```

### HitboxId

```rust
pub struct HitboxId(u64);  // Opaque, unique per hitbox within a frame
```

## Usage and examples

### Canvas with hitbox regions

```rust
impl Render for HitboxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            canvas(
                |bounds, window, _cx| {
                    // Prepaint: register hitboxes
                    let hitbox = window.insert_hitbox(
                        Bounds::new(
                            point(px(10.0), px(10.0)),
                            size(px(100.0), px(100.0)),
                        ),
                        HitboxBehavior::Normal,
                    );
                    hitbox
                },
                |_bounds, hitbox, window, _cx| {
                    // Paint: use hover state for visual feedback
                    if hitbox.is_hovered(window) {
                        // Draw highlighted
                    } else {
                        // Draw normal
                    }
                },
            )
            .size_full(),
        )
    }
}
```

### Multiple hitboxes with unique IDs

```rust
canvas(
    |_bounds, window, _cx| {
        let h1 = window.insert_hitbox(
            Bounds::new(point(px(0.0), px(0.0)), size(px(50.0), px(50.0))),
            HitboxBehavior::Normal,
        );
        let h2 = window.insert_hitbox(
            Bounds::new(point(px(60.0), px(0.0)), size(px(50.0), px(50.0))),
            HitboxBehavior::Normal,
        );
        assert_ne!(h1.id, h2.id);  // Each has a unique ID
        (h1, h2)
    },
    |_bounds, (h1, h2), window, _cx| { /* paint */ },
)
```

### Blocking hitbox (modal overlay)

```rust
// Background clickable area
let _bg = window.insert_hitbox(large_bounds, HitboxBehavior::Normal);

// Overlay that blocks clicks to background
let _overlay = window.insert_hitbox(overlay_bounds, HitboxBehavior::BlockMouse);
```

## Post-conditions / destruction requirements

- Hitboxes are recreated every prepaint — they are NOT retained across frames
- No explicit cleanup needed
- Hitbox IDs are valid only within the current frame

## Testing

```rust
#[gpui::test]
fn test_hitbox(cx: &mut TestAppContext) {
    struct HitboxView;
    impl Render for HitboxView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(canvas(
                |_bounds, window, _cx| {
                    window.insert_hitbox(
                        Bounds::new(point(px(0.), px(0.)), size(px(50.), px(50.))),
                        HitboxBehavior::Normal,
                    )
                },
                |_bounds, _hitbox, _window, _cx| {},
            ).size_full())
        }
    }
    let _window = cx.add_window(|_w, _cx| HitboxView);
}
```

Run tests: `cargo test --test hitbox_test`

## Surprises, Anti-patterns, and Bugs

### Hitboxes are RECTANGULAR only

All hitboxes are axis-aligned rectangles. For non-rectangular shapes (circles, polygons), you must check the actual shape geometry inside your mouse event handler after the rectangular hitbox fires.

### Hitboxes must be recreated every prepaint

They are not persistent. If you skip creating a hitbox in a frame, it doesn't exist for that frame's mouse events.

### Coordinates are in window space

Hitbox bounds are in window coordinates, not element-local. When using a canvas inside a scrollable container, you must account for scroll offset.

### Insertion order determines priority

Later-inserted hitboxes take priority over earlier ones. There is no explicit z-index — paint order determines interaction order.

### `BlockMouse` blocks hover on hitboxes behind it

A `BlockMouse` hitbox prevents `is_hovered()` from returning true for any hitbox behind it, even if the mouse is within those bounds.

### Mouse event listeners receive the hitbox

Internal listener signatures include `&Hitbox` as a parameter. Use `hitbox.is_hovered(window)` in the listener to check if the event applies to your hitbox.
