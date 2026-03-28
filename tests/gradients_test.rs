use gpui::{
    div, linear_color_stop, linear_gradient, ColorSpace, Context, IntoElement, ParentElement,
    Render, Styled, TestAppContext, Window,
};

struct GradientTestView;

impl Render for GradientTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(div().size_full().bg(linear_gradient(
                180.0,
                linear_color_stop(gpui::red(), 0.0),
                linear_color_stop(gpui::blue(), 1.0),
            )))
    }
}

#[gpui::test]
fn test_gradient_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| GradientTestView);
}

#[gpui::test]
fn test_gradient_horizontal(cx: &mut TestAppContext) {
    struct HorizontalGradientView;
    impl Render for HorizontalGradientView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().bg(linear_gradient(
                90.0, // left to right
                linear_color_stop(gpui::red(), 0.0),
                linear_color_stop(gpui::blue(), 1.0),
            ))
        }
    }
    let _window = cx.add_window(|_window, _cx| HorizontalGradientView);
}

#[gpui::test]
fn test_gradient_diagonal(cx: &mut TestAppContext) {
    struct DiagonalGradientView;
    impl Render for DiagonalGradientView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().bg(linear_gradient(
                45.0, // diagonal
                linear_color_stop(gpui::green(), 0.0),
                linear_color_stop(gpui::blue(), 1.0),
            ))
        }
    }
    let _window = cx.add_window(|_window, _cx| DiagonalGradientView);
}

#[gpui::test]
fn test_gradient_with_oklab_color_space(cx: &mut TestAppContext) {
    struct OklabGradientView;
    impl Render for OklabGradientView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().bg(linear_gradient(
                180.0,
                linear_color_stop(gpui::red(), 0.0),
                linear_color_stop(gpui::blue(), 1.0),
            )
            .color_space(ColorSpace::Oklab))
        }
    }
    let _window = cx.add_window(|_window, _cx| OklabGradientView);
}

#[test]
fn test_linear_color_stop_construction() {
    let stop = linear_color_stop(gpui::red(), 0.5);
    assert_eq!(stop.percentage, 0.5);
}

#[test]
fn test_color_space_variants_exist() {
    let _srgb = ColorSpace::Srgb;
    let _oklab = ColorSpace::Oklab;
}
