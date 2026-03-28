use gpui::{
    div, Context, IntoElement, ParentElement, Render, Styled, TestAppContext, Window,
};

// -- Holy grail grid layout --

struct GridTestView;

impl Render for GridTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .grid()
            .grid_cols(3)
            .grid_rows(3)
            .gap_2()
            .child(div().col_span_full().bg(gpui::blue()).child("Header"))
            .child(div().row_span(1).bg(gpui::rgb(0x888888)).child("Sidebar"))
            .child(div().col_span(2).bg(gpui::white()).child("Content"))
            .child(div().col_span_full().bg(gpui::blue()).child("Footer"))
    }
}

#[gpui::test]
fn test_grid_layout_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| GridTestView);
}

#[gpui::test]
fn test_grid_with_explicit_cols_and_rows(cx: &mut TestAppContext) {
    struct ExplicitGridView;
    impl Render for ExplicitGridView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .grid()
                .grid_cols(5)
                .grid_rows(5)
                .child(div().col_span(3).row_span(2).child("Wide cell"))
                .child(div().child("Auto-placed"))
        }
    }
    let _window = cx.add_window(|_window, _cx| ExplicitGridView);
}

#[gpui::test]
fn test_grid_with_col_start_end(cx: &mut TestAppContext) {
    struct PlacedGridView;
    impl Render for PlacedGridView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .grid()
                .grid_cols(4)
                .grid_rows(2)
                .child(div().col_start(1).col_end(3).child("Cols 1-2"))
                .child(div().col_start(3).col_end(5).child("Cols 3-4"))
                .child(div().row_start(2).col_span_full().child("Full row 2"))
        }
    }
    let _window = cx.add_window(|_window, _cx| PlacedGridView);
}

#[gpui::test]
fn test_grid_with_gap(cx: &mut TestAppContext) {
    struct GapGridView;
    impl Render for GapGridView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .grid()
                .grid_cols(3)
                .gap_4()
                .child(div().child("A"))
                .child(div().child("B"))
                .child(div().child("C"))
        }
    }
    let _window = cx.add_window(|_window, _cx| GapGridView);
}

#[gpui::test]
fn test_grid_col_span_full(cx: &mut TestAppContext) {
    struct FullSpanView;
    impl Render for FullSpanView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .grid()
                .grid_cols(4)
                .child(div().col_span_full().child("Full width"))
                .child(div().col_span(2).child("Half"))
                .child(div().col_span(2).child("Half"))
        }
    }
    let _window = cx.add_window(|_window, _cx| FullSpanView);
}
