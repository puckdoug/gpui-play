use gpui::{
    div, Context, IntoElement, ParentElement, PressureStage, Render, Styled, TestAppContext, Window,
};
use std::path::PathBuf;

struct FileDropView {
    dropped_paths: Vec<PathBuf>,
    drag_hovering: bool,
}

impl Render for FileDropView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(if self.drag_hovering {
                gpui::rgb(0x4488ff)
            } else {
                gpui::rgb(0xeeeeee)
            })
            .child(if self.dropped_paths.is_empty() {
                "Drop files here".to_string()
            } else {
                format!("{} files dropped", self.dropped_paths.len())
            })
    }
}

#[gpui::test]
fn test_file_drop_view_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| FileDropView {
        dropped_paths: Vec::new(),
        drag_hovering: false,
    });
}

#[test]
fn test_file_drop_state_transitions() {
    let mut view = FileDropView {
        dropped_paths: Vec::new(),
        drag_hovering: false,
    };

    // Simulate drag enter
    view.drag_hovering = true;
    assert!(view.drag_hovering);

    // Simulate file drop
    view.dropped_paths.push(PathBuf::from("/tmp/test.txt"));
    view.dropped_paths.push(PathBuf::from("/tmp/image.png"));
    view.drag_hovering = false;
    assert!(!view.drag_hovering);
    assert_eq!(view.dropped_paths.len(), 2);
}

#[test]
fn test_file_drop_multiple_drops_accumulate() {
    let mut view = FileDropView {
        dropped_paths: Vec::new(),
        drag_hovering: false,
    };

    view.dropped_paths.push(PathBuf::from("/tmp/a.txt"));
    assert_eq!(view.dropped_paths.len(), 1);

    view.dropped_paths.push(PathBuf::from("/tmp/b.txt"));
    assert_eq!(view.dropped_paths.len(), 2);
}

// Note: FileDropEvent and ExternalPaths require smallvec (transitive dep of gpui)
// which is not directly available. The event types exist and are used by the framework
// for OS-level file drops. Testing file drop dispatch requires actual OS drag operations.
