# Styled & Interactive Text

## Goal

Document `StyledText`, `InteractiveText`, `TextRun`, `HighlightStyle`, and `TextOverflow`.

## Design

A view showing rich text paragraphs with per-range styling (bold, italic, colored spans) using `StyledText`. Below that, an `InteractiveText` block with clickable ranges (simulating hyperlinks). A truncated text label demonstrates `TextOverflow` with start/end ellipsis.

### Data Model

```rust
struct StyledTextDemo {
    click_log: Vec<String>,  // log of clicked ranges
}
```

### Key Concepts

- `StyledText` takes a string and a `Vec<TextRun>` defining font/style per character range
- `TextRun` specifies font, color, weight, style, underline, strikethrough for a byte range
- `HighlightStyle` is a partial style overlay (can override just color, just weight, etc.)
- `InteractiveText` extends StyledText with clickable regions
- `TextOverflow` truncates text with ellipsis when it doesn't fit

## View Layer (src/bin/styled_text_test.rs)

- Paragraph with mixed bold/italic/colored ranges via StyledText
- Paragraph with clickable "links" via InteractiveText — clicks logged to a list below
- Row of labels with TextOverflow::Ellipsis at start vs end
- Row showing highlight overlay on existing styled text

## TDD Tests

### StyledText (3)
1. StyledText with empty runs renders plain text
2. StyledText with multiple TextRuns applies correct ranges
3. TextRun byte ranges must not overlap or exceed string length

### InteractiveText (3)
4. Click on a clickable range fires the handler
5. Click outside clickable ranges does not fire
6. Multiple clickable ranges are independently clickable

### TextOverflow (2)
7. Text shorter than container shows no ellipsis
8. Text longer than container shows ellipsis

## Documentation (docs/gpui-usage/styled-text.md)

### Sections
1. **What it is** — rich text elements with per-range styling and optional click handlers
2. **Preconditions** — `use gpui::{StyledText, InteractiveText, TextRun, HighlightStyle, TextOverflow}`; text content and runs must use byte offsets (not char offsets)
3. **Signatures** — `StyledText::new(text).with_runs(runs)`, `InteractiveText::new(text, runs).on_click(range, handler)`, `.text_overflow(TextOverflow::Ellipsis)`
4. **Relevant traits** — `IntoElement`
5. **Usage examples** — styled paragraph, clickable text, truncated label
6. **Post-conditions** — no cleanup; click handlers are closures captured by the element
7. **Testing** — can test element construction; click testing via VisualTestContext.simulate_click
8. **Surprises** — TextRun ranges are BYTE offsets not char indices (critical for non-ASCII); overlapping runs may cause panics or undefined rendering; InteractiveText click regions are character-based not pixel-based; TextOverflow only works with single-line text; HighlightStyle merges with (not replaces) underlying TextRun style
