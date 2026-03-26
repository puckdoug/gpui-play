use gpui::{
    actions, App, Bounds, Context, KeyBinding, Menu, MenuItem, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

actions!(draw_test, [Quit, About, NewWindow, CloseWindow, Undo, Redo, Cut, Copy, Paste, SelectAll, NewOval, Search]);

/// Returns the version string for the About window.
pub fn about_version_string() -> String {
    format!("DrawTest: {}", env!("CARGO_PKG_VERSION"))
}

/// Returns the window options for the About window.
pub fn about_window_options() -> WindowOptions {
    WindowOptions {
        is_minimizable: false,
        is_resizable: false,
        ..Default::default()
    }
}

struct AboutView {
    focus_handle: gpui::FocusHandle,
    version: String,
}

impl AboutView {
    fn close_window(&mut self, _: &CloseWindow, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }
}

impl gpui::Focusable for AboutView {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AboutView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(gpui::rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(gpui::rgb(0x000000))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::close_window))
            .child(self.version.clone())
    }
}

/// Returns the menu definition for the DrawTest application.
pub fn menus() -> Vec<Menu> {
    vec![
        Menu::new("DrawTest").items([
            MenuItem::action("About DrawTest", About),
        ]),
        Menu::new("File").items([
            MenuItem::action("New Window", NewWindow),
            MenuItem::action("Close Window", CloseWindow),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ]),
        Menu::new("Edit").items([
            MenuItem::action("Undo", Undo),
            MenuItem::action("Redo", Redo),
            MenuItem::separator(),
            MenuItem::action("Cut", Cut),
            MenuItem::action("Copy", Copy),
            MenuItem::action("Paste", Paste),
            MenuItem::separator(),
            MenuItem::action("Select All", SelectAll),
        ]),
        Menu::new("Shapes").items([
            MenuItem::action("New Oval", NewOval),
        ]),
        Menu::new("Help").items([
            MenuItem::action("Search", Search).disabled(true),
        ]),
    ]
}

/// Returns the keyboard shortcuts for menu actions.
pub fn key_bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("cmd-n", NewWindow, None),
        KeyBinding::new("cmd-w", CloseWindow, None),
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-z", Undo, None),
        KeyBinding::new("cmd-shift-z", Redo, None),
        KeyBinding::new("cmd-x", Cut, None),
        KeyBinding::new("cmd-c", Copy, None),
        KeyBinding::new("cmd-v", Paste, None),
        KeyBinding::new("cmd-a", SelectAll, None),
        KeyBinding::new("cmd-shift-n", NewOval, None),
    ]
}

/// Sets up the DrawTest application menus and registers action handlers.
pub fn setup_menus(cx: &mut App) {
    cx.bind_keys(key_bindings());
    cx.set_menus(menus());
    cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
    cx.on_action(|_: &About, cx: &mut App| {
        let version = about_version_string();
        let bounds = Bounds::centered(None, size(px(300.), px(150.)), cx);
        if let Ok(window) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_minimizable: false,
                is_resizable: false,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| AboutView {
                focus_handle: cx.focus_handle(),
                version,
            }),
        ) {
            window
                .update(cx, |view, window, _cx| {
                    window.focus(&view.focus_handle, _cx);
                })
                .ok();
        }
    });
}
