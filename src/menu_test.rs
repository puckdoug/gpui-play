use gpui::{
    actions, App, Bounds, Context, KeyBinding, Menu, MenuItem, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

actions!(menu_test, [Quit, About, Undo, Redo, Cut, Copy, Paste, Search]);

/// Returns the version string for the About window.
pub fn about_version_string() -> String {
    format!("MenuTest: {}", env!("CARGO_PKG_VERSION"))
}

/// Returns the window options for the About window.
///
/// Close button enabled, minimize and zoom/fullscreen disabled.
/// Note: window_bounds is not set here since centering requires App context.
/// The caller should set bounds when opening the window.
pub fn about_window_options() -> WindowOptions {
    WindowOptions {
        is_minimizable: false,
        is_resizable: false,
        ..Default::default()
    }
}

struct AboutView {
    version: String,
}

impl Render for AboutView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(gpui::rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(gpui::rgb(0x000000))
            .child(self.version.clone())
    }
}

/// Returns the menu definition for the MenuTest application.
///
/// Four top-level menus: MenuTest, File, Edit, Help.
/// Quit and About MenuTest are enabled; all other items are disabled (grayed out).
pub fn menus() -> Vec<Menu> {
    vec![
        Menu::new("MenuTest").items([
            MenuItem::action("About MenuTest", About),
        ]),
        Menu::new("File").items([
            MenuItem::action("Quit", Quit),
        ]),
        Menu::new("Edit").items([
            MenuItem::action("Undo", Undo).disabled(true),
            MenuItem::action("Redo", Redo).disabled(true),
            MenuItem::separator(),
            MenuItem::action("Cut", Cut).disabled(true),
            MenuItem::action("Copy", Copy).disabled(true),
            MenuItem::action("Paste", Paste).disabled(true),
        ]),
        Menu::new("Help").items([
            MenuItem::action("Search", Search).disabled(true),
        ]),
    ]
}

/// Returns the keyboard shortcuts for menu actions.
///
/// Must be registered via `cx.bind_keys()` before `cx.set_menus()` so that
/// macOS displays the shortcuts next to menu items.
pub fn key_bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-c", Copy, None),
        KeyBinding::new("cmd-v", Paste, None),
        KeyBinding::new("cmd-z", Undo, None),
        KeyBinding::new("cmd-shift-z", Redo, None),
    ]
}

/// Sets up the MenuTest application menus and registers action handlers.
pub fn setup_menus(cx: &mut App) {
    cx.bind_keys(key_bindings());
    cx.set_menus(menus());
    cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
    cx.on_action(|_: &About, cx: &mut App| {
        let version = about_version_string();
        let bounds = Bounds::centered(None, size(px(300.), px(150.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_minimizable: false,
                is_resizable: false,
                ..Default::default()
            },
            |_, cx| cx.new(|_| AboutView { version }),
        )
        .ok();
    });
}
