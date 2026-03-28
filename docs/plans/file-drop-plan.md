# File Drop

## Goal

Document `FileDropEvent` (Entered, Pending, Submit, Exited) and `ExternalPaths`.

## Design

A drop zone that accepts files from the OS file manager. Visual states for drag-enter, drag-over, and drop. Dropped files are listed with name and size.

### Data Model

```rust
struct FileDropDemo {
    dropped_files: Vec<DroppedFile>,
    drag_state: FileDragState,
}

struct DroppedFile {
    path: PathBuf,
    name: String,
    size: u64,
}

enum FileDragState {
    Idle,
    Hovering,    // files dragged over zone
    Dropped,     // files just dropped
}
```

### Key Concepts

- `FileDropEvent::Entered` — external files enter window bounds
- `FileDropEvent::Pending` — files hovering over the drop zone
- `FileDropEvent::Submit(ExternalPaths)` — files dropped, contains paths
- `FileDropEvent::Exited` — files dragged out without dropping
- `ExternalPaths` contains `Vec<PathBuf>` of dropped files

## View Layer (src/bin/file_drop_test.rs)

- Large drop zone with dashed border
- Idle: "Drop files here" label
- Hovering: border turns blue, background highlights
- Dropped: list of file names and sizes
- Handles single and multiple file drops

## TDD Tests

### File drop events (4)
1. FileDropEvent::Entered transitions state to Hovering
2. FileDropEvent::Exited transitions state back to Idle
3. FileDropEvent::Submit provides ExternalPaths with file paths
4. Multiple files in single drop are all captured

### State (2)
5. Dropped files list grows with each drop
6. File metadata (name, size) is correctly extracted from paths

## Documentation (docs/gpui-usage/file-drop.md)

### Sections
1. **What it is** — OS-level file drag-and-drop into GPUI windows
2. **Preconditions** — `use gpui::{FileDropEvent, ExternalPaths}`; must handle events on a div with `.on_file_drop()` or similar handler
3. **Signatures** — `.on_file_drop(|event, cx| {})`, `FileDropEvent` variants, `ExternalPaths { paths: Vec<PathBuf> }`
4. **Relevant types** — `FileDropEvent`, `ExternalPaths`
5. **Usage examples** — basic drop zone, visual feedback, file list
6. **Post-conditions** — no cleanup; paths are filesystem paths (files must still exist when accessed)
7. **Testing** — difficult to test without OS-level drag simulation; test state transitions manually
8. **Surprises** — Entered fires when files enter the WINDOW not the drop zone; must distinguish window-level vs element-level handling; ExternalPaths may contain directories not just files; file paths may be symlinks; no MIME type information (only paths); cannot reject specific file types during drag (only after drop)
