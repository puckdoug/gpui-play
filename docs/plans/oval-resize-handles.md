# Oval Resize Handles

## Design

Four cardinal-point resize handles (top, right, bottom, left) on selected ovals:
- Left/Right handles control `rx`, Top/Bottom control `ry`
- Handles appear only on selected shapes (not during text editing)
- 8x8 pixel squares at each cardinal point
- Minimum radius: 20.0 pixels
- Single undo entry per drag (begin/update/commit pattern)

## State Layer (src/shape.rs)

- `ResizeHandle` enum: `Top`, `Right`, `Bottom`, `Left`
- `OvalShape::handle_position(handle) -> (f32, f32)`
- `OvalShape::hit_test_handle(px, py, handle_radius) -> Option<ResizeHandle>`
- `OvalShape::resize(handle, px, py)` with MIN_RADIUS enforcement
- `CanvasState::hit_test_handle(px, py)` — checks selected shape only
- `CanvasState` resize methods: `begin_resize_selected`, `update_resize`, `commit_resize`
- `UndoAction::ResizeShape` variant
- `ShapeRenderData::resize_handles: Option<[(f32, f32); 4]>`

## View Layer (src/bin/draw_test.rs)

- `DrawTestView::resizing: Option<ResizeHandle>` field
- Mouse down: handle hit-test before shape body (priority)
- Mouse move: update resize during drag, hover detection for cursor
- Mouse up: commit resize
- Canvas paint: render handle squares on selected shapes
- Cursor: `ResizeLeftRight` for L/R handles, `ResizeUpDown` for T/B handles

## TDD: Red Phase Tests

See tests/shape_test.rs — ~20 tests covering:
- Handle positions
- Handle hit testing
- Resize with minimum enforcement
- Center preservation during resize
- Undo/redo for resize
- Render data with/without handles
