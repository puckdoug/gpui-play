# CSS Grid Layout

## Goal

Document CSS Grid support: `Display::Grid`, `GridLocation`, `GridTemplate`, `GridPlacement`, `TemplateColumnMinSize`.

## Design

Two layouts demonstrating grid capabilities:

1. **Holy grail layout** — header (full width), sidebar (fixed), content (flexible), footer (full width)
2. **Auto-flow grid** — responsive grid of cards that reflows based on window width using `minmax()` and `auto-fill`

### Key Concepts

- `Display::Grid` switches layout from flexbox to CSS Grid
- `GridTemplate` defines rows and columns (fixed, fractional, auto, minmax)
- `GridPlacement` positions a child within the grid (row/column start and span)
- `GridLocation` specifies grid line numbers
- `TemplateColumnMinSize` sets minimum size for auto-fill columns

## View Layer (src/bin/grid_test.rs)

- Top half: holy grail layout with colored regions (header=blue, sidebar=gray, content=white, footer=blue)
- Bottom half: card grid that reflows — each card has a colored background and label
- Cards demonstrate `GridPlacement` with explicit row/column and span

### Grid Configuration

```
Holy grail:
  columns: [200px, 1fr]
  rows: [60px, 1fr, 40px]
  header: col 1..3, row 1
  sidebar: col 1, row 2
  content: col 2, row 2
  footer: col 1..3, row 3

Auto-flow card grid:
  columns: repeat(auto-fill, minmax(150px, 1fr))
  rows: auto
```

## TDD Tests

### Grid construction (3)
1. Grid with explicit columns and rows creates correct template
2. GridPlacement with row/column span positions element correctly
3. Grid with auto-fill creates responsive columns

### Layout behavior (3)
4. Holy grail layout positions all regions correctly
5. Card grid reflows when container width changes
6. GridLocation accepts both line numbers and named lines

## Documentation (docs/gpui-usage/css-grid.md)

### Sections
1. **What it is** — CSS Grid layout engine (via Taffy) for 2D element positioning
2. **Preconditions** — parent element must have `display(Display::Grid)` and grid template defined; children need `GridPlacement` for explicit positioning
3. **Signatures** — `.display(Display::Grid)`, `.grid_template_columns(vec![...])`, `.grid_template_rows(vec![...])`, `.grid_column(GridPlacement::from_line_and_span(1, 2))`, `.grid_row(...)`
4. **Relevant types** — `Display::Grid`, `GridTemplate`, `GridPlacement`, `GridLocation`, `TemplateColumnMinSize`
5. **Usage examples** — holy grail layout, responsive card grid
6. **Post-conditions** — no cleanup; grid recalculates on window resize automatically
7. **Testing** — test layout bounds after render via `cx.layout_bounds()`
8. **Surprises** — not all CSS Grid features may be supported by Taffy; `gap()` for grid spacing; grid and flex cannot be mixed on same element; auto-placement order may differ from CSS spec; named grid lines may not be supported

**Note:** Reference Zed's `crates/gpui/examples/grid_layout.rs`.
