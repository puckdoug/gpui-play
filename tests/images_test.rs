use gpui::{
    div, img, Context, IntoElement, ObjectFit, ParentElement, Render, Styled, StyledImage,
    TestAppContext, Window,
};

struct ImageTestView;

impl Render for ImageTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            img("assets/test.png")
                .object_fit(ObjectFit::Contain)
                .size_full(),
        )
    }
}

#[gpui::test]
fn test_img_element_creates_from_path_string(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| ImageTestView);
}

#[gpui::test]
fn test_each_object_fit_variant_accepted(cx: &mut TestAppContext) {
    struct ObjectFitView;
    impl Render for ObjectFitView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .child(img("test.png").object_fit(ObjectFit::Fill))
                .child(img("test.png").object_fit(ObjectFit::Contain))
                .child(img("test.png").object_fit(ObjectFit::Cover))
                .child(img("test.png").object_fit(ObjectFit::ScaleDown))
                .child(img("test.png").object_fit(ObjectFit::None))
        }
    }
    let _window = cx.add_window(|_window, _cx| ObjectFitView);
}

#[gpui::test]
fn test_img_with_fallback_and_loading(cx: &mut TestAppContext) {
    struct FallbackView;
    impl Render for FallbackView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                img("nonexistent.png")
                    .with_fallback(|| div().child("Failed to load").into_any_element())
                    .with_loading(|| div().child("Loading...").into_any_element())
                    .size_full(),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| FallbackView);
}

#[test]
fn test_object_fit_variants_exist() {
    let fits = [
        ObjectFit::Fill,
        ObjectFit::Contain,
        ObjectFit::Cover,
        ObjectFit::ScaleDown,
        ObjectFit::None,
    ];
    assert_eq!(fits.len(), 5);
}

#[gpui::test]
fn test_img_with_grayscale(cx: &mut TestAppContext) {
    struct GrayscaleView;
    impl Render for GrayscaleView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(img("test.png").grayscale(true).size_full())
        }
    }
    let _window = cx.add_window(|_window, _cx| GrayscaleView);
}
