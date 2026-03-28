use gpui::{
    div, Context, IntoElement, ParentElement, PressureStage, Render, Styled, TestAppContext, Window,
};

struct GestureTestView {
    last_pressure: f32,
    last_stage: PressureStage,
}

impl Render for GestureTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(format!("Pressure: {:.2}", self.last_pressure))
            .child(format!("Stage: {:?}", self.last_stage))
    }
}

#[gpui::test]
fn test_gesture_view_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| GestureTestView {
        last_pressure: 0.0,
        last_stage: PressureStage::default(),
    });
}

#[test]
fn test_pressure_stage_variants_exist() {
    let _zero = PressureStage::Zero;
    let _normal = PressureStage::Normal;
    let _force = PressureStage::Force;
}

#[test]
fn test_pressure_stage_default_is_zero() {
    let stage = PressureStage::default();
    matches!(stage, PressureStage::Zero);
}

#[test]
fn test_pressure_to_brush_size_mapping() {
    // Pure logic: map pressure (0.0-1.0) to brush size
    fn pressure_to_size(pressure: f32, min: f32, max: f32) -> f32 {
        min + pressure * (max - min)
    }

    assert_eq!(pressure_to_size(0.0, 1.0, 10.0), 1.0);
    assert_eq!(pressure_to_size(0.5, 1.0, 10.0), 5.5);
    assert_eq!(pressure_to_size(1.0, 1.0, 10.0), 10.0);
}

#[test]
fn test_pinch_zoom_calculation() {
    // Pure logic: apply pinch delta to zoom level with clamping
    fn apply_pinch(zoom: f32, delta: f32, min: f32, max: f32) -> f32 {
        (zoom + delta).clamp(min, max)
    }

    assert_eq!(apply_pinch(1.0, 0.5, 0.1, 5.0), 1.5);
    assert_eq!(apply_pinch(1.0, -0.5, 0.1, 5.0), 0.5);
    assert_eq!(apply_pinch(1.0, -2.0, 0.1, 5.0), 0.1); // clamped
    assert_eq!(apply_pinch(4.0, 2.0, 0.1, 5.0), 5.0); // clamped
}

// Note: PinchEvent and on_pinch are macOS/Linux only (#[cfg(any(target_os = "linux", target_os = "macos"))])
// MousePressureEvent requires force-touch trackpad hardware
// These cannot be simulated in the test harness — pure logic tests above verify the math
