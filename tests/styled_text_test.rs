use gpui::{
    div, Context, FontWeight, HighlightStyle, InteractiveText, IntoElement, ParentElement, Render,
    Styled, StyledText, TestAppContext, TextOverflow, Window,
};

struct StyledTextTestView;

impl Render for StyledTextTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(StyledText::new("Hello world"))
    }
}

// -- StyledText construction --

#[gpui::test]
fn test_styled_text_creates_plain(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| StyledTextTestView);
}

#[gpui::test]
fn test_styled_text_with_highlights(cx: &mut TestAppContext) {
    struct HighlightView;
    impl Render for HighlightView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let bold_highlight = HighlightStyle {
                font_weight: Some(FontWeight::BOLD),
                ..Default::default()
            };
            div().child(
                StyledText::new("Hello bold world").with_highlights([(0..5, bold_highlight)]),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| HighlightView);
}

#[gpui::test]
fn test_styled_text_with_default_highlights(cx: &mut TestAppContext) {
    struct DefaultHighlightView;
    impl Render for DefaultHighlightView {
        fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let text_style = window.text_style();
            let color_highlight = HighlightStyle {
                color: Some(gpui::red()),
                ..Default::default()
            };
            div().child(
                StyledText::new("Hello colored world")
                    .with_default_highlights(&text_style, [(6..13, color_highlight)]),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| DefaultHighlightView);
}

#[gpui::test]
fn test_styled_text_multiple_highlight_ranges(cx: &mut TestAppContext) {
    struct MultiHighlightView;
    impl Render for MultiHighlightView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let bold = HighlightStyle {
                font_weight: Some(FontWeight::BOLD),
                ..Default::default()
            };
            let italic = HighlightStyle {
                font_style: Some(gpui::FontStyle::Italic),
                ..Default::default()
            };
            div().child(
                StyledText::new("Bold and italic text")
                    .with_highlights([(0..4, bold), (9..15, italic)]),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| MultiHighlightView);
}

// -- TextOverflow --

#[test]
fn test_text_overflow_truncate_variants_exist() {
    let _end = TextOverflow::Truncate("…".into());
    let _start = TextOverflow::TruncateStart("…".into());
}

// -- InteractiveText --

#[gpui::test]
fn test_interactive_text_with_click_handler(cx: &mut TestAppContext) {
    struct ClickableTextView;
    impl Render for ClickableTextView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let text = StyledText::new("Click here for info");
            div().child(
                InteractiveText::new("clickable", text)
                    .on_click(vec![6..10], |range_ix, _window, _cx| {
                        println!("clicked range {}", range_ix);
                    }),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| ClickableTextView);
}

#[gpui::test]
fn test_interactive_text_multiple_click_ranges(cx: &mut TestAppContext) {
    struct MultiClickTextView;
    impl Render for MultiClickTextView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let text = StyledText::new("Link one and link two");
            div().child(
                InteractiveText::new("multi-click", text)
                    .on_click(vec![0..8, 13..21], |range_ix, _window, _cx| {
                        println!("clicked link {}", range_ix);
                    }),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| MultiClickTextView);
}

// -- HighlightStyle --

#[test]
fn test_highlight_style_default_is_all_none() {
    let style = HighlightStyle::default();
    assert!(style.color.is_none());
    assert!(style.font_weight.is_none());
    assert!(style.font_style.is_none());
    assert!(style.background_color.is_none());
    assert!(style.underline.is_none());
    assert!(style.strikethrough.is_none());
    assert!(style.fade_out.is_none());
}
