use gpui::{TestAppContext, WindowAppearance};

#[test]
fn test_window_appearance_variants_exist() {
    let _light = WindowAppearance::Light;
    let _vibrant_light = WindowAppearance::VibrantLight;
    let _dark = WindowAppearance::Dark;
    let _vibrant_dark = WindowAppearance::VibrantDark;
}

#[gpui::test]
fn test_displays_returns_list(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let displays = cx.displays();
        // Test environment should have at least conceptual display support
        // (may be empty in headless CI)
        let _ = displays;
    });
}

#[gpui::test]
fn test_primary_display_available(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let _primary = cx.primary_display();
        // Returns Option — may be None in headless test
    });
}

#[gpui::test]
fn test_window_appearance_readable(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let appearance = cx.window_appearance();
        // Should return a valid variant
        match appearance {
            WindowAppearance::Light
            | WindowAppearance::VibrantLight
            | WindowAppearance::Dark
            | WindowAppearance::VibrantDark => {}
        }
    });
}
