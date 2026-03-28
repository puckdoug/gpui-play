# State Management (Entity, EventEmitter, Subscriptions)

**Components:** [`Entity<T>`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/entity_map.rs), [`EventEmitter<E>`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/gpui.rs#L242), [`Subscription`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/subscription.rs), [`Context<T>`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/context.rs)

## What is the component and what it does

GPUI's state management is built on **entities** — observable, reference-counted state containers. An `Entity<T>` is a handle to a value of type `T` stored in the app's entity map. Multiple handles can point to the same entity, and when the last handle is dropped, the entity is released.

The reactive system has two mechanisms:

- **`cx.observe()`** — watch for *any* change to an entity (triggered by `cx.notify()`)
- **`cx.subscribe()`** — listen for *specific typed events* emitted by an entity (triggered by `cx.emit()`)

Both return a `Subscription` handle. Dropping the subscription stops the callback from firing — this is the primary lifecycle control.

Note: GPUI documentation and some older code refer to `Model<T>`. In current GPUI, the type is `Entity<T>`. They are the same concept.

## Preconditions for use

```rust
use gpui::{AppContext, EventEmitter, Entity, Subscription};
```

- Entities must be created via `cx.new()` which requires the `AppContext` trait in scope
- `cx.new()` is available on `App`, `Context<T>`, and `TestAppContext`
- The `EventEmitter<E>` trait must be implemented on your type to use `cx.emit()` and `cx.subscribe()`
- `cx.notify()` and `cx.emit()` are only available on `Context<T>` (inside `entity.update()`)

## Signature for usage

### Creating an entity

```rust
// cx.new() requires AppContext trait in scope
use gpui::AppContext;

let counter: Entity<CounterState> = cx.new(|_cx: &mut Context<CounterState>| {
    CounterState::new(10)
});
```

### Reading an entity

```rust
let count = counter.read(cx).count();
```

### Updating an entity

```rust
counter.update(cx, |state, cx| {
    state.increment(cx);  // cx here is &mut Context<CounterState>
});
```

### EventEmitter trait

```rust
#[derive(Clone, Debug)]
pub enum CounterEvent {
    ThresholdReached(i32),
}

impl EventEmitter<CounterEvent> for CounterState {}
```

`EventEmitter` is a marker trait — no methods to implement.

### Emitting events (inside Context<T>)

```rust
cx.emit(CounterEvent::ThresholdReached(self.count));
```

### Notifying observers (inside Context<T>)

```rust
cx.notify();
```

### Observing changes

```rust
// From App context — callback gets (Entity<T>, &mut App)
let sub: Subscription = cx.observe(&counter, |_entity, _cx| {
    println!("counter changed!");
});

// From Context<V> (inside a view) — callback gets (&mut V, Entity<T>, &mut Context<V>)
let sub = cx.observe(&counter, |this_view, _entity, cx| {
    cx.notify(); // re-render this view
});
```

### Subscribing to events

```rust
// From App context
let sub: Subscription = cx.subscribe(&counter, |_entity, event: &CounterEvent, _cx| {
    match event {
        CounterEvent::ThresholdReached(v) => println!("threshold: {}", v),
    }
});

// From Context<V> (inside a view)
let sub = cx.subscribe(&counter, |this_view, _entity, event: &CounterEvent, cx| {
    // handle event with access to the subscribing view
});
```

### Observing entity release

```rust
let sub = cx.observe_release(&counter, |_released_state, _cx| {
    println!("entity was dropped");
});
```

### Observing new entity creation

```rust
// Watch for creation of any new entity of type T
let sub = cx.observe_new::<MyView>(|new_view, window, cx| {
    // new_view: &mut MyView
    // window: Option<&mut Window> — None if created outside a window
    // cx: &mut Context<MyView>
    println!("A new MyView was created");
});
```

This fires every time `cx.new(|cx| MyView { ... })` is called for the observed type. Useful for plugins or systems that need to hook into all instances of a type. The `window` parameter is `None` when the entity is created outside a window context.

## Relevant Macros

None specific to state management. The `actions!` macro is often used alongside for action-driven state changes.

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `EventEmitter<E>` | Marker trait enabling `cx.emit()` and `cx.subscribe()` for event type `E` |
| `AppContext` | Provides `cx.new()`, `cx.observe()`, `cx.subscribe()` — must be in scope |

## Usage and examples

### Complete example: observable counter with events

From `src/state_management.rs`:

```rust
use gpui::EventEmitter;

#[derive(Clone, Debug)]
pub enum CounterEvent {
    ThresholdReached(i32),
}

pub struct CounterState {
    count: i32,
    threshold: i32,
}

impl EventEmitter<CounterEvent> for CounterState {}

impl CounterState {
    pub fn new(threshold: i32) -> Self {
        Self { count: 0, threshold }
    }

    pub fn increment(&mut self, cx: &mut gpui::Context<Self>) {
        let was_below = self.count < self.threshold;
        self.count += 1;
        if was_below && self.count >= self.threshold {
            cx.emit(CounterEvent::ThresholdReached(self.count));
        }
        cx.notify();
    }
}
```

### Wiring up observers and subscribers

```rust
// Create the entity
let counter = cx.new(|_cx| CounterState::new(3));

// Observe any change (fires on cx.notify())
let _obs = cx.observe(&counter, |_entity, _cx| {
    println!("counter value changed");
});

// Subscribe to specific events (fires on cx.emit())
let _sub = cx.subscribe(&counter, |_entity, event: &CounterEvent, _cx| {
    match event {
        CounterEvent::ThresholdReached(v) => println!("reached {}", v),
    }
});

// Mutate — triggers both observer and subscriber (when threshold crossed)
counter.update(cx, |state, cx| state.increment(cx));
```

### Subscription lifetime control

```rust
let sub = cx.observe(&counter, |_entity, _cx| { /* ... */ });

// Subscription is active — callback fires on changes
counter.update(cx, |state, cx| state.increment(cx));

// Drop the subscription — callback stops firing
drop(sub);

// This change will NOT trigger the callback
counter.update(cx, |state, cx| state.increment(cx));
```

### Detaching subscriptions

If you want a subscription to live as long as the entity (not tied to a local variable):

```rust
cx.observe(&counter, |_entity, _cx| { /* ... */ }).detach();
```

**Warning:** A detached subscription cannot be cancelled. It lives until the observed entity is released.

## Post-conditions / destruction requirements

- **Subscription drop = unsubscribe.** The `Subscription` type is `#[must_use]` — if you don't store it, Rust warns you. Dropping it immediately stops callbacks.
- **Entity release.** When the last `Entity<T>` handle is dropped, the entity is released from the app's entity map. Any `observe_release` callbacks fire at this point.
- **No explicit cleanup needed** for observers or subscribers beyond managing the `Subscription` lifetime.
- **Detached subscriptions** live until the entity they observe is released.

## Testing

Use `#[gpui::test]` with `TestAppContext`. Key patterns:

```rust
#[gpui::test]
fn test_observe(cx: &mut TestAppContext) {
    // Create entity in one cx.update()
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    // Set up observer in another
    let notified = Arc::new(AtomicBool::new(false));
    let notified_clone = notified.clone();
    cx.update(|cx| {
        cx.observe(&counter, move |_entity, _cx| {
            notified_clone.store(true, Ordering::SeqCst);
        }).detach();
    });

    // Mutate in another — effects flush when cx.update() returns
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));

    // Now the observer has fired
    assert!(notified.load(Ordering::SeqCst));
}
```

Run tests: `cargo test --test state_management_test`

## Surprises, Anti-patterns, and Bugs

### Effects are deferred, not immediate

`cx.notify()` and `cx.emit()` queue effects. They fire when the **outermost** `cx.update()` completes — not during nested updates. In tests, this means you must split entity creation, subscription setup, and mutation into **separate `cx.update()` calls** to observe the results.

```rust
// WRONG — observer won't have fired yet inside this closure
cx.update(|cx| {
    let counter = cx.new(|_cx| CounterState::new(10));
    let _sub = cx.observe(&counter, |_, _| { /* ... */ });
    counter.update(cx, |s, cx| s.increment(cx));
    // observer has NOT fired here — we're still in the outermost update
});

// RIGHT — separate updates
let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));
cx.update(|cx| { cx.observe(&counter, |_, _| { /* ... */ }).detach(); });
cx.update(|cx| counter.update(cx, |s, cx| s.increment(cx)));
// observer has fired after the last cx.update() returned
```

### Subscription must be stored

`Subscription` is `#[must_use]`. If you call `cx.observe()` without storing the return value, the subscription is immediately dropped and the callback never fires. Either store it in a variable or call `.detach()`.

### `cx.observe()` fires on ANY notify, not field-specific changes

There is no way to observe a specific field. `cx.notify()` triggers ALL observers of that entity, regardless of what changed.

### `cx.emit()` only works inside `entity.update()` closures

You cannot emit events from outside the entity's update context. The `cx` in `cx.emit()` must be a `Context<T>` for the entity type.

### `observe_release` requires dropping inside `cx.update()`

In tests, dropping an entity outside of `cx.update()` does not flush effects. Wrap the drop in an update call:

```rust
cx.update(|_cx| { drop(counter); });
```

### Entity vs Model naming

Older GPUI docs and code may reference `Model<T>`. In current GPUI, the type is `Entity<T>`. They are functionally identical.
