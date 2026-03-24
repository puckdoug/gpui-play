# Canvas & Custom Drawing

**Components:** [`Canvas`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/canvas.rs#L10), [`PathBuilder`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/path_builder.rs#L86), [`Path`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/path_builder.rs#L322), [`ShapedLine`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system/line.rs#L43), [`WrappedLine`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system/line.rs#L249), [`TextRun`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system.rs#L970), [`ElementInputHandler`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/input.rs#L82), [`EntityInputHandler`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/input.rs#L10)

## What is the component and what it does

The `canvas()` element provides low-level custom drawing within GPUI's element tree. It takes two closures — prepaint and paint — that give direct access to the window's painting API. This is used for anything that can't be expressed with `div()` styling: custom shapes, vector paths, freeform drawing, data visualizations.

`PathBuilder` constructs vector paths (lines, arcs, bezier curves, polygons) that are tessellated and rendered via `window.paint_path()`. Paths can be filled or stroked with configurable width.

Text inside custom-painted shapes is rendered via `ShapedLine` (single-line) or `WrappedLine` (multi-line with word wrapping) — the same text system used by text input, but positioned manually. For editable text in canvas shapes, `EntityInputHandler` and `ElementInputHandler` connect the OS input system (IME, keyboard) to your canvas.

## Signature for usage

### canvas() element

```rust
canvas(
    prepaint: impl FnOnce(Bounds<Pixels>, &mut Window, &mut App) -> T,
    paint: impl FnOnce(Bounds<Pixels>, T, &mut Window, &mut App),
) -> Canvas<T>
```

- **prepaint** runs during layout, returns data `T` for paint phase
- **paint** receives bounds and prepaint data, draws into the window
- Canvas itself has no event handlers — attach mouse events to a parent div

### PathBuilder

```rust
// Stroked path (outline)
PathBuilder::stroke(width: Pixels) -> PathBuilder

// Filled path (solid)
PathBuilder::fill() -> PathBuilder

// Path operations
builder.move_to(point: Point<Pixels>)
builder.line_to(point: Point<Pixels>)
builder.arc_to(
    radii: Point<Pixels>,    // x and y radii for ellipse
    x_rotation: Pixels,      // rotation in degrees
    large_arc: bool,          // use arc > 180°
    sweep: bool,              // clockwise direction
    to: Point<Pixels>,        // end point
)
builder.cubic_bezier_to(to, control_a, control_b)
builder.close()
builder.build() -> Result<Path<Pixels>>
```

### Painting

```rust
// Paint a vector path
window.paint_path(path: Path<Pixels>, color: impl Into<Background>)

// Paint a rectangle
window.paint_quad(quad: PaintQuad)

// Paint single-line text (via ShapedLine)
let line = window.text_system().shape_line(text, font_size, &runs, None);
line.paint(origin, line_height, TextAlign::Left, None, window, cx)

// Paint wrapped multi-line text (via WrappedLine)
let lines = window.text_system().shape_text(text, font_size, &runs, Some(wrap_width), None)?;
for line in &lines {
    line.paint(origin, line_height, TextAlign::Center, Some(bounds), window, cx)?;
    origin.y += line.size(line_height).height;
}

// Register text input handler inside canvas paint closure
window.handle_input(&focus_handle, ElementInputHandler::new(bounds, entity), cx);
```

## Relevant Macros

None specific to canvas.

## Relevant Traits

### `Styled`

`Canvas<T>` implements `Styled`, so you can size it with `.size_full()`, `.w_full()`, `.h(px(300.))`, etc.

## Usage and examples

### Drawing an ellipse/oval

An ellipse is drawn with two `arc_to()` calls — one for each half:

```rust
let center = point(px(cx), px(cy));
let radii = point(px(rx), px(ry));
let right = point(center.x + px(rx), center.y);
let left = point(center.x - px(rx), center.y);

let mut builder = PathBuilder::stroke(px(1.0));
builder.move_to(right);
builder.arc_to(radii, px(0.0), false, true, left);   // top half
builder.arc_to(radii, px(0.0), false, true, right);  // bottom half

if let Ok(path) = builder.build() {
    window.paint_path(path, rgb(0x000000));
}
```

### Canvas with mouse interaction (draggable shapes)

Canvas doesn't handle events — attach handlers to a parent div:

```rust
div()
    .size_full()
    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
    .on_mouse_move(cx.listener(Self::on_mouse_move))
    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
    .child(
        canvas(
            |_bounds, _window, _cx| {},
            move |_bounds, _, window, _cx| {
                // paint shapes here
            },
        )
        .size_full(),
    )
```

Mouse event positions are in window coordinates (`event.position: Point<Pixels>`). Convert to `f32` with `f32::from(event.position.x)`.

### Hit testing for custom shapes

GPUI provides no automatic hit testing for canvas-drawn shapes. Implement it manually in mouse event handlers:

```rust
// Point-in-ellipse test
fn contains_point(px: f32, py: f32, cx: f32, cy: f32, rx: f32, ry: f32) -> bool {
    let dx = (px - cx) / rx;
    let dy = (py - cy) / ry;
    (dx * dx + dy * dy) <= 1.0
}
```

For selection among multiple shapes, iterate in reverse (topmost first):

```rust
let selected = shapes.iter().enumerate().rev()
    .find(|(_, shape)| shape.contains_point(mx, my))
    .map(|(i, _)| i);
```

### Rendering text centered in a shape (single-line)

Text must be shaped and painted manually inside canvas callbacks:

```rust
let style = window.text_style();
let font_size = style.font_size.to_pixels(window.rem_size());
let run = TextRun {
    len: text.len(),
    font: style.font(),
    color: style.color,
    background_color: None,
    underline: None,
    strikethrough: None,
};
let display_text: SharedString = text.into();
let shaped = window.text_system().shape_line(display_text, font_size, &[run], None);

// Center the text in the shape
let text_origin = point(
    center.x - shaped.width() / 2.0,
    center.y - window.line_height() / 2.0,
);
shaped.paint(text_origin, window.line_height(), TextAlign::Left, None, window, cx).ok();
```

### Rendering wrapped centered text in a shape (multi-line)

Use `shape_text()` instead of `shape_line()` to get word-wrapped lines. Pass a `wrap_width` to control where text breaks. Each `WrappedLine` can span multiple visual lines if wrapping occurs.

```rust
let style = window.text_style();
let font_size = style.font_size.to_pixels(window.rem_size());
let run = TextRun {
    len: text.len(),
    font: style.font(),
    color: style.color,
    background_color: None,
    underline: None,
    strikethrough: None,
};
let display_text: SharedString = text.into();
let wrap_width = px(text_box_width);

if let Ok(lines) = window.text_system().shape_text(
    display_text,
    font_size,
    &[run],
    Some(wrap_width),
    None, // line_clamp: Option<usize> — pass Some(n) to limit lines
) {
    let line_height = window.line_height();

    // Calculate total height for vertical centering
    let total_height: Pixels = lines.iter().map(|l| l.size(line_height).height).sum();

    let text_origin = point(
        center.x - wrap_width / 2.0,
        center.y - total_height / 2.0,
    );

    // Create bounds for TextAlign::Center to align against
    let text_bounds = Bounds::new(text_origin, size(wrap_width, total_height));

    let mut y = text_origin.y;
    for line in &lines {
        let line_origin = point(text_origin.x, y);
        line.paint(
            line_origin,
            line_height,
            TextAlign::Center,
            Some(text_bounds), // bounds for alignment reference
            window,
            cx,
        ).ok();
        y += line.size(line_height).height;
    }
}
```

**Key differences from `shape_line()`:**
- `shape_text()` returns `Result<SmallVec<[WrappedLine; 1]>>` — multiple lines if text contains `\n`
- Each `WrappedLine` may itself span multiple visual lines via wrap boundaries
- `WrappedLine.size(line_height).height` accounts for wrapped sub-lines (height = `line_height × (wrap_boundaries + 1)`)
- `TextAlign::Center` requires a `bounds` parameter to know the alignment width — pass `Some(bounds)` or it falls back to `wrap_width`

### Extracting shape data for canvas closures

Canvas paint closures must be `'static` (they outlive the render call). You cannot borrow view state directly. Extract shape data into an owned struct before passing to the closure:

```rust
#[derive(Clone)]
struct ShapeRenderData {
    cx: f32, cy: f32, rx: f32, ry: f32,
    border_width: f32, selected: bool, text: String,
}

// In render():
let shapes: Vec<ShapeRenderData> = self.canvas_state.shapes()
    .iter().enumerate()
    .map(|(i, s)| { /* extract owned data */ })
    .collect();

canvas(
    move |_, _, _| {},
    move |_, _, window, cx| {
        for shape in &shapes {  // shapes is moved into closure
            // paint...
        }
    },
)
```

### Inline text editing in canvas shapes

To make canvas-drawn shapes editable (e.g., double-click to type), you need to:

1. **Track editing state** — which shape is being edited
2. **Use `TextInputState`** — reuse the text editing model for cursor, selection, undo/redo
3. **Implement `EntityInputHandler`** on the view — connects OS text input to your editing state
4. **Call `window.handle_input()` in the canvas paint closure** — registers the input handler with proper bounds
5. **Detect double-click** — `MouseDownEvent.click_count == 2`

```rust
// In render(), clone entity and focus handle for the 'static closure
let entity = cx.entity().clone();
let focus = self.focus_handle.clone();
let is_editing = self.canvas_state.editing().is_some();

canvas(
    move |_bounds, _window, _cx| {},
    move |bounds, _, window, cx| {
        // Register input handler INSIDE the canvas paint closure
        if is_editing {
            window.handle_input(
                &focus,
                ElementInputHandler::new(bounds, entity.clone()),
                cx,
            );
        }
        // ... paint shapes ...
    },
)
```

The view must also implement `EntityInputHandler` to forward OS input events to the `TextInputState`:

```rust
impl EntityInputHandler for DrawTestView {
    fn replace_text_in_range(&mut self, range_utf16, new_text, window, cx) {
        if let Some(ref mut state) = self.editing_state {
            let range = range_utf16.map(|r| state.range_from_utf16(&r))
                .unwrap_or_else(|| state.selected_range());
            state.replace_range(range, new_text);
            cx.notify();
        }
    }
    // ... other required methods delegate to TextInputState ...
}
```

### Mouse click dispatch for editing shapes

`MouseDownEvent` includes a `click_count: usize` field populated by the platform. When a shape supports text editing, mouse dispatch needs two layers: editing-mode clicks (cursor positioning, word/all selection) and non-editing clicks (shape selection, drag, enter editing).

```rust
fn on_mouse_down(&mut self, event: &MouseDownEvent, window, cx) {
    // When currently editing, handle clicks on the editing shape first
    if let Some(editing_idx) = self.canvas_state.editing() {
        let on_editing_shape = self.canvas_state.shapes()[editing_idx]
            .contains_point(mx, my);

        if on_editing_shape {
            if event.click_count >= 3 {
                // Triple-click: select all text
                self.editing_state.as_mut().unwrap().select_all();
                return;
            }
            if event.click_count == 2 {
                // Double-click: select word at click position
                let offset = self.hit_test_text(event.position, window);
                self.editing_state.as_mut().unwrap().select_word_at(offset);
                return;
            }
            // Single click: position cursor
            let offset = self.hit_test_text(event.position, window);
            self.editing_state.as_mut().unwrap().move_to(offset);
            return;
        }
        // Clicked outside editing shape — commit and exit
        self.commit_editing();
    }

    // Not editing: double-click enters editing, single-click selects/drags
    if event.click_count == 2 {
        self.canvas_state.select_at(mx, my);
        if let Some(idx) = self.canvas_state.selected() {
            self.start_editing(idx, cx);
            return;
        }
    }
    // ... normal select + drag ...
}
```

**Key points:**
- Check editing-mode clicks first, before non-editing dispatch
- `click_count >= 3` catches triple-click (and beyond)
- Single-click on the editing shape positions cursor; single-click *outside* commits editing
- `on_mouse_down` fires for every click in a multi-click sequence: click_count 1, then 2, then 3

### Click-to-position in wrapped text (hit testing)

Map a window pixel position to a byte offset using `closest_index_for_position`. When text is painted with `TextAlign::Center`, you must subtract the per-row centering offset from the click x before hit-testing, since layout coordinates don't include centering.

```rust
fn hit_test_text(&self, position: Point<Pixels>, window: &mut Window) -> usize {
    // ... shape_text() to get wrapped lines, compute text_origin ...

    let mut local = point(position.x - text_origin.x, position.y - text_origin.y);

    if let Some(first_line) = lines.first() {
        // Subtract per-row centering offset
        let rows = row_layout_info(first_line, wrap_width, line_height);
        let clicked_row = (f32::from(local.y) / f32::from(line_height)).max(0.0) as usize;
        if let Some(&(_, _, center_offset)) = rows.get(clicked_row) {
            local.x -= center_offset;
        }

        match first_line.closest_index_for_position(local, line_height) {
            Ok(idx) | Err(idx) => idx,
        }
    } else { 0 }
}
```

`closest_index_for_position` returns `Result<usize, usize>` — `Ok` when the point is within the text, `Err` when it's outside but returning the nearest boundary. Use both variants.

### Cursor blinking

GPUI has no built-in cursor blink. Implement with `cx.background_executor().timer()` and an epoch counter to cancel stale blink tasks:

```rust
const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

fn show_cursor(&mut self, cx: &mut Context<Self>) {
    self.cursor_visible = true;
    self.blink_epoch += 1;  // cancel any pending blink
    let epoch = self.blink_epoch;
    cx.spawn(async move |this, cx| {
        cx.background_executor().timer(CURSOR_BLINK_INTERVAL).await;
        if let Some(this) = this.upgrade() {
            this.update(cx, |this, cx| this.blink_cursor(epoch, cx));
        }
    }).detach();
    cx.notify();
}

fn blink_cursor(&mut self, epoch: usize, cx: &mut Context<Self>) {
    if epoch != self.blink_epoch { return; }  // stale
    self.cursor_visible = !self.cursor_visible;
    cx.notify();
    // Schedule next blink with same epoch...
}
```

Call `show_cursor()` after every user input to reset the blink (cursor stays visible during typing).

### Two-layer architecture for testability

Separate pure state from GPUI rendering (same pattern as TextInput):

- **`CanvasState`** — owns shapes, handles selection, move, undo/redo. Unit-testable.
- **`DrawTestView`** — GPUI view that holds `CanvasState`, renders via canvas, handles mouse/action events.

This allows testing all shape logic (hit testing, undo/redo, selection) without GPUI context.

### `Pixels` field is private

`Pixels` wraps an `f32` but the field is private. Use `f32::from(pixels)` to extract the value, and `px(value)` to create one. This affects mouse event positions:

```rust
let mx: f32 = f32::from(event.position.x);
let my: f32 = f32::from(event.position.y);
```

## Surprises, Anti-patterns, and Bugs

### Canvas has no event handling

`canvas()` elements cannot receive mouse or keyboard events. You must attach all event handlers to a parent `div()` and perform manual hit testing against your shapes. This is fundamentally different from DOM-based frameworks where each visual element can be interactive.

### Paint closures must be `'static`

Both the prepaint and paint closures passed to `canvas()` must own all their data. You cannot borrow `&self` or any view state. Extract all needed data into owned `Vec`s or structs before creating the canvas element. This means shape data is copied every render cycle.

### `PathBuilder::build()` returns `Result`

Path building can fail (e.g., degenerate paths). Always handle the `Result`:

```rust
if let Ok(path) = builder.build() {
    window.paint_path(path, color);
}
```

### No automatic repainting

After modifying shapes (add, move, delete), you must call `cx.notify()` to trigger a re-render. The canvas doesn't observe state changes automatically.

### `handle_input` must be called inside the canvas paint closure, not in a sibling element

To accept text input in a canvas, `window.handle_input()` must be called with bounds that have non-zero area. If you try to create a separate sibling element (e.g., in a `flex_col` alongside the canvas), the canvas will consume all available space via `size_full()`, leaving the sibling element with zero-sized bounds — and `handle_input` silently fails.

**Wrong** — sibling element gets zero bounds:
```rust
div().flex().flex_col().child(
    canvas(...).size_full(),  // takes all space
).child(
    input_element  // gets 0×0 bounds → handle_input fails
)
```

**Right** — call `handle_input` inside the canvas paint closure:
```rust
let entity = cx.entity().clone();  // clone for 'static closure
let focus = self.focus_handle.clone();

canvas(
    move |_, _, _| {},
    move |bounds, _, window, cx| {
        window.handle_input(&focus, ElementInputHandler::new(bounds, entity.clone()), cx);
        // ... paint shapes ...
    },
)
```

### Text baseline positioning

`ShapedLine::paint()` takes an origin where `y` is the **top** of the line, not the baseline. To vertically center text in a shape, use `center.y - line_height / 2.0`.

### `shape_text()` vs `shape_line()` — wrapping vs single-line

- `shape_line()` returns a single `ShapedLine` — no wrapping, no newline handling. Use for single-line labels.
- `shape_text()` returns `Result<SmallVec<[WrappedLine; 1]>>` — handles `\n` as line breaks, wraps at `wrap_width`. Use for text that needs to fit within a constrained area.

`shape_text()` takes two extra parameters: `wrap_width: Option<Pixels>` and `line_clamp: Option<usize>`. When `wrap_width` is `None`, lines only break at `\n`. When `line_clamp` is `Some(n)`, at most `n` visual lines are produced.

### `TextAlign::Center` creates a coordinate system mismatch

This is the most important gotcha for interactive text in canvas shapes. When you paint with `TextAlign::Center`, GPUI shifts each wrapped row visually by `(wrap_width - row_width) / 2`. However, `position_for_index()` and `closest_index_for_position()` on `WrappedLineLayout` return/expect coordinates in **layout space** — relative to the left edge of each row, *without* any centering offset.

This means:
- **Cursor rendering**: `position_for_index(offset)` gives you `(x, y)` but `x` doesn't account for centering. You must add the per-row centering offset.
- **Click hit-testing**: `closest_index_for_position(local_point)` expects `x` in layout space. You must subtract the per-row centering offset from the click position.
- **Selection highlighting**: Same as cursor — add per-row offset to highlight rectangle positions.

Compute per-row centering offsets from wrap boundaries:

```rust
fn row_layout_info(
    layout: &WrappedLineLayout,
    wrap_width: Pixels,
    line_height: Pixels,
) -> Vec<(usize, Pixels, Pixels)> {  // (row_byte_start, row_width, center_offset)
    let mut row_starts: Vec<usize> = vec![0];
    for wb in layout.wrap_boundaries() {
        row_starts.push(layout.runs()[wb.run_ix].glyphs[wb.glyph_ix].index);
    }
    let text_len = layout.len();

    row_starts.iter().enumerate().map(|(i, &start)| {
        let end = row_starts.get(i + 1).copied().unwrap_or(text_len);
        let row_width = layout.position_for_index(end, line_height)
            .map(|p| p.x).unwrap_or(wrap_width);
        let center_offset = (wrap_width - row_width) / 2.0;
        (start, row_width, center_offset)
    }).collect()
}
```

If you only need simple centered text display (no cursor/selection/click), `TextAlign::Center` works fine. The mismatch only matters when you need to map between pixel positions and byte offsets.

### `TextAlign::Center` needs explicit bounds

When painting a `WrappedLine` with `TextAlign::Center`, the alignment width comes from:
1. The `bounds` parameter passed to `paint()` (if `Some`)
2. Falling back to the `wrap_width` used during shaping

If you pass `None` for bounds and didn't use a `wrap_width`, centering has no reference width and behaves like `Left`. Always pass `Some(bounds)` when centering wrapped text.

### `position_for_index` at wrap boundaries reports end-of-previous-row

When calling `position_for_index(byte_index, line_height)` where `byte_index` is exactly at a wrap boundary, the returned position is at the **end of the previous row** (large x, previous row's y), not at the start of the next row (x=0, next row's y). This is because the internal loop uses `index <= line_end_ix`, so the boundary byte is considered part of the previous line.

This affects selection highlighting: if a selection starts at a wrap boundary, `position_for_index` reports its position on the wrong row, causing the highlight to bleed into the line above.

**Workaround**: Don't use `position_for_index` at wrap boundary indices for selection rendering. Instead, compute row byte ranges from `wrap_boundaries()` and for each row, only use `position_for_index` for indices strictly within the row (not at the boundary). For the start of a row, use `x = 0` directly:

```rust
let sel_start_in_row = sel.start.max(row_byte_start);
let left_x = if sel_start_in_row == row_byte_start {
    px(0.0)  // don't call position_for_index at row boundary
} else {
    first_line.position_for_index(sel_start_in_row, line_height)
        .map(|p| p.x).unwrap_or(px(0.0))
};
```

### Undo/redo for canvas operations needs shape snapshots

Unlike text input where undo stores string snapshots, canvas undo must store full shape data (position, size, properties) for each operation. The `AddShape` undo action must store the shape data so it can be re-inserted on redo.
