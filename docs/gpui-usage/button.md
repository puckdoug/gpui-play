# Button

**Components:** [`ClickEvent`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/interactive.rs#L135), [`StatefulInteractiveElement`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/div.rs#L1250), [`CursorStyle`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/scene.rs#L510)

## What is the component and what it does

GPUI has **no built-in button widget**. Buttons are created from `div()` elements with click handlers, cursor styling, and hover/active state styling. Any `div` with an `.id()` becomes a `StatefulInteractiveElement` that can receive `.on_click()`, `.hover()`, and `.active()` calls.

This is a pattern, not a component — you compose buttons from the same primitives used for any other UI element.

## Signature for usage

### Making a clickable element

```rust
div()
    .id("my-button")          // REQUIRED — makes it a StatefulInteractiveElement
    .on_click(listener)        // click handler
    .cursor_pointer()          // show pointer cursor on hover
    .child("Button text")
```

### Click handler signatures

```rust
// Standalone function
.on_click(|event: &ClickEvent, window: &mut Window, cx: &mut App| {
    // handle click
})

// With view access via cx.listener()
.on_click(cx.listener(|this: &mut MyView, event: &ClickEvent, window: &mut Window, cx: &mut Context<MyView>| {
    // access view state via `this`
}))
```

### Hover and active states

```rust
div()
    .id("button")
    .bg(rgb(0x4488ff))
    .hover(|style| style.bg(rgb(0x3377ee)))    // lighter/darker on hover
    .active(|style| style.bg(rgb(0x2266dd)))   // pressed state
```

### CursorStyle options

```rust
.cursor_pointer()       // hand cursor (buttons)
.cursor(CursorStyle::IBeam)       // text cursor (inputs)
.cursor(CursorStyle::Arrow)       // default arrow
.cursor(CursorStyle::ResizeLeftRight)  // resize handles
```

## Relevant Macros

None specific to buttons. Use `actions!()` if the button triggers an action rather than inline logic.

## Relevant Traits

### `StatefulInteractiveElement`

A `div()` with `.id()` becomes a `Div` that implements `StatefulInteractiveElement`, enabling `.on_click()`, `.on_drag()`, `.on_drop()`, and other stateful interactions. Without `.id()`, the element is stateless and cannot receive click events.

### `InteractiveElement`

Both stateful and stateless elements implement `InteractiveElement`, which provides `.hover()`, `.active()`, `.on_mouse_down()`, `.on_mouse_up()`, `.on_mouse_move()`, and `.cursor()`.

## Usage and examples

### Simple Ok button that closes the window

```rust
div()
    .id("ok-button")
    .px_4()
    .py_1()
    .bg(rgb(0x4488ff))
    .text_color(gpui::white())
    .text_sm()
    .rounded_md()
    .cursor_pointer()
    .hover(|s| s.bg(rgb(0x3377ee)))
    .active(|s| s.bg(rgb(0x2266dd)))
    .child("Ok")
    .on_click(cx.listener(|this, _, window, cx| {
        this.close_window(&CloseWindow, window, cx);
    }))
```

### Button that dispatches an action

Rather than inline logic, buttons can dispatch actions that are handled elsewhere:

```rust
div()
    .id("new-window-btn")
    .px_4()
    .py_2()
    .bg(rgb(0x44aa44))
    .text_color(gpui::white())
    .rounded_md()
    .cursor_pointer()
    .hover(|s| s.bg(rgb(0x339933)))
    .child("New Window")
    .on_click(cx.listener(|_this, _, _window, cx| {
        cx.dispatch_action(Box::new(NewWindow));
    }))
```

### Right-aligned button row

```rust
div()
    .flex()
    .justify_end()       // push children to the right
    .child(
        div()
            .id("ok-button")
            .px_4()
            .py_1()
            .bg(rgb(0x4488ff))
            .text_color(gpui::white())
            .rounded_md()
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x3377ee)))
            .child("Ok")
            .on_click(cx.listener(|this, _, window, cx| {
                window.remove_window();
            })),
    )
```

### Button with disabled state

```rust
let enabled = self.can_submit;
div()
    .id("submit")
    .px_4()
    .py_1()
    .rounded_md()
    .when(enabled, |d| {
        d.bg(rgb(0x4488ff))
            .text_color(gpui::white())
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x3377ee)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.submit(cx);
            }))
    })
    .when(!enabled, |d| {
        d.bg(rgb(0xcccccc))
            .text_color(rgb(0x888888))
    })
    .child("Submit")
```

## Surprises, Anti-patterns, and Bugs

### `.id()` is required for click events

A `div()` without `.id()` is a stateless element. Calling `.on_click()` on it will not compile — the method only exists on `StatefulInteractiveElement`. Always add `.id("unique-name")` to clickable elements.

### No built-in button styling

There is no `.button()` helper or default button appearance. You must style every button manually with background color, text color, padding, border radius, hover state, and active state. Consider creating a helper function for consistent button styling across your app.

### Hover and active are style transformers, not callbacks

`.hover(|style| ...)` and `.active(|style| ...)` take a closure that receives the current style and returns a modified style. They are **not** event callbacks — they declare what the element should look like in that state. The framework handles state transitions automatically.

### Click handler receives `ClickEvent`, not mouse position

`on_click` provides a `ClickEvent` which includes the click count (for double-click detection) but the primary use is just detecting that a click occurred. For position-sensitive interaction, use `.on_mouse_down()` / `.on_mouse_up()` which provide `MouseDownEvent` / `MouseUpEvent` with a `position` field.

### Text inside buttons needs explicit sizing

Like all GPUI text, button labels need a text size set on the element or an ancestor. Use `.text_sm()`, `.text_base()`, `.text_lg()`, etc., or the text will inherit from the parent (which may have no size set).
