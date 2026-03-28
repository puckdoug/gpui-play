use gpui::{
    div, AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, TestAppContext, Window,
};

// Drag payload type
#[derive(Clone)]
struct CardPayload {
    id: usize,
    title: String,
}

// Drag preview view
struct DragPreview {
    title: String,
}

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::blue())
            .text_color(gpui::white())
            .p_2()
            .child(self.title.clone())
    }
}

// Main view with drag and drop
struct DragDropView;

impl Render for DragDropView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .gap_4()
            // Draggable card
            .child(
                div()
                    .id("card-1")
                    .bg(gpui::white())
                    .p_4()
                    .child("Drag me")
                    .on_drag(
                        CardPayload { id: 1, title: "Card 1".into() },
                        |payload: &CardPayload, _position, _window, cx: &mut gpui::App| {
                            cx.new(|_cx| DragPreview {
                                title: payload.title.clone(),
                            })
                        },
                    ),
            )
            // Drop zone
            .child(
                div()
                    .id("drop-zone")
                    .bg(gpui::rgb(0xeeeeee))
                    .size_full()
                    .child("Drop here")
                    .on_drop::<CardPayload>(cx.listener(
                        |_this: &mut Self, payload: &CardPayload, _window, _cx| {
                            println!("Dropped card: {}", payload.title);
                        },
                    )),
            )
    }
}

#[gpui::test]
fn test_drag_drop_view_renders(cx: &mut TestAppContext) {
    let _window = cx.add_window(|_window, _cx| DragDropView);
}

#[gpui::test]
fn test_drag_payload_construction(_cx: &mut TestAppContext) {
    let payload = CardPayload {
        id: 42,
        title: "Test card".to_string(),
    };
    assert_eq!(payload.id, 42);
    assert_eq!(payload.title, "Test card");
}

#[gpui::test]
fn test_drag_move_handler(cx: &mut TestAppContext) {
    struct DragMoveView;
    impl Render for DragMoveView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("draggable")
                .size_20()
                .on_drag(
                    CardPayload { id: 1, title: "Card".into() },
                    |payload: &CardPayload, _pos, _window, cx: &mut gpui::App| {
                        cx.new(|_cx| DragPreview { title: payload.title.clone() })
                    },
                )
                .on_drag_move::<CardPayload>(|event, _window, _cx| {
                    let _position = event.event.position;
                })
        }
    }
    let _window = cx.add_window(|_window, _cx| DragMoveView);
}
