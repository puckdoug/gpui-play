# Testing Utilities

## Goal

Document GPUI's test harness: `VisualTestContext`, `cx.simulate_keystrokes()`, `cx.simulate_click()`, property testing with `iterations=N`, and multi-context testing.

## Design

No standalone binary — this is a test-only feature. Create a focused test file that exercises every testing utility against a simple interactive view (a counter with keyboard shortcuts and clickable buttons).

### Test Subject View

```rust
struct TestableView {
    count: i32,
    clicked: bool,
    last_key: Option<String>,
}
```

Actions: `Increment`, `Decrement`, `Reset`
Key bindings: `up` → Increment, `down` → Decrement, `cmd-r` → Reset

## Test File (tests/testing_test.rs)

### VisualTestContext (4)
1. Create window with VisualTestContext, verify view renders
2. Dispatch action via `cx.dispatch_action()`, verify state change
3. Access view state via `view.update(cx, |v, _| v.count)`
4. Multiple windows in same test context

### Simulate keystrokes (4)
5. `cx.simulate_keystrokes("up")` increments count
6. `cx.simulate_keystrokes("down down down")` decrements three times
7. `cx.simulate_keystrokes("cmd-r")` resets
8. Keystroke sequence with modifier keys

### Simulate click (3)
9. `cx.simulate_click(point, modifiers)` triggers on_click handler
10. Click at specific coordinates hits correct element
11. Click with modifier keys (shift-click, cmd-click)

### Property testing (2)
12. `#[gpui::test(iterations=100)]` — count never goes below 0 with random operations
13. Property test with seeded randomness for reproducibility

### Multi-context (2)
14. Two `TestAppContext` params simulate two separate app instances
15. Verify isolation between contexts

## Documentation (docs/gpui-usage/testing.md)

### Sections
1. **What it is** — GPUI's built-in test harness for headless UI testing
2. **Preconditions** — `gpui = { features = ["test-support"] }` in dev-dependencies; `#[gpui::test]` macro on test functions; function takes `&mut TestAppContext` param
3. **Signatures** — `#[gpui::test]`, `TestAppContext`, `VisualTestContext`, `cx.simulate_keystrokes()`, `cx.simulate_click()`, `cx.update()`, `cx.run_until_parked()`
4. **Relevant macros** — `#[gpui::test]`, `#[gpui::test(iterations=N)]`
5. **Usage examples** — basic test, action dispatch, keystroke simulation, click simulation, property test
6. **Post-conditions** — test contexts clean up automatically; windows closed on drop
7. **Testing** — (meta: this IS the testing doc)
8. **Surprises** — `simulate_keystrokes` requires focus on the correct element; clicks use window coordinates not screen coordinates; `run_until_parked()` needed to flush async work; iterations use different random seeds but same test logic; VisualTestContext vs TestAppContext distinction is subtle
