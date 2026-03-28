use gpui::{
    canvas, div, point, px, size, Bounds, Context, HitboxBehavior, IntoElement, ParentElement,
    Pixels, Render, Size, Styled, TestAppContext, Window,
};

struct HitboxTestView;

impl Render for HitboxTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            canvas(
                |bounds, window, _cx| {
                    // Prepaint: insert hitboxes
                    let hitbox = window.insert_hitbox(
                        Bounds::new(point(px(10.0), px(10.0)), size(px(100.0), px(100.0))),
                        HitboxBehavior::Normal,
                    );
                    hitbox
                },
                |_bounds, hitbox, _window, _cx| {
                    // Paint: use hitbox for rendering feedback
                    let _id = hitbox.id;
                    let _is_hovered = hitbox.is_hovered(_window);
                },
            )
            .size_full(),
        )
    }
}

#[gpui::test]
fn test_hitbox_view_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| HitboxTestView);
}

#[test]
fn test_hitbox_behavior_variants_exist() {
    let _normal = HitboxBehavior::Normal;
    let _block = HitboxBehavior::BlockMouse;
    let _block_except_scroll = HitboxBehavior::BlockMouseExceptScroll;
}

#[gpui::test]
fn test_multiple_hitboxes_in_canvas(cx: &mut TestAppContext) {
    struct MultiHitboxView;
    impl Render for MultiHitboxView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                canvas(
                    |_bounds, window, _cx| {
                        let h1 = window.insert_hitbox(
                            Bounds::new(point(px(0.0), px(0.0)), size(px(50.0), px(50.0))),
                            HitboxBehavior::Normal,
                        );
                        let h2 = window.insert_hitbox(
                            Bounds::new(point(px(60.0), px(0.0)), size(px(50.0), px(50.0))),
                            HitboxBehavior::Normal,
                        );
                        (h1, h2)
                    },
                    |_bounds, (h1, h2), _window, _cx| {
                        // Each hitbox has a unique ID
                        assert_ne!(h1.id, h2.id);
                    },
                )
                .size_full(),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| MultiHitboxView);
}

#[gpui::test]
fn test_blocking_hitbox(cx: &mut TestAppContext) {
    struct BlockingHitboxView;
    impl Render for BlockingHitboxView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                canvas(
                    |_bounds, window, _cx| {
                        let _background = window.insert_hitbox(
                            Bounds::new(point(px(0.0), px(0.0)), size(px(200.0), px(200.0))),
                            HitboxBehavior::Normal,
                        );
                        let _overlay = window.insert_hitbox(
                            Bounds::new(point(px(50.0), px(50.0)), size(px(100.0), px(100.0))),
                            HitboxBehavior::BlockMouse,
                        );
                    },
                    |_bounds, _, _window, _cx| {},
                )
                .size_full(),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| BlockingHitboxView);
}
