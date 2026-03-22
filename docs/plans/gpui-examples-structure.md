# GPUI Playground Examples - Project Structure Plan

## Context

Setting up the gpui-play project as a collection of example executables that each explore different GPUI concepts (menus, windows, drawing, buttons) plus a combined example. The goal is a learning playground with runnable, isolated examples.

## Project Structure

```
gpui-play/
  Cargo.toml          (add gpui_platform dep)
  src/
    lib.rs             (new: pub mod common)
    common.rs          (new: shared bootstrap helpers)
    main.rs            (modified: example launcher/index)
    bin/
      menus.rs         (menu creation and management)
      windows.rs       (window management)
      drawing.rs       (freehand mouse drawing)
      buttons.rs       (button handling and states)
      text_input.rs    (text input with vim-mode keymaps)
      sketchpad.rs     (combined application)
```

Run examples via `cargo run --bin <name>` (e.g., `cargo run --bin menus`).

## Implementation Steps

### 1. Cargo.toml - Add gpui_platform dependency
Add `gpui_platform = { git = "https://github.com/zed-industries/zed" }` alongside gpui. This provides the `application()` entry point.

### 2. src/lib.rs + src/common.rs - Shared helpers
- `lib.rs`: just `pub mod common;`
- `common.rs`:
  - `run_app<V: Render>(title, build_view_fn)` - bootstraps app, opens one window, activates
  - `default_window_options(title) -> WindowOptions` - 800x600, normal, titled, resizable
  - Small color palette constants for use across examples

### 3. src/bin/buttons.rs - Button Handling
Simplest interactive example, proves the structure works.
- `ButtonDemo` view with click counter, toggle state, last-clicked tracking
- Button variants: counter, hover/active states, toggle, colored row, disabled-style
- Status bar showing state

### 4. src/bin/menus.rs - Menu Creation
- `MenuDemo` view displaying last-triggered action
- `actions!()` macro for NewFile, OpenFile, Save, Quit, etc.
- `cx.set_menus(...)` with App, File, Edit, Options menus
- Checked items, disabled items, submenus, separators
- `cx.on_action()` handlers updating the view

### 5. src/bin/windows.rs - Window Management
- `ControlPanel` main window with buttons to open different window types
- Opens: normal, popup, dialog, non-resizable, titlebar-less windows
- Each child window shows its type and has a Close button
- Tracks open windows via `Vec<WindowHandle>`

### 6. src/bin/drawing.rs - Mouse Drawing
- `DrawingCanvas` view with strokes vec, current stroke, drawing flag, color
- `canvas()` element for painting completed + in-progress strokes
- `PathBuilder::stroke()` for each stroke, `window.paint_path()` to render
- Mouse handlers on wrapping `div().id("canvas")`: mouse_down starts stroke, mouse_move extends it, mouse_up completes it
- Clear button to reset

### 7. src/bin/text_input.rs - Text Input with Vim-Mode Keymaps

A text input widget demonstrating `EntityInputHandler`, styled text rendering, and switchable keymaps including a vim mode.

**View struct: `TextEditor`**
- `focus_handle: FocusHandle`
- `content: String` - the editable text buffer (UTF-8)
- `selected_range: Range<usize>` - cursor/selection as byte offsets
- `selection_reversed: bool` - selection direction
- `marked_range: Option<Range<usize>>` - IME composition range
- `last_layout: Option<ShapedLine>` - cached text layout for hit testing
- `last_bounds: Option<Bounds<Pixels>>` - cached element bounds
- `mode: EditorMode` - current keymap mode (Normal, Insert, Visual)
- `status_message: String` - mode indicator / command feedback

**EditorMode enum:**
```
enum EditorMode { Insert, Normal, Visual }
```

**Implements `EntityInputHandler`:**
- `text_for_range()` - read text in UTF-16 range (convert from UTF-8)
- `selected_text_range()` - return current selection
- `replace_text_in_range()` - insert/delete text
- `replace_and_mark_text_in_range()` - IME composition support
- `marked_text_range()` / `unmark_text()` - IME lifecycle
- `bounds_for_range()` - map text range to screen coords (for IME popup)
- `character_index_for_point()` - mouse click to cursor position

