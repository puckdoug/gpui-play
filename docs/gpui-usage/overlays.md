# Overlays (Anchored, Deferred, Tooltips)

**Components:** [`anchored`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/anchored.rs), [`deferred`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/deferred.rs), [`tooltip`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/div.rs)

## What is the component and what it does

GPUI's overlay system uses two complementary elements:

- **`deferred`** — renders its child *after* all ancestors in the paint order. This ensures overlays appear on top of other content, not clipped by parent bounds.
- **`anchored`** — positions content at a specific point in window coordinates, with automatic adjustment to stay within window bounds.

The standard pattern for popovers, dropdowns, and context menus is `deferred(anchored().child(content))`.

**Tooltips** are a specialized overlay that appears on hover with a delay, using `.tooltip()` on any stateful element.

## Preconditions for use

```rust
use gpui::{anchored, deferred, Corner};
use gpui::{InteractiveElement, StatefulInteractiveElement}; // For .id() and .tooltip()
use gpui::AppContext; // For cx.new() in tooltip view creation
```

- `deferred` wraps any element
- `anchored` takes children via `.child()`
- `.tooltip()` requires the element to have `.id()` (stateful)
- Tooltip closure must return `AnyView` (not `AnyElement`)

## Signature for usage

### deferred

```rust
deferred(child: impl IntoElement) -> Deferred

// With priority (higher = painted later = on top)
deferred(child).priority(10)
deferred(child).with_priority(10)  // alias
```

### anchored

```rust
anchored() -> Anchored

// Key methods
.anchor(Corner::TopLeft)                          // Which corner anchors to position
.position(point(px(100.0), px(200.0)))            // Position in window coordinates
.offset(point(px(5.0), px(5.0)))                  // Offset from position
.snap_to_window()                                 // Adjust to stay within window bounds
.snap_to_window_with_margin(Edges { ... })        // Snap with margins
.position_mode(AnchoredPositionMode::Window)      // Window or Local coordinates
.child(element)                                   // Add content
```

### Corner

```rust
Corner::TopLeft
Corner::TopRight
Corner::BottomLeft
Corner::BottomRight
```

### tooltip

```rust
div()
    .id("my-element")
    .tooltip(|window, cx| {
        // Must return AnyView, not AnyElement
        cx.new(|_cx| TooltipContent).into()
    })
```

### hoverable_tooltip (stays visible when hovered)

```rust
div()
    .id("my-element")
    .hoverable_tooltip(|window, cx| {
        cx.new(|_cx| TooltipContent).into()
    })
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `InteractiveElement` | Provides `.id()` — needed for tooltips |
| `StatefulInteractiveElement` | Provides `.tooltip()` and `.hoverable_tooltip()` |
| `ParentElement` | Provides `.child()` on Anchored |

## Usage and examples

### Basic popover (deferred + anchored)

```rust
impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut root = div().size_full().child("Main content");

        if self.show_popover {
            root = root.child(
                deferred(
                    anchored()
                        .anchor(Corner::TopLeft)
                        .position(point(px(50.0), px(100.0)))
                        .child(
                            div()
                                .bg(white())
                                .p_4()
                                .shadow_md()
                                .child("Menu item 1")
                                .child("Menu item 2"),
                        ),
                )
                .with_priority(1),
            );
        }

        root
    }
}
```

### Tooltip

```rust
struct TooltipContent;
impl Render for TooltipContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(gpui::rgb(0x333333)).text_color(gpui::white()).p_2()
            .child("This is a tooltip")
    }
}

// In parent render:
div()
    .id("hover-target")
    .child("Hover me")
    .tooltip(|_window, cx| cx.new(|_cx| TooltipContent).into())
```

### Deferred with priority (layering)

```rust
// Lower priority renders first (behind)
div()
    .child(deferred(div().child("Background overlay")).priority(1))
    .child(deferred(div().child("Foreground overlay")).priority(10))
```

### Anchored with boundary snapping

```rust
deferred(
    anchored()
        .snap_to_window()  // Adjust position to stay within window
        .anchor(Corner::BottomRight)
        .position(point(px(800.0), px(600.0)))  // Near window edge
        .child(div().child("Will adjust to fit")),
)
```

## Post-conditions / destruction requirements

- Overlays are removed from the tree when their parent conditionally excludes them
- No explicit cleanup needed
- Tooltip visibility is managed by the framework (appears on hover, disappears on leave)
- `deferred` elements have a minor performance cost (extra paint pass)

## Testing

```rust
#[gpui::test]
fn test_popover(cx: &mut TestAppContext) {
    struct PopoverView { show: bool }
    impl Render for PopoverView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let mut root = div().child("Content");
            if self.show {
                root = root.child(deferred(
                    anchored().anchor(Corner::TopLeft)
                        .child(div().child("Popover"))
                ));
            }
            root
        }
    }
    let _window = cx.add_window(|_w, _cx| PopoverView { show: true });
}
```

Run tests: `cargo test --test overlays_test`

## Surprises, Anti-patterns, and Bugs

### Anchored without Deferred renders behind siblings

If you use `anchored()` without wrapping it in `deferred()`, it paints in normal order — likely behind later siblings. Always use `deferred(anchored(...))` for overlays.

### Tooltip requires `AnyView`, not `AnyElement`

The `.tooltip()` closure must return `AnyView`. You need to create a view with `cx.new(|_cx| MyView)` and call `.into()` on it. Using `.into_any_element()` produces the wrong type.

### Tooltip requires `.id()` on the element

Without `.id()`, the element is not stateful and `.tooltip()` is not available. The compiler says "method not found" — the fix is adding `.id("some-id")`.

### Only one tooltip per element

`debug_assert!` fires if `.tooltip()` is called twice on the same element. Use a single tooltip that conditionally shows different content.

### Focus management for dismiss-on-outside-click

GPUI does not automatically dismiss popovers on outside click. You must handle this yourself — typically by listening for mouse down events and checking if the click is outside the popover bounds.

### Deferred priority determines layering

Higher priority = painted later = appears on top. If two overlays overlap, use different priorities. Default priority is 0.

### Anchored position is in window coordinates

The `.position()` value is in window coordinates, not relative to the parent element. To position relative to a button, you need to track the button's bounds and pass them to `.position()`.
