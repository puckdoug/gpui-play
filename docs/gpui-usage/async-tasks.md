# Async & Tasks

**Components:** [`Task<T>`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/executor.rs), [`AsyncApp`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/async_context.rs), [`BackgroundExecutor`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/executor.rs), [`ForegroundExecutor`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/executor.rs)

## What is the component and what it does

GPUI has its own async runtime (not tokio). It provides two execution contexts:

- **Foreground (main thread)** — UI work, entity updates, rendering. Spawned via `cx.spawn()`.
- **Background (thread pool)** — CPU-bound work that shouldn't block the UI. Spawned via `async_cx.background_spawn()`.

`Task<T>` is the handle to a spawned future. It implements `Future`, so it can be awaited. Critically, **dropping a `Task` cancels the future** — this is the primary cancellation mechanism.

`cx.defer()` schedules a synchronous callback to run at the end of the current effect cycle, after all pending effects are flushed.

## Preconditions for use

```rust
use gpui::{App, Task};
```

- `cx.spawn()` is available on `App`, `Window`, `Context<T>`, `AsyncApp`, and `AsyncWindowContext`
- `background_spawn()` is available on `AsyncApp` and `AsyncWindowContext` (not directly on `App`)
- Background closures must be `Send + 'static` — no references to UI state
- `Task<T>` must be stored or `.detach()`'d — dropping it cancels the future
- `cx.defer()` is available on `App` and `Window`

## Signature for usage

### Foreground spawn (from App)

```rust
let task: Task<()> = cx.spawn(async move |async_cx: &mut AsyncApp| {
    // runs on main thread
    // async_cx provides access to app state across await points
});
```

### Foreground spawn (from Context<T>)

```rust
let task = cx.spawn(async move |entity: WeakEntity<T>, async_cx: &mut AsyncApp| {
    // entity is a weak reference to the owning view/model
});
```

### Background spawn

```rust
// From within a foreground task (via AsyncApp)
let result = async_cx.background_spawn(async move {
    // runs on background thread pool
    // must be Send — no &mut App, no entity access
    heavy_computation(42)
}).await;
```

### Task lifecycle

```rust
let task = cx.spawn(async move |async_cx| { /* ... */ });

// Option 1: store the task (cancel on drop)
self.active_task = Some(task);

// Option 2: detach (runs to completion, no way to cancel)
task.detach();

// Option 3: await (blocks the current async context)
let result = task.await;
```

### Defer

```rust
cx.defer(|cx: &mut App| {
    // runs at end of current effect cycle
    // NOT async — this is a synchronous callback
});
```

### Updating entities from async

```rust
cx.spawn(async move |async_cx| {
    // Do background work
    let result = async_cx.background_spawn(async { compute() }).await;

    // Update entity on the main thread
    async_cx.update(|cx| {
        entity.update(cx, |state, _cx| {
            state.complete(result);
        });
    });
});
```

## Relevant Macros

None specific to async.

## Relevant Traits

| Type | Purpose |
|------|---------|
| `Task<T>` | Future handle — cancels on drop, implements `Future` |
| `AsyncApp` | App context usable across await points in foreground tasks |
| `AsyncWindowContext` | Window context usable across await points |
| `BackgroundExecutor` | Background thread pool — also provides `timer()`, `advance_clock()` for tests |

## Usage and examples

### Foreground task that updates an entity

From `tests/async_tasks_test.rs`:

```rust
let demo = cx.update(|cx| cx.new(|_cx| AsyncDemo::new()));

cx.update(|cx: &mut gpui::App| {
    let demo = demo.clone();
    let task = cx.spawn(async move |async_cx| {
        async_cx.update(|cx| {
            demo.update(cx, |state, _cx| {
                state.start();
                state.complete("foreground done".to_string());
            });
        });
    });
    task.detach();
});
```

### Background computation with UI update

```rust
cx.update(|cx: &mut gpui::App| {
    let demo = demo.clone();
    let task = cx.spawn(async move |async_cx| {
        // Heavy work on background thread
        let result = async_cx.background_spawn(async move {
            heavy_computation(7)
        }).await;

        // Push result to UI on main thread
        async_cx.update(|cx| {
            demo.update(cx, |state, _cx| {
                state.complete(result);
            });
        });
    });
    task.detach();
});
```

