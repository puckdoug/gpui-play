# File Dialogs & System Prompts

**Components:** [`prompt_for_paths`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app.rs), [`prompt_for_new_path`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app.rs), [`PromptLevel`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs), [`PathPromptOptions`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs)

## What is the component and what it does

GPUI provides native OS file pickers and system prompt dialogs:

- **`prompt_for_paths()`** — opens a native open-file dialog, returns selected paths
- **`prompt_for_new_path()`** — opens a native save-file dialog, returns the chosen path
- **`prompt()`** — shows a system alert/confirmation dialog with customizable buttons

All dialog calls are async — they return a `oneshot::Receiver` with the result.

## Preconditions for use

```rust
use gpui::{PromptLevel, PathPromptOptions};
```

- Dialogs are called on `App` (file pickers) or `Window` (system prompts)
- Results are received via `oneshot::Receiver` — must be awaited in a spawned task
- Dialogs are modal to the window (block interaction with parent)
- **Not available in the test platform** — calling them panics with "not implemented"

## Signature for usage

### Open file dialog

```rust
cx.prompt_for_paths(PathPromptOptions {
    files: true,          // Allow file selection
    directories: false,   // Don't allow directory selection
    multiple: true,       // Allow multiple selection
    prompt: None,         // Optional custom button label
}) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>>
```

### Save file dialog

```rust
cx.prompt_for_new_path(
    &Path::new("/default/directory"),    // Starting directory
    Some("untitled.txt"),                // Suggested file name
) -> oneshot::Receiver<Result<Option<PathBuf>>>
```

### System prompt

```rust
window.prompt(
    PromptLevel::Warning,               // Info, Warning, or Critical
    "Delete file?",                      // Message
    Some("This cannot be undone."),      // Detail text (optional)
    &[PromptButton::default_confirm(), PromptButton::cancel()],  // Buttons
) -> Option<oneshot::Receiver<usize>>   // Index of clicked button
```

### PromptLevel

```rust
PromptLevel::Info      // Informational
PromptLevel::Warning   // Warning (yellow icon)
PromptLevel::Critical  // Destructive/critical (red icon)
```

### PathPromptOptions

```rust
pub struct PathPromptOptions {
    pub files: bool,                // Allow selecting files
    pub directories: bool,          // Allow selecting directories
    pub multiple: bool,             // Allow multiple selection
    pub prompt: Option<SharedString>, // Custom button label
}
```

## Usage and examples

### Open file picker in spawned task

```rust
let task = cx.spawn(async move |async_cx| {
    let receiver = async_cx.update(|cx| {
        cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: None,
        })
    });

    if let Ok(Ok(Some(paths))) = receiver.await {
        // paths: Vec<PathBuf>
        println!("Selected: {:?}", paths);
    }
});
task.detach();
```

### Save file picker

```rust
let receiver = cx.prompt_for_new_path(Path::new("/tmp"), Some("document.txt"));
// Await in a spawned task...
```

## Post-conditions / destruction requirements

- Dialogs are modal — they block the parent window until dismissed
- Returns `None` if the user cancels
- No cleanup needed — the OS manages the dialog lifecycle

## Testing

Native dialogs **panic** in the test platform. Only test type construction:

```rust
#[test]
fn test_options() {
    let opts = PathPromptOptions {
        files: true, directories: false, multiple: false, prompt: None,
    };
    assert!(opts.files);
}
```

Run tests: `cargo test --test dialogs_test`

## Surprises, Anti-patterns, and Bugs

### Dialogs are async — must spawn + await

You cannot call `prompt_for_paths()` synchronously. The return is a `oneshot::Receiver` that must be awaited in a spawned task.

### Not available in tests

The test platform does not implement file dialogs. Calling them panics. Test dialog integration logic by mocking at a higher level.

### `prompt_for_paths()` can return empty Vec

Even with `multiple: true`, the user might select nothing and click "Open" — check the returned paths.

### File type filtering is limited

`PathPromptOptions` has no file type filter field. The user can select any file. Filter after selection if needed.

### Returned paths may not exist yet (save dialog)

`prompt_for_new_path()` returns a path the user typed — the file may not exist. You must create it.
