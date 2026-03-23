# Canvas & Custom Drawing

**Components:** [`Canvas`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/canvas.rs#L10), [`PathBuilder`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/path_builder.rs#L86), [`Path`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/path_builder.rs#L322), [`ShapedLine`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system/line.rs#L43), [`TextRun`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system.rs#L970)

## What is the component and what it does

The `canvas()` element provides low-level custom drawing within GPUI's element tree. It takes two closures — prepaint and paint — that give direct access to the window's painting API. This is used for anything that can't be expressed with `div()` styling: custom shapes, vector paths, freeform drawing, data visualizations.

`PathBuilder` constructs vector paths (lines, arcs, bezier curves, polygons) that are tessellated and rendered via `window.paint_path()`. Paths can be filled or stroked with configurable width.

Text inside custom-painted shapes is rendered via `ShapedLine` — the same system used by text input, but positioned manually.

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

// Paint text (via ShapedLine)
let line = window.text_system().shape_line(text, font_size, &runs, None);
line.paint(origin, line_height, TextAlign::Left, None, window, cx)
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

### Rendering text centered in a shape

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

### Text baseline positioning

`ShapedLine::paint()` takes an origin where `y` is the **top** of the line, not the baseline. To vertically center text in a shape, use `center.y - line_height / 2.0`.

### Undo/redo for canvas operations needs shape snapshots

Unlike text input where undo stores string snapshots, canvas undo must store full shape data (position, size, properties) for each operation. The `AddShape` undo action must store the shape data so it can be re-inserted on redo.
