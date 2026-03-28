# Scroll Containers

**Components:** [`ScrollHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/div.rs), [`ScrollWheelEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs), [`ScrollDelta`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs)

## What is the component and what it does

Any div can become a scrollable container by setting `overflow_scroll()`. Content that exceeds the container's bounds becomes scrollable. A `ScrollHandle` provides programmatic scroll control — scroll to items, get current offset, scroll to top/bottom.

## Preconditions for use

```rust
use gpui::{ScrollHandle, InteractiveElement, StatefulInteractiveElement, Styled};
```

- The div must have an `.id()` (making it "stateful") — `InteractiveElement` provides `.id()`
- `StatefulInteractiveElement` provides `.overflow_scroll()`, `.overflow_y_scroll()`, `.overflow_x_scroll()`, `.track_scroll()`
- The container must have bounded size (explicit height/width or flex constraints)
- `ScrollHandle` must be stored in the view struct — recreating it each render resets scroll

## Signature for usage

### Making a div scrollable

```rust
div()
    .id("scroll-area")
    .overflow_y_scroll()      // Vertical scroll only
    .h_full()                 // Must have bounded height
    .children(items)

div()
    .id("both-scroll")
    .overflow_scroll()        // Both horizontal and vertical
    .size_full()
    .children(items)

div()
    .id("h-scroll")
    .overflow_x_scroll()      // Horizontal scroll only
    .flex()
    .children(items)
```

### ScrollHandle

```rust
let handle = ScrollHandle::new();

// Attach to container
div().id("scroll").overflow_y_scroll().track_scroll(&handle)

// Query
handle.offset()                    // Point<Pixels> — current scroll offset
handle.max_offset()                // Point<Pixels> — maximum scroll offset
handle.top_item()                  // usize — index of top visible child
handle.bottom_item()               // usize — index of bottom visible child
handle.children_count()            // usize — total child count
handle.bounds()                    // Bounds<Pixels> — container bounds
handle.bounds_for_item(ix)         // Option<Bounds<Pixels>> — specific child bounds

// Control
handle.scroll_to_item(ix)          // Scroll to make item visible
handle.scroll_to_top_of_item(ix)   // Scroll item to top of viewport
handle.scroll_to_bottom()          // Scroll to the end
handle.set_offset(point)           // Set exact scroll position
```

### ScrollWheelEvent

```rust
pub struct ScrollWheelEvent {
    pub position: Point<Pixels>,   // Mouse position in window
    pub delta: ScrollDelta,        // Scroll amount
    pub modifiers: Modifiers,      // Keys held during scroll
    pub touch_phase: TouchPhase,   // Touch/trackpad phase
}

pub enum ScrollDelta {
    Pixels(Point<Pixels>),  // Exact pixel delta (trackpad)
    Lines(Point<f32>),      // Line-based delta (mouse wheel)
}
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `InteractiveElement` | Provides `.id()` — required to make div stateful |
| `StatefulInteractiveElement` | Provides `.overflow_scroll()`, `.track_scroll()`, `.on_scroll_wheel()` |
| `Styled` | Provides sizing (`.h_full()`, `.size_full()`) |

## Usage and examples

### Basic scrollable list

```rust
struct ScrollView {
    scroll_handle: ScrollHandle,
}

impl Render for ScrollView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            div()
                .id("scroll-container")
                .overflow_y_scroll()
                .track_scroll(&self.scroll_handle)
                .h_full()
                .children((0..100).map(|i| div().child(format!("Row {}", i)))),
        )
    }
}
```

### Programmatic scrolling

```rust
// Scroll to item 50
self.scroll_handle.scroll_to_item(50);

// Scroll to bottom
self.scroll_handle.scroll_to_bottom();

// Get current position
let offset = self.scroll_handle.offset();
```

### Horizontal scroll

```rust
div()
    .id("h-scroll")
    .overflow_x_scroll()
    .flex()
    .children((0..50).map(|i| div().w_20().child(format!("Col {}", i))))
```

## Post-conditions / destruction requirements

- `ScrollHandle` retains position across re-renders if the same handle instance is reused
- Recreating `ScrollHandle::new()` each render resets to offset (0, 0)
- No explicit cleanup needed

## Testing

```rust
#[gpui::test]
fn test_scroll_default_offset(_cx: &mut TestAppContext) {
    let handle = ScrollHandle::new();
    let offset = handle.offset();
    assert_eq!(offset.x, px(0.0));
    assert_eq!(offset.y, px(0.0));
}
```

Run tests: `cargo test --test scroll_test`

## Surprises, Anti-patterns, and Bugs

### Container must have bounded size

`overflow_scroll()` only works if the container has a fixed or constrained size. Without `.h_full()`, `.h(px(300.0))`, or flex constraints, the container grows to fit all content and nothing scrolls.

### Div must have an `.id()` for scroll to work

`overflow_scroll()` and `track_scroll()` are on `StatefulInteractiveElement`, which requires `.id()` on the div first. Without `.id()`, the methods are not available and the compiler says "method not found".

### No built-in scrollbar rendering

GPUI does not render scrollbars. You must build your own using the `ScrollHandle` offset and max_offset if you want visual scrollbar indicators.

### `ScrollHandle` must be stored, not recreated

Store the handle in your view struct. If you create `ScrollHandle::new()` inside `render()`, it will be a new handle each frame and scroll position resets.

### `ScrollDelta::Lines` vs `ScrollDelta::Pixels`

Mouse wheels typically produce `Lines` deltas (discrete steps). Trackpads produce `Pixels` deltas (smooth). Your scroll handler should handle both.

### Smooth scrolling depends on platform

On macOS with a trackpad, scrolling is smooth (pixel-level). With a mouse wheel, scrolling is step-based. GPUI does not add momentum/inertia — the OS provides this for trackpad gestures.