### Task cancellation via drop

```rust
cx.update(|cx: &mut gpui::App| {
    let task = cx.spawn(async move |async_cx| {
        async_cx.background_spawn(async {}).await;
        // This line never executes — task was cancelled
        println!("completed");
    });
    drop(task); // Cancels the future immediately
});
```

### Defer for end-of-cycle work

```rust
cx.update(|cx| {
    cx.defer(|_cx| {
        println!("runs second");
    });
    println!("runs first");
});
// Output: "runs first", "runs second"
```

Multiple defers run in order:

```rust
cx.defer(|_cx| { println!("1"); });
cx.defer(|_cx| { println!("2"); });
cx.defer(|_cx| { println!("3"); });
// Output: "1", "2", "3"
```

## Post-conditions / destruction requirements

- **Task dropped = future cancelled.** This is the primary cancellation mechanism. If you forget to store a Task, the future silently never runs.
- **Task detached = runs to completion.** A detached task cannot be cancelled. It runs until done or the app exits.
- **`detach_and_log_err(cx)`** — convenience for tasks returning `Result`. Detaches and logs errors.
- **No cleanup needed** for defer callbacks — they run once and are discarded.

## Testing

Use `cx.run_until_parked()` to advance async tasks in tests:

```rust
#[gpui::test]
fn test_async_work(cx: &mut TestAppContext) {
    let demo = cx.update(|cx| cx.new(|_cx| AsyncDemo::new()));

    cx.update(|cx: &mut gpui::App| {
        let demo = demo.clone();
        cx.spawn(async move |async_cx| {
            async_cx.update(|cx| {
                demo.update(cx, |s, _| s.complete("done".into()));
            });
        }).detach();
    });

    cx.run_until_parked(); // Flushes all pending async work

    cx.update(|cx| {
        assert_eq!(demo.read(cx).result(), Some("done"));
    });
}
```

The `BackgroundExecutor` also provides test helpers:
- `executor.run_until_parked()` — drain all runnable tasks
- `executor.advance_clock(duration)` — advance fake time for timer-based tests
- `executor.timer(duration)` — create a timer task (respects fake time in tests)

Run tests: `cargo test --test async_tasks_test`

## Surprises, Anti-patterns, and Bugs

### Forgetting to store a Task silently cancels it

This is the #1 gotcha. The future never runs:

```rust
// WRONG — task is immediately dropped and cancelled
cx.spawn(async move |async_cx| { /* never executes */ });

// RIGHT — store it or detach it
let _task = cx.spawn(async move |async_cx| { /* runs */ });
// or
cx.spawn(async move |async_cx| { /* runs */ }).detach();
```

### Background tasks cannot access `cx` directly

`background_spawn` requires `Send + 'static`. You cannot capture `&mut App` or entity references. Do the work, return the result, then update the entity in the foreground:

```rust
// WRONG — won't compile, entity is not Send
async_cx.background_spawn(async move {
    entity.update(cx, |s, _| s.do_thing()); // ERROR
});

// RIGHT — compute in background, update in foreground
let result = async_cx.background_spawn(async move { compute() }).await;
async_cx.update(|cx| entity.update(cx, |s, _| s.set_result(result)));
```

### `cx.spawn()` closure signatures differ by context

| Context | Closure receives |
|---------|-----------------|
| `App` | `async \|async_cx: &mut AsyncApp\|` |
| `Context<T>` | `async \|weak_entity: WeakEntity<T>, async_cx: &mut AsyncApp\|` |
| `Window` | `async \|async_cx: &mut AsyncWindowContext\|` |

### `cx.defer()` is NOT async

Defer runs a synchronous closure at the end of the current effect cycle. It does not spawn a future. Use `cx.spawn()` for async work.

### GPUI uses its own executor, not tokio

You cannot use `tokio::spawn()` or `#[tokio::main]` with GPUI. The framework manages its own foreground and background executors. In tests, these are deterministic (same seed = same scheduling order).

### `run_until_parked()` in tests

After spawning tasks in tests, you must call `cx.run_until_parked()` to advance the executor. Without this, foreground tasks may not complete before your assertions run. Note that `cx.update()` does flush synchronous effects, but async tasks require explicit parking.
