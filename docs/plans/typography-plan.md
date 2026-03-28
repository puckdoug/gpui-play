# Typography Controls

## Goal

Document `LineWrapperHandle`, `TextOverflow` (ellipsis), and fill gaps around font configuration not covered in canvas.md or text-input.md.

## Design

A view showing text blocks with different wrapping and overflow behaviors. Font specimen section showing the same text in different weights, styles, and sizes.

### Key Concepts

- `LineWrapperHandle` provides manual control over how text wraps at specific widths
- `TextOverflow::Ellipsis` truncates text with "..." when it exceeds container width
- TextOverflow can be at start or end of text
- Font configuration via `TextRun` (covered in canvas.md but with new focus on typography)

## View Layer (src/bin/typography_test.rs)

- Section 1: Text with `TextOverflow::Ellipsis` at end (default) — long text in narrow container
- Section 2: Text with ellipsis at start — shows "...end of text"
- Section 3: Text without overflow — wraps naturally
- Section 4: Font weight specimen — Thin through Black
- Section 5: Font style specimen — Normal, Italic, Oblique
- Section 6: Different font sizes with same content

## TDD Tests

### TextOverflow (3)
1. TextOverflow::Ellipsis truncates long text
2. No overflow when text fits container
3. Ellipsis at start vs end produces different truncation

### LineWrapperHandle (3)
4. LineWrapperHandle wraps text at specified width
5. Wrap produces correct number of lines for known text
6. Wrap preserves word boundaries

## Documentation (docs/gpui-usage/typography.md)

### Sections
1. **What it is** — text wrapping control and overflow handling for text elements
2. **Preconditions** — `use gpui::{TextOverflow, LineWrapperHandle}`; text must be in a bounded container for overflow to take effect
3. **Signatures** — `.text_overflow(TextOverflow::Ellipsis)`, `LineWrapperHandle::new(font_id, font_size)`, `handle.wrap_line(text, max_width)`
4. **Relevant types** — `TextOverflow`, `LineWrapperHandle`, `WrappedLine`
5. **Usage examples** — truncated label, wrapped text block, font specimen
6. **Post-conditions** — no cleanup; wrapping recalculated on resize
7. **Testing** — wrap output is testable (line count, content); overflow requires visual verification
8. **Surprises** — TextOverflow only works on single-line text (multiline needs WrappedLine + manual truncation); ellipsis character may not match the text font; LineWrapperHandle caches internal state (reuse it, don't recreate); wrap width is in pixels not characters; different fonts wrap differently at same width
