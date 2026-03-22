use gpui::{App, Context, Entity, Focusable, Render, Window, WindowOptions, div, prelude::*, rgb};
use gpui_platform::application;

use gpui_play::menu_test::setup_menus;
use gpui_play::text_input::{TextInput, text_input_key_bindings};

struct MenuTestView {
    input1: Entity<TextInput>,
    input2: Entity<TextInput>,
}

impl Render for MenuTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xf0f0f0))
            .size_full()
            .p_4()
            .gap_4()
            .child(self.input1.clone())
            .child(self.input2.clone())
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        cx.bind_keys(text_input_key_bindings());
        setup_menus(cx);

        let window = cx
            .open_window(WindowOptions::default(), |_, cx| {
                let input1 = cx.new(|cx| {
                    TextInput::new(cx, "MenuTest - Check the menu bar", "")
                });
                let input2 = cx.new(|cx| {
                    TextInput::new(cx, "", "Type here...")
                });
                cx.new(|_| MenuTestView { input1, input2 })
            })
            .unwrap();

        // Focus the first input
        window
            .update(cx, |view, window, cx| {
                window.focus(&view.input1.focus_handle(cx), cx);
            })
            .unwrap();
    });
}