**Actions (via `actions!()` macro):**
- Standard: `Backspace`, `Delete`, `Left`, `Right`, `SelectLeft`, `SelectRight`, `SelectAll`, `Home`, `End`, `Copy`, `Cut`, `Paste`
- Vim normal mode: `VimLeft`, `VimRight`, `VimUp`, `VimDown`, `VimWordForward`, `VimWordBack`, `VimStartOfLine`, `VimEndOfLine`, `VimInsertBefore`, `VimInsertAfter`, `VimInsertLineBelow`, `VimInsertLineAbove`, `VimDeleteChar`, `VimDeleteLine`, `VimVisualMode`
- Vim visual mode: `VimYank`, `VimDeleteSelection`
- Mode switching: `EnterNormalMode`, `EnterInsertMode`, `EnterVisualMode`

**Keymap architecture using `KeyContext` scoping:**

The render method sets context dynamically based on current mode:
```rust
div()
    .key_context("TextEditor")  // always present
    .key_context(match self.mode {
        EditorMode::Insert => "editor_mode_insert",
        EditorMode::Normal => "editor_mode_normal",
        EditorMode::Visual => "editor_mode_visual",
    })
    .track_focus(&self.focus_handle)
    .on_action(cx.listener(Self::backspace))
    // ... register all action handlers
```

Keybindings registered at app level with context predicates:
```rust
cx.bind_keys([
    // Always active (insert mode / standard editing)
    KeyBinding::new("backspace", Backspace, Some("TextEditor && editor_mode_insert")),
    KeyBinding::new("left", Left, Some("TextEditor && editor_mode_insert")),
    KeyBinding::new("cmd-a", SelectAll, Some("TextEditor")),
    KeyBinding::new("cmd-c", Copy, Some("TextEditor")),
    KeyBinding::new("cmd-v", Paste, Some("TextEditor")),

    // Vim normal mode
    KeyBinding::new("h", VimLeft, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("l", VimRight, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("w", VimWordForward, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("b", VimWordBack, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("0", VimStartOfLine, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("shift-$", VimEndOfLine, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("i", VimInsertBefore, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("a", VimInsertAfter, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("o", VimInsertLineBelow, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("x", VimDeleteChar, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("v", VimVisualMode, Some("TextEditor && editor_mode_normal")),
    KeyBinding::new("escape", EnterNormalMode, Some("TextEditor && editor_mode_insert")),
    KeyBinding::new("escape", EnterNormalMode, Some("TextEditor && editor_mode_visual")),

    // Vim visual mode
    KeyBinding::new("y", VimYank, Some("TextEditor && editor_mode_visual")),
    KeyBinding::new("d", VimDeleteSelection, Some("TextEditor && editor_mode_visual")),
    KeyBinding::new("h", VimLeft, Some("TextEditor && editor_mode_visual")),
    KeyBinding::new("l", VimRight, Some("TextEditor && editor_mode_visual")),
]);
```

**Text rendering with formatting:**
- Use `window.text_system().shape_line(text, font_size, &runs, None)` for layout
- `TextRun` array for styled segments:
  - Normal text in default color
  - Selection range highlighted with background color
  - IME marked text with underline (`UnderlineStyle { wavy: false }`)
- Cursor rendered as a painted rect via `canvas()` overlay:
  - Insert mode: thin line cursor (1px wide)
  - Normal mode: block cursor (character-width)
  - Visual mode: selection highlight over range
- Mode indicator bar at bottom: shows "-- INSERT --", "-- NORMAL --", or "-- VISUAL --" with distinct colors

**Mouse interaction:**
- Click to position cursor (using `character_index_for_point()` via `ShapedLine::closest_index_for_x()`)
- Click-and-drag for selection
- Double-click to select word

