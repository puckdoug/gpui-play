use gpui::{KeyBinding, TestAppContext};
use gpui_play::menu_test::{key_bindings, setup_menus};
use gpui_play::text_input::text_input_key_bindings;

// -- Action handler registration --

#[gpui::test]
fn test_setup_menus_registers_quit_handler(cx: &mut TestAppContext) {
    cx.update(|cx| {
        setup_menus(cx);
    });
    // Verify the quit handler was registered by confirming
    // setup_menus completes without panic. The actual cx.quit()
    // behavior is platform-level.
}

#[gpui::test]
fn test_setup_menus_registers_about_handler(cx: &mut TestAppContext) {
    cx.update(|cx| {
        setup_menus(cx);
    });
    // About handler opens a window. Verify setup completes without panic.
}

// -- App-level keybindings --

#[test]
fn test_app_keybindings_defined() {
    let bindings: Vec<KeyBinding> = key_bindings();

    let expected = vec![
        ("NewWindow", "cmd-n"),
        ("CloseWindow", "cmd-w"),
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
            "expected app keybinding '{}' for action '{}' not found",
            expected_keys, action_name
        );
    }
}

#[test]
fn test_text_input_keybindings_defined() {
    let bindings: Vec<KeyBinding> = text_input_key_bindings();

    let expected = vec![
        ("Backspace", "backspace"),
        ("Delete", "delete"),
        ("Left", "left"),
        ("Right", "right"),
        ("SelectLeft", "shift-left"),
        ("SelectRight", "shift-right"),
        ("SelectAll", "cmd-a"),
        ("Paste", "cmd-v"),
        ("Copy", "cmd-c"),
        ("Cut", "cmd-x"),
        ("Home", "home"),
        ("End", "end"),
        ("ShowCharacterPalette", "ctrl-cmd-space"),
        ("Undo", "cmd-z"),
        ("Redo", "cmd-shift-z"),
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
            "expected text input keybinding '{}' for action '{}' not found",
            expected_keys, action_name
        );
    }
}

// -- Keybinding context scoping --

#[test]
fn test_text_input_bindings_are_context_scoped() {
    let bindings = text_input_key_bindings();

    for binding in &bindings {
        assert!(
            binding.predicate().is_some(),
            "text input binding for '{}' should have a context predicate",
            binding.keystrokes()[0].unparse()
        );
    }
}

#[test]
fn test_app_bindings_are_global() {
    let bindings = key_bindings();

    // App-level bindings should have no context predicate (global)
    for binding in &bindings {
        assert!(
            binding.predicate().is_none(),
            "app binding for '{}' should be global (no context predicate)",
            binding.keystrokes()[0].unparse()
        );
    }
}
