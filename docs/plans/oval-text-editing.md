# Oval Text Editing Plan

## Goal
Double-clicking an oval enters text editing mode. Text is centered and wraps within the oval.

## Design

### State Layer (shape.rs)
- Add `editing: Option<usize>` to `CanvasState`
- `start_editing(index)` — sets editing + selects shape
- `stop_editing()` — clears editing state
- `editing()` — returns current editing index
- `OvalShape::text_box_width()` — inscribed rectangle width (`rx * √2`) for wrap calculation

### View Layer (bin/draw_test.rs)
- **Double-click detection:** `MouseDownEvent.click_count == 2` on an oval → `start_editing()`
- **Keyboard input:** When editing, use `TextInputState` to handle text input (backspace, typing, cursor movement)
- **Exit editing:** Escape key or click outside the oval → `stop_editing()`, commit text to shape
- **Text rendering:** Use `window.text_system().shape_text()` with `wrap_width` for multi-line wrapping
- **Centering:** Use `TextAlign::Center` and vertically center the text block in the oval

### Text Wrapping Strategy
- Wrap width = `OvalShape::text_box_width()` (inscribed rectangle width)
- Use GPUI's built-in `shape_text(text, font_size, runs, Some(wrap_width), None)` which returns `SmallVec<[WrappedLine]>`
- Each `WrappedLine` is painted with `TextAlign::Center`
- Total text block height = sum of `line.size(line_height).height`
- Vertical centering: start y = `center_y - total_height / 2`

### Undo Integration
- Text changes during editing tracked by `TextInputState`'s own undo
- When editing stops, the final text is set on the shape via `set_text()`
- Shape-level undo records text changes as part of `OvalShapeData`

## Test Plan
1. Pure state tests for editing lifecycle (start/stop/query)
2. Pure state tests for `text_box_width()` calculations
3. Manual testing for double-click, text entry, wrapping, and centering
