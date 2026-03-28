# Drag and Drop

## Goal

Document `.on_drag()`, `.on_drag_move()`, `.on_drop()`, and `DragMoveEvent`.

## Design

A kanban-style board with three columns. Cards can be dragged between columns. A custom drag preview follows the cursor. Drop zones highlight when a draggable card hovers over them.

### Data Model

```rust
struct KanbanBoard {
    columns: Vec<Column>,
}

struct Column {
    title: String,
    cards: Vec<Card>,
}

struct Card {
    id: usize,
    title: String,
}

// Drag payload
struct DraggedCard {
    card: Card,
    source_column: usize,
}
```

### Key Concepts

- `.on_drag(payload, drag_view_fn)` — makes element draggable with a typed payload and custom drag preview
- `.on_drag_move(handler)` — fires continuously during drag with `DragMoveEvent` (position info)
- `.on_drop(handler)` — makes element a drop target for a specific payload type
- Type safety: drop zones only accept matching payload types
- Drag preview: a view rendered at cursor position during drag

## View Layer (src/bin/drag_drop_test.rs)

- Three columns: "To Do", "In Progress", "Done"
- Each card is draggable (`.on_drag()` with `DraggedCard` payload)
- Drag preview: semi-transparent card clone
- Columns are drop targets (`.on_drop::<DraggedCard>()`)
- Visual feedback: column background changes on hover during drag
- `.on_drag_move()` logs position for debugging

## TDD Tests

### Drag setup (3)
1. Element with on_drag creates draggable element
2. Drag payload carries correct data
3. Drag preview view renders during drag

### Drop (3)
4. Drop on valid target fires handler with correct payload
5. Drop on non-matching type does not fire
6. Card moves from source to target column after drop

### Drag move (2)
7. on_drag_move fires during drag with position
8. DragMoveEvent contains current cursor position

### State (2)
9. Source column removes card after successful drop
10. Cancelled drag (drop outside targets) returns card to source

## Documentation (docs/gpui-usage/drag-drop.md)

### Sections
1. **What it is** — typed drag-and-drop system with custom previews and type-safe drop zones
2. **Preconditions** — `use gpui::DragMoveEvent`; payload type must be `Clone + 'static`; drag source and drop target in same window
3. **Signatures** — `.on_drag(payload, |cx| drag_view)`, `.on_drag_move(|event, cx| {})`, `.on_drop::<PayloadType>(|payload, cx| {})`, `DragMoveEvent { position, bounds }`
4. **Relevant types** — `DragMoveEvent`
5. **Usage examples** — draggable card, drop zone, drag preview, kanban move
6. **Post-conditions** — drag state is cleared on mouse up (drop or cancel); no explicit cleanup
7. **Testing** — simulate drag via mouse down + move + up sequence; verify state changes
8. **Surprises** — drag preview is a full view (can be complex but has perf cost); drop type must match exactly (no trait-based polymorphism); on_drag_move fires for ALL drags on the element, not just matching types; drag cannot cross window boundaries; no built-in drag cancel gesture (Esc must be wired manually)

**Note:** Reference Zed's `crates/gpui/examples/drag_drop.rs`.
