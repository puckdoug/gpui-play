# Meta-Plan: GPUI Feature Documentation & Examples

Goal: For every item in `docs/gpui-usage/untested.md`, produce a focused documentation page in `docs/gpui-usage/` and (where appropriate) a minimal example binary in `src/bin/`. Each doc follows the standard template: Preconditions, Usage Examples, Post-conditions/Destruction, Testing, Surprises/Anti-patterns/Bugs.

---

## Already Covered (no new work needed)

These items are substantially documented in existing pages and can be removed from `untested.md`:

| Item | Covered in |
|------|-----------|
| `PathBuilder` | `canvas.md` — full API, examples, gotchas |
| `window.paint_quad()` | `canvas.md` — quad painting via `fill()`, `outline()`, `quad()` |
| `window.paint_path()` | `canvas.md` — path painting with fill/stroke |
| `fill()`, `outline()`, `quad()` | `canvas.md` — scene builder functions |
| `WrappedLine` | `canvas.md` — multi-line text wrapping in canvas |
| `Font`, `FontWeight`, `FontStyle` | `canvas.md` / `text-input.md` — TextRun font configuration |

---

## Phase 1 — Foundation (State, Async, Testing)

These underpin nearly everything else. Document first so later phases can reference them.

### 1.1 State Management
**Doc:** `docs/gpui-usage/state-management.md`
**Example:** `src/bin/state_test.rs`
**Covers:** `Model<T>`, `EventEmitter<E>`, `cx.subscribe()`, `cx.observe()`, `cx.observe_new()`, `cx.observe_release()`, `Subscription`
**Why first:** Every non-trivial example uses Model or subscriptions. Documenting the reactive primitives gives a foundation for all later work.
**Example concept:** A counter `Model<i32>` shared between two views. One view mutates, the other observes. An event emitter notifies on threshold crossing. Demonstrates subscription lifetime.
**Estimated complexity:** Medium — requires multi-view window setup.

### 1.2 Async & Tasks
**Doc:** `docs/gpui-usage/async-tasks.md`
**Example:** `src/bin/async_test.rs`
**Covers:** `Task<T>`, `cx.spawn()`, `cx.background_spawn()`, `ForegroundExecutor`, `BackgroundExecutor`, `cx.defer()`
**Why second:** Async patterns are needed for image loading, file dialogs, and many real-world patterns.
**Example concept:** A view that spawns a background computation, updates a Model when done, and uses `cx.defer()` to batch UI updates. Shows task cancellation on drop.
**Estimated complexity:** Medium — async lifetime management is tricky.

### 1.3 Testing Utilities
**Doc:** `docs/gpui-usage/testing.md`
**Example:** Tests in `tests/testing_test.rs`
**Covers:** `VisualTestContext`, `cx.simulate_keystrokes()`, `cx.simulate_click()`, property testing (`iterations=N`), multiple `cx` params
**Why here:** Establishes testing patterns used by all subsequent phases.
**Example concept:** A small interactive view tested with simulated clicks and keystrokes. Property test for layout invariants.
**Estimated complexity:** Low-Medium — mostly test code, no binary needed.

---

## Phase 2 — Visual Elements

### 2.1 Images
**Doc:** `docs/gpui-usage/images.md`
**Example:** `src/bin/image_test.rs`
**Covers:** `Img`, `ImageCache`, `ImageSource`, `ObjectFit`
**Depends on:** Phase 1.2 (async loading)
**Example concept:** Load local and embedded images. Show all `ObjectFit` modes (Contain, Cover, Fill, ScaleDown, None) side by side. Demonstrate `ImageCache` reuse.
**Estimated complexity:** Medium — needs asset files, async loading path.
**Note:** Reference Zed's `image_gallery.rs` example.

### 2.2 SVG
**Doc:** `docs/gpui-usage/svg.md`
**Example:** `src/bin/svg_test.rs`
**Covers:** `Svg`, `Transformation` (rotate, scale, translate)
**Example concept:** Render SVG icons with different transformations. Show color tinting.
**Estimated complexity:** Low — straightforward element.

### 2.3 Styled & Interactive Text
**Doc:** `docs/gpui-usage/styled-text.md`
**Example:** `src/bin/styled_text_test.rs`
**Covers:** `StyledText`, `InteractiveText`, `TextRun`, `HighlightStyle`, `TextOverflow`
**Example concept:** Rich text paragraph with bold/italic/colored ranges. Clickable links via `InteractiveText`. Text truncation with ellipsis.
**Estimated complexity:** Medium — text run management, click handler ranges.

