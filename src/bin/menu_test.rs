use gpui::{App, Context, Render, Window, WindowOptions, div, prelude::*};
use gpui_platform::application;

use gpui_play::menu_test::setup_menus;

struct MenuTestView;

impl Render for MenuTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(gpui::white())
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(gpui::black())
            .child("MenuTest - Check the menu bar")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        setup_menus(cx);
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MenuTestView))
            .unwrap();
    });
}
