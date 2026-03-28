# CSS Grid Layout

**Components:** [`Display::Grid`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`GridTemplate`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`GridPlacement`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/geometry.rs)

## What is the component and what it does

GPUI supports CSS Grid layout via Taffy. Grid enables 2D positioning of elements in rows and columns. You define a grid template on the parent, then position children using column/row spans and explicit placement.

## Preconditions for use

```rust
use gpui::Styled; // Provides .grid(), .grid_cols(), .col_span(), .gap_2(), etc.
```

- Parent element must call `.grid()` to switch from flexbox to grid display
- Children use `.col_span()`, `.row_span()`, `.col_start()`, etc. for placement
- All grid methods are on the `Styled` trait

## Signature for usage

### Parent: defining the grid

```rust
div()
    .grid()                // Display::Grid
    .grid_cols(3)          // 3 equal-width columns (repeat(3, minmax(0, 1fr)))
    .grid_rows(2)          // 2 equal-height rows
    .gap_2()               // 8px gap between cells
```

### Variants for column sizing

```rust
.grid_cols(N)              // N columns, each minmax(0, 1fr)
.grid_cols_min_content(N)  // N columns, each minmax(min-content, 1fr)
.grid_cols_max_content(N)  // N columns, each minmax(max-content, 1fr)
```

### Children: placement

```rust
// Span multiple columns/rows
.col_span(2)          // Span 2 columns
.row_span(3)          // Span 3 rows
.col_span_full()      // Span all columns (line 1 to -1)
.row_span_full()      // Span all rows

// Explicit line placement (1-indexed)
.col_start(1)         // Start at column line 1
.col_end(3)           // End at column line 3
.row_start(2)         // Start at row line 2
.row_end(4)           // End at row line 4

// Auto placement
.col_start_auto()     // Auto-place column start
.row_start_auto()     // Auto-place row start
```

### Gap (spacing between cells)

```rust
.gap_0()    // 0px
.gap_1()    // 4px
.gap_2()    // 8px
.gap_4()    // 16px
.gap_x_2()  // Column gap only: 8px
.gap_y_4()  // Row gap only: 16px
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Styled` | All grid methods — `.grid()`, `.grid_cols()`, `.col_span()`, `.gap_2()`, etc. |

## Usage and examples

### Holy grail layout

```rust
impl Render for HolyGrailView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .grid()
            .grid_cols(3)
            .grid_rows(3)
            .gap_2()
            .child(div().col_span_full().bg(gpui::blue()).child("Header"))
            .child(div().row_span(1).bg(gpui::rgb(0x888888)).child("Sidebar"))
            .child(div().col_span(2).bg(gpui::white()).child("Content"))
            .child(div().col_span_full().bg(gpui::blue()).child("Footer"))
    }
}
```

### Card grid with explicit placement

```rust
div()
    .grid()
    .grid_cols(4)
    .grid_rows(2)
    .gap_4()
    .child(div().col_start(1).col_end(3).child("Spans cols 1-2"))
    .child(div().col_start(3).col_end(5).child("Spans cols 3-4"))
    .child(div().row_start(2).col_span_full().child("Full-width row 2"))
```

### Wide cell spanning

```rust
div()
    .grid()
    .grid_cols(5)
    .grid_rows(5)
    .child(div().col_span(3).row_span(2).child("3x2 cell"))
    .child(div().child("Auto-placed"))
```

## Post-conditions / destruction requirements

- No cleanup needed — grid recalculates on window resize automatically
- Grid layout is computed by Taffy during the layout phase

## Testing

```rust
#[gpui::test]
fn test_grid(cx: &mut TestAppContext) {
    struct GridView;
    impl Render for GridView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().grid().grid_cols(3).gap_2()
                .child(div().col_span_full().child("Header"))
                .child(div().child("A"))
                .child(div().child("B"))
        }
    }
    let _window = cx.add_window(|_w, _cx| GridView);
}
```

Run tests: `cargo test --test css_grid_test`

## Surprises, Anti-patterns, and Bugs

### Grid columns use `repeat(N, minmax(0, 1fr))` internally

`.grid_cols(3)` creates 3 equal fractional columns. There is no direct way to create columns with fixed pixel widths or mixed fixed/fractional. For mixed layouts, use nested flex containers within grid cells.

### Line numbers are 1-indexed

`.col_start(1)` is the first column line. `.col_end(-1)` is the last line. This matches CSS Grid conventions.

### `.col_span_full()` uses `Line(1)..Line(-1)`

This spans from the first to the last grid line — equivalent to spanning all columns.

### No named grid lines or areas

GPUI's grid does not support `grid-template-areas` or named lines. Use explicit line numbers for placement.

### Grid and flex cannot be mixed

An element is either `Display::Flex` (default) or `Display::Grid`. Calling `.grid()` switches the display mode. Children of a grid parent are grid items, not flex items.

### Not all CSS Grid features are supported

Taffy implements a subset of CSS Grid. Features like `auto-fill`, `auto-fit`, `minmax()` with arbitrary values, and subgrid are not available. The grid template is defined by column/row count, not by arbitrary track definitions.
