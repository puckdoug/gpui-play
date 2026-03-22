use gpui::{actions, App, Menu, MenuItem};

actions!(menu_test, [Quit, About, Undo, Redo, Cut, Copy, Paste, Search]);

/// Sets up the MenuTest application menus and registers action handlers.
///
/// Creates four top-level menus: MenuTest, File, Edit, Help.
/// Only the Quit action is enabled; all other items are disabled (grayed out).
pub fn setup_menus(_cx: &mut App) {
    // Stub: implementation will be added in the green phase
}
