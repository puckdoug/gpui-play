# GPUI Documentation Index

A guide to all GPUI component documentation in this project. Start with the [Overview](an_overview.md) for architecture and key concepts, then use the sections below to find what you need.

## Getting Started

| Doc | What you'll learn |
|-----|-------------------|
| [Overview](an_overview.md) | Architecture, lifecycle, views, elements, styling, focus |
| [App](app.md) | Application startup, global state, action handlers, quit modes |
| [Window](window.md) | Creating windows, WindowOptions, traffic light buttons |
| [Testing](testing.md) | `#[gpui::test]`, TestAppContext, simulating clicks/keystrokes |

## Building UI

### Layout

| Doc | When to use |
|-----|-------------|
| [CSS Grid](css-grid.md) | 2D grid layouts — dashboards, holy grail, card grids |
| [Lists](lists.md) | Large data sets — `uniform_list` (10k+ rows) and `list` (variable height) |
| [Scroll](scroll.md) | Scrollable containers — `overflow_scroll()`, `ScrollHandle` |
| [Overlays](overlays.md) | Popovers, dropdowns, tooltips — `deferred` + `anchored` |

### Elements

| Doc | When to use |
|-----|-------------|
| [Button](button.md) | Clickable elements with hover/active states |
| [Text Input](text-input.md) | Editable text fields, IME, undo/redo |
| [Styled Text](styled-text.md) | Rich text — bold/italic ranges, clickable links |
| [Images](images.md) | PNG/JPEG/GIF/WebP rendering, `ObjectFit` modes |
| [SVG](svg.md) | Icon rendering with color tinting and transforms |
| [Canvas & Drawing](canvas.md) | Custom painting — PathBuilder, shapes, hit testing |

### Visual Effects

| Doc | When to use |
|-----|-------------|
| [Animation](animation.md) | Time-based property interpolation, easing curves |
| [Gradients](gradients.md) | Linear gradient backgrounds, sRGB vs Oklab |
| [Shadows](shadows.md) | Drop shadows — presets (sm/md/lg) and custom BoxShadow |
| [Transforms](transforms.md) | 2D affine transforms (rotate, scale, translate) — SVG and canvas only |
| [Typography](typography.md) | Text overflow/truncation, line wrapping control |

## Interaction

| Doc | When to use |
|-----|-------------|
| [Menus](menus.md) | Application menu bar, keyboard shortcuts |
| [Drag & Drop](drag-drop.md) | Typed drag payloads, custom previews, drop zones |
| [File Drop](file-drop.md) | OS file drag-and-drop into windows |
| [Hitbox](hitbox.md) | Custom click regions in canvas elements |
| [Gestures](gestures.md) | Force Touch pressure, pinch-to-zoom (macOS) |

## State & Async

| Doc | When to use |
|-----|-------------|
| [State Management](state-management.md) | `Entity<T>`, `EventEmitter`, `cx.observe()`, `cx.subscribe()` |
| [Async & Tasks](async-tasks.md) | `cx.spawn()`, background tasks, `cx.defer()`, cancellation |

## Platform Integration

| Doc | When to use |
|-----|-------------|
| [Display & Appearance](display-appearance.md) | Connected displays, dark/light mode detection |
| [Dialogs](dialogs.md) | Native file pickers, system alert/confirmation prompts |
| [Platform Utils](platform-utils.md) | Keychain, URL schemes, thermal state, dock menu, reveal in Finder |
| [Screen Capture](screen-capture.md) | Screen/window capture, Surface element (macOS) |

## Quick Reference: Common Tasks

