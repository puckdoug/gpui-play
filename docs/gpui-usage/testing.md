# Testing Utilities

**Components:** [`TestAppContext`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/test_context.rs), [`VisualTestContext`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/test_context.rs), [`#[gpui::test]`](https://github.com/zed-industries/zed/blob/main/crates/gpui_macros/src/test.rs)

## What is the component and what it does

GPUI provides a headless test harness that runs the full UI stack (layout, painting, event dispatch) without opening real windows. This enables testing views, actions, keyboard input, and mouse clicks in fast, deterministic unit tests.

- `TestAppContext` — the primary test context. Wraps an `App` with test helpers for input simulation, window management, and effect flushing.
- `VisualTestContext` — a window-scoped test context. Provides `simulate_keystrokes()`, `simulate_click()`, and direct window access.
- `#[gpui::test]` — proc macro that sets up the test runtime with deterministic scheduling.

## Preconditions for use

**Cargo.toml** — enable the `test-support` feature in dev-dependencies:

```toml
[dev-dependencies]
gpui = { git = "https://github.com/zed-industries/zed", features = ["test-support"] }
```

**Imports:**

```rust
use gpui::{TestAppContext, VisualTestContext};
```

**Test function signature:**

```rust
#[gpui::test]
fn test_name(cx: &mut TestAppContext) {
    // ...
}
```

## Signature for usage

### `#[gpui::test]` macro

```rust
#[gpui::test]                              // Default: 1 iteration, seed 0
#[gpui::test(iterations = 100)]            // Run 100 times with random seeds
#[gpui::test(seed = 42)]                   // Run with specific seed
#[gpui::test(retries = 3)]                 // Retry failures up to 3 times
```

### Function parameter injection

```rust
#[gpui::test]
fn test(cx: &mut TestAppContext) { }                              // Single context

#[gpui::test]
fn test(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) { } // Multi-client

#[gpui::test]
fn test(executor: BackgroundExecutor, cx: &mut TestAppContext) { } // With executor
```

### TestAppContext key methods

```rust
cx.update(|cx: &mut App| { ... })            // Run closure with mutable App access
cx.read(|cx: &App| { ... })                  // Run closure with immutable App access
cx.run_until_parked()                         // Flush all pending async tasks
cx.add_window_view(|window, cx| View::new(cx)) // Create window, returns (Entity<V>, &mut VisualTestContext)
```

### VisualTestContext key methods

```rust
cx.simulate_keystrokes("cmd-shift-p escape")  // Space-separated keystrokes
cx.simulate_click(point, modifiers)            // Left mouse click at position
cx.dispatch_action(MyAction)                   // Dispatch action to focused element
cx.simulate_input("hello")                     // Type raw text characters
```

### Entity access in tests

```rust
// Read view state
view.update(cx, |v, _cx| {
    assert_eq!(v.count, 42);
});

// Mutate view state
view.update(cx, |v, _cx| {
    v.count = 0;
});
```

## Relevant Macros

| Macro | Purpose |
|-------|---------|
| `#[gpui::test]` | Test function setup with deterministic scheduling |
| `#[gpui::test(iterations = N)]` | Property testing with N random seeds |
| `actions!(module, [Action1, Action2])` | Define action types for key binding tests |

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `Render` | View must implement `Render` to be used in `add_window_view` |
| `InteractiveElement` | Required for `on_action`, `on_mouse_down` on elements |

## Usage and examples

### Creating a testable view

The view needs a `FocusHandle` and `track_focus()` on its root element for action dispatch to work:

```rust
use gpui::{
    actions, div, Context, FocusHandle, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window,
};

actions!(testing, [Increment, Decrement, Reset]);

struct TestableView {
    count: i32,
    focus_handle: FocusHandle,
}

impl TestableView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            count: 0,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for TestableView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Focus the handle so actions dispatch to this view
        self.focus_handle.focus(window, cx);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .on_action(cx.listener(|this, _action: &Increment, _window, cx| {
                this.count += 1;
                cx.notify();
            }))
            .child(format!("Count: {}", self.count))
    }
}
```

### Testing action dispatch

```rust
#[gpui::test]
fn test_dispatch_action(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.dispatch_action(Increment);

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 1);
    });
}
```

### Testing keystroke simulation

```rust
#[gpui::test]
fn test_keystrokes(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("cmd-r", Reset, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_keystrokes("up up up");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 3);
    });

    cx.simulate_keystrokes("cmd-r");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 0);
    });
}
```

### Testing mouse clicks

```rust
use gpui::{point, px, Modifiers};

#[gpui::test]
fn test_click(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_click(point(px(50.0), px(50.0)), Modifiers::default());

    view.update(cx, |v, _cx| {
        assert!(v.clicked);
    });
}
```

### Property testing

```rust
#[gpui::test(iterations = 10)]
fn test_deterministic(cx: &mut TestAppContext) {
    // Runs 10 times with different random seeds
    // Use for testing invariants that should hold regardless of scheduling
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_keystrokes("up up up down");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 2); // Always true regardless of seed
    });
}
```

### Multiple app contexts (distributed testing)

The `#[gpui::test]` macro supports multiple `TestAppContext` parameters for simulating multi-client scenarios (e.g., collaborative editing):

```rust
#[gpui::test]
async fn test_collaboration(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // cx_a and cx_b are independent app contexts
    // They share the same TestDispatcher for deterministic task interleaving

    cx_a.update(|cx| {
        // Client A operations
    });

    cx_b.update(|cx| {
        // Client B operations
    });
}
```

Each context gets its own `App` instance but shares the same dispatcher, ensuring deterministic scheduling across both. This is how Zed tests real-time collaboration features.

## Post-conditions / destruction requirements

- Test contexts clean up automatically when the test function returns
- Windows created with `add_window_view` are closed on drop
- No explicit teardown needed
- Entity handles created in tests are released when dropped (same as production)

## Testing

This **is** the testing documentation. Run the test suite:

```
cargo test --test testing_test
```

## Surprises, Anti-patterns, and Bugs

### Actions require focus

Actions dispatch to the **focused element** and bubble up. If no element in your view has focus, `dispatch_action` and `simulate_keystrokes` silently do nothing. Your view must:

1. Create a `FocusHandle` via `cx.focus_handle()` in the constructor
2. Call `.track_focus(&self.focus_handle)` on the root div
3. Focus the handle: `self.focus_handle.focus(window, cx)` in `render()`

### `simulate_keystrokes` requires key bindings

Keystrokes are matched against registered key bindings to produce actions. If no binding matches, nothing happens. Register bindings before creating the window:

```rust
cx.update(|cx| {
    cx.bind_keys([
        gpui::KeyBinding::new("up", Increment, None),
    ]);
});
```

### `view.update()` takes a 2-argument closure, not 3

The closure signature is `|&mut V, &mut Context<V>|`. There is no `Window` parameter. If you need window access, use `view.update_in(cx, |v, window, cx| { ... })`.

### `cx.listener()` is essential for element event handlers

Element methods like `on_action` and `on_mouse_down` take closures of type `Fn(&Event, &mut Window, &mut App)` — no view self-reference. Use `cx.listener()` to create a closure that captures the view entity:

```rust
// WRONG — no access to `self`
div().on_action(|_action: &Increment, _window, _cx| {
    // can't modify view state here
})

// RIGHT — cx.listener wraps the closure with entity access
div().on_action(cx.listener(|this, _action: &Increment, _window, cx| {
    this.count += 1;
    cx.notify();
}))
```

### Click coordinates are window-relative

`simulate_click(point, modifiers)` uses window coordinates, not screen coordinates. `point(px(0.0), px(0.0))` is the top-left corner of the window content area.

### `simulate_keystrokes` auto-parks

`simulate_keystrokes`, `simulate_click`, and `dispatch_action` all call `run_until_parked()` internally. You don't need to call it after these methods. You DO need it after `cx.spawn()`.

### `iterations` uses different random seeds

`#[gpui::test(iterations = 100)]` runs the test 100 times, each with a different random seed controlling task scheduling order. When a test fails, it prints the failing seed so you can reproduce with `SEED=N cargo test`.

### `add_window_view` rebinds `cx`

The pattern `let (view, cx) = cx.add_window_view(...)` shadows the outer `cx` (TestAppContext) with a `&mut VisualTestContext`. This is intentional — the returned `cx` is window-scoped and provides `simulate_keystrokes` etc.
