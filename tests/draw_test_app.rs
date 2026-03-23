use gpui::{KeyBinding, TestAppContext};
use gpui_play::draw_test::{key_bindings, setup_menus};

// -- Action handler registration --

#[gpui::test]
fn test_setup_menus_registers_handlers(cx: &mut TestAppContext) {
    cx.update(|cx| {
        setup_menus(cx);
    });
}

// -- App-level keybindings --

#[test]
fn test_app_keybindings_defined() {
    let bindings: Vec<KeyBinding> = key_bindings();

    let expected = vec![
        ("NewWindow", "cmd-n"),
        ("CloseWindow", "cmd-w"),
        ("Quit", "cmd-q"),
        ("Undo", "cmd-z"),
        ("Redo", "cmd-shift-z"),
        ("Cut", "cmd-x"),
        ("Copy", "cmd-c"),
        ("Paste", "cmd-v"),
        ("SelectAll", "cmd-a"),
        ("NewOval", "cmd-shift-n"),
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
            "expected app keybinding '{}' for action '{}' not found",
            expected_keys, action_name
        );
    }
}

#[test]
fn test_app_bindings_are_global() {
    let bindings = key_bindings();

    for binding in &bindings {
        assert!(
            binding.predicate().is_none(),
            "app binding for '{}' should be global (no context predicate)",
            binding.keystrokes()[0].unparse()
        );
    }
}
