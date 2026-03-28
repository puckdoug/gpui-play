# Untested / Undocumented GPUI Features

Features available in GPUI that have not yet been explored or documented in this project. Organized by category. Items will be moved to their own docs as they are tested.

## Elements

| Element | Source | Description |
|---------|--------|-------------|
| `Img` | `src/elements/img.rs` | Image rendering (PNG, JPEG, GIF, WebP) with async loading, `ObjectFit`, `ImageSource` |
| `Svg` | `src/elements/svg.rs` | SVG rendering with `Transformation` (rotate, scale, translate) |
| `List` | `src/elements/list.rs` | Virtualized list for variable-height items with `ListState` |
| `UniformList` | `src/elements/uniform_list.rs` | Optimized list for equal-height items (10k+ rows) |
| `Anchored` | `src/elements/anchored.rs` | Floating UI positioned relative to anchor, avoids window bounds |
| `Deferred` | `src/elements/deferred.rs` | Renders children after ancestors (for overlays, popovers) |
| `AnimationElement` | `src/elements/animation.rs` | Animate elements with easing (linear, ease_in_out, bounce, etc.) |
| `StyledText` | `src/elements/text.rs` | Rich text with per-range styling via `TextRun` and `HighlightStyle` |
| `InteractiveText` | `src/elements/text.rs` | Styled text with clickable ranges |
| `Surface` | `src/elements/surface.rs` | CoreVideo pixel buffer rendering (macOS only) |
| `ImageCache` | `src/elements/image_cache.rs` | Image caching and loading management |

## Layout

| Feature | Source | Description |
|---------|--------|-------------|
| CSS Grid | `src/style.rs` | Grid layout via `Display::Grid`, `GridLocation`, `GridTemplate` |
| Grid placement | `src/geometry.rs` | `GridPlacement`, `TemplateColumnMinSize` |
| Scroll containers | `src/elements/div.rs` | `overflow_scroll()`, `ScrollHandle` for programmatic scroll |

## Rendering / Painting

| Feature | Source | Description |
|---------|--------|-------------|
| `PathBuilder` | `src/path_builder.rs` | Build vector paths: lines, arcs, bezier curves, polygons |
| `window.paint_quad()` | `src/window.rs` | Paint rectangles (fill, outline, rounded corners) |
| `window.paint_path()` | `src/window.rs` | Paint vector paths |
| `window.paint_shadows()` | `src/window.rs` | Drop shadows |
| Linear gradients | `src/color.rs` | `linear_gradient()`, `LinearColorStop`, `ColorSpace` (sRGB/Oklab) |
| Box shadows | `src/style.rs` | `BoxShadow`, `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()` |
| Transforms | `src/scene.rs` | `TransformationMatrix` for 2D transforms |
| `fill()`, `outline()`, `quad()` | `src/scene.rs` | Quick paint builder functions |

## Interaction / Events

| Feature | Source | Description |
|---------|--------|-------------|
| Drag and drop | `src/elements/div.rs` | `.on_drag()`, `.on_drag_move()`, `.on_drop()`, `DragMoveEvent` |
| File drop | `src/interactive.rs` | `FileDropEvent` (Entered, Pending, Submit, Exited), `ExternalPaths` |
| Scroll wheel | `src/interactive.rs` | `ScrollWheelEvent`, `ScrollDelta` (line/pixel/page) |
| Mouse pressure | `src/interactive.rs` | `MousePressureEvent`, `PressureStage` (force touch) |
| Pinch gesture | `src/interactive.rs` | `PinchEvent` (macOS) |
| Tooltips | `src/window.rs` | `.tooltip()` on elements, `TooltipId` |
| Hitbox | `src/window.rs` | `Hitbox`, `HitboxId` for custom hit detection |

## State Management

Documented in [state-management.md](state-management.md). Remaining:

| Feature | Source | Description |
|---------|--------|-------------|
| `cx.observe_new::<T>()` | `src/app.rs` | Watch creation of new views of a type |

## Async / Tasks

Documented in [async-tasks.md](async-tasks.md).

## Text / Typography

| Feature | Source | Description |
|---------|--------|-------------|
| `Font` | `src/text_system.rs` | Font builder (family, weight, style) |
| `FontWeight`, `FontStyle` | `src/text_system.rs` | Typography controls |
| `WrappedLine` | `src/text_system.rs` | Multi-line text wrapping |
| `LineWrapperHandle` | `src/text_system.rs` | Text wrapping control |
| `TextOverflow` | `src/style.rs` | Truncate with ellipsis (start or end) |

## Platform Integration

| Feature | Source | Description |
|---------|--------|-------------|
| File dialogs | `src/platform.rs` | `prompt_for_paths()`, `prompt_for_new_path()` |
| System prompts | `src/platform.rs` | `PromptLevel`, `PromptButton` |
| Keychain | `src/app.rs` | `write_credentials()`, `read_credentials()` |
| URL schemes | `src/app.rs` | `register_url_scheme()`, `on_open_urls()` |
| Display info | `src/app.rs` | `displays()`, `primary_display()`, `find_display()` |
| Dark/light mode | `src/app.rs` | `window_appearance() -> WindowAppearance` |
| Thermal state | `src/app.rs` | `thermal_state()`, `on_thermal_state_change()` |
| Keyboard layout | `src/app.rs` | `keyboard_layout()`, `on_keyboard_layout_change()` |
| Screen capture | `src/platform.rs` | `ScreenCaptureSource`, `ScreenCaptureStream` |
| Dock menu | `src/app.rs` | `set_dock_menu()` (macOS dock right-click menu) |
| Recent documents | `src/app.rs` | `add_recent_document()` |
| Open with system | `src/app.rs` | `open_with_system()`, `reveal_path()` |

## Testing

Documented in [testing.md](testing.md). Remaining:

| Feature | Source | Description |
|---------|--------|-------------|
| Multiple app contexts | `#[gpui::test]` with multiple `cx` params | Distributed system testing |

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
