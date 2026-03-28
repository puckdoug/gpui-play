# Styled & Interactive Text

**Components:** [`StyledText`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/text.rs), [`InteractiveText`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/text.rs), [`HighlightStyle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`TextOverflow`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs)

## What is the component and what it does

`StyledText` renders rich text with per-range styling — bold, italic, colored spans, underline, strikethrough, and background highlights. It takes a plain string and a set of `HighlightStyle` ranges that override the default text style.

`InteractiveText` extends `StyledText` with clickable ranges, hover detection, and per-character tooltips. This is used for hyperlinks, code references, and any text that responds to user interaction.

`TextOverflow` controls text truncation with ellipsis when content exceeds the container width.

## Preconditions for use

```rust
use gpui::{StyledText, InteractiveText, HighlightStyle, TextOverflow, FontWeight, FontStyle};
```

- All ranges use **UTF-8 byte offsets**, not character indices
- Ranges must start and end on valid UTF-8 character boundaries
- `InteractiveText` requires an `ElementId` for state tracking
- `HighlightStyle` fields are all `Option` — only set fields override the base style

## Signature for usage

### StyledText

```rust
// Plain text (no highlights)
StyledText::new("Hello world")

// With highlight ranges (delayed styling — uses inherited text style as base)
StyledText::new("Hello bold world")
    .with_highlights([
        (0..5, HighlightStyle { font_weight: Some(FontWeight::BOLD), ..Default::default() }),
    ])

// With explicit base style + highlights
StyledText::new("Hello colored world")
    .with_default_highlights(&window.text_style(), [
        (6..13, HighlightStyle { color: Some(red()), ..Default::default() }),
    ])

// With explicit TextRuns (full control)
StyledText::new("Hello world")
    .with_runs(vec![
        TextRun { len: 5, font: bold_font, color: white(), ..Default::default() },
        TextRun { len: 6, font: regular_font, color: gray(), ..Default::default() },
    ])
```

### HighlightStyle

```rust
HighlightStyle {
    color: Option<Hsla>,              // Text color
    font_weight: Option<FontWeight>,  // e.g., FontWeight::BOLD
    font_style: Option<FontStyle>,    // e.g., FontStyle::Italic
    background_color: Option<Hsla>,   // Background highlight
    underline: Option<UnderlineStyle>,
    strikethrough: Option<StrikethroughStyle>,
    fade_out: Option<f32>,            // Opacity reduction (0.0–1.0)
}
```

All fields default to `None` (inherit from base style).

### TextRun

```rust
TextRun {
    len: usize,                                // UTF-8 byte length of this run
    font: Font,                                // Font family, weight, style
    color: Hsla,                               // Text color
    background_color: Option<Hsla>,            // Background highlight
    underline: Option<UnderlineStyle>,
    strikethrough: Option<StrikethroughStyle>,
}
```

### InteractiveText

```rust
InteractiveText::new("element-id", styled_text)
    .on_click(
        vec![0..5, 10..15],  // Clickable byte ranges
        |range_index, window, cx| {
            // range_index: which range was clicked (0, 1, ...)
        },
    )
    .on_hover(|char_index, event, window, cx| {
        // char_index: Option<usize> — character under cursor
    })
    .tooltip(|char_index, window, cx| {
        // Return Optional tooltip view for character at index
        None
    })
```

### TextOverflow

```rust
TextOverflow::Truncate("…".into())       // "very long te…"
TextOverflow::TruncateStart("…".into())  // "…ong text here"
```

Applied via the `text_overflow` style on a parent element.

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `IntoElement` | Both `StyledText` and `InteractiveText` implement this |

## Usage and examples

### Rich text with multiple styles

```rust
impl Render for RichTextView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bold = HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        };
        let italic = HighlightStyle {
            font_style: Some(FontStyle::Italic),
            ..Default::default()
        };

        div().child(
            StyledText::new("Bold and italic text")
                .with_highlights([(0..4, bold), (9..15, italic)]),
        )
    }
}
```

### Clickable text (hyperlinks)

```rust
impl Render for LinkView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = StyledText::new("Click here for info");

        div().child(
            InteractiveText::new("link", text)
                .on_click(vec![6..10], |range_ix, _window, _cx| {
                    println!("clicked range {}", range_ix);
                }),
        )
    }
}
```

### Multiple clickable ranges

```rust
let text = StyledText::new("Link one and link two");

InteractiveText::new("multi-link", text)
    .on_click(vec![0..8, 13..21], |range_ix, _window, _cx| {
        match range_ix {
            0 => println!("clicked first link"),
            1 => println!("clicked second link"),
            _ => {}
        }
    })
```

### Highlight with base style

```rust
fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let base_style = window.text_style();
    let highlight = HighlightStyle {
        color: Some(gpui::red()),
        ..Default::default()
    };

    div().child(
        StyledText::new("Error: something went wrong")
            .with_default_highlights(&base_style, [(0..5, highlight)]),
    )
}
```

## Post-conditions / destruction requirements

- No cleanup needed — text elements are released when removed from the tree
- Click handlers are closures captured by the element — they live as long as the element
- `InteractiveText` state (hover, click tracking) is keyed by the `ElementId`

## Testing

```rust
#[gpui::test]
fn test_styled_text(cx: &mut TestAppContext) {
    struct TextView;
    impl Render for TextView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                StyledText::new("Hello bold")
                    .with_highlights([(6..10, HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        ..Default::default()
                    })]),
            )
        }
    }
    let _window = cx.add_window(|_w, _cx| TextView);
}
```

Run tests: `cargo test --test styled_text_test`

## Surprises, Anti-patterns, and Bugs

### Ranges are UTF-8 byte offsets, not character indices

This is critical for non-ASCII text. `"café"` is 5 bytes (`é` = 2 bytes), so the range for `"é"` is `3..5`, not `3..4`. Using wrong byte offsets causes panics (`debug_assert!` on `is_char_boundary()`).

### HighlightStyle merges with base style, not replaces

Setting `font_weight: Some(FontWeight::BOLD)` on a highlight only changes the weight. All other properties (color, size, family) are inherited from the base text style. This is intentional — highlights are overlays.

### `with_highlights` vs `with_default_highlights`

- `with_highlights(ranges)` — delays style resolution until layout. Ranges are `(Range<usize>, HighlightStyle)`.
- `with_default_highlights(&text_style, ranges)` — resolves immediately against the provided `TextStyle`. Use this when you have a specific base style.

### InteractiveText click ranges are byte ranges

The `on_click` ranges correspond to UTF-8 byte ranges in the text. The callback receives the **index** of which range was clicked (0-based), not the range itself.

### TextOverflow only works on single-line text

`TextOverflow::Truncate` and `TruncateStart` only affect single-line text. For multi-line truncation, you need to use `WrappedLine` with manual line counting and truncation logic.

### No built-in link styling

`InteractiveText` provides click detection but no visual styling (no underline, no color change on hover). You must apply `HighlightStyle` with underline and color to the clickable ranges yourself.

### TextRun `len` is byte length, not character count

When using `with_runs()`, each `TextRun.len` is the number of UTF-8 bytes the run covers. The sum of all run lengths must equal the byte length of the text string.
