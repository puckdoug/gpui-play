use gpui::{TestAppContext, ThermalState};

#[test]
fn test_thermal_state_variants_exist() {
    let _nominal = ThermalState::Nominal;
    let _fair = ThermalState::Fair;
    let _serious = ThermalState::Serious;
    let _critical = ThermalState::Critical;
}

#[gpui::test]
fn test_thermal_state_readable(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let state = cx.thermal_state();
        match state {
            ThermalState::Nominal
            | ThermalState::Fair
            | ThermalState::Serious
            | ThermalState::Critical => {}
        }
    });
}

#[gpui::test]
fn test_keyboard_layout_available(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let _layout = cx.keyboard_layout();
        // Returns &dyn PlatformKeyboardLayout
    });
}
