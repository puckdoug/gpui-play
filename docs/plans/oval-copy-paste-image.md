# Copy/Paste Oval Shapes

## Design

Copy one or more selected ovals (with text) to the clipboard for pasting back into the canvas. Pasted shapes preserve relative spacing, sizes, and text.

### Multi-select

- `select_at(px, py)` — single-click selects one shape, clears others
- `toggle_selection_at(px, py)` — shift-click adds/removes a shape from selection
- `selected()` returns `Option<usize>` (first selected, backwards-compatible)
- `selected_indices()` returns `&[usize]` (all selected)

### Clipboard

- **Cmd-C on selected shapes (not editing)**: Serialize all selected shapes as JSON array with metadata tag `"gpui-play-shape"`.
- **Cmd-V (Paste)**: Check for `gpui-play-shape` metadata first (internal paste = duplicate shapes with offset, preserving relative spacing). Fall back to text paste when editing.
- **Cmd-X on selected shapes (not editing)**: Copy shapes then delete them.

## State Layer (src/shape.rs)

- `OvalShape::to_json() -> String` / `OvalShape::from_json(&str) -> Option<Self>` (serde)
- `CanvasState::toggle_selection_at(px, py)` — add/remove shape from multi-selection
- `CanvasState::selected_indices() -> &[usize]` — all selected shape indices
- `CanvasState::copy_selected() -> Option<String>` — serialize all selected shapes as JSON array
- `CanvasState::paste_shapes(json)` — deserialize and add shapes with offset, preserving relative positions
- `CanvasState::delete_selected()` — remove all selected shapes with single undo entry
- `UndoAction::DeleteShapes` / `PasteShapes` variants

## View Layer (src/bin/draw_test.rs)

- Shift-click calls `toggle_selection_at` instead of `select_at`
- Modify `on_copy`: if not editing, copy all selected shapes as JSON with metadata
- Modify `on_paste`: if not editing, check for shape metadata, paste shapes
- Modify `on_cut`: if not editing, copy shapes then delete

## Dependencies

- `serde` + `serde_json` for shape serialization

## TDD Tests

### Serialization (3)
1. Shape JSON roundtrip preserves all fields
2. Custom size preserved
3. Invalid JSON returns None

### Multi-select (4)
4. toggle_selection_at adds shape to selection
5. toggle_selection_at removes already-selected shape
6. select_at clears multi-selection
7. selected() returns first from multi-selection

### Copy (3)
8. Single shape copy returns JSON
9. No selection returns None
10. Multiple shapes copy includes all

### Paste (8)
11. Single shape paste adds shape
12. Paste offsets position
13. Multiple paste preserves spacing between shapes
14. Multiple paste preserves sizes
15. Multiple paste preserves text
16. Paste selects only new shapes
17. Paste is undoable (single undo removes all pasted)
18. Invalid JSON is no-op

### Delete (6)
19. Single delete removes shape
20. Multiple delete removes all selected
21. No-op when none selected
22. Single delete is undoable
23. Multiple delete is undoable (single undo restores all)
24. Delete redo
