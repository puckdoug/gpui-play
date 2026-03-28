use gpui::{
    div, list, px, uniform_list, Context, InteractiveElement, IntoElement, ListAlignment,
    ListState, ParentElement, Render, ScrollStrategy, Styled, TestAppContext,
    UniformListScrollHandle, Window,
};

// -- UniformList --

struct UniformListView {
    scroll_handle: UniformListScrollHandle,
}

impl Render for UniformListView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            uniform_list("test-list", 1000, |range, _window, _cx| {
                range
                    .map(|ix| div().id(ix).child(format!("Item {}", ix)))
                    .collect()
            })
            .track_scroll(&self.scroll_handle)
            .h_full(),
        )
    }
}

#[gpui::test]
fn test_uniform_list_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| UniformListView {
        scroll_handle: UniformListScrollHandle::new(),
    });
}

#[gpui::test]
fn test_uniform_list_with_10k_items(cx: &mut TestAppContext) {
    struct BigListView;
    impl Render for BigListView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                uniform_list("big-list", 10_000, |range, _window, _cx| {
                    range
                        .map(|ix| div().id(ix).child(format!("Row {}", ix)))
                        .collect()
                })
                .h_full(),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| BigListView);
}

#[gpui::test]
fn test_uniform_list_scroll_handle(cx: &mut TestAppContext) {
    let handle = UniformListScrollHandle::new();
    let handle_clone = handle.clone();

    let _window = cx.add_window(move |_window, _cx| UniformListView {
        scroll_handle: handle_clone,
    });

    // Scroll to item 500
    handle.scroll_to_item(500, ScrollStrategy::Center);
}

// -- List (variable height) --

struct VariableListView {
    list_state: ListState,
}

impl Render for VariableListView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.list_state.clone();
        div().size_full().child(
            list(state, |ix, _window, _cx| {
                if ix % 3 == 0 {
                    div()
                        .h(px(60.0))
                        .child(format!("Tall item {}", ix))
                        .into_any_element()
                } else {
                    div()
                        .h(px(30.0))
                        .child(format!("Item {}", ix))
                        .into_any_element()
                }
            })
            .h_full(),
        )
    }
}

#[gpui::test]
fn test_variable_list_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| VariableListView {
        list_state: ListState::new(100, ListAlignment::Top, px(50.0)),
    });
}

#[gpui::test]
fn test_list_state_item_count(_cx: &mut TestAppContext) {
    let state = ListState::new(42, ListAlignment::Top, px(50.0));
    assert_eq!(state.item_count(), 42);
}

#[gpui::test]
fn test_list_state_reset(_cx: &mut TestAppContext) {
    let state = ListState::new(100, ListAlignment::Top, px(50.0));
    assert_eq!(state.item_count(), 100);
    state.reset(50);
    assert_eq!(state.item_count(), 50);
}

#[gpui::test]
fn test_list_alignment_bottom(cx: &mut TestAppContext) {
    struct BottomListView {
        list_state: ListState,
    }
    impl Render for BottomListView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let state = self.list_state.clone();
            div().size_full().child(
                list(state, |ix, _window, _cx| {
                    div().child(format!("Msg {}", ix)).into_any_element()
                })
                .h_full(),
            )
        }
    }
    let _window = cx.add_window(|_window, _cx| BottomListView {
        list_state: ListState::new(50, ListAlignment::Bottom, px(50.0)),
    });
}
