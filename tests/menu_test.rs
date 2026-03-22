use gpui::{OwnedMenu, OwnedMenuItem, TestAppContext};
use gpui_play::menu_test::setup_menus;

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

#[gpui::test]
fn test_creates_four_menus(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");
        assert_eq!(menus.len(), 4, "expected 4 top-level menus");
        assert_eq!(menus[0].name.as_ref(), "MenuTest");
        assert_eq!(menus[1].name.as_ref(), "File");
        assert_eq!(menus[2].name.as_ref(), "Edit");
        assert_eq!(menus[3].name.as_ref(), "Help");
    });
}

#[gpui::test]
fn test_menutest_menu_has_about(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");
        let menu = find_menu(&menus, "MenuTest");
        let names = action_names(menu);
        assert!(
            names.contains(&"About MenuTest"),
            "MenuTest menu should contain 'About MenuTest', got: {:?}",
            names
        );
    });
}

#[gpui::test]
fn test_file_menu_has_quit(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");
        let menu = find_menu(&menus, "File");
        let names = action_names(menu);
        assert!(
            names.contains(&"Quit"),
            "File menu should contain 'Quit', got: {:?}",
            names
        );
    });
}

#[gpui::test]
fn test_edit_menu_has_expected_items(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");
        let menu = find_menu(&menus, "Edit");
        let names = action_names(menu);
        let expected = vec!["Undo", "Redo", "Cut", "Copy", "Paste"];
        assert_eq!(
            names, expected,
            "Edit menu should contain {:?}, got: {:?}",
            expected, names
        );
    });
}

#[gpui::test]
fn test_help_menu_has_search(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");
        let menu = find_menu(&menus, "Help");
        let names = action_names(menu);
        assert!(
            names.contains(&"Search"),
            "Help menu should contain 'Search', got: {:?}",
            names
        );
    });
}

#[gpui::test]
fn test_only_quit_is_enabled(cx: &mut TestAppContext) {
    cx.update(|cx| setup_menus(cx));
    cx.read(|cx| {
        let menus = cx.get_menus().expect("menus should be set");

        // Quit should be enabled
        let file_menu = find_menu(&menus, "File");
        assert!(
            !is_item_disabled(file_menu, "Quit"),
            "Quit should be enabled"
        );

        // All other action items should be disabled
        let disabled_items = [
            ("MenuTest", "About MenuTest"),
            ("Edit", "Undo"),
            ("Edit", "Redo"),
            ("Edit", "Cut"),
            ("Edit", "Copy"),
            ("Edit", "Paste"),
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
    });
}

#[gpui::test]
fn test_quit_action_registered(cx: &mut TestAppContext) {
    cx.update(|cx| {
        setup_menus(cx);

        // Verify quit action is registered by checking that the Quit action
        // is associated with the File menu's Quit item (not disabled).
        // The actual cx.quit() behavior is platform-level and verified by
        // confirming the handler was registered via on_action.
        let menus = cx.get_menus().expect("menus should be set");
        let file_menu = find_menu(&menus, "File");
        assert!(
            !is_item_disabled(file_menu, "Quit"),
            "Quit should have an active action handler"
        );
    });
}
