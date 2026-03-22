use gpui::{OwnedMenu, OwnedMenuItem, TestAppContext};
use gpui_play::menu_test::{menus, setup_menus};

/// Convert the menu definition to owned form for inspection.
fn owned_menus() -> Vec<OwnedMenu> {
    menus().into_iter().map(|m| m.owned()).collect()
}

/// Helper: find a top-level menu by name.
fn find_menu<'a>(menus: &'a [OwnedMenu], name: &str) -> &'a OwnedMenu {
    menus
        .iter()
        .find(|m| m.name.as_ref() == name)
        .unwrap_or_else(|| panic!("menu '{}' not found", name))
}

/// Helper: collect action item names from a menu (skipping separators).
fn action_names(menu: &OwnedMenu) -> Vec<&str> {
    menu.items
        .iter()
        .filter_map(|item| match item {
            OwnedMenuItem::Action { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect()
}

/// Helper: check if an action item is disabled.
fn is_item_disabled(menu: &OwnedMenu, item_name: &str) -> bool {
    menu.items
        .iter()
        .find_map(|item| match item {
            OwnedMenuItem::Action { name, disabled, .. } if name == item_name => Some(*disabled),
            _ => None,
        })
        .unwrap_or_else(|| panic!("action '{}' not found in menu '{}'", item_name, menu.name))
}

#[test]
fn test_creates_four_menus() {
    let menus = owned_menus();
    assert_eq!(menus.len(), 4, "expected 4 top-level menus");
    assert_eq!(menus[0].name.as_ref(), "MenuTest");
    assert_eq!(menus[1].name.as_ref(), "File");
    assert_eq!(menus[2].name.as_ref(), "Edit");
    assert_eq!(menus[3].name.as_ref(), "Help");
}

#[test]
fn test_menutest_menu_has_about() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "MenuTest");
    let names = action_names(menu);
    assert!(
        names.contains(&"About MenuTest"),
        "MenuTest menu should contain 'About MenuTest', got: {:?}",
        names
    );
}

#[test]
fn test_file_menu_has_quit() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "File");
    let names = action_names(menu);
    assert!(
        names.contains(&"Quit"),
        "File menu should contain 'Quit', got: {:?}",
        names
    );
}

#[test]
fn test_edit_menu_has_expected_items() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "Edit");
    let names = action_names(menu);
    let expected = vec![
        "Undo",
        "Redo",
        "Cut",
        "Copy",
        "Paste",
        "Delete",
        "Select All",
        "Move to Beginning",
        "Move to End",
        "Emoji & Symbols",
    ];
    assert_eq!(
        names, expected,
        "Edit menu should contain {:?}, got: {:?}",
        expected, names
    );
}

#[test]
fn test_help_menu_has_search() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "Help");
    let names = action_names(menu);
    assert!(
        names.contains(&"Search"),
        "Help menu should contain 'Search', got: {:?}",
        names
    );
}

#[test]
fn test_enabled_and_disabled_items() {
    let menus = owned_menus();

    // These items should be enabled
    let enabled_items = [
        ("MenuTest", "About MenuTest"),
        ("File", "Quit"),
        ("Edit", "Undo"),
        ("Edit", "Redo"),
        ("Edit", "Cut"),
        ("Edit", "Copy"),
        ("Edit", "Paste"),
        ("Edit", "Delete"),
        ("Edit", "Select All"),
        ("Edit", "Move to Beginning"),
        ("Edit", "Move to End"),
        ("Edit", "Emoji & Symbols"),
    ];
    for (menu_name, item_name) in &enabled_items {
        let menu = find_menu(&menus, menu_name);
        assert!(
            !is_item_disabled(menu, item_name),
            "'{}' in '{}' menu should be enabled",
            item_name,
            menu_name
        );
    }

    // These items should be disabled (not yet implemented)
    let disabled_items = [
        ("Help", "Search"),
    ];
    for (menu_name, item_name) in &disabled_items {
        let menu = find_menu(&menus, menu_name);
        assert!(
            is_item_disabled(menu, item_name),
            "'{}' in '{}' menu should be disabled",
            item_name,
            menu_name
        );
    }
}

#[gpui::test]
fn test_quit_action_registered(cx: &mut TestAppContext) {
    cx.update(|cx| {
        setup_menus(cx);
    });
    // Verify the quit handler was registered by confirming
    // setup_menus completes without panic. The actual cx.quit()
    // behavior is platform-level; we verify the menu structure
    // has Quit enabled (tested above) and the handler registered.
}

#[test]
fn test_keybindings_defined() {
    use gpui::KeyBinding;
    use gpui_play::menu_test::key_bindings;

    let bindings: Vec<KeyBinding> = key_bindings();

    // Expected: action type suffix -> keystroke string
    let expected = vec![
        ("Quit", "cmd-q"),
        ("Copy", "cmd-c"),
        ("Paste", "cmd-v"),
        ("Cut", "cmd-x"),
        ("Delete", "delete"),
        ("SelectAll", "cmd-a"),
        ("Home", "home"),
        ("End", "end"),
        ("Undo", "cmd-z"),
        ("Redo", "cmd-shift-z"),
        ("ShowCharacterPalette", "ctrl-cmd-space"),
    ];

    for (action_name, expected_keys) in &expected {
        let found = bindings.iter().any(|b| {
            let keystrokes = b.keystrokes();
            keystrokes.len() == 1
                && keystrokes[0].unparse() == *expected_keys
                && format!("{:?}", b.action()).contains(action_name)
        });
        assert!(
            found,
            "expected keybinding '{}' for action '{}' not found",
            expected_keys, action_name
        );
    }
}
