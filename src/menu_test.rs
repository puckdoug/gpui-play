use gpui::{actions, App, KeyBinding, Menu, MenuItem};

actions!(menu_test, [Quit, About, Undo, Redo, Cut, Copy, Paste, Search]);

/// Returns the menu definition for the MenuTest application.
///
/// Four top-level menus: MenuTest, File, Edit, Help.
/// Only the Quit action is enabled; all other items are disabled (grayed out).
pub fn menus() -> Vec<Menu> {
    vec![
        Menu::new("MenuTest").items([
            MenuItem::action("About MenuTest", About).disabled(true),
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
}
