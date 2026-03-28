# File Dialogs & System Prompts

## Goal

Document `prompt_for_paths()`, `prompt_for_new_path()`, `PromptLevel`, and `PromptButton`.

## Design

A view with buttons that open native OS dialogs: file open picker, file save picker, and system confirmation/warning/critical prompts.

### Key Concepts

- `prompt_for_paths()` — opens native open-file dialog, returns `Option<Vec<PathBuf>>`
- `prompt_for_new_path()` — opens native save-file dialog, returns `Option<PathBuf>`
- `PromptLevel` — Info, Warning, Critical — controls dialog appearance
- `PromptButton` — button labels for system prompt dialogs
- All dialog calls are async (return a Future)

## View Layer (src/bin/dialog_test.rs)

- "Open File" button → file picker, show selected paths
- "Save File" button → save picker, show chosen path
- "Info Prompt" button → informational dialog with OK
- "Warning Prompt" button → warning dialog with OK/Cancel
- "Critical Prompt" button → critical dialog with destructive action confirmation
- Results display area showing last dialog result

## TDD Tests

### Construction (3)
1. PromptLevel variants exist (Info, Warning, Critical)
2. PromptButton can be created with label
3. Dialog functions are available on window context

### Note on testability
Native dialogs cannot be automated in tests — they block and require user interaction. Test that the API surface exists and types are correct. Actual dialog behavior is manual testing only.

## Documentation (docs/gpui-usage/dialogs.md)

### Sections
1. **What it is** — native OS file pickers and system prompt dialogs
2. **Preconditions** — must be called from a window context (`cx.prompt_for_paths()` etc.); async — must be awaited in a spawned task
3. **Signatures** — `cx.prompt_for_paths(PathPromptOptions { ... })`, `cx.prompt_for_new_path(directory)`, `cx.prompt(PromptLevel, msg, detail, buttons)`
4. **Relevant types** — `PromptLevel`, `PromptButton`, `PathPromptOptions`
5. **Usage examples** — open file, save file, confirmation prompt
6. **Post-conditions** — dialog is modal (blocks interaction with parent window); returns None/Cancel if dismissed
7. **Testing** — not automatable; test API availability only
8. **Surprises** — dialogs are ASYNC (must spawn + await, not call directly); dialog blocks the window but not the app; prompt_for_paths can allow multiple selection or single; no built-in file type filtering (may need PathPromptOptions); returned paths may not exist yet (save dialog); dialog appearance varies by OS