### 2.4 Animation
**Doc:** `docs/gpui-usage/animation.md`
**Example:** `src/bin/animation_test.rs`
**Covers:** `AnimationElement`, easing functions (linear, ease_in_out, bounce, etc.)
**Example concept:** Animate position, size, and opacity of elements. Show different easing curves side by side.
**Estimated complexity:** Medium — animation lifecycle, frame timing.
**Note:** Reference Zed's `animation.rs` example.

---

## Phase 3 — Layout

### 3.1 CSS Grid
**Doc:** `docs/gpui-usage/css-grid.md`
**Example:** `src/bin/grid_test.rs`
**Covers:** `Display::Grid`, `GridLocation`, `GridTemplate`, `GridPlacement`, `TemplateColumnMinSize`
**Example concept:** Holy grail layout (header, sidebar, content, footer). Grid with auto-flow, span, and min-size columns.
**Estimated complexity:** Medium — grid API surface is large.
**Note:** Reference Zed's `grid_layout.rs` example.

### 3.2 Virtualized Lists
**Doc:** `docs/gpui-usage/lists.md`
**Example:** `src/bin/list_test.rs`
**Covers:** `List`, `ListState`, `UniformList`
**Depends on:** Phase 1.1 (Model for list data)
**Example concept:** `UniformList` with 10k rows. `List` with variable-height items. Show scroll-to-item, dynamic insertion/removal.
**Estimated complexity:** High — virtualization is complex, performance-sensitive.
**Note:** Reference Zed's `data_table.rs` and `uniform_list.rs` examples.

### 3.3 Scroll Containers
**Doc:** `docs/gpui-usage/scroll.md`
**Example:** `src/bin/scroll_test.rs`
**Covers:** `overflow_scroll()`, `ScrollHandle`, `ScrollWheelEvent`, `ScrollDelta`
**Example concept:** Scrollable content area with programmatic scroll-to via `ScrollHandle`. Log scroll events showing delta types (line/pixel/page).
**Estimated complexity:** Low-Medium.
**Note:** Reference Zed's `scrollable.rs` example.

### 3.4 Overlays (Anchored & Deferred)
**Doc:** `docs/gpui-usage/overlays.md`
**Example:** `src/bin/overlay_test.rs`
**Covers:** `Anchored`, `Deferred`, `Tooltips` (`tooltip()`, `TooltipId`)
**Example concept:** Button that shows a popover (Anchored + Deferred). Tooltip on hover. Nested overlays. Window-boundary avoidance.
**Estimated complexity:** Medium — positioning logic, render ordering.
**Note:** Reference Zed's `popover.rs` example.

---

## Phase 4 — Rendering & Painting

### 4.1 Gradients
**Doc:** `docs/gpui-usage/gradients.md`
**Example:** `src/bin/gradient_test.rs`
**Covers:** `linear_gradient()`, `LinearColorStop`, `ColorSpace` (sRGB, Oklab)
**Example concept:** Gradient backgrounds, multi-stop gradients, color space comparison (sRGB vs Oklab).
**Estimated complexity:** Low.
**Note:** Reference Zed's `gradient.rs` example.

### 4.2 Shadows
**Doc:** `docs/gpui-usage/shadows.md`
**Example:** `src/bin/shadow_test.rs`
**Covers:** `window.paint_shadows()`, `BoxShadow`, `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`
**Example concept:** Cards with different shadow presets. Custom shadow with offset, blur, spread, color.
**Estimated complexity:** Low.
**Note:** Reference Zed's `shadow.rs` example.

### 4.3 Transforms
**Doc:** `docs/gpui-usage/transforms.md`
**Example:** `src/bin/transform_test.rs`
**Covers:** `TransformationMatrix`, 2D transforms (rotate, scale, skew, translate)
**Example concept:** Apply transforms to elements. Compose multiple transforms. Show transform origin behavior.
**Estimated complexity:** Medium — matrix math, coordinate space understanding.

---

## Phase 5 — Interaction & Events

### 5.1 Drag and Drop
**Doc:** `docs/gpui-usage/drag-drop.md`
**Example:** `src/bin/drag_drop_test.rs`
**Covers:** `.on_drag()`, `.on_drag_move()`, `.on_drop()`, `DragMoveEvent`
**Example concept:** Draggable cards between columns (kanban-style). Custom drag preview. Drop zone highlighting.
**Estimated complexity:** High — drag state management, visual feedback.
**Note:** Reference Zed's `drag_drop.rs` example.

### 5.2 File Drop
**Doc:** `docs/gpui-usage/file-drop.md`
**Example:** `src/bin/file_drop_test.rs`
**Covers:** `FileDropEvent` (Entered, Pending, Submit, Exited), `ExternalPaths`
**Example concept:** Drop zone that accepts files, shows file names and sizes. Visual feedback for drag-over state.
**Estimated complexity:** Medium — platform event integration.

