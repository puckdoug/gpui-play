# Virtualized Lists (List, UniformList)

## Goal

Document `List` with `ListState` (variable-height items) and `UniformList` (equal-height items, optimized for 10k+ rows).

## Design

A split view: left side shows a `UniformList` with 10,000 simple rows (index + label). Right side shows a `List` with variable-height items (some have descriptions, some don't). Both support scroll-to-item.

### Data Model

```rust
struct ListDemo {
    uniform_items: Vec<String>,      // 10k items
    variable_items: Vec<ListItem>,   // mixed-height items
    scroll_target: Option<usize>,
}

struct ListItem {
    title: String,
    description: Option<String>,     // present = taller row
}
```

### Key Concepts

- `UniformList` only measures one item, assumes all same height — O(1) scroll positioning
- `List` measures each item individually — supports variable heights but more expensive
- `ListState` manages scroll position, visible range, and item count
- Both virtualize: only visible items are rendered (critical for performance)
- Scroll-to-item via `ListState` or `ScrollHandle`

## View Layer (src/bin/list_test.rs)

- Left panel: UniformList with 10k rows, each row shows "Item {n}"
- Right panel: List with ~100 variable-height items
- Input field + "Scroll To" button for each list
- Item count display and visible range indicator

## TDD Tests

### UniformList (4)
1. UniformList renders without panic with 10k items
2. Only visible items are in the element tree (virtualization works)
3. Scroll-to-item positions the target item in view
4. Dynamic item insertion updates the list

### List (4)
5. List renders variable-height items correctly
6. Tall items get more space than short items
7. Scroll-to-item works with variable heights
8. Item removal updates the list and scroll position

### ListState (2)
9. ListState tracks visible range
10. ListState can be reset (scroll to top)

## Documentation (docs/gpui-usage/lists.md)

### Sections
1. **What it is** — virtualized list elements that only render visible items for performance
2. **Preconditions** — `use gpui::{List, ListState, UniformList}`; item count known upfront or dynamically queryable; item render closure provided
3. **Signatures** — `uniform_list(state, item_count, render_fn)`, `list(state, render_fn)`, `ListState::new()`, `.scroll_to_item(index)`
4. **Relevant traits** — render closure signature
5. **Usage examples** — 10k row UniformList, variable-height List, scroll-to-item
6. **Post-conditions** — ListState must be retained across renders; dropping ListState resets scroll
7. **Testing** — test item rendering via view update; test scroll position via ListState query
8. **Surprises** — UniformList silently misbehaves if items have different heights; List re-measures on every render (perf cost); item indices may shift if items are inserted/removed; render closure called with visible range only; no built-in selection or keyboard navigation

**Note:** Reference Zed's `crates/gpui/examples/data_table.rs` and `uniform_list.rs`.