| I want to... | Start here |
|--------------|------------|
| Show a list of 1000+ items | [Lists](lists.md) — `uniform_list` |
| Add a popup menu | [Overlays](overlays.md) — `deferred(anchored(...))` |
| Make text bold/colored | [Styled Text](styled-text.md) — `StyledText` + `HighlightStyle` |
| Animate an element | [Animation](animation.md) — `.with_animation()` |
| Handle drag and drop | [Drag & Drop](drag-drop.md) — `.on_drag()` / `.on_drop()` |
| Share state between views | [State Management](state-management.md) — `Entity<T>` + `cx.observe()` |
| Run work off the main thread | [Async & Tasks](async-tasks.md) — `cx.spawn()` + `background_spawn()` |
| Open a file picker | [Dialogs](dialogs.md) — `prompt_for_paths()` |
| Respond to dark mode | [Display & Appearance](display-appearance.md) — `cx.window_appearance()` |
| Test my view with keystrokes | [Testing](testing.md) — `simulate_keystrokes()` |
| Paint custom shapes | [Canvas](canvas.md) — `canvas()` + `PathBuilder` |
| Add a tooltip | [Overlays](overlays.md) — `.tooltip()` |

## Zed GPUI Examples

Reference implementations from the [Zed repository](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples):

| Example | Features | Related docs |
|---------|----------|-------------|
| `animation.rs` | Element animation with easing | [Animation](animation.md) |
| `data_table.rs` | Virtual list with 10k rows, sorting | [Lists](lists.md) |
| `drag_drop.rs` | Drag payload, drag view, drop zones | [Drag & Drop](drag-drop.md) |
| `focus_visible.rs` | Focus ring styling (keyboard vs mouse) | [Testing](testing.md) |
| `gif_viewer.rs` | Animated GIF loading and display | [Images](images.md) |
| `gradient.rs` | Linear gradients, color space switching | [Gradients](gradients.md) |
| `grid_layout.rs` | CSS Grid holy grail layout | [CSS Grid](css-grid.md) |
| `image_gallery.rs` | Image loading, ObjectFit modes | [Images](images.md) |
| `painting.rs` | PathBuilder, strokes, fills, gradients | [Canvas](canvas.md) |
| `popover.rs` | Anchored floating UI, nested deferred | [Overlays](overlays.md) |
| `scrollable.rs` | Scroll handling | [Scroll](scroll.md) |
| `shadow.rs` | Box shadows | [Shadows](shadows.md) |
| `tab_stop.rs` | Tab navigation | [Overview](an_overview.md) |
| `text_layout.rs` | Text measurement and layout | [Styled Text](styled-text.md) |
| `uniform_list.rs` | Virtual scrolling for uniform items | [Lists](lists.md) |
| `window.rs` | Multiple window types and configurations | [Window](window.md) |
| `window_positioning.rs` | Window placement on displays | [Display & Appearance](display-appearance.md) |

---

## Alphabetical Index

Every type, trait, function, method, and concept documented in this project, with links to all pages where it appears.

