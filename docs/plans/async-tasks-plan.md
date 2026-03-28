# Async & Tasks

## Goal

Document and demonstrate GPUI's async primitives: `Task<T>`, `cx.spawn()`, `cx.background_spawn()`, `ForegroundExecutor`, `BackgroundExecutor`, and `cx.defer()`.

## Design

A view that simulates a long-running computation. A "Compute" button spawns a background task that does work, then updates the UI on the foreground. A progress indicator shows status. Cancellation is demonstrated by navigating away or clicking "Cancel" (which drops the Task handle).

### Data Model

```rust
struct AsyncDemo {
    result: Option<String>,
    status: TaskStatus,
    active_task: Option<Task<()>>,
}

enum TaskStatus {
    Idle,
    Running,
    Complete,
    Cancelled,
}
```

### Key Patterns

- `cx.spawn(|this, mut cx| async move { ... })` — foreground task with view access
- `cx.background_spawn(async move { ... })` — CPU work off main thread
- Combining both: background_spawn for computation, then `cx.update()` to push results to UI
- `cx.defer(|cx| { ... })` — schedule work for end of current effect cycle
- Task cancellation via drop — dropping `Task<T>` cancels the future

## State Layer

- `AsyncDemo` struct with status tracking
- Pure state transitions: Idle → Running → Complete/Cancelled

## View Layer (src/bin/async_test.rs)

- "Compute" button starts background task
- "Cancel" button sets `active_task = None` (drops Task, cancels)
- Status label shows current TaskStatus
- Result display area
- "Defer Demo" button that queues multiple deferred callbacks to show batching

## TDD Tests

### Task lifecycle (6)
1. Spawned foreground task runs to completion
2. Spawned background task runs to completion
3. Background result can update model via cx.update()
4. Dropping Task cancels the future (result never arrives)
5. cx.defer() callback runs after current effect cycle
6. Multiple deferred callbacks run in order

### Integration (3)
7. Clicking "Compute" transitions status to Running then Complete
8. Clicking "Cancel" during Running transitions to Cancelled
9. Starting new computation while one is running cancels the old one

## Documentation (docs/gpui-usage/async-tasks.md)

### Sections
1. **What it is** — async task system built on GPUI's executor (not tokio)
2. **Preconditions** — must have access to `cx` (App, Window, or view Context); `Task<T>` must be stored to keep it alive
3. **Signatures** — `cx.spawn()`, `cx.background_spawn()`, `cx.defer()`, `Task::detach()`, `task.await`
4. **Relevant traits** — none specific; closures must be `Send` for background tasks
5. **Usage examples** — foreground spawn, background spawn, combined pattern, defer
6. **Post-conditions / destruction** — Task cancelled on drop unless `.detach()`'d; detached tasks run to completion; no way to cancel a detached task
7. **Testing** — `cx.run_until_parked()` advances async tasks in tests; `cx.executor().run_until_parked()`
8. **Surprises** — forgetting to store Task silently cancels it; background tasks cannot access cx directly; cx.spawn closure params differ between App/Window/View contexts; defer is NOT async
