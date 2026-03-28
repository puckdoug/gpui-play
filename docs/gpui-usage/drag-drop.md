# Drag and Drop

**Components:** [`on_drag`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/div.rs), [`on_drop`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/div.rs), [`DragMoveEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs)

## What is the component and what it does

GPUI provides typed drag-and-drop between elements. A draggable element carries a typed payload and renders a custom drag preview. Drop zones accept payloads of specific types — type safety is enforced at compile time.

## Preconditions for use

```rust
use gpui::{InteractiveElement, StatefulInteractiveElement, AppContext};
```

- Draggable element must have `.id()` (stateful) — `InteractiveElement` provides `.id()`
- `StatefulInteractiveElement` provides `.on_drag()`, `.on_drop()`, `.on_drag_move()`
- Payload type must be `'static`
- Drag preview must implement `Render`

## Signature for usage

### Making an element draggable

```rust
div()
    .id("my-card")
    .on_drag(
        payload_value,                                        // T: 'static
        |payload: &T, position: Point<Pixels>, window: &mut Window, cx: &mut App| {
            cx.new(|_cx| DragPreviewView { ... })             // Returns Entity<W: Render>
        },
    )
```

### Drop zone

```rust
div()
    .id("drop-target")
    .on_drop::<PayloadType>(cx.listener(|this, payload: &PayloadType, window, cx| {
        // Handle the drop
    }))
```

### Drag move tracking

```rust
div()
    .id("draggable")
    .on_drag_move::<PayloadType>(|event, window, cx| {
        let position = event.event.position;  // Current mouse position
        let bounds = event.bounds;             // Element bounds
        let payload = event.drag(cx);          // &PayloadType
    })
```

### DragMoveEvent<T>

```rust
pub struct DragMoveEvent<T> {
    pub event: MouseMoveEvent,     // position, pressed_button, modifiers
    pub bounds: Bounds<Pixels>,    // Element bounds
}

impl<T> DragMoveEvent<T> {
    pub fn drag<'b>(&self, cx: &'b App) -> &'b T    // Get typed payload
}
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `InteractiveElement` | Provides `.id()` — required for drag/drop |
| `StatefulInteractiveElement` | Provides `.on_drag()`, `.on_drop()`, `.on_drag_move()` |
| `Render` | Drag preview view must implement this |

## Usage and examples

### Complete drag and drop

```rust
#[derive(Clone)]
struct CardPayload { id: usize, title: String }

struct DragPreview { title: String }
impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(gpui::blue()).text_color(gpui::white()).p_2()
            .child(self.title.clone())
    }
}

impl Render for KanbanView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().gap_4()
            // Draggable card
            .child(
                div().id("card-1").p_4().child("Drag me")
                    .on_drag(
                        CardPayload { id: 1, title: "Card 1".into() },
                        |payload: &CardPayload, _pos, _window, cx: &mut App| {
                            cx.new(|_cx| DragPreview { title: payload.title.clone() })
                        },
                    )
            )
            // Drop zone
            .child(
                div().id("drop-zone").size_full().child("Drop here")
                    .on_drop::<CardPayload>(cx.listener(
                        |_this: &mut Self, payload: &CardPayload, _window, _cx| {
                            println!("Dropped: {}", payload.title);
                        },
                    ))
            )
    }
}
```

## Post-conditions / destruction requirements

- Drag state is cleared on mouse up (drop or cancel)
- No explicit cleanup needed
- Drag preview view is dropped when the drag ends

## Testing

```rust
#[gpui::test]
fn test_drag_drop(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| KanbanView { ... });
}
```

Run tests: `cargo test --test drag_drop_test`

## Surprises, Anti-patterns, and Bugs

### Drop type must match exactly

`on_drop::<CardPayload>()` only accepts `CardPayload`. There is no trait-based polymorphism — you cannot drop a `dyn Droppable`.

### Drag preview is a full view

The `on_drag` constructor creates an `Entity<W: Render>`. This is a complete view, rendered at the cursor position. Complex previews have a performance cost.

### `on_drag_move` fires for ALL drags

`on_drag_move::<T>` fires when any drag of type `T` moves over the element, not just drags originating from it.

### Drag cannot cross window boundaries

Drag and drop is within a single window. You cannot drag between GPUI windows.

### No built-in cancel gesture

There is no built-in Escape-to-cancel for drags. If needed, you must wire this up yourself using key event handlers.

### `cx.listener()` for on_drop in views

When using `on_drop` inside a `Render` implementation, wrap the handler with `cx.listener()` to get access to `&mut Self`.
