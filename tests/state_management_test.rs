use gpui::{AppContext, TestAppContext};
use gpui_play::state_management::{CounterEvent, CounterState};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

// -- Pure state tests --

#[gpui::test]
fn test_increment_increases_count(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        counter.update(cx, |state, cx| {
            state.increment(cx);
        });
        assert_eq!(counter.read(cx).count(), 1);
    });
}

#[gpui::test]
fn test_decrement_decreases_count(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        counter.update(cx, |state, cx| {
            state.set_count(5, cx);
        });
        counter.update(cx, |state, cx| {
            state.decrement(cx);
        });
        assert_eq!(counter.read(cx).count(), 4);
    });
}

#[gpui::test]
fn test_threshold_event_fires_when_crossing(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(3));
        let event_fired = Arc::new(AtomicI32::new(-1));
        let event_fired_clone = event_fired.clone();

        let _sub = cx.subscribe(&counter, move |_entity, event: &CounterEvent, _cx| {
            match event {
                CounterEvent::ThresholdReached(value) => {
                    event_fired_clone.store(*value, Ordering::SeqCst);
                }
            }
        });

        // Increment to 1, 2 — no event
        counter.update(cx, |state, cx| state.increment(cx));
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(event_fired.load(Ordering::SeqCst), -1);

        // Increment to 3 — crosses threshold, event should fire
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(event_fired.load(Ordering::SeqCst), 3);
    });
}

#[gpui::test]
fn test_threshold_event_does_not_fire_below(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        let event_count = Arc::new(AtomicI32::new(0));
        let event_count_clone = event_count.clone();

        let _sub = cx.subscribe(&counter, move |_entity, _event: &CounterEvent, _cx| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Increment 3 times, all below threshold of 10
        counter.update(cx, |state, cx| state.increment(cx));
        counter.update(cx, |state, cx| state.increment(cx));
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(event_count.load(Ordering::SeqCst), 0);
    });
}

// -- Reactive subscription tests --

#[gpui::test]
fn test_observe_fires_on_model_update(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        let notify_count = Arc::new(AtomicI32::new(0));
        let notify_count_clone = notify_count.clone();

        let _sub = cx.observe(&counter, move |_entity, _cx| {
            notify_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        counter.update(cx, |state, cx| state.increment(cx));
        assert!(notify_count.load(Ordering::SeqCst) > 0);
    });
}

#[gpui::test]
fn test_observe_stops_after_subscription_dropped(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        let notify_count = Arc::new(AtomicI32::new(0));
        let notify_count_clone = notify_count.clone();

        let sub = cx.observe(&counter, move |_entity, _cx| {
            notify_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        counter.update(cx, |state, cx| state.increment(cx));
        let count_before = notify_count.load(Ordering::SeqCst);
        assert!(count_before > 0);

        // Drop the subscription
        drop(sub);

        // Further updates should NOT trigger the observer
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(notify_count.load(Ordering::SeqCst), count_before);
    });
}

#[gpui::test]
fn test_subscribe_receives_emitted_events(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(2));
        let received_events: Arc<std::sync::Mutex<Vec<CounterEvent>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = received_events.clone();

        let _sub = cx.subscribe(&counter, move |_entity, event: &CounterEvent, _cx| {
            received_clone.lock().unwrap().push(event.clone());
        });

        // Cross threshold at 2
        counter.update(cx, |state, cx| state.increment(cx));
        counter.update(cx, |state, cx| state.increment(cx));

        let events = received_events.lock().unwrap();
        assert_eq!(events.len(), 1);
        match &events[0] {
            CounterEvent::ThresholdReached(v) => assert_eq!(*v, 2),
        }
    });
}

#[gpui::test]
fn test_subscribe_stops_after_subscription_dropped(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(1));
        let event_count = Arc::new(AtomicI32::new(0));
        let event_count_clone = event_count.clone();

        let sub = cx.subscribe(&counter, move |_entity, _event: &CounterEvent, _cx| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // First increment crosses threshold=1
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(event_count.load(Ordering::SeqCst), 1);

        // Drop subscription
        drop(sub);

        // Reset and cross threshold again — should NOT fire
        counter.update(cx, |state, cx| state.set_count(0, cx));
        counter.update(cx, |state, cx| state.increment(cx));
        assert_eq!(event_count.load(Ordering::SeqCst), 1);
    });
}

#[gpui::test]
fn test_observe_release_fires_on_entity_drop(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let released = Arc::new(AtomicI32::new(0));
        let released_clone = released.clone();

        let counter = cx.new(|_cx| CounterState::new(10));
        let _sub = cx.observe_release(&counter, move |_state, _cx| {
            released_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Drop the entity by letting it go out of scope
        drop(counter);

        assert_eq!(released.load(Ordering::SeqCst), 1);
    });
}

#[gpui::test]
fn test_multiple_observers_all_notified(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let counter = cx.new(|_cx| CounterState::new(10));
        let notify_a = Arc::new(AtomicI32::new(0));
        let notify_b = Arc::new(AtomicI32::new(0));
        let notify_a_clone = notify_a.clone();
        let notify_b_clone = notify_b.clone();

        let _sub_a = cx.observe(&counter, move |_entity, _cx| {
            notify_a_clone.fetch_add(1, Ordering::SeqCst);
        });
        let _sub_b = cx.observe(&counter, move |_entity, _cx| {
            notify_b_clone.fetch_add(1, Ordering::SeqCst);
        });

        counter.update(cx, |state, cx| state.increment(cx));

        assert!(notify_a.load(Ordering::SeqCst) > 0);
        assert!(notify_b.load(Ordering::SeqCst) > 0);
    });
}
