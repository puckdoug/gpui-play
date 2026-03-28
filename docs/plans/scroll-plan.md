# Scroll Containers

## Goal

Document `overflow_scroll()`, `ScrollHandle`, `ScrollWheelEvent`, and `ScrollDelta`.

## Design

A view with a scrollable content area containing more content than fits. A `ScrollHandle` enables programmatic scrolling via buttons ("Scroll to Top", "Scroll to Bottom", "Scroll to Item"). Scroll events are logged to show `ScrollDelta` types.

### Key Concepts

- `overflow_scroll()` on a div makes it scrollable (like CSS `overflow: scroll`)
- `ScrollHandle` provides programmatic control over scroll position
- `ScrollWheelEvent` fires on mouse wheel / trackpad scroll
- `ScrollDelta` has variants: `Lines`, `Pixels`, `Pages`

## View Layer (src/bin/scroll_test.rs)

- Scrollable div with 50+ items (enough to require scrolling)
- Buttons: "Top", "Bottom", "To Item 25"
- Event log panel showing recent ScrollWheelEvents with delta type and magnitude
- Visual scroll position indicator (scrollbar-like)

## TDD Tests

### Scroll container (3)
1. overflow_scroll() div clips content beyond bounds
2. ScrollHandle.scroll_to_top() positions at top
3. ScrollHandle.scroll_to_item() positions target item in view

### Scroll events (3)
4. ScrollWheelEvent fires on simulated scroll
5. ScrollDelta reports correct magnitude
6. Scroll position updates after event

## Documentation (docs/gpui-usage/scroll.md)

### Sections
1. **What it is** — scrollable container with programmatic scroll control and event handling
2. **Preconditions** — `use gpui::ScrollHandle`; parent div needs `.overflow_scroll()` or `.overflow_y_scroll()`; ScrollHandle must be created and attached to the div
3. **Signatures** — `.overflow_scroll()`, `.overflow_x_scroll()`, `.overflow_y_scroll()`, `ScrollHandle::new()`, `.track_scroll(handle)`, `.scroll_to_item(index)`, `.on_scroll_wheel(handler)`
4. **Relevant types** — `ScrollHandle`, `ScrollWheelEvent`, `ScrollDelta`
5. **Usage examples** — basic scrollable area, programmatic scroll, event logging
6. **Post-conditions** — ScrollHandle retains position across re-renders; scroll position lost if handle is recreated
7. **Testing** — simulate scroll via dispatch; verify position via ScrollHandle query
8. **Surprises** — overflow_scroll only works on divs with bounded size (needs explicit height or flex constraints); horizontal and vertical scroll are separate; no built-in scrollbar rendering (must build your own); smooth scrolling behavior depends on platform; ScrollHandle must be stored in view struct (not recreated each render)

**Note:** Reference Zed's `crates/gpui/examples/scrollable.rs`.
