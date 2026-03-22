use gpui::{
    App, Bounds, Context, Render, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

use gpui_play::menu_test::setup_menus;

struct MenuTestView;

impl Render for MenuTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xcdd6f4))
            .child("MenuTest - Check the menu bar")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        setup_menus(cx);

        let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MenuTestView),
        )
        .unwrap();
    });
}