**Key APIs exercised:**
- `EntityInputHandler` trait (full implementation)
- `KeyBinding::new()` with context predicates for modal keymaps
- `.key_context()` for dynamic context switching
- `StyledText` / `TextRun` for formatted text rendering
- `ShapedLine` for text layout and hit testing
- `FocusHandle` and `.track_focus()` for keyboard input routing
- UTF-8 ↔ UTF-16 conversion for OS input compatibility
- `cx.read_from_clipboard()` / `cx.write_to_clipboard()` for copy/paste

### 8. src/bin/sketchpad.rs - Combined Application
- Drawing canvas + color picker toolbar + stroke width buttons
- App menu bar (File: New/Export, Edit: Undo/Clear, View: Color Palette window)
- Secondary color palette window opened via menu
- `impl Global for AppState` for cross-window color/width sharing

### 9. src/main.rs - Launcher Index
Simple window listing all examples with `cargo run --bin <name>` instructions.

## Development Approach: Red-Green TDD

All examples follow test-driven development. For each example:
1. **Red:** Write tests first that define expected behavior (they will fail)
2. **Run tests** to confirm they fail
3. **Green:** Write the minimum implementation to make tests pass
4. **Run tests** to confirm they pass
5. **Refactor** if needed, keeping tests green

### What to test per example
GPUI provides a test harness via `gpui::TestAppContext`. Testable logic should be separated from rendering where possible:
- **State logic** (pure functions): button counters, toggle state, mode transitions, text buffer operations, vim motions — test directly as unit tests
- **View behavior** (requires `TestAppContext`): action dispatch, keybinding resolution, focus changes, state mutations via actions
- **Rendering** (visual verification): run the example manually — not unit-tested

Each `src/bin/<example>.rs` should have a corresponding test module (`#[cfg(test)] mod tests`) or, for shared/reusable logic, tests in `src/lib.rs` or `src/common.rs`.

### Test structure for text_input.rs (most complex example)
Tests for the text editor should cover:
- **Buffer operations:** insert text, delete (backspace/forward), replace range
- **Cursor movement:** left, right, home, end, word-forward, word-back
- **Selection:** select left/right, select all, visual mode selection
- **Mode transitions:** Insert→Normal (escape), Normal→Insert (i/a/o), Normal→Visual (v), Visual→Normal (escape)
- **Vim motions in normal mode:** h/l/w/b/0/$/x navigation and deletion
- **Vim visual mode:** selection extension with h/l, yank (y), delete (d)
- **Copy/paste:** cut, copy, paste via clipboard
- **UTF-8 ↔ UTF-16 conversion:** boundary handling for multi-byte characters

## Implementation Order
1. Cargo.toml + `cargo check`
2. lib.rs + common.rs + `cargo check`
3. buttons.rs: tests → implementation → green
4. menus.rs: tests → implementation → green
5. windows.rs: tests → implementation → green
6. drawing.rs: tests → implementation → green
7. text_input.rs: tests → implementation → green (most test-heavy)
8. sketchpad.rs: tests → implementation → green
9. main.rs (update anytime)

## Key API Notes
- Entry: `gpui_platform::application().run(|cx: &mut App| { ... })`
- Windows: `cx.open_window(opts, |_, cx| cx.new(|_| view))`
- Views: `impl Render for V { fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement }`
- Mouse handlers need stateful element (`.id("name")`)
- Canvas wrapping: attach mouse handlers to parent div, not canvas directly
- `PathBuilder::build()` returns `Result<Vec<Path<Pixels>>>` - iterate to paint each

## Additional GPUI Capabilities

Beyond the examples planned above, GPUI offers these additional capabilities that could be explored in future examples.

### Animation
- `Animation` / `AnimationExt` with customizable duration, easing (linear, ease_in_out, bounce, quadratic), looping
- Chain multiple animations on a single element; progress callback receives delta 0.0-1.0
- `window.request_animation_frame()` for frame-by-frame rendering
- Reference: `examples/animation.rs`

### Text Input
- `EntityInputHandler` trait for custom text editing views
- Text range selection/replacement, marked text (composition input), character index from point
- Clipboard integration: `cx.read_from_clipboard()`, `cx.write_to_clipboard()`
- Reference: `examples/input.rs`

