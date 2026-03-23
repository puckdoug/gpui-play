# DrawTest: Draggable Oval Shapes with Text

## Context

Adding the ability to create oval shapes on the DrawTest canvas, position them by dragging, and type centered text inside them. This is the first drawing primitive for the DrawTest application.

## Architecture

### Two-layer design (matching TextInput pattern)

**`ShapeState`** — pure state, unit-testable without GPUI:
- `OvalShape` struct: position (center x,y), size (rx, ry), border width, text content
- `CanvasState`: collection of shapes, selected shape index, drag state
- Operations: add_oval, select_shape_at_point, move_shape, update_shape_text
- Hit testing: point-in-ellipse math `((x-cx)/rx)² + ((y-cy)/ry)² <= 1`

**`DrawTestView`** — GPUI rendering layer:
- Canvas element for painting ovals via PathBuilder (arc_to for ellipses)
- Mouse handlers on parent div for click-to-select and drag
- Text rendered inside ovals via ShapedLine in the canvas paint callback
- Keyboard input for typing text into selected oval

### Shape data model

```rust
pub struct OvalShape {
    pub center: (f32, f32),      // position on canvas
    pub rx: f32,                  // horizontal radius
    pub ry: f32,                  // vertical radius
    pub border_width: f32,        // default 1.0
    pub text: String,             // centered text content
}

pub struct CanvasState {
    pub shapes: Vec<OvalShape>,
    pub selected: Option<usize>,  // index of selected shape
    drag_offset: Option<(f32, f32)>,  // offset from shape center to mouse
}
```

### Rendering approach

- Parent div with `.size_full()` and mouse event handlers
- Single `canvas()` child that paints all shapes
- For each oval: use `PathBuilder::stroke(border_width)` with two `arc_to()` calls to draw the ellipse outline
- For text: use `window.text_system().shape_line()` in prepaint, then `ShapedLine::paint()` centered in the oval during paint
- Selected oval gets a visual indicator (e.g., blue border or handles)

### Interaction flow

1. **⌘⇧N** — creates a new oval at canvas center with default size (100x70)
2. **Click on oval** — selects it (visual highlight)
3. **Click and drag** — moves the selected oval; uses mouse_down to detect hit + record offset, mouse_move to update position, mouse_up to end drag
4. **Type text** — when an oval is selected and focused, keyboard input goes to the oval's text field
5. **Click on empty space** — deselects

### Hit testing

Manual point-in-ellipse test in mouse_down handler:
```rust
fn point_in_oval(px: f32, py: f32, oval: &OvalShape) -> bool {
    let dx = (px - oval.center.0) / oval.rx;
    let dy = (py - oval.center.1) / oval.ry;
    (dx * dx + dy * dy) <= 1.0
}
```

### Menu integration

- New action: `NewOval` (⌘⇧N) added to DrawTest Edit menu
- Undo/Redo will track shape additions and moves

## Files to create/modify

1. **`src/draw_test.rs`** — add `NewOval` action, menu item, keybinding
2. **`src/shape.rs`** (new) — `OvalShape`, `CanvasState`, pure state logic
3. **`src/bin/draw_test.rs`** — update `DrawTestView` to use canvas + shapes
4. **`tests/shape_test.rs`** (new) — unit tests for shape state
5. **`tests/draw_test_menu.rs`** — update for NewOval menu item

## Test plan

### Pure state tests (`tests/shape_test.rs`)

1. **Oval creation**: create oval, verify center/size/border/text defaults
2. **Hit testing**: point inside oval returns true, outside returns false
3. **Hit testing edge cases**: point on boundary, elongated oval, off-center
4. **Move shape**: change position, verify new center
5. **Select shape at point**: with multiple shapes, correct one selected
6. **Select empty space**: returns None
7. **Shape text**: set text, verify content
8. **Add shape to canvas**: canvas shape count increases
9. **Undo add**: shape removed after undo
10. **Undo move**: shape returns to previous position

### Menu tests (`tests/draw_test_menu.rs`)

11. **Edit menu has NewOval**: verify menu item present
12. **NewOval enabled**: verify not disabled
13. **NewOval keybinding**: ⌘⇧N bound

## Implementation order

1. Write tests (red)
2. Create `src/shape.rs` with `OvalShape` + `CanvasState` stubs
3. Implement pure state (green)
4. Update `draw_test.rs` menus with NewOval
5. Update `DrawTestView` to render shapes via canvas
6. Add mouse interaction (select, drag)
7. Add text input to selected oval

## Verification

- `cargo test` — all tests pass
- `cargo run --bin DrawTest` — ⌘⇧N creates oval, click-drag moves it, typing adds text
