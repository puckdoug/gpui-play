# State Management (Model, Events, Subscriptions)

## Goal

Document and demonstrate GPUI's reactive state primitives: `Model<T>`, `EventEmitter<E>`, `cx.subscribe()`, `cx.observe()`, `cx.observe_new()`, `cx.observe_release()`, and `Subscription` lifetime.

## Design

A two-panel window where both panels share a `Model<CounterState>`. The left panel has increment/decrement buttons. The right panel observes the model and displays the current count. An `EventEmitter` fires a `ThresholdReached` event when the count crosses a configurable boundary. A status bar subscribes to that event and shows a flash message.

### Data Model

```rust
struct CounterState {
    count: i32,
    threshold: i32,
}

enum CounterEvent {
    ThresholdReached(i32),
}

impl EventEmitter<CounterEvent> for CounterState {}
```

### Views

- `ControlPanel` — holds `Model<CounterState>`, has +/- buttons, calls `cx.update_model()` and `cx.emit()`
- `DisplayPanel` — holds `Model<CounterState>`, uses `cx.observe()` to re-render on change
- `StatusBar` — uses `cx.subscribe()` to listen for `CounterEvent::ThresholdReached`
- `RootView` — composes all three, demonstrates `cx.observe_new()` if a second display panel is created dynamically

### Subscription Lifetime

- Store `Subscription` in view struct fields
- Demonstrate that dropping a subscription stops notifications
- Show `cx.observe_release()` logging when a view is dropped

## State Layer (src/state_test.rs or inline in binary)

- `CounterState` with increment/decrement methods
- Threshold check emits event
- Pure state logic testable without GPUI views

## View Layer (src/bin/state_test.rs)

- Window with flex row: ControlPanel | DisplayPanel
- StatusBar at bottom
- Button to add/remove a second DisplayPanel (demonstrates observe_new/observe_release)

## TDD Tests

### Pure state (4)
1. Increment increases count
2. Decrement decreases count
3. Threshold event fires when count crosses threshold
4. Threshold event does not fire when count stays below

### Reactive subscriptions (6)
5. `cx.observe()` callback fires on model update
6. `cx.observe()` callback does NOT fire after subscription dropped
7. `cx.subscribe()` receives emitted events
8. `cx.subscribe()` stops after subscription dropped
9. `cx.observe_release()` fires when entity is released
10. Multiple observers all receive notifications

## Documentation (docs/gpui-usage/state-management.md)

### Sections
1. **What it is** — GPUI's reactive state primitives for sharing state across views
2. **Preconditions** — `use gpui::{Model, EventEmitter, Subscription}`, entity must be created via `cx.new()`
3. **Signatures** — `cx.new()`, `model.update()`, `model.read()`, `cx.observe()`, `cx.subscribe()`, `cx.emit()`, `cx.observe_new()`, `cx.observe_release()`
4. **Relevant traits** — `EventEmitter<E>`, `Entity`
5. **Usage examples** — from our state_test binary
6. **Post-conditions / destruction** — Subscription drop stops callbacks; Model dropped when all references released; observe_release for cleanup
7. **Testing** — how to test with `#[gpui::test]` and `TestAppContext`
8. **Surprises** — subscription must be stored (not just created); observe fires on ANY notify, not specific field changes; emit only works inside update closure