### 5.3 Hitbox
**Doc:** `docs/gpui-usage/hitbox.md`
**Example:** Can extend existing `draw_test` or standalone `src/bin/hitbox_test.rs`
**Covers:** `Hitbox`, `HitboxId`, custom hit detection
**Example concept:** Non-rectangular clickable regions. Overlapping hitboxes with priority. Hit testing in canvas.
**Estimated complexity:** Medium — already partially used in draw_test.

### 5.4 Gestures (Pressure & Pinch)
**Doc:** `docs/gpui-usage/gestures.md`
**Example:** `src/bin/gesture_test.rs`
**Covers:** `MousePressureEvent`, `PressureStage` (force touch), `PinchEvent`
**Example concept:** Pressure-sensitive drawing (line width varies with force). Pinch-to-zoom on a canvas.
**Estimated complexity:** Medium — macOS-specific, needs trackpad/Force Touch hardware.

---

## Phase 6 — Text & Typography

### 6.1 Typography Controls
**Doc:** `docs/gpui-usage/typography.md`
**Example:** `src/bin/typography_test.rs`
**Covers:** `LineWrapperHandle`, `TextOverflow` (ellipsis start/end)
**Note:** `Font`, `FontWeight`, `FontStyle`, `WrappedLine` are partially covered in `canvas.md` and `text-input.md`. This doc fills the gaps: line wrapping control and text overflow.
**Example concept:** Text block with configurable wrapping. Truncated labels with start/end ellipsis. Font specimen display.
**Estimated complexity:** Low-Medium.

---

## Phase 7 — Platform Integration

These are macOS-specific or platform-dependent. Group into a single doc with sub-sections for the simpler APIs, and separate docs for the complex ones.

### 7.1 File Dialogs & System Prompts
**Doc:** `docs/gpui-usage/dialogs.md`
**Example:** `src/bin/dialog_test.rs`
**Covers:** `prompt_for_paths()`, `prompt_for_new_path()`, `PromptLevel`, `PromptButton`
**Example concept:** Open file picker, save file picker, confirmation dialog with different prompt levels.
**Estimated complexity:** Low — thin wrappers over native APIs.

### 7.2 Display & Appearance
**Doc:** `docs/gpui-usage/display-appearance.md`
**Example:** `src/bin/appearance_test.rs`
**Covers:** `displays()`, `primary_display()`, `find_display()`, `window_appearance()` (dark/light mode)
**Example concept:** List connected displays with resolution/scale. Adapt UI colors to system appearance. Window positioning on specific displays.
**Estimated complexity:** Low.

### 7.3 Platform Utilities (Bundled)
**Doc:** `docs/gpui-usage/platform-utils.md`
**Example:** `src/bin/platform_test.rs`
**Covers:** `Keychain`, `URL schemes`, `keyboard_layout()`, `thermal_state()`, `set_dock_menu()`, `add_recent_document()`, `open_with_system()`, `reveal_path()`
**Example concept:** A utility dashboard that demonstrates each API. Read/write keychain. Open URLs. Show thermal state. Set dock menu.
**Estimated complexity:** Medium (breadth) — many small APIs to wire up.

### 7.4 Screen Capture & Surface
**Doc:** `docs/gpui-usage/screen-capture.md`
**Example:** `src/bin/capture_test.rs`
**Covers:** `ScreenCaptureSource`, `ScreenCaptureStream`, `Surface`
**Example concept:** Capture screen content, display in a Surface element.
**Estimated complexity:** High — platform permissions, pixel buffer management.
**Note:** This is the most complex platform feature. May require entitlements.

---

## Execution Order & Dependencies

```
Phase 1 (Foundation)
  1.1 State Management
  1.2 Async & Tasks
  1.3 Testing Utilities
      │
      ▼
Phase 2 (Visual Elements)          Phase 4 (Rendering)
  2.1 Images                         4.1 Gradients
  2.2 SVG                            4.2 Shadows
  2.3 Styled Text                    4.3 Transforms
  2.4 Animation
      │
      ▼
Phase 3 (Layout)              Phase 5 (Interaction)
  3.1 CSS Grid                  5.1 Drag and Drop
  3.2 Virtualized Lists         5.2 File Drop
  3.3 Scroll Containers         5.3 Hitbox
  3.4 Overlays                  5.4 Gestures
      │                              │
      ▼                              ▼
Phase 6 (Text)                Phase 7 (Platform)
  6.1 Typography                7.1 Dialogs
                                7.2 Display & Appearance
                                7.3 Platform Utilities
                                7.4 Screen Capture
```

