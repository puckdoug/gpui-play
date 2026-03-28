# Untested / Undocumented GPUI Features

Features available in GPUI that have not yet been explored or documented in this project. Most items have been documented — remaining items are listed below.

## Documented Features

| Category | Documentation |
|----------|--------------|
| App & Window | [app.md](app.md), [window.md](window.md) |
| Menus | [menus.md](menus.md) |
| Button | [button.md](button.md) |
| Text Input | [text-input.md](text-input.md) |
| Canvas & Drawing | [canvas.md](canvas.md) |
| State Management | [state-management.md](state-management.md) |
| Async & Tasks | [async-tasks.md](async-tasks.md) |
| Testing | [testing.md](testing.md) |
| Images | [images.md](images.md) |
| SVG | [svg.md](svg.md) |
| Styled Text | [styled-text.md](styled-text.md) |
| Animation | [animation.md](animation.md) |
| CSS Grid | [css-grid.md](css-grid.md) |
| Lists | [lists.md](lists.md) |
| Scroll | [scroll.md](scroll.md) |
| Overlays | [overlays.md](overlays.md) |
| Gradients | [gradients.md](gradients.md) |
| Shadows | [shadows.md](shadows.md) |
| Transforms | [transforms.md](transforms.md) |
| Drag & Drop | [drag-drop.md](drag-drop.md) |
| File Drop | [file-drop.md](file-drop.md) |
| Hitbox | [hitbox.md](hitbox.md) |
| Gestures | [gestures.md](gestures.md) |
| Typography | [typography.md](typography.md) |
| Dialogs | [dialogs.md](dialogs.md) |
| Display & Appearance | [display-appearance.md](display-appearance.md) |
| Platform Utils | [platform-utils.md](platform-utils.md) |
| Screen Capture | [screen-capture.md](screen-capture.md) |

## Remaining Undocumented

None — all items have been folded into existing documentation:
- `cx.observe_new()` → [state-management.md](state-management.md)
- Multiple `cx` params → [testing.md](testing.md)
- `Surface` → [screen-capture.md](screen-capture.md)
- `fill()`, `outline()`, `quad()` → [canvas.md](canvas.md)

## GPUI Examples (reference implementations)

| Example | Features demonstrated |
|---------|----------------------|
| `animation.rs` | Element animation with easing |
| `data_table.rs` | Virtual list with 10k rows, sorting |
| `drag_drop.rs` | Drag payload, drag view, drop zones |
| `focus_visible.rs` | Focus ring styling (keyboard vs mouse) |
| `gif_viewer.rs` | Animated GIF loading and display |
| `gradient.rs` | Linear gradients, color space switching |
| `grid_layout.rs` | CSS Grid holy grail layout |
| `image_gallery.rs` | Image loading, ObjectFit modes |
| `painting.rs` | PathBuilder, strokes, fills, gradients (~1600 lines) |
| `popover.rs` | Anchored floating UI, nested deferred |
| `scrollable.rs` | Scroll handling |
| `shadow.rs` | Box shadows |
| `tab_stop.rs` | Tab navigation |
| `text_layout.rs` | Text measurement and layout |
| `uniform_list.rs` | Virtual scrolling for uniform items |
| `window.rs` | Multiple window types and configurations |
| `window_positioning.rs` | Window placement on displays |
