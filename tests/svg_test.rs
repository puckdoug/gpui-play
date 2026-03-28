use gpui::{
    div, percentage, svg, Context, IntoElement, ParentElement, Render, Styled, TestAppContext,
    Transformation, Window,
};

struct SvgTestView;

impl Render for SvgTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(svg().path("icons/test.svg").size_8())
    }
}

#[gpui::test]
fn test_svg_element_creates_from_path(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| SvgTestView);
}

#[gpui::test]
fn test_svg_with_rotation(cx: &mut TestAppContext) {
    struct RotatedSvgView;
    impl Render for RotatedSvgView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                svg()
                    .path("icons/test.svg")
                    .size_8()
                    .with_transformation(Transformation::rotate(percentage(0.25))),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| RotatedSvgView);
}

#[gpui::test]
fn test_svg_with_scale(cx: &mut TestAppContext) {
    struct ScaledSvgView;
    impl Render for ScaledSvgView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                svg()
                    .path("icons/test.svg")
                    .size_8()
                    .with_transformation(Transformation::scale(gpui::size(2.0, 2.0))),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| ScaledSvgView);
}

#[gpui::test]
fn test_svg_with_composed_transformation(cx: &mut TestAppContext) {
    struct ComposedSvgView;
    impl Render for ComposedSvgView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                svg()
                    .path("icons/test.svg")
                    .size_8()
                    .with_transformation(
                        Transformation::rotate(percentage(0.125))
                            .with_scaling(gpui::size(1.5, 1.5)),
                    ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| ComposedSvgView);
}

#[gpui::test]
fn test_svg_with_text_color(cx: &mut TestAppContext) {
    struct ColoredSvgView;
    impl Render for ColoredSvgView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                svg()
                    .path("icons/test.svg")
                    .size_8()
                    .text_color(gpui::red()),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| ColoredSvgView);
}
