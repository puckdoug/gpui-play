use gpui::{AppContext, TestAppContext};
use gpui_play::state_management::{CounterEvent, CounterState};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

// -- Pure state tests --

#[gpui::test]
fn test_increment_increases_count(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    cx.update(|cx| {
        counter.update(cx, |state, cx| state.increment(cx));
    });

    cx.update(|cx| {
        assert_eq!(counter.read(cx).count(), 1);
    });
}

#[gpui::test]
fn test_decrement_decreases_count(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    cx.update(|cx| {
        counter.update(cx, |state, cx| state.set_count(5, cx));
    });
    cx.update(|cx| {
        counter.update(cx, |state, cx| state.decrement(cx));
    });

    cx.update(|cx| {
        assert_eq!(counter.read(cx).count(), 4);
    });
}

#[gpui::test]
fn test_threshold_event_fires_when_crossing(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(3)));

    let event_fired = Arc::new(AtomicI32::new(-1));
    let event_fired_clone = event_fired.clone();

    cx.update(|cx| {
        cx.subscribe(&counter, move |_entity, event: &CounterEvent, _cx| {
            match event {
                CounterEvent::ThresholdReached(value) => {
                    event_fired_clone.store(*value, Ordering::SeqCst);
                }
            }
        })
        .detach();
    });

    // Increment to 1, 2 — no event
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(event_fired.load(Ordering::SeqCst), -1);

    // Increment to 3 — crosses threshold, event should fire
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(event_fired.load(Ordering::SeqCst), 3);
}

#[gpui::test]
fn test_threshold_event_does_not_fire_below(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    let event_count = Arc::new(AtomicI32::new(0));
    let event_count_clone = event_count.clone();

    cx.update(|cx| {
        cx.subscribe(&counter, move |_entity, _event: &CounterEvent, _cx| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        })
        .detach();
    });

    // Increment 3 times, all below threshold of 10
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(event_count.load(Ordering::SeqCst), 0);
}

// -- Reactive subscription tests --

#[gpui::test]
fn test_observe_fires_on_model_update(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    let notify_count = Arc::new(AtomicI32::new(0));
    let notify_count_clone = notify_count.clone();

    cx.update(|cx| {
        cx.observe(&counter, move |_entity, _cx| {
            notify_count_clone.fetch_add(1, Ordering::SeqCst);
        })
        .detach();
    });

    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert!(notify_count.load(Ordering::SeqCst) > 0);
}

#[gpui::test]
fn test_observe_stops_after_subscription_dropped(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    let notify_count = Arc::new(AtomicI32::new(0));
    let notify_count_clone = notify_count.clone();

    let sub = cx.update(|cx| {
        cx.observe(&counter, move |_entity, _cx| {
            notify_count_clone.fetch_add(1, Ordering::SeqCst);
        })
    });

    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    let count_before = notify_count.load(Ordering::SeqCst);
    assert!(count_before > 0);

    // Drop the subscription
    drop(sub);

    // Further updates should NOT trigger the observer
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(notify_count.load(Ordering::SeqCst), count_before);
}

#[gpui::test]
fn test_subscribe_receives_emitted_events(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(2)));

    let received_events: Arc<std::sync::Mutex<Vec<CounterEvent>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = received_events.clone();

    cx.update(|cx| {
        cx.subscribe(&counter, move |_entity, event: &CounterEvent, _cx| {
            received_clone.lock().unwrap().push(event.clone());
        })
        .detach();
    });

    // Cross threshold at 2
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));

    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    match &events[0] {
        CounterEvent::ThresholdReached(v) => assert_eq!(*v, 2),
    }
}

#[gpui::test]
fn test_subscribe_stops_after_subscription_dropped(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(1)));

    let event_count = Arc::new(AtomicI32::new(0));
    let event_count_clone = event_count.clone();

    let sub = cx.update(|cx| {
        cx.subscribe(&counter, move |_entity, _event: &CounterEvent, _cx| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        })
    });

    // First increment crosses threshold=1
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(event_count.load(Ordering::SeqCst), 1);

    // Drop subscription
    drop(sub);

    // Reset and cross threshold again — should NOT fire
    cx.update(|cx| counter.update(cx, |state, cx| state.set_count(0, cx)));
    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));
    assert_eq!(event_count.load(Ordering::SeqCst), 1);
}

#[gpui::test]
fn test_observe_release_fires_on_entity_drop(cx: &mut TestAppContext) {
    let released = Arc::new(AtomicI32::new(0));
    let released_clone = released.clone();

    // Create in one update, observe_release in another, drop in another
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    cx.update(|cx| {
        cx.observe_release(&counter, move |_state, _cx| {
            released_clone.fetch_add(1, Ordering::SeqCst);
        })
        .detach();
    });

    // Drop the entity inside an update so effects are flushed
    cx.update(|_cx| {
        drop(counter);
    });

    assert_eq!(released.load(Ordering::SeqCst), 1);
}

#[gpui::test]
fn test_multiple_observers_all_notified(cx: &mut TestAppContext) {
    let counter = cx.update(|cx| cx.new(|_cx| CounterState::new(10)));

    let notify_a = Arc::new(AtomicI32::new(0));
    let notify_b = Arc::new(AtomicI32::new(0));
    let notify_a_clone = notify_a.clone();
    let notify_b_clone = notify_b.clone();

    cx.update(|cx| {
        cx.observe(&counter, move |_entity, _cx| {
            notify_a_clone.fetch_add(1, Ordering::SeqCst);
        })
        .detach();
        cx.observe(&counter, move |_entity, _cx| {
            notify_b_clone.fetch_add(1, Ordering::SeqCst);
        })
        .detach();
    });

    cx.update(|cx| counter.update(cx, |state, cx| state.increment(cx)));

    assert!(notify_a.load(Ordering::SeqCst) > 0);
    assert!(notify_b.load(Ordering::SeqCst) > 0);
}
