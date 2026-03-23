use gpui::{
    App, Context, FocusHandle, Focusable, Render, Window, WindowOptions,
    actions, div, prelude::*, rgb,
};
use gpui_platform::application;

use gpui_play::draw_test::{self, setup_menus};

actions!(draw_test_app, [FocusNext, FocusPrev]);

struct DrawTestView {
    focus_handle: FocusHandle,
}

impl DrawTestView {
    fn close_window(
        &mut self,
        _: &draw_test::CloseWindow,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        window.remove_window();
    }
}

impl Focusable for DrawTestView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DrawTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::close_window))
            .child(
                div()
                    .flex_grow()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xcccccc))
                    .m_2()
                    .rounded_sm(),
            )
    }
}

fn open_draw_window(cx: &mut App) {
    let window = cx
        .open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| DrawTestView {
                focus_handle: cx.focus_handle(),
            })
        })
        .unwrap();

    window
        .update(cx, |view, window, cx| {
            window.focus(&view.focus_handle(cx), cx);
        })
        .unwrap();
}

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        setup_menus(cx);

        cx.on_action(|_: &draw_test::NewWindow, cx: &mut App| {
            open_draw_window(cx);
        });

        open_draw_window(cx);
    });
}
