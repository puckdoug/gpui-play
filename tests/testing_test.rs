use gpui::{
    actions, div, point, px, Context, FocusHandle, InteractiveElement, IntoElement, Modifiers,
    MouseButton, ParentElement, Render, Styled, TestAppContext, Window,
};

// -- A simple interactive view for testing the test utilities --

actions!(testing, [Increment, Decrement, Reset]);

struct TestableView {
    count: i32,
    clicked: bool,
    last_action: Option<String>,
    focus_handle: FocusHandle,
}

impl TestableView {
    fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            count: 0,
            clicked: false,
            last_action: None,
            focus_handle,
        }
    }
}

impl Render for TestableView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Ensure the view is focused so actions dispatch to it
        self.focus_handle.focus(window, cx);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .on_action(cx.listener(|this, _action: &Increment, _window, cx| {
                this.count += 1;
                this.last_action = Some("increment".to_string());
                cx.notify();
            }))
            .on_action(cx.listener(|this, _action: &Decrement, _window, cx| {
                this.count -= 1;
                this.last_action = Some("decrement".to_string());
                cx.notify();
            }))
            .on_action(cx.listener(|this, _action: &Reset, _window, cx| {
                this.count = 0;
                this.last_action = Some("reset".to_string());
                cx.notify();
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.clicked = true;
                cx.notify();
            }))
            .child(format!("Count: {}", self.count))
    }
}

// -- VisualTestContext tests --

#[gpui::test]
fn test_create_window_with_visual_context(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
            gpui::KeyBinding::new("cmd-r", Reset, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    // View should be created with initial state
    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 0);
        assert!(!v.clicked);
    });
}

#[gpui::test]
fn test_dispatch_action_changes_state(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.dispatch_action(Increment);

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 1);
        assert_eq!(v.last_action.as_deref(), Some("increment"));
    });
}

#[gpui::test]
fn test_access_view_state_via_update(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    // Directly mutate state via update
    view.update(cx, |v, _cx| {
        v.count = 42;
    });

    // Read it back
    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 42);
    });
}

// -- Simulate keystrokes tests --

#[gpui::test]
fn test_simulate_keystroke_up_increments(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
            gpui::KeyBinding::new("cmd-r", Reset, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_keystrokes("up");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 1);
    });
}

#[gpui::test]
fn test_simulate_multiple_keystrokes(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
            gpui::KeyBinding::new("cmd-r", Reset, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_keystrokes("down down down");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, -3);
    });
}

#[gpui::test]
fn test_simulate_keystroke_with_modifier(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
            gpui::KeyBinding::new("cmd-r", Reset, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    // First increment a few times
    cx.simulate_keystrokes("up up up");
    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 3);
    });

    // Then reset with cmd-r
    cx.simulate_keystrokes("cmd-r");
    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 0);
        assert_eq!(v.last_action.as_deref(), Some("reset"));
    });
}

// -- Simulate click tests --

#[gpui::test]
fn test_simulate_click_sets_clicked(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    cx.simulate_click(point(px(50.0), px(50.0)), Modifiers::default());

    view.update(cx, |v, _cx| {
        assert!(v.clicked);
    });
}

// -- Property testing --

#[gpui::test(iterations = 10)]
fn test_count_is_deterministic_across_operations(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Increment, None),
            gpui::KeyBinding::new("down", Decrement, None),
        ]);
    });

    let (view, cx) = cx.add_window_view(|_window, cx| TestableView::new(cx));

    // Regardless of seed/iteration, 3 ups and 1 down should always equal 2
    cx.simulate_keystrokes("up up up down");

    view.update(cx, |v, _cx| {
        assert_eq!(v.count, 2);
    });
}
