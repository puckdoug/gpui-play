use gpui::{
    App, Context, FocusHandle, Focusable, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PathBuilder, Pixels, Render, SharedString, Window, WindowOptions, canvas, div,
    point, prelude::*, px, rgb,
};
use gpui_platform::application;

use gpui_play::draw_test::{self, setup_menus};
use gpui_play::shape::CanvasState;

struct DrawTestView {
    focus_handle: FocusHandle,
    canvas_state: CanvasState,
    dragging: bool,
    drag_offset: Option<(f32, f32)>,
}

fn px_to_f32(p: Pixels) -> f32 {
    f32::from(p)
}

impl DrawTestView {
    fn close_window(
        &mut self,
        _: &draw_test::CloseWindow,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        window.remove_window();
    }

    fn new_oval(&mut self, _: &draw_test::NewOval, window: &mut Window, cx: &mut Context<Self>) {
        let bounds = window.bounds();
        let center_x = px_to_f32(bounds.size.width) / 2.0;
        let center_y = px_to_f32(bounds.size.height) / 2.0;
        self.canvas_state.add_oval(center_x, center_y);
        cx.notify();
    }

    fn undo(&mut self, _: &draw_test::Undo, _window: &mut Window, cx: &mut Context<Self>) {
        self.canvas_state.undo();
        cx.notify();
    }

    fn redo(&mut self, _: &draw_test::Redo, _window: &mut Window, cx: &mut Context<Self>) {
        self.canvas_state.redo();
        cx.notify();
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mx = px_to_f32(event.position.x);
        let my = px_to_f32(event.position.y);

        self.canvas_state.select_at(mx, my);

        if let Some(idx) = self.canvas_state.selected() {
            let (shape_cx, shape_cy) = self.canvas_state.shapes()[idx].center();
            self.dragging = true;
            self.drag_offset = Some((mx - shape_cx, my - shape_cy));
        } else {
            self.dragging = false;
            self.drag_offset = None;
        }
        cx.notify();
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging
            && let Some((offset_x, offset_y)) = self.drag_offset
        {
            let new_cx = px_to_f32(event.position.x) - offset_x;
            let new_cy = px_to_f32(event.position.y) - offset_y;
            self.canvas_state.move_selected(new_cx, new_cy);
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.dragging = false;
        self.drag_offset = None;
    }
}

impl Focusable for DrawTestView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Shape data extracted for the canvas paint closure (must be 'static).
#[derive(Clone)]
struct ShapeRenderData {
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    border_width: f32,
    selected: bool,
    text: String,
}

impl Render for DrawTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let shapes: Vec<ShapeRenderData> = self
            .canvas_state
            .shapes()
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let (cx, cy) = s.center();
                ShapeRenderData {
                    cx,
                    cy,
                    rx: s.rx(),
                    ry: s.ry(),
                    border_width: s.border_width(),
                    selected: self.canvas_state.selected() == Some(i),
                    text: s.text().to_string(),
                }
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::close_window))
            .on_action(cx.listener(Self::new_oval))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(
                canvas(
                    move |_bounds, _window, _cx| {},
                    move |_bounds, _, window, cx| {
                        for shape in &shapes {
                            let center = point(px(shape.cx), px(shape.cy));
                            let radii = point(px(shape.rx), px(shape.ry));
                            let right = point(center.x + px(shape.rx), center.y);
                            let left = point(center.x - px(shape.rx), center.y);

                            let stroke_width = if shape.selected {
                                px(2.0)
                            } else {
                                px(shape.border_width)
                            };

                            let mut builder = PathBuilder::stroke(stroke_width);
                            builder.move_to(right);
                            builder.arc_to(radii, px(0.0), false, true, left);
                            builder.arc_to(radii, px(0.0), false, true, right);

                            if let Ok(path) = builder.build() {
                                let color = if shape.selected {
                                    rgb(0x4488ff)
                                } else {
                                    rgb(0x000000)
                                };
                                window.paint_path(path, color);
                            }

                            if !shape.text.is_empty() {
                                let style = window.text_style();
                                let font_size = style.font_size.to_pixels(window.rem_size());
                                let run = gpui::TextRun {
                                    len: shape.text.len(),
                                    font: style.font(),
                                    color: style.color,
                                    background_color: None,
                                    underline: None,
                                    strikethrough: None,
                                };
                                let display_text: SharedString =
                                    shape.text.clone().into();
                                let shaped = window
                                    .text_system()
                                    .shape_line(display_text, font_size, &[run], None);
                                let text_width = shaped.width();
                                let line_height = window.line_height();
                                let text_origin = point(
                                    center.x - text_width / 2.0,
                                    center.y - line_height / 2.0,
                                );
                                shaped
                                    .paint(
                                        text_origin,
                                        line_height,
                                        gpui::TextAlign::Left,
                                        None,
                                        window,
                                        cx,
                                    )
                                    .ok();
                            }
                        }
                    },
                )
                .size_full(),
            )
    }
}

fn open_draw_window(cx: &mut App) {
    let window = cx
        .open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| DrawTestView {
                focus_handle: cx.focus_handle(),
                canvas_state: CanvasState::new(),
                dragging: false,
                drag_offset: None,
            })
        })
        .unwrap();

    window
        .update(cx, |view, window, cx| {
            window.focus(&view.focus_handle(cx), cx);
        })
        .unwrap();
}

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        setup_menus(cx);

        cx.on_action(|_: &draw_test::NewWindow, cx: &mut App| {
            open_draw_window(cx);
        });

        open_draw_window(cx);
    });
}
