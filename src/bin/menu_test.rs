use gpui::{
    App, Context, Entity, FocusHandle, Focusable, KeyBinding, Render, Window, WindowOptions,
    actions, div, prelude::*, rgb,
};
use gpui_platform::application;

use gpui_play::menu_test::{self, setup_menus};
use gpui_play::text_input::{TextInput, text_input_key_bindings};

actions!(menu_test_app, [FocusNext, FocusPrev]);

struct MenuTestView {
    focus_handle: FocusHandle,
    input1: Entity<TextInput>,
    input2: Entity<TextInput>,
}

impl MenuTestView {
    fn focus_next(&mut self, _: &FocusNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn focus_prev(&mut self, _: &FocusPrev, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
    }

    fn close_window(
        &mut self,
        _: &menu_test::CloseWindow,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        window.remove_window();
    }
}

impl Focusable for MenuTestView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MenuTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xf0f0f0))
            .size_full()
            .p_4()
            .gap_4()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_prev))
            .on_action(cx.listener(Self::close_window))
            .child(self.input1.clone())
            .child(self.input2.clone())
            .child(div().flex_grow())
            .child(
                div()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .id("ok-button")
                            .px_4()
                            .py_1()
                            .bg(rgb(0x4488ff))
                            .text_color(gpui::white())
                            .text_sm()
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x3377ee)))
                            .active(|s| s.bg(rgb(0x2266dd)))
                            .child("Ok")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.close_window(&menu_test::CloseWindow, window, cx);
                            })),
                    ),
            )
    }
}

fn open_main_window(cx: &mut App) {
    let window = cx
        .open_window(WindowOptions::default(), |_, cx| {
            let input1 = cx.new(|cx| {
                TextInput::new(cx, "MenuTest - Check the menu bar", "")
            });
            let input2 = cx.new(|cx| TextInput::new(cx, "", "Type here..."));
            cx.new(|cx| MenuTestView {
                focus_handle: cx.focus_handle(),
                input1,
                input2,
            })
        })
        .unwrap();

    window
        .update(cx, |view, window, cx| {
            window.focus(&view.input1.focus_handle(cx), cx);
        })
        .unwrap();
}

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        cx.bind_keys(text_input_key_bindings());
        cx.bind_keys([
            KeyBinding::new("tab", FocusNext, None),
            KeyBinding::new("shift-tab", FocusPrev, None),
        ]);
        setup_menus(cx);

        cx.on_action(|_: &menu_test::NewWindow, cx: &mut App| {
            open_main_window(cx);
        });

        open_main_window(cx);
    });
}
