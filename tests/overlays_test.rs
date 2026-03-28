use gpui::{
    anchored, deferred, div, AppContext, Context, Corner, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, TestAppContext, Window,
};

// -- Deferred element --

struct DeferredTestView;

impl Render for DeferredTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(div().child("Background content"))
            .child(deferred(div().child("Overlay content")))
    }
}

#[gpui::test]
fn test_deferred_element_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| DeferredTestView);
}

#[gpui::test]
fn test_deferred_with_priority(cx: &mut TestAppContext) {
    struct PriorityView;
    impl Render for PriorityView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .child(deferred(div().child("Low priority")).priority(1))
                .child(deferred(div().child("High priority")).priority(10))
        }
    }
    let _window = cx.add_window(|_window, _cx| PriorityView);
}

// -- Anchored element --

#[gpui::test]
fn test_anchored_element_renders(cx: &mut TestAppContext) {
    struct AnchoredView;
    impl Render for AnchoredView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                deferred(
                    anchored()
                        .anchor(Corner::TopLeft)
                        .child(div().child("Popover content")),
                ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| AnchoredView);
}

#[gpui::test]
fn test_anchored_with_position(cx: &mut TestAppContext) {
    struct PositionedView;
    impl Render for PositionedView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                deferred(
                    anchored()
                        .anchor(Corner::BottomRight)
                        .position(gpui::point(gpui::px(100.0), gpui::px(200.0)))
                        .child(div().child("Positioned popover")),
                ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| PositionedView);
}

#[gpui::test]
fn test_anchored_snap_to_window(cx: &mut TestAppContext) {
    struct SnapView;
    impl Render for SnapView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                deferred(
                    anchored()
                        .snap_to_window()
                        .child(div().child("Snapped content")),
                ),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| SnapView);
}

// -- Tooltip --

#[gpui::test]
fn test_tooltip_on_element(cx: &mut TestAppContext) {
    struct TooltipContent;
    impl Render for TooltipContent {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child("Tooltip text")
        }
    }

    struct TooltipView;
    impl Render for TooltipView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .id("with-tooltip")
                    .child("Hover me")
                    .tooltip(|window, cx| cx.new(|_cx| TooltipContent).into()),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| TooltipView);
}

// -- Combined popover pattern --

#[gpui::test]
fn test_popover_pattern_deferred_plus_anchored(cx: &mut TestAppContext) {
    struct PopoverView {
        show_popover: bool,
    }
    impl Render for PopoverView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let mut root = div()
                .size_full()
                .child(div().child("Main content"));

            if self.show_popover {
                root = root.child(
                    deferred(
                        anchored()
                            .anchor(Corner::TopLeft)
                            .position(gpui::point(gpui::px(50.0), gpui::px(50.0)))
                            .child(
                                div()
                                    .bg(gpui::white())
                                    .p_4()
                                    .child("Popover menu item 1")
                                    .child("Popover menu item 2"),
                            ),
                    )
                    .with_priority(1),
                );
            }

            root
        }
    }
    let _window = cx.add_window(|_window, _cx| PopoverView { show_popover: true });
}
