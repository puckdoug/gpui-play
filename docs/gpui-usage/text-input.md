# Text Input

**Components:** [`TextInputState`](https://github.com/puckdoug/gpui-play/blob/main/src/text_input.rs#L22) (local), [`TextInput`](https://github.com/puckdoug/gpui-play/blob/main/src/text_input.rs#L306) (local), [`TextInputElement`](https://github.com/puckdoug/gpui-play/blob/main/src/text_input.rs#L606) (local), [`UndoEntry`](https://github.com/puckdoug/gpui-play/blob/main/src/text_input.rs#L14) (local), [`EntityInputHandler`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/input.rs#L10), [`ElementInputHandler`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/input.rs#L82), [`FocusHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs#L334), [`ShapedLine`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system/line.rs#L43), [`TextRun`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system.rs#L970), [`UTF16Selection`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs#L1103)

## What is the component and what it does

GPUI does **not** provide a built-in text input widget. To create an editable text field, you must implement the full text input stack yourself:

1. A **model** struct holding text content, cursor position, selection range, and focus state
2. The **`EntityInputHandler`** trait (8 methods) to integrate with the OS input system (IME, clipboard, text ranges)
3. A custom **`Element`** that handles layout, text shaping, cursor/selection rendering via `ShapedLine`
4. A **`Render`** impl with focus tracking, keyboard action handlers, and mouse event handlers
5. The **`Focusable`** trait to participate in the focus system

The reference implementation is `examples/input.rs` in the GPUI source (~770 lines for a single-line input with selection, IME, clipboard, and mouse interaction).

## Signature for usage

### Model struct (two-layer architecture)

The text input uses a separated architecture for testability:

**`TextInputState`** — pure state, no GPUI dependencies, unit-testable:

```rust
pub struct TextInputState {
    content: String,
    selected_range: Range<usize>,      // UTF-8 byte offsets
    selection_reversed: bool,
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
}
```

**`TextInput`** — GPUI view wrapping the state:

```rust
pub struct TextInput {
    focus_handle: FocusHandle,
    state: TextInputState,              // delegates buffer ops here
    placeholder: SharedString,
    marked_range: Option<Range<usize>>, // IME composition
    last_layout: Option<ShapedLine>,    // cached for hit testing
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,                 // mouse drag state
}
```

This separation allows unit testing of all buffer operations (insert, delete, cursor movement, undo/redo, UTF-16 conversion) without GPUI context.

### EntityInputHandler (8 required methods)

```rust
impl EntityInputHandler for TextInput {
    // Read text in a UTF-16 range
    fn text_for_range(&mut self, range_utf16: Range<usize>, actual_range: &mut Option<Range<usize>>, window, cx) -> Option<String>;

    // Current cursor/selection position (UTF-16)
    fn selected_text_range(&mut self, ignore_disabled: bool, window, cx) -> Option<UTF16Selection>;

    // IME marked (composing) text range
    fn marked_text_range(&self, window, cx) -> Option<Range<usize>>;

    // Clear IME marked text
    fn unmark_text(&mut self, window, cx);

    // Insert/delete text at range (or selection if None)
    fn replace_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, window, cx);

    // Insert text and mark it for IME composition
    fn replace_and_mark_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, new_selected_range_utf16: Option<Range<usize>>, window, cx);

    // Map text range to screen coordinates (for IME popup positioning)
    fn bounds_for_range(&mut self, range_utf16: Range<usize>, bounds: Bounds<Pixels>, window, cx) -> Option<Bounds<Pixels>>;

    // Map screen point to text offset (for mouse click)
    fn character_index_for_point(&mut self, point: Point<Pixels>, window, cx) -> Option<usize>;
}
```

### Custom Element (3 phases)

```rust
impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    // Phase 1: request layout — set width=100%, height=line_height
    fn request_layout(&mut self, ...) -> (LayoutId, ());

    // Phase 2: prepaint — shape text, compute cursor/selection quads
    fn prepaint(&mut self, ..., bounds: Bounds<Pixels>, ...) -> PrepaintState;

    // Phase 3: paint — register input handler, paint selection, text, cursor
    fn paint(&mut self, ..., bounds: Bounds<Pixels>, ...);
}
```

### Key paint-phase call

```rust
// Register this element as the input handler for the focused element
window.handle_input(
    &focus_handle,
    ElementInputHandler::new(bounds, self.input.clone()),
    cx,
);
```

### Text shaping and painting

```rust
// Shape text into a line for rendering
let line = window.text_system().shape_line(display_text, font_size, &runs, None);

// Paint the shaped line
line.paint(bounds.origin, window.line_height(), TextAlign::Left, None, window, cx).unwrap();

// Get pixel position for a text offset (cursor positioning)
let x = line.x_for_index(cursor_offset);

// Get text offset for a pixel position (mouse click)
let index = line.index_for_x(x_position);
```

## Relevant Macros

### `actions!()`

Text input needs keyboard actions:

```rust
actions!(text_input, [
    Backspace, Delete, Left, Right,
    SelectLeft, SelectRight, SelectAll,
    Home, End, ShowCharacterPalette,
    Paste, Cut, Copy,
]);
```

These must be bound via `cx.bind_keys()` and registered on the view with `.on_action(cx.listener(...))`.

## Relevant Traits

### `EntityInputHandler`

The core trait connecting your text model to the OS input system. All 8 methods are called by the platform's IME/input pipeline. You cannot omit any of them.

### `Element`

Custom element trait for rendering. Required because text input needs low-level control over text shaping, cursor painting, and input handler registration that `div()` elements don't provide.

### `IntoElement`

Trivial wrapper:
```rust
impl IntoElement for TextElement {
    type Element = Self;
    fn into_element(self) -> Self::Element { self }
}
```

### `Focusable`

Required for keyboard input routing:
```rust
impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```

### `Render`

The view's render method must:
- Set `.key_context("TextInput")` for scoped keybindings
- Call `.track_focus(&self.focus_handle(cx))`
- Set `.cursor(CursorStyle::IBeam)`
- Register all action listeners with `.on_action(cx.listener(...))`
- Register mouse handlers for click-to-position and drag-to-select
- Include the custom `TextElement` as a child

## Usage and examples

### Creating a text input with initial content

```rust
let text_input = cx.new(|cx| TextInput {
    focus_handle: cx.focus_handle(),
    content: "Initial text".into(),
    placeholder: "Type here...".into(),
    selected_range: 0..0,
    selection_reversed: false,
    marked_range: None,
    last_layout: None,
    last_bounds: None,
    is_selecting: false,
});
```

### Creating an empty text input

```rust
let text_input = cx.new(|cx| TextInput {
    focus_handle: cx.focus_handle(),
    content: "".into(),
    placeholder: "Placeholder text...".into(),
    selected_range: 0..0,
    // ... same defaults
});
```

### Embedding in a view

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.text_input_1.clone())
            .child(self.text_input_2.clone())
    }
}
```

### Required keybindings

```rust
cx.bind_keys([
    KeyBinding::new("backspace", Backspace, None),
    KeyBinding::new("delete", Delete, None),
    KeyBinding::new("left", Left, None),
    KeyBinding::new("right", Right, None),
    KeyBinding::new("shift-left", SelectLeft, None),
    KeyBinding::new("shift-right", SelectRight, None),
    KeyBinding::new("cmd-a", SelectAll, None),
    KeyBinding::new("cmd-v", Paste, None),
    KeyBinding::new("cmd-c", Copy, None),
    KeyBinding::new("cmd-x", Cut, None),
    KeyBinding::new("home", Home, None),
    KeyBinding::new("end", End, None),
    KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
]);
```

## Surprises, Anti-patterns, and Bugs

### No built-in text input widget

This is the biggest surprise. Most UI frameworks provide a text field. GPUI requires ~400 lines of code for a minimal single-line input. Plan accordingly — build it as a reusable component in a shared module.

### UTF-8 vs UTF-16 mismatch

Rust strings are UTF-8. The OS input system (macOS IME) uses UTF-16 offsets. You must convert between them in every `EntityInputHandler` method. The reference implementation includes `offset_to_utf16()` and `offset_from_utf16()` helpers. Getting this wrong breaks emoji, CJK input, and any multi-byte characters.

### `window.handle_input()` must be called during paint

The `ElementInputHandler` must be registered during the `paint` phase of your custom element. This is what connects your `EntityInputHandler` to the OS input pipeline. Without it, typing produces no input.

### Grapheme boundaries required for correct cursor movement

Using byte offsets for cursor movement breaks on multi-byte characters and grapheme clusters (e.g., emoji with skin tone modifiers). The reference implementation uses `unicode_segmentation::UnicodeSegmentation` for correct boundary detection. Add `unicode-segmentation` to your dependencies.

### Focus must be explicitly set

A text input doesn't receive keyboard input until focused. Use `window.focus(&handle, cx)` to set initial focus, and `.track_focus()` in the render tree to maintain it.

### macOS IME log noise

When using emoji input or other IME features, macOS may print harmless errors to stderr like:

```
error messaging the mach port for IMKCFRunLoopWakeUpReliable
```

This is macOS system noise, not a GPUI or application bug. It appears in many macOS apps including Zed. Ignore it.

### Text overflows the container without `overflow_hidden()`

The custom `TextInputElement` renders text via `ShapedLine::paint()` which paints at the given origin with no built-in clipping. If the text is wider than the container, it overflows visually. The fix is to add `.overflow_hidden()` on the wrapper div that contains the `TextInputElement`:

```rust
div()
    .w_full()
    .overflow_hidden()  // clips text that exceeds container width
    .child(TextInputElement { input: cx.entity() })
```

GPUI's overflow control follows CSS conventions: `overflow_hidden()` clips content at the element's bounds. Without it, elements paint beyond their layout bounds.

### Placeholder text is rendered by the same pipeline

When content is empty, the reference implementation renders the placeholder with a dimmed color (`hsla(0., 0., 0., 0.2)`) using the same `ShapedLine` system. There's no separate placeholder mechanism.