- **Phase 1** must come first — everything depends on state and async patterns.
- **Phases 2-5** can be worked in parallel or interleaved; dependencies are noted per item.
- **Phase 6** is small and can slot in anywhere after Phase 1.
- **Phase 7** is independent but least valuable for general GPUI understanding — do last.

---

## Per-Item Deliverables

For each item:

1. **Documentation page** (`docs/gpui-usage/<name>.md`) following the template:
   - What it is and what it does
   - Preconditions for use (imports, setup, dependencies)
   - Signature for usage
   - Relevant macros and traits
   - Usage examples (working code from our project)
   - Post-conditions / destruction requirements (cleanup, drop behavior, subscription lifetime)
   - Testing (how to test this feature, example test code)
   - Surprises, anti-patterns, and bugs

2. **Example binary** (`src/bin/<name>.rs`) — minimal, focused, runnable via `cargo run --bin <name>`

3. **Integration test** (`tests/<name>.rs`) — where testable without a live window

4. **Update `untested.md`** — remove the item once documented

---

## Tracking

| # | Item | Plan | Doc | Example | Tests | Status |
|---|------|------|-----|---------|-------|--------|
| 1.1 | State Management | [plan](state-management-plan.md) | `state-management.md` | — | `tests/state_management_test.rs` | **Done** |
| 1.2 | Async & Tasks | [plan](async-tasks-plan.md) | `async-tasks.md` | — | `tests/async_tasks_test.rs` | **Done** |
| 1.3 | Testing Utilities | [plan](testing-plan.md) | `testing.md` | — | `tests/testing_test.rs` | **Done** |
| 2.1 | Images | [plan](images-plan.md) | `images.md` | — | `tests/images_test.rs` | **Done** |
| 2.2 | SVG | [plan](svg-plan.md) | `svg.md` | — | `tests/svg_test.rs` | **Done** |
| 2.3 | Styled & Interactive Text | [plan](styled-text-plan.md) | `styled-text.md` | — | `tests/styled_text_test.rs` | **Done** |
| 2.4 | Animation | [plan](animation-plan.md) | `animation.md` | — | `tests/animation_test.rs` | **Done** |
| 3.1 | CSS Grid | [plan](css-grid-plan.md) | `css-grid.md` | — | `tests/css_grid_test.rs` | **Done** |
| 3.2 | Virtualized Lists | [plan](lists-plan.md) | `lists.md` | — | `tests/lists_test.rs` | **Done** |
| 3.3 | Scroll Containers | [plan](scroll-plan.md) | `scroll.md` | — | `tests/scroll_test.rs` | **Done** |
| 3.4 | Overlays | [plan](overlays-plan.md) | `overlays.md` | — | `tests/overlays_test.rs` | **Done** |
| 4.1 | Gradients | [plan](gradients-plan.md) | `gradients.md` | — | `tests/gradients_test.rs` | **Done** |
| 4.2 | Shadows | [plan](shadows-plan.md) | `shadows.md` | — | `tests/shadows_test.rs` | **Done** |
| 4.3 | Transforms | [plan](transforms-plan.md) | `transforms.md` | — | `tests/transforms_test.rs` | **Done** |
| 5.1 | Drag and Drop | [plan](drag-drop-plan.md) | `drag-drop.md` | — | `tests/drag_drop_test.rs` | **Done** |
| 5.2 | File Drop | [plan](file-drop-plan.md) | `file-drop.md` | — | `tests/file_drop_test.rs` | **Done** |
| 5.3 | Hitbox | [plan](hitbox-plan.md) | `hitbox.md` | — | `tests/hitbox_test.rs` | **Done** |
| 5.4 | Gestures | [plan](gestures-plan.md) | `gestures.md` | — | `tests/gestures_test.rs` | **Done** |
| 6.1 | Typography | [plan](typography-plan.md) | `typography.md` | — | `tests/typography_test.rs` | **Done** |
| 7.1 | File Dialogs | [plan](dialogs-plan.md) | `dialogs.md` | — | `tests/dialogs_test.rs` | **Done** |
| 7.2 | Display & Appearance | [plan](display-appearance-plan.md) | `display-appearance.md` | — | `tests/display_appearance_test.rs` | **Done** |
| 7.3 | Platform Utilities | [plan](platform-utils-plan.md) | `platform-utils.md` | — | `tests/platform_utils_test.rs` | **Done** |
| 7.4 | Screen Capture | [plan](screen-capture-plan.md) | `screen-capture.md` | — | `tests/screen_capture_test.rs` | **Done** |

**Total: 23 new items across 7 phases (6 items already covered in existing docs)**
