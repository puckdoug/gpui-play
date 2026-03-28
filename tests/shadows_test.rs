use gpui::{
    div, hsla, point, px, BoxShadow, Context, IntoElement, ParentElement, Render, Styled,
    TestAppContext, Window,
};

struct ShadowTestView;

impl Render for ShadowTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .gap_4()
            .child(div().size_20().bg(gpui::white()).shadow_sm().child("sm"))
            .child(div().size_20().bg(gpui::white()).shadow_md().child("md"))
            .child(div().size_20().bg(gpui::white()).shadow_lg().child("lg"))
    }
}

#[gpui::test]
fn test_shadow_presets_render(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| ShadowTestView);
}

#[gpui::test]
fn test_shadow_sm(cx: &mut TestAppContext) {
    struct SmView;
    impl Render for SmView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_20().bg(gpui::white()).shadow_sm()
        }
    }
    let _window = cx.add_window(|_window, _cx| SmView);
}

#[gpui::test]
fn test_shadow_xl(cx: &mut TestAppContext) {
    struct XlView;
    impl Render for XlView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_20().bg(gpui::white()).shadow_xl()
        }
    }
    let _window = cx.add_window(|_window, _cx| XlView);
}

#[gpui::test]
fn test_custom_box_shadow(cx: &mut TestAppContext) {
    struct CustomShadowView;
    impl Render for CustomShadowView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_20()
                .bg(gpui::white())
                .shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.3),
                    offset: point(px(4.0), px(4.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                }])
        }
    }
    let _window = cx.add_window(|_window, _cx| CustomShadowView);
}

#[gpui::test]
fn test_multiple_shadows(cx: &mut TestAppContext) {
    struct MultiShadowView;
    impl Render for MultiShadowView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_20()
                .bg(gpui::white())
                .shadow(vec![
                    BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.1),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(3.0),
                        spread_radius: px(0.0),
                    },
                    BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.2),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    },
                ])
        }
    }
    let _window = cx.add_window(|_window, _cx| MultiShadowView);
}

#[gpui::test]
fn test_colored_shadow(cx: &mut TestAppContext) {
    struct ColoredShadowView;
    impl Render for ColoredShadowView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_20()
                .bg(gpui::white())
                .shadow(vec![BoxShadow {
                    color: hsla(0.6, 0.8, 0.5, 0.5), // Blue-ish colored shadow
                    offset: point(px(0.0), px(4.0)),
                    blur_radius: px(12.0),
                    spread_radius: px(2.0),
                }])
        }
    }
    let _window = cx.add_window(|_window, _cx| ColoredShadowView);
}
