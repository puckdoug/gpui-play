use gpui::{OwnedMenu, OwnedMenuItem};
use gpui_play::draw_test::menus;

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

// -- Menu structure --

#[test]
fn test_creates_four_menus() {
    let menus = owned_menus();
    assert_eq!(menus.len(), 5, "expected 5 top-level menus");
    assert_eq!(menus[0].name.as_ref(), "DrawTest");
    assert_eq!(menus[1].name.as_ref(), "File");
    assert_eq!(menus[2].name.as_ref(), "Edit");
    assert_eq!(menus[3].name.as_ref(), "Shapes");
    assert_eq!(menus[4].name.as_ref(), "Help");
}

#[test]
fn test_drawtest_menu_has_about() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "DrawTest");
    let names = action_names(menu);
    assert!(
        names.contains(&"About DrawTest"),
        "DrawTest menu should contain 'About DrawTest', got: {:?}",
        names
    );
}

#[test]
fn test_file_menu_has_new_close_quit() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "File");
    let names = action_names(menu);
    assert!(
        names.contains(&"New Window"),
        "File menu should contain 'New Window', got: {:?}",
        names
    );
    assert!(
        names.contains(&"Close Window"),
        "File menu should contain 'Close Window', got: {:?}",
        names
    );
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
    let expected = vec!["Undo", "Redo", "Cut", "Copy", "Paste", "Select All"];
    assert_eq!(
        names, expected,
        "Edit menu should contain {:?}, got: {:?}",
        expected, names
    );
}

#[test]
fn test_shapes_menu_has_new_oval() {
    let menus = owned_menus();
    let menu = find_menu(&menus, "Shapes");
    let names = action_names(menu);
    assert!(
        names.contains(&"New Oval"),
        "Shapes menu should contain 'New Oval', got: {:?}",
        names
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
        ("DrawTest", "About DrawTest"),
        ("File", "New Window"),
        ("File", "Close Window"),
        ("File", "Quit"),
        ("Edit", "Undo"),
        ("Edit", "Redo"),
        ("Shapes", "New Oval"),
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
        ("Edit", "Cut"),
        ("Edit", "Copy"),
        ("Edit", "Paste"),
        ("Edit", "Select All"),
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