| Item | Documents |
|------|-----------|
| `.active()` | [button](button.md) |
| `.anchor()` | [overlays](overlays.md) |
| `.bg()` | [an_overview](an_overview.md), [window](window.md), [button](button.md), [gradients](gradients.md), [shadows](shadows.md) |
| `.bind_keys()` | [an_overview](an_overview.md), [app](app.md), [menus](menus.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `.child()` | [an_overview](an_overview.md), [window](window.md), [button](button.md), [overlays](overlays.md) |
| `.col_end()` | [css-grid](css-grid.md) |
| `.col_span()` | [css-grid](css-grid.md) |
| `.col_span_full()` | [css-grid](css-grid.md) |
| `.col_start()` | [css-grid](css-grid.md) |
| `.cursor_pointer()` | [button](button.md) |
| `.detach()` | [app](app.md), [state-management](state-management.md), [async-tasks](async-tasks.md) |
| `.detach_and_log_err()` | [async-tasks](async-tasks.md) |
| `.external_path()` | [svg](svg.md) |
| `.flex()` | [an_overview](an_overview.md), [window](window.md) |
| `.focus()` | [testing](testing.md) |
| `.gap_2()` / `.gap_4()` | [css-grid](css-grid.md) |
| `.grayscale()` | [images](images.md) |
| `.grid()` | [css-grid](css-grid.md) |
| `.grid_cols()` | [css-grid](css-grid.md) |
| `.grid_rows()` | [css-grid](css-grid.md) |
| `.h_full()` | [scroll](scroll.md), [lists](lists.md) |
| `.hover()` | [button](button.md) |
| `.hoverable_tooltip()` | [overlays](overlays.md) |
| `.id()` | [button](button.md), [scroll](scroll.md), [lists](lists.md), [overlays](overlays.md), [drag-drop](drag-drop.md) |
| `.image_cache()` | [images](images.md) |
| `.object_fit()` | [images](images.md) |
| `.on_action()` | [an_overview](an_overview.md), [app](app.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `.on_click()` | [button](button.md) |
| `.on_drag()` | [drag-drop](drag-drop.md) |
| `.on_drag_move()` | [drag-drop](drag-drop.md) |
| `.on_drop()` | [drag-drop](drag-drop.md) |
| `.on_mouse_down()` | [button](button.md), [canvas](canvas.md), [testing](testing.md) |
| `.on_pinch()` | [gestures](gestures.md) |
| `.on_scroll_wheel()` | [scroll](scroll.md) |
| `.overflow_scroll()` | [scroll](scroll.md) |
| `.overflow_x_scroll()` | [scroll](scroll.md) |
| `.overflow_y_scroll()` | [scroll](scroll.md) |
| `.path()` (on Svg) | [svg](svg.md) |
| `.position()` (on Anchored) | [overlays](overlays.md) |
| `.priority()` | [overlays](overlays.md) |
| `.repeat()` | [animation](animation.md) |
| `.row_span()` | [css-grid](css-grid.md) |
| `.row_span_full()` | [css-grid](css-grid.md) |
| `.row_start()` | [css-grid](css-grid.md) |
| `.shadow()` | [shadows](shadows.md) |
| `.shadow_lg()` | [shadows](shadows.md) |
| `.shadow_md()` | [shadows](shadows.md) |
| `.shadow_sm()` | [shadows](shadows.md) |
| `.shadow_xl()` | [shadows](shadows.md) |
| `.size_full()` | [an_overview](an_overview.md), [window](window.md), [canvas](canvas.md) |
| `.snap_to_window()` | [overlays](overlays.md) |
| `.text_color()` | [an_overview](an_overview.md), [window](window.md), [svg](svg.md) |
| `.text_overflow()` | [styled-text](styled-text.md), [typography](typography.md) |
| `.tooltip()` | [overlays](overlays.md), [styled-text](styled-text.md) |
| `.track_focus()` | [an_overview](an_overview.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `.track_scroll()` | [scroll](scroll.md), [lists](lists.md) |
| `.when()` | [button](button.md) |
| `.with_animation()` | [animation](animation.md) |
| `.with_animations()` | [animation](animation.md) |
| `.with_default_highlights()` | [styled-text](styled-text.md) |
| `.with_easing()` | [animation](animation.md) |
| `.with_fallback()` | [images](images.md) |
| `.with_highlights()` | [styled-text](styled-text.md) |
| `.with_loading()` | [images](images.md) |
| `.with_runs()` | [styled-text](styled-text.md) |
| `.with_transformation()` | [svg](svg.md), [transforms](transforms.md), [animation](animation.md) |
| `#[gpui::test]` | [an_overview](an_overview.md), [state-management](state-management.md), [async-tasks](async-tasks.md), [testing](testing.md) |
| `#[gpui::test(iterations = N)]` | [testing](testing.md) |
| `Action` (trait) | [an_overview](an_overview.md), [app](app.md), [menus](menus.md) |
| `actions!()` | [an_overview](an_overview.md), [app](app.md), [menus](menus.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `Anchored` | [overlays](overlays.md) |
| `anchored()` | [overlays](overlays.md) |
| `Animation` | [animation](animation.md) |
| `Animation::new()` | [animation](animation.md) |
| `AnimationElement` | [animation](animation.md) |
| `AnimationExt` (trait) | [animation](animation.md) |
| `AnyElement` | [lists](lists.md) |
| `AnyView` | [overlays](overlays.md) |
| `App` | [an_overview](an_overview.md), [app](app.md), [window](window.md), [menus](menus.md), [state-management](state-management.md), [async-tasks](async-tasks.md), [testing](testing.md), [dialogs](dialogs.md) |
| `AppContext` (trait) | [state-management](state-management.md), [overlays](overlays.md) |
| `application()` | [an_overview](an_overview.md), [app](app.md) |
| `AsyncApp` | [app](app.md), [async-tasks](async-tasks.md) |
| `AsyncWindowContext` | [async-tasks](async-tasks.md) |
| `BackgroundExecutor` | [async-tasks](async-tasks.md), [testing](testing.md) |
| `bounce()` | [animation](animation.md) |
| `Bounds<Pixels>` | [window](window.md), [canvas](canvas.md), [scroll](scroll.md), [hitbox](hitbox.md), [shadows](shadows.md) |
| `BoxShadow` | [shadows](shadows.md) |
| `canvas()` | [canvas](canvas.md), [hitbox](hitbox.md) |
| `ClickEvent` | [button](button.md) |
| `ClipboardItem` | [app](app.md) |
| `ColorSpace` | [gradients](gradients.md) |
| `ColorSpace::Oklab` | [gradients](gradients.md) |
| `ColorSpace::Srgb` | [gradients](gradients.md) |
| `Context<T>` | [an_overview](an_overview.md), [state-management](state-management.md), [async-tasks](async-tasks.md) |
| `Corner` | [overlays](overlays.md) |
| `CursorStyle` | [button](button.md), [text-input](text-input.md) |
| `cx.activate()` | [an_overview](an_overview.md), [app](app.md) |
| `cx.add_recent_document()` | [platform-utils](platform-utils.md) |
| `cx.background_spawn()` | [async-tasks](async-tasks.md) |
| `cx.bind_keys()` | [an_overview](an_overview.md), [app](app.md), [menus](menus.md), [testing](testing.md) |
| `cx.defer()` | [app](app.md), [async-tasks](async-tasks.md) |
| `cx.delete_credentials()` | [platform-utils](platform-utils.md) |
| `cx.dispatch_action()` | [app](app.md), [button](button.md), [testing](testing.md) |
| `cx.displays()` | [app](app.md), [display-appearance](display-appearance.md) |
| `cx.emit()` | [state-management](state-management.md) |
| `cx.focus_handle()` | [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `cx.global()` / `cx.global_mut()` | [app](app.md), [menus](menus.md) |
| `cx.keyboard_layout()` | [app](app.md), [platform-utils](platform-utils.md) |
| `cx.listener()` | [an_overview](an_overview.md), [button](button.md), [testing](testing.md), [drag-drop](drag-drop.md) |
| `cx.new()` | [an_overview](an_overview.md), [state-management](state-management.md), [overlays](overlays.md) |
| `cx.notify()` | [an_overview](an_overview.md), [state-management](state-management.md), [canvas](canvas.md) |
| `cx.observe()` | [state-management](state-management.md) |
| `cx.observe_new()` | [app](app.md), [state-management](state-management.md) |
| `cx.observe_release()` | [app](app.md), [state-management](state-management.md) |
| `cx.on_action()` | [an_overview](an_overview.md), [app](app.md), [menus](menus.md), [window](window.md) |
| `cx.on_open_urls()` | [app](app.md), [platform-utils](platform-utils.md) |
| `cx.open_window()` | [an_overview](an_overview.md), [app](app.md), [window](window.md) |
| `cx.open_with_system()` | [platform-utils](platform-utils.md) |
| `cx.primary_display()` | [app](app.md), [display-appearance](display-appearance.md) |
| `cx.prompt_for_new_path()` | [app](app.md), [dialogs](dialogs.md) |
| `cx.prompt_for_paths()` | [app](app.md), [dialogs](dialogs.md) |
| `cx.quit()` | [an_overview](an_overview.md), [app](app.md) |
| `cx.read_credentials()` | [app](app.md), [platform-utils](platform-utils.md) |
| `cx.read_from_clipboard()` | [app](app.md) |
| `cx.register_url_scheme()` | [platform-utils](platform-utils.md) |
| `cx.reveal_path()` | [app](app.md), [platform-utils](platform-utils.md) |
| `cx.run_until_parked()` | [async-tasks](async-tasks.md), [testing](testing.md) |
| `cx.screen_capture_sources()` | [screen-capture](screen-capture.md) |
| `cx.set_dock_menu()` | [platform-utils](platform-utils.md) |
| `cx.set_menus()` | [an_overview](an_overview.md), [app](app.md), [menus](menus.md) |
| `cx.spawn()` | [app](app.md), [async-tasks](async-tasks.md), [canvas](canvas.md) |
| `cx.subscribe()` | [state-management](state-management.md) |
| `cx.thermal_state()` | [platform-utils](platform-utils.md) |
| `cx.window_appearance()` | [app](app.md), [display-appearance](display-appearance.md) |
| `cx.write_credentials()` | [app](app.md), [platform-utils](platform-utils.md) |
| `cx.write_to_clipboard()` | [app](app.md) |
| `Deferred` | [overlays](overlays.md) |
| `deferred()` | [overlays](overlays.md) |
| `Display::Grid` | [css-grid](css-grid.md) |
| `div()` | [an_overview](an_overview.md), [window](window.md), [button](button.md), [text-input](text-input.md), [canvas](canvas.md), [scroll](scroll.md) |
| `DragMoveEvent<T>` | [drag-drop](drag-drop.md) |
| `ease_in_out()` | [animation](animation.md) |
| `ease_out_quint()` | [animation](animation.md) |
| `ElementInputHandler` | [text-input](text-input.md), [canvas](canvas.md) |
| `Entity<T>` | [an_overview](an_overview.md), [state-management](state-management.md), [window](window.md) |
| `entity.read()` | [state-management](state-management.md) |
| `entity.update()` | [state-management](state-management.md), [async-tasks](async-tasks.md), [testing](testing.md) |
| `EntityInputHandler` (trait) | [text-input](text-input.md), [canvas](canvas.md) |
| `EventEmitter<E>` (trait) | [app](app.md), [state-management](state-management.md) |
| `ExternalPaths` | [file-drop](file-drop.md) |
| `FileDropEvent` | [file-drop](file-drop.md) |
| `Focusable` (trait) | [an_overview](an_overview.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `FocusHandle` | [an_overview](an_overview.md), [window](window.md), [text-input](text-input.md), [testing](testing.md) |
| `Font` | [styled-text](styled-text.md), [canvas](canvas.md) |
| `FontStyle` | [styled-text](styled-text.md) |
| `FontWeight` | [styled-text](styled-text.md) |
| `ForegroundExecutor` | [async-tasks](async-tasks.md), [screen-capture](screen-capture.md) |
| `Global` (trait) | [an_overview](an_overview.md), [app](app.md), [menus](menus.md) |
| `GridPlacement` | [css-grid](css-grid.md) |
| `GridTemplate` | [css-grid](css-grid.md) |
| `HighlightStyle` | [styled-text](styled-text.md) |
| `Hitbox` | [hitbox](hitbox.md) |
| `Hitbox::is_hovered()` | [hitbox](hitbox.md) |
| `HitboxBehavior` | [hitbox](hitbox.md) |
| `HitboxId` | [hitbox](hitbox.md) |
| `Hsla` | [gradients](gradients.md), [shadows](shadows.md), [styled-text](styled-text.md) |
| `Hsla::fade_out()` | [animation](animation.md) |
| `Image` | [images](images.md) |
| `ImageCache` (trait) | [images](images.md) |
| `ImageFormat` | [images](images.md) |
| `ImageSource` | [images](images.md) |
| `Img` | [images](images.md) |
| `img()` | [images](images.md) |
| `InteractiveElement` (trait) | [button](button.md), [testing](testing.md), [scroll](scroll.md), [lists](lists.md), [svg](svg.md), [drag-drop](drag-drop.md), [overlays](overlays.md) |
| `InteractiveText` | [styled-text](styled-text.md) |
| `IntoElement` (trait) | [an_overview](an_overview.md), [text-input](text-input.md), [images](images.md), [svg](svg.md), [styled-text](styled-text.md), [animation](animation.md), [lists](lists.md) |
| `KeyBinding` | [app](app.md), [menus](menus.md) |
| `linear()` | [animation](animation.md) |
| `linear_color_stop()` | [gradients](gradients.md) |
| `linear_gradient()` | [gradients](gradients.md) |
| `LinearColorStop` | [gradients](gradients.md) |
| `LineWrapperHandle` | [typography](typography.md) |
| `list()` | [lists](lists.md) |
| `ListAlignment` | [lists](lists.md) |
| `ListOffset` | [lists](lists.md) |
| `ListState` | [lists](lists.md) |
| `Menu` | [menus](menus.md) |
| `MenuItem` | [menus](menus.md) |
| `Model<T>` | [an_overview](an_overview.md), [state-management](state-management.md) |
| `Modifiers` | [testing](testing.md), [gestures](gestures.md), [scroll](scroll.md) |
| `MouseDownEvent` | [button](button.md), [canvas](canvas.md) |
| `MousePressureEvent` | [gestures](gestures.md) |
| `ObjectFit` | [images](images.md) |
| `PathBuilder` | [canvas](canvas.md) |
| `PathPromptOptions` | [dialogs](dialogs.md) |
| `percentage()` | [svg](svg.md), [animation](animation.md), [transforms](transforms.md) |
| `PinchEvent` | [gestures](gestures.md) |
| `Pixels` | [canvas](canvas.md), [transforms](transforms.md) |
| `PlatformDisplay` (trait) | [app](app.md), [display-appearance](display-appearance.md) |
| `point()` | [canvas](canvas.md), [svg](svg.md), [transforms](transforms.md), [hitbox](hitbox.md), [shadows](shadows.md) |
| `PressureStage` | [gestures](gestures.md) |
| `PromptButton` | [dialogs](dialogs.md) |
| `PromptLevel` | [dialogs](dialogs.md) |
| `pulsating_between()` | [animation](animation.md) |
| `px()` | [canvas](canvas.md), [transforms](transforms.md), [hitbox](hitbox.md) |
| `quadratic()` | [animation](animation.md) |
| `QuitMode` | [app](app.md), [window](window.md) |
| `Radians` | [transforms](transforms.md) |
| `Render` (trait) | [an_overview](an_overview.md), [window](window.md), [text-input](text-input.md), [testing](testing.md), [drag-drop](drag-drop.md) |
| `RenderImage` | [images](images.md) |
| `Resource` | [images](images.md) |
| `RetainAllImageCache` | [images](images.md) |
| `ScaledPixels` | [transforms](transforms.md) |
| `ScreenCaptureFrame` | [screen-capture](screen-capture.md) |
| `ScreenCaptureSource` (trait) | [screen-capture](screen-capture.md) |
| `ScreenCaptureStream` (trait) | [screen-capture](screen-capture.md) |
| `ScrollDelta` | [scroll](scroll.md) |
| `ScrollHandle` | [scroll](scroll.md) |
| `ScrollStrategy` | [lists](lists.md) |
| `ScrollWheelEvent` | [scroll](scroll.md), [gestures](gestures.md) |
| `ShapedLine` | [text-input](text-input.md), [canvas](canvas.md) |
| `SharedString` | [window](window.md), [menus](menus.md), [canvas](canvas.md), [typography](typography.md) |
| `size()` | [window](window.md), [svg](svg.md), [transforms](transforms.md), [hitbox](hitbox.md) |
| `SourceMetadata` | [screen-capture](screen-capture.md) |
| `StatefulInteractiveElement` (trait) | [button](button.md), [scroll](scroll.md), [overlays](overlays.md), [drag-drop](drag-drop.md) |
| `Styled` (trait) | [canvas](canvas.md), [images](images.md), [svg](svg.md), [css-grid](css-grid.md), [scroll](scroll.md), [lists](lists.md), [gradients](gradients.md), [shadows](shadows.md), [typography](typography.md) |
| `StyledImage` (trait) | [images](images.md) |
| `StyledText` | [styled-text](styled-text.md) |
| `Subscription` | [app](app.md), [state-management](state-management.md) |
| `Surface` | [screen-capture](screen-capture.md) |
| `Svg` | [svg](svg.md) |
| `svg()` | [svg](svg.md), [animation](animation.md) |
| `Taffy` | [an_overview](an_overview.md), [css-grid](css-grid.md) |
| `Task<T>` | [app](app.md), [async-tasks](async-tasks.md) |
| `TemplateColumnMinSize` | [css-grid](css-grid.md) |
| `TestAppContext` | [an_overview](an_overview.md), [testing](testing.md), [state-management](state-management.md), [async-tasks](async-tasks.md) |
| `TextInputState` | [an_overview](an_overview.md), [text-input](text-input.md), [canvas](canvas.md) |
| `TextOverflow` | [styled-text](styled-text.md), [typography](typography.md) |
| `TextRun` | [text-input](text-input.md), [canvas](canvas.md), [styled-text](styled-text.md) |
| `TextStyle` | [styled-text](styled-text.md) |
| `ThermalState` | [platform-utils](platform-utils.md) |
| `TitlebarOptions` | [window](window.md) |
| `TouchPhase` | [scroll](scroll.md), [gestures](gestures.md) |
| `Transformation` | [svg](svg.md), [transforms](transforms.md), [animation](animation.md) |
| `TransformationMatrix` | [transforms](transforms.md) |
| `uniform_list()` | [lists](lists.md) |
| `UniformListScrollHandle` | [lists](lists.md) |
| `VisualTestContext` | [an_overview](an_overview.md), [window](window.md), [testing](testing.md) |
| `WeakEntity<T>` | [async-tasks](async-tasks.md) |
| `Window` | [an_overview](an_overview.md), [window](window.md) |
| `window.handle_input()` | [text-input](text-input.md), [canvas](canvas.md) |
| `window.insert_hitbox()` | [hitbox](hitbox.md) |
| `window.paint_path()` | [canvas](canvas.md) |
| `window.paint_quad()` | [canvas](canvas.md) |
| `window.paint_shadows()` | [shadows](shadows.md) |
| `window.prompt()` | [dialogs](dialogs.md) |
| `window.request_animation_frame()` | [animation](animation.md) |
| `window.text_style()` | [canvas](canvas.md), [styled-text](styled-text.md) |
| `window.text_system()` | [text-input](text-input.md), [canvas](canvas.md) |
| `WindowAppearance` | [app](app.md), [display-appearance](display-appearance.md) |
| `WindowBounds` | [window](window.md) |
| `WindowHandle<V>` | [window](window.md) |
| `WindowKind` | [window](window.md) |
| `WindowOptions` | [window](window.md) |
| `WrappedLine` | [canvas](canvas.md), [typography](typography.md) |
