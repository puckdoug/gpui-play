use gpui::{
    div, ease_in_out, linear, px, quadratic, Context, IntoElement, ParentElement, Render, Styled,
    TestAppContext, Window,
};
use gpui::{Animation, AnimationExt};
use std::time::Duration;

struct AnimationTestView;

impl Render for AnimationTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            div()
                .size_8()
                .bg(gpui::red())
                .with_animation(
                    "test-anim",
                    Animation::new(Duration::from_secs(1)),
                    |el, delta| el.left(px(delta * 100.0)),
                ),
        )
    }
}

// -- Animation builder --

#[test]
fn test_animation_new_creates_with_duration() {
    let anim = Animation::new(Duration::from_millis(500));
    assert_eq!(anim.duration, Duration::from_millis(500));
    assert!(anim.oneshot);
}

#[test]
fn test_animation_repeat_sets_looping() {
    let anim = Animation::new(Duration::from_secs(1)).repeat();
    assert!(!anim.oneshot);
}

#[test]
fn test_animation_with_easing() {
    let anim = Animation::new(Duration::from_secs(1)).with_easing(ease_in_out);
    let result = (anim.easing)(0.5);
    assert!(result > 0.0 && result < 1.0);
}

// -- Easing functions --

#[test]
fn test_easing_linear() {
    assert_eq!(linear(0.0), 0.0);
    assert_eq!(linear(0.5), 0.5);
    assert_eq!(linear(1.0), 1.0);
}

#[test]
fn test_easing_ease_in_out_endpoints() {
    let start = ease_in_out(0.0);
    let end = ease_in_out(1.0);
    assert!((start - 0.0).abs() < 0.01);
    assert!((end - 1.0).abs() < 0.01);
}

#[test]
fn test_easing_ease_in_out_midpoint() {
    let mid = ease_in_out(0.5);
    assert!((mid - 0.5).abs() < 0.1);
}

#[test]
fn test_easing_quadratic() {
    assert_eq!(quadratic(0.0), 0.0);
    assert_eq!(quadratic(1.0), 1.0);
    assert!(quadratic(0.25) < 0.25);
}

// -- AnimationElement in view --

#[gpui::test]
fn test_animation_element_renders_in_window(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| AnimationTestView);
}

#[gpui::test]
fn test_animation_with_repeat_in_view(cx: &mut TestAppContext) {
    struct RepeatView;
    impl Render for RepeatView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                div()
                    .size_8()
                    .bg(gpui::blue())
                    .with_animation(
                        "repeat-test",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |el, delta| el.left(px(delta * 200.0)),
                    ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| RepeatView);
}

#[gpui::test]
fn test_animation_with_custom_easing_in_view(cx: &mut TestAppContext) {
    struct EasingView;
    impl Render for EasingView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                div()
                    .size_8()
                    .bg(gpui::green())
                    .with_animation(
                        "easing-test",
                        Animation::new(Duration::from_secs(1)).with_easing(ease_in_out),
                        |el, delta| el.left(px(delta * 100.0)),
                    ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| EasingView);
}
