# Copy/Paste Oval Shapes

## Design

Copy a selected oval (with text) to the clipboard for pasting back into the canvas as a shape duplicate.

- **Cmd-C on selected shape (not editing)**: Write JSON shape data as `ClipboardString` with metadata tag `"gpui-play-shape"`. Shape text is the string fallback.
- **Cmd-V (Paste)**: Check for `gpui-play-shape` metadata first (internal paste = duplicate shape with offset). Fall back to text paste when editing.
- **Cmd-X on selected shape (not editing)**: Copy shape then delete it.

## State Layer (src/shape.rs)

- `OvalShape::to_json() -> String` / `OvalShape::from_json(&str) -> Option<Self>` (serde)
- `CanvasState::copy_selected() -> Option<String>` — serialize selected shape
- `CanvasState::paste_shape(json)` — deserialize and add with offset
- `CanvasState::delete_selected()` — remove selected shape with undo
- `UndoAction::DeleteShape` variant

## View Layer (src/bin/draw_test.rs)

- Modify `on_copy`: if not editing, copy shape JSON with metadata
- Modify `on_paste`: if not editing, check for shape metadata, paste shape
- Modify `on_cut`: if not editing, copy shape then delete

## Dependencies

- `serde` + `serde_json` for shape serialization

## TDD Tests

1. Shape JSON roundtrip
2. copy_selected returns data / None
3. paste_shape adds shape with offset
4. paste is undoable
5. delete_selected removes shape
6. delete is undoable
