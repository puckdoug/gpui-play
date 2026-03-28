# Virtualized Lists (UniformList, List)

**Components:** [`uniform_list`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/uniform_list.rs), [`list`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/list.rs), [`ListState`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/list.rs), [`UniformListScrollHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/uniform_list.rs)

## What is the component and what it does

GPUI provides two virtualized list elements that only render visible items:

- **`uniform_list`** — all items have the same height. Only one item is measured; scroll position is calculated with O(1) arithmetic. Ideal for 10k+ row tables and homogeneous lists.
- **`list`** — items can have variable heights. Each visible item is measured individually. More flexible but more expensive. Ideal for chat logs, mixed-content feeds.

Both avoid rendering off-screen items, making large datasets performant.

## Preconditions for use

```rust
use gpui::{uniform_list, list, ListState, ListAlignment, UniformListScrollHandle, ScrollStrategy};
use gpui::{InteractiveElement, Styled}; // For .id() and .h_full()
```

- `uniform_list` requires an `ElementId`, item count, and render closure
- `list` requires a `ListState` and render closure
- `InteractiveElement` must be in scope for `.id()` on child divs
- Items returned from the render closure must implement `IntoElement`

## Signature for usage

### uniform_list

```rust
uniform_list(
    id: impl Into<ElementId>,       // e.g., "my-list"
    item_count: usize,               // Total number of items
    f: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,  // Render visible range
) -> UniformList
where R: IntoElement
```

The render closure receives the **range of visible indices** and must return elements for that range.

### UniformListScrollHandle

```rust
let handle = UniformListScrollHandle::new();

// Attach to list
uniform_list("id", count, render_fn).track_scroll(&handle)

// Programmatic scrolling
handle.scroll_to_item(500, ScrollStrategy::Center);
handle.scroll_to_bottom();
handle.is_scrollable()  // bool
```

### ScrollStrategy

```rust
ScrollStrategy::Top       // Item at top of viewport
ScrollStrategy::Center    // Item in center
ScrollStrategy::Bottom    // Item at bottom
ScrollStrategy::Nearest   // Minimal scroll to make visible
```

### list (variable height)

```rust
list(
    state: ListState,
    render_item: impl FnMut(usize, &mut Window, &mut App) -> AnyElement + 'static,
) -> List
```

The render closure receives a **single item index** and must return `AnyElement` (use `.into_any_element()`).

### ListState

```rust
// Create
let state = ListState::new(
    item_count: usize,
    alignment: ListAlignment,    // Top or Bottom
    overdraw: Pixels,            // Extra render margin (e.g., px(50.0))
);

// Key methods
state.item_count()                    // Current item count
state.reset(new_count)                // Reset with new count
state.splice(old_range, new_count)    // Insert/remove items
state.scroll_to_reveal_item(ix)       // Scroll to make item visible
state.scroll_to(ListOffset { ... })   // Jump to exact scroll position
state.logical_scroll_top()            // Current scroll position
state.bounds_for_item(ix)             // Rendered bounds (if visible)
```

### ListAlignment

```rust
ListAlignment::Top     // Standard top-to-bottom scrolling
ListAlignment::Bottom  // Bottom-to-top (chat log style)
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `InteractiveElement` | Required for `.id()` on list item divs |
| `Styled` | For `.h_full()`, sizing on the list element |

## Usage and examples

### UniformList with 10k rows

```rust
impl Render for TableView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            uniform_list("rows", 10_000, |range, _window, _cx| {
                range
                    .map(|ix| div().id(ix).child(format!("Row {}", ix)))
                    .collect()
            })
            .h_full(),
        )
    }
}
```

### UniformList with scroll handle

```rust
struct ScrollableList {
    scroll_handle: UniformListScrollHandle,
}

impl Render for ScrollableList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            uniform_list("list", 1000, |range, _window, _cx| {
                range.map(|ix| div().id(ix).child(format!("Item {}", ix))).collect()
            })
            .track_scroll(&self.scroll_handle)
            .h_full(),
        )
    }
}

// Later: self.scroll_handle.scroll_to_item(500, ScrollStrategy::Center);
```

### Variable-height list

```rust
struct ChatView {
    list_state: ListState,
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.list_state.clone();
        div().size_full().child(
            list(state, |ix, _window, _cx| {
                if ix % 3 == 0 {
                    div().h(px(60.0)).child(format!("Long message {}", ix)).into_any_element()
                } else {
                    div().h(px(30.0)).child(format!("Message {}", ix)).into_any_element()
                }
            })
            .h_full(),
        )
    }
}
```

### Chat log (bottom-aligned)

```rust
let state = ListState::new(messages.len(), ListAlignment::Bottom, px(50.0));
```

## Post-conditions / destruction requirements

- `ListState` must be retained across renders — store it in your view struct
- `UniformListScrollHandle` must also be stored to persist scroll position
- Dropping and recreating either resets scroll position to the start
- No explicit cleanup needed

## Testing

```rust
#[gpui::test]
fn test_list_state(_cx: &mut TestAppContext) {
    let state = ListState::new(42, ListAlignment::Top, px(50.0));
    assert_eq!(state.item_count(), 42);

    state.reset(10);
    assert_eq!(state.item_count(), 10);
}
```

Run tests: `cargo test --test lists_test`

## Surprises, Anti-patterns, and Bugs

### uniform_list silently breaks if items have different heights

`uniform_list` measures ONE item and assumes all items are the same height. If items vary in height, scroll positions will be incorrect and items may overlap or gap. Use `list` for variable heights.

### Render closure signatures differ

- `uniform_list`: closure receives `Range<usize>`, returns `Vec<impl IntoElement>`
- `list`: closure receives single `usize`, returns `AnyElement`

### `list` render closure must return `AnyElement`

Use `.into_any_element()` on your element. Forgetting this causes a type error.

### Item IDs are important for uniform_list

Each item should have a unique `.id()` for correct diffing and event handling. Without IDs, click handlers and focus may not work correctly.

### `ListState` is `Clone` but shares internal state

Cloning `ListState` creates another handle to the same internal `Rc<RefCell<...>>`. All clones share scroll position and item count.

### No built-in selection or keyboard navigation

Neither list type provides selection, focus traversal, or keyboard navigation. You must implement these yourself using focus handles and key bindings.

### Overdraw parameter

The `overdraw: Pixels` parameter in `ListState::new()` controls how many extra pixels of items are rendered beyond the visible viewport. Larger values reduce flicker during fast scrolling but render more items.
