use gpui::{
    div, Context, IntoElement, ParentElement, Render, Styled, StyledText, TestAppContext,
    TextOverflow, Window,
};

// -- TextOverflow --

struct TruncatedTextView;

impl Render for TruncatedTextView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_32()
            .child(StyledText::new("This is a very long text that should be truncated"))
    }
}

#[gpui::test]
fn test_truncated_text_view_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| TruncatedTextView);
}

#[test]
fn test_text_overflow_truncate_end() {
    let overflow = TextOverflow::Truncate("…".into());
    match overflow {
        TextOverflow::Truncate(s) => assert_eq!(s.as_ref(), "…"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn test_text_overflow_truncate_start() {
    let overflow = TextOverflow::TruncateStart("…".into());
    match overflow {
        TextOverflow::TruncateStart(s) => assert_eq!(s.as_ref(), "…"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn test_text_overflow_custom_ellipsis() {
    let _overflow = TextOverflow::Truncate("...".into());
    let _overflow2 = TextOverflow::Truncate(">>".into());
    // Any string can be used as the truncation indicator
}