### Image Rendering
- `Img` element with sources: file paths, URLs, `Arc<RenderImage>`, custom loaders
- Formats: PNG, GIF (animated), WebP, JPEG
- `ObjectFit` modes: Fill, Contain, Cover, ScaleDown, None
- Built-in image caching and loading state
- Reference: `examples/image_gallery.rs`, `examples/gif_viewer.rs`

### Drag and Drop
- Generic drag payload (any `Clone` type) with custom drag view rendering
- `on_drag()`, `on_drag_move()`, `on_drop()` handlers
- File drop events: Entered, Pending, Submit, Exited
- Reference: `examples/drag_drop.rs`

### Keyboard & Focus
- Action-based key binding system via `actions!()` macro and `KeyBinding`
- Multi-key sequences (e.g., "cmd-k left")
- Scoped bindings via `KeyContext`
- Focus management: `FocusHandle`, `tab_index()`, `tab_stop()`, focus trapping
- Focus-visible distinction (keyboard vs mouse focus)
- Reference: `examples/focus_visible.rs`

### Layout System
- **Flexbox:** direction, wrap, justify, align, gap
- **CSS Grid:** rows, columns, col/row span, placement via `GridLocation` / `GridTemplate`
- Absolute/relative positioning, z-index, overflow control
- Powered by Taffy layout engine
- Reference: `examples/grid_layout.rs`

### Scrolling & Virtual Lists
- `uniform_list()` - optimized for equal-height items, handles 10k+ rows
- `list()` - variable height items with scroll tracking
- `ScrollHandle` for programmatic scroll control
- Reference: `examples/data_table.rs`, `examples/uniform_list.rs`

### Tooltips & Popovers
- `anchored()` - position elements relative to anchor corner (TopLeft, TopRight, etc.)
- `deferred()` - render in separate layer for floating UI
- Snap-to-window with configurable margins
- Reference: `examples/popover.rs`

### Gradients
- `linear_gradient()` with angle and color stops at percentage positions
- Color space selection: sRGB or Oklab
- Reference: `examples/gradient.rs`

### SVG Rendering
- `Svg` element loading from assets or file paths
- Transformations: rotation, scaling, translation
- Color and styling applied to SVGs
- Reference: `examples/animation.rs` (animated SVG)

### Shadows
- `BoxShadow` with blur radius, spread, offset, color
- Utility methods: `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`
- Multiple shadows per element
- Reference: `examples/shadow.rs`

### Async / Timers
- `Task<T>` with foreground/background executors
- `.detach()` for fire-and-forget, `.fallible()` for graceful cancellation
- Priority scheduling: High, Normal, Low
- Timer creation with `Duration`

### Reactive State / Observers
- `Model<T>` for shared observable state across views
- `EventEmitter` trait for event emission
- `cx.subscribe()`, `cx.observe()`, `cx.observe_global()` for reactive updates
- `cx.notify()` to trigger view re-render

### Platform Integration
- Window appearance (dark/light mode detection)
- Display info and DPI scaling
- Cursor style control per element region
- System prompt dialogs (`PromptLevel`, `PromptButton`)

### Text Styling
- Font selection, weight, size, style (italic)
- Text alignment, line clamping, text overflow (ellipsis)
- Text decoration (underline, strikethrough)
- Font features

### Events (Full List)
- **Mouse:** down, up, move, click, pressure, double-click
- **Keyboard:** key down, key up, modifiers changed
- **Touch:** multi-touch with phases
- **Gesture:** pinch (macOS)
- **File drop:** entered, pending, submitted, exited
- **Wheel:** scroll wheel movement

### Not Available
- No dedicated video/media playback API (GIF animation is supported via the image system)

## Verification
- `cargo test` after writing each test (confirm red) and after each implementation (confirm green)
- `cargo check` after each file addition
- `cargo run --bin <name>` to visually verify each example
- `cargo clippy` for lint check
