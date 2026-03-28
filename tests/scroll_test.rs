use gpui::{
    div, Context, InteractiveElement, IntoElement, ParentElement, Render, ScrollHandle,
    StatefulInteractiveElement, Styled, TestAppContext, Window,
};

struct ScrollTestView {
    scroll_handle: ScrollHandle,
}

impl Render for ScrollTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                div()
                    .id("scroll-container")
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .h_full()
                    .children((0..100).map(|i| div().child(format!("Row {}", i)))),
            )
    }
}

#[gpui::test]
fn test_scroll_container_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| ScrollTestView {
        scroll_handle: ScrollHandle::new(),
    });
}

#[gpui::test]
fn test_scroll_handle_default_offset_is_zero(_cx: &mut TestAppContext) {
    let handle = ScrollHandle::new();
    let offset = handle.offset();
    assert_eq!(offset.x, gpui::px(0.0));
    assert_eq!(offset.y, gpui::px(0.0));
}

#[gpui::test]
fn test_overflow_x_scroll(cx: &mut TestAppContext) {
    struct HorizontalScrollView;
    impl Render for HorizontalScrollView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .id("h-scroll")
                    .overflow_x_scroll()
                    .flex()
                    .children((0..50).map(|i| div().w_20().child(format!("Col {}", i)))),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| HorizontalScrollView);
}

#[gpui::test]
fn test_overflow_scroll_both_axes(cx: &mut TestAppContext) {
    struct BothScrollView;
    impl Render for BothScrollView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .id("both-scroll")
                    .overflow_scroll()
                    .size_full()
                    .children((0..100).map(|i| div().w_96().child(format!("Item {}", i)))),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| BothScrollView);
}

#[gpui::test]
fn test_scroll_handle_scroll_to_item(cx: &mut TestAppContext) {
    let handle = ScrollHandle::new();
    let handle_clone = handle.clone();

    let _window = cx.add_window(move |_window, _cx| ScrollTestView {
        scroll_handle: handle_clone,
    });

    // Programmatic scroll
    handle.scroll_to_item(50);
}

#[gpui::test]
fn test_scroll_handle_scroll_to_bottom(cx: &mut TestAppContext) {
    let handle = ScrollHandle::new();
    let handle_clone = handle.clone();

    let _window = cx.add_window(move |_window, _cx| ScrollTestView {
        scroll_handle: handle_clone,
    });

    handle.scroll_to_bottom();
}
