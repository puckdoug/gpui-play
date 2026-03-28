# File Drop

**Components:** [`FileDropEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs), [`ExternalPaths`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs)

## What is the component and what it does

GPUI handles OS-level file drag-and-drop into windows. When files are dragged from Finder/Explorer over a GPUI window, `FileDropEvent` variants track the lifecycle: entered, pending (hovering), submit (dropped), and exited.

## Preconditions for use

```rust
use gpui::{FileDropEvent, ExternalPaths};
```

- File drop events are window-level, dispatched through the mouse event system
- `ExternalPaths` contains `SmallVec<[PathBuf; 2]>` (requires `smallvec` as transitive dep)
- Cannot be simulated in the test harness — requires actual OS drag operations

## Signature for usage

### FileDropEvent variants

```rust
pub enum FileDropEvent {
    Entered {
        position: Point<Pixels>,     // Mouse position in window
        paths: ExternalPaths,        // File paths being dragged
    },
    Pending {
        position: Point<Pixels>,     // Mouse position while hovering
    },
    Submit {
        position: Point<Pixels>,     // Mouse position where dropped
    },
    Exited,                          // Files dragged out of window
}
```

### ExternalPaths

```rust
pub struct ExternalPaths(pub SmallVec<[PathBuf; 2]>);

impl ExternalPaths {
    pub fn paths(&self) -> &[PathBuf]
}
```

## Usage and examples

### File drop state management

```rust
struct FileDropView {
    dropped_paths: Vec<PathBuf>,
    drag_hovering: bool,
}

impl Render for FileDropView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(if self.drag_hovering { gpui::rgb(0x4488ff) } else { gpui::rgb(0xeeeeee) })
            .child(if self.dropped_paths.is_empty() {
                "Drop files here".to_string()
            } else {
                format!("{} files dropped", self.dropped_paths.len())
            })
    }
}
```

### Handling events (conceptual)

File drop events flow through the window's mouse event dispatch system. Handle them via `window.on_mouse_event()` or through element `Interactivity` handlers:

```rust
// In a window event handler:
match event {
    FileDropEvent::Entered { position, paths } => {
        self.drag_hovering = true;
        // paths.paths() gives &[PathBuf]
    }
    FileDropEvent::Pending { position } => {
        // Update hover position if needed
    }
    FileDropEvent::Submit { position } => {
        // Files dropped — access paths from the Entered event
        self.drag_hovering = false;
    }
    FileDropEvent::Exited => {
        self.drag_hovering = false;
    }
}
```

## Post-conditions / destruction requirements

- No cleanup needed for file drop state
- File paths reference files on the filesystem — they must still exist when accessed
- `ExternalPaths` is owned data — paths are valid even if the OS drag operation has ended

## Testing

File drop events cannot be simulated in GPUI's test harness. Test state transition logic in pure Rust:

```rust
#[test]
fn test_state_transitions() {
    let mut view = FileDropView { dropped_paths: vec![], drag_hovering: false };
    view.drag_hovering = true;
    assert!(view.drag_hovering);
    view.dropped_paths.push(PathBuf::from("/tmp/test.txt"));
    view.drag_hovering = false;
    assert_eq!(view.dropped_paths.len(), 1);
}
```

Run tests: `cargo test --test file_drop_test`

## Surprises, Anti-patterns, and Bugs

### `Entered` fires at the WINDOW level, not element level

`FileDropEvent::Entered` fires when files enter the window bounds, not a specific element. You must manually check positions against element bounds for element-level drop zones.

### Paths are only available in `Entered`

The `ExternalPaths` are provided in the `Entered` variant. `Submit` only provides position — you must store the paths from `Entered` to use them at submit time.

### No MIME type or file type filtering during drag

You cannot reject files by type during the drag. Type checking can only happen after the `Submit` event, by examining file extensions or contents.

### Paths may be symlinks or directories

`ExternalPaths` may contain directories, symlinks, or any filesystem path — not just regular files. Check `path.is_file()` before assuming.

### `ExternalPaths` uses `SmallVec`

The internal type is `SmallVec<[PathBuf; 2]>`, optimized for 1-2 files. Accessing it requires `smallvec` as a transitive dependency (provided by gpui, but not directly usable without adding it to your `Cargo.toml`).
