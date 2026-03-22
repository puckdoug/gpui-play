use gpui_play::menu_test::{about_version_string, about_window_options};

#[test]
fn test_about_window_not_minimizable() {
    let opts = about_window_options();
    assert!(!opts.is_minimizable, "About window should not be minimizable");
}

#[test]
fn test_about_window_not_resizable() {
    let opts = about_window_options();
    assert!(!opts.is_resizable, "About window should not be resizable");
}

#[test]
fn test_about_version_string_format() {
    let version = about_version_string();
    assert!(
        version.starts_with("MenuTest: "),
        "version string should start with 'MenuTest: ', got: '{}'",
        version
    );
    let version_part = version.strip_prefix("MenuTest: ").unwrap();
    assert!(!version_part.is_empty(), "version should not be empty");
    assert!(
        version_part.contains('.'),
        "version should contain a dot (semver), got: '{}'",
        version_part
    );
}
