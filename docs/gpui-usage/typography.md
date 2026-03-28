# Typography Controls

**Components:** [`TextOverflow`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`LineWrapperHandle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/text_system.rs)

## What is the component and what it does

GPUI provides two text-specific controls beyond basic font styling:

- **`TextOverflow`** — truncates text with a custom indicator (usually "…") when it exceeds the container width. Supports truncation at the end or start.
- **`LineWrapperHandle`** — low-level API for manually wrapping text at a specific pixel width. Used internally by `WrappedLine` (see [canvas.md](canvas.md)).

Font configuration (`Font`, `FontWeight`, `FontStyle`) is covered in [canvas.md](canvas.md) and [styled-text.md](styled-text.md) via `TextRun` and `HighlightStyle`.

## Preconditions for use

```rust
use gpui::{TextOverflow, Styled};
```

- `TextOverflow` is applied via `.text_overflow()` on styled elements
- Element must have constrained width for overflow to take effect
- `LineWrapperHandle` is obtained from `TextSystem::line_wrapper(font, font_size)`

## Signature for usage

### TextOverflow

```rust
pub enum TextOverflow {
    Truncate(SharedString),        // "very long te…"
    TruncateStart(SharedString),   // "…ong text here"
}
```

Applied via style:

```rust
div()
    .w_32()                                         // Constrained width
    .text_overflow(TextOverflow::Truncate("…".into()))
    .child("This very long text will be truncated")
```

### Custom truncation indicators

```rust
TextOverflow::Truncate("...".into())       // Three dots
TextOverflow::Truncate(">>".into())        // Custom indicator
TextOverflow::TruncateStart("…".into())    // Truncate from start (useful for paths)
```

### LineWrapperHandle (low-level)

```rust
// Obtain from TextSystem
let wrapper = text_system.line_wrapper(font, font_size);

// LineWrapperHandle provides controlled wrapping at a pixel width
// Used internally by WrappedLine for multi-line text rendering
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Styled` | Provides `.text_overflow()` method |

## Usage and examples

### Truncated label

```rust
div()
    .w_48()  // Fixed width
    .text_overflow(TextOverflow::Truncate("…".into()))
    .child("This is a very long label that won't fit in the container")
```

### Path truncation (keep the end)

```rust
div()
    .w_48()
    .text_overflow(TextOverflow::TruncateStart("…".into()))
    .child("/Users/doug/very/deep/nested/path/to/file.rs")
// Displays: "…nested/path/to/file.rs"
```

## Post-conditions / destruction requirements

- No cleanup needed
- Wrapping/truncation recalculates on resize

## Testing

```rust
#[test]
fn test_text_overflow_variants() {
    let end = TextOverflow::Truncate("…".into());
    match end {
        TextOverflow::Truncate(s) => assert_eq!(s.as_ref(), "…"),
        _ => panic!("wrong variant"),
    }
}
```

Run tests: `cargo test --test typography_test`

## Surprises, Anti-patterns, and Bugs

### TextOverflow only works on single-line text

Truncation only applies to single-line text. For multi-line text with a line limit, use `WrappedLine` with manual line counting (see [canvas.md](canvas.md)).

### Container must have constrained width

Without a fixed or max width, the container grows to fit all text and no truncation occurs.

### The truncation indicator can be any string

It's not limited to "…" — any `SharedString` works. The indicator replaces the truncated content.

### LineWrapperHandle caches internal state

Reuse the handle for multiple wrapping operations on text with the same font/size. Recreating it each time has a performance cost.
