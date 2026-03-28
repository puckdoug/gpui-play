use gpui::{AppContext, TestAppContext};
use gpui_play::async_tasks::{heavy_computation, AsyncDemo, TaskStatus};

// -- Pure state tests --

#[test]
fn test_async_demo_initial_state() {
    let demo = AsyncDemo::new();
    assert_eq!(demo.status(), &TaskStatus::Idle);
    assert!(demo.result().is_none());
}

#[test]
fn test_start_sets_running() {
    let mut demo = AsyncDemo::new();
    demo.start();
    assert_eq!(demo.status(), &TaskStatus::Running);
}

#[test]
fn test_complete_sets_result() {
    let mut demo = AsyncDemo::new();
    demo.start();
    demo.complete("done".to_string());
    assert_eq!(demo.status(), &TaskStatus::Complete);
    assert_eq!(demo.result(), Some("done"));
}

#[test]
fn test_cancel_sets_cancelled() {
    let mut demo = AsyncDemo::new();
    demo.start();
    demo.cancel();
    assert_eq!(demo.status(), &TaskStatus::Cancelled);
    assert!(demo.result().is_none());
}

#[test]
fn test_heavy_computation_returns_result() {
    let result = heavy_computation(42);
    assert!(!result.is_empty());
    assert!(result.contains("42"));
}

// -- Task lifecycle tests (require GPUI test context) --

#[gpui::test]
fn test_foreground_spawn_runs_to_completion(cx: &mut TestAppContext) {
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

    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(demo.read(cx).status(), &TaskStatus::Complete);
        assert_eq!(demo.read(cx).result(), Some("foreground done"));
    });
}

#[gpui::test]
fn test_background_spawn_runs_to_completion(cx: &mut TestAppContext) {
    let demo = cx.update(|cx| cx.new(|_cx| AsyncDemo::new()));

    cx.update(|cx: &mut gpui::App| {
        let demo = demo.clone();
        let task = cx.spawn(async move |async_cx| {
            let result = async_cx.background_spawn(async move {
                heavy_computation(7)
            }).await;

            async_cx.update(|cx| {
                demo.update(cx, |state, _cx| {
                    state.complete(result);
                });
            });
        });
        task.detach();
    });

    cx.run_until_parked();

    cx.update(|cx| {
        assert_eq!(demo.read(cx).status(), &TaskStatus::Complete);
        assert!(demo.read(cx).result().unwrap().contains("7"));
    });
}

#[gpui::test]
fn test_dropping_task_cancels_future(cx: &mut TestAppContext) {
    let completed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let completed_clone = completed.clone();

    cx.update(|cx: &mut gpui::App| {
        // Spawn a task but drop it immediately — should cancel
        let task = cx.spawn(async move |async_cx| {
            // Yield to allow cancellation
            async_cx.background_spawn(async {}).await;
            completed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });
        drop(task); // Cancel by dropping
    });

    cx.run_until_parked();
    // The task was dropped before it could complete
    assert!(!completed.load(std::sync::atomic::Ordering::SeqCst));
}

#[gpui::test]
fn test_defer_runs_after_current_effect(cx: &mut TestAppContext) {
    let order = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let order_clone = order.clone();

    cx.update(|cx| {
        let order_inner = order_clone.clone();
        cx.defer(move |_cx| {
            order_inner.lock().unwrap().push("deferred");
        });
        order_clone.lock().unwrap().push("immediate");
    });

    let log = order.lock().unwrap();
    // "immediate" should have been pushed before "deferred"
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "immediate");
    assert_eq!(log[1], "deferred");
}

#[gpui::test]
fn test_multiple_defers_run_in_order(cx: &mut TestAppContext) {
    let order = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let order_clone = order.clone();

    cx.update(|cx| {
        let o1 = order_clone.clone();
        let o2 = order_clone.clone();
        let o3 = order_clone.clone();

        cx.defer(move |_cx| { o1.lock().unwrap().push(1); });
        cx.defer(move |_cx| { o2.lock().unwrap().push(2); });
        cx.defer(move |_cx| { o3.lock().unwrap().push(3); });
    });

    let log = order.lock().unwrap();
    assert_eq!(*log, vec![1, 2, 3]);
}
