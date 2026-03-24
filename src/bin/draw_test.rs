use std::ops::Range;

use gpui::{
    actions, App, Bounds, Context, CursorStyle, ElementInputHandler, EntityInputHandler,
    FocusHandle, Focusable, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PathBuilder, Pixels, Point, Render, SharedString, TextAlign, TextRun, UTF16Selection, Window,
    WindowOptions, canvas, div, point, prelude::*, px, rgb, size,
};
use gpui_platform::application;

use gpui_play::draw_test::{self, setup_menus};
use gpui_play::shape::CanvasState;
use gpui_play::text_input::TextInputState;

actions!(draw_test_canvas, [StopEditing, Backspace, Delete]);

struct DrawTestView {
    focus_handle: FocusHandle,
    canvas_state: CanvasState,
    dragging: bool,
    drag_offset: Option<(f32, f32)>,
    editing_state: Option<TextInputState>,
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
        if self.editing_state.is_some() {
            // When editing, undo text changes
            if let Some(ref mut state) = self.editing_state {
                state.undo();
            }
        } else {
            self.canvas_state.undo();
        }
        cx.notify();
    }

    fn redo(&mut self, _: &draw_test::Redo, _window: &mut Window, cx: &mut Context<Self>) {
        if self.editing_state.is_some() {
            if let Some(ref mut state) = self.editing_state {
                state.redo();
            }
        } else {
            self.canvas_state.redo();
        }
        cx.notify();
    }

    fn start_editing(&mut self, index: usize) {
        let text = self.canvas_state.shapes()[index].text().to_string();
        let mut state = TextInputState::new(&text);
        state.move_to_end();
        self.editing_state = Some(state);
        self.canvas_state.start_editing(index);
    }

    fn commit_editing(&mut self) {
        if let (Some(index), Some(state)) =
            (self.canvas_state.editing(), &self.editing_state)
        {
            let text = state.content().to_string();
            self.canvas_state.set_shape_text(index, &text);
        }
        self.editing_state = None;
        self.canvas_state.stop_editing();
    }

    fn on_stop_editing(
        &mut self,
        _: &StopEditing,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.canvas_state.editing().is_some() {
            self.commit_editing();
            cx.notify();
        }
    }

    fn on_backspace(&mut self, _: &Backspace, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.backspace();
            cx.notify();
        }
    }

    fn on_delete(&mut self, _: &Delete, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.delete();
            cx.notify();
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mx = px_to_f32(event.position.x);
        let my = px_to_f32(event.position.y);

        if event.click_count == 2 {
            // Double-click: enter or re-enter editing if clicking on a shape
            self.canvas_state.select_at(mx, my);
            if let Some(idx) = self.canvas_state.selected() {
                // Commit any prior editing before starting fresh
                if self.canvas_state.editing().is_some() {
                    self.commit_editing();
                }
                self.start_editing(idx);
                self.dragging = false;
                self.drag_offset = None;
                cx.notify();
                return;
            }
        }

        // If currently editing, check what we clicked
        if self.canvas_state.editing().is_some() {
            // Clicking the shape being edited — stay in editing mode, don't drag
            let editing_idx = self.canvas_state.editing().unwrap();
            if self.canvas_state.shapes()[editing_idx].contains_point(mx, my) {
                return;
            }
            // Clicked elsewhere — commit and exit editing
            self.commit_editing();
        }

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

impl EntityInputHandler for DrawTestView {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let state = self.editing_state.as_ref()?;
        let range = state.range_from_utf16(&range_utf16);
        actual_range.replace(state.range_to_utf16(&range));
        Some(state.content()[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let state = self.editing_state.as_ref()?;
        Some(UTF16Selection {
            range: state.range_to_utf16(&state.selected_range()),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ref mut state) = self.editing_state {
            let range = range_utf16
                .as_ref()
                .map(|r| state.range_from_utf16(r))
                .unwrap_or_else(|| state.selected_range());
            state.replace_range(range, new_text);
            cx.notify();
        }
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Simplified: just replace, don't handle marked text for now
        if let Some(ref mut state) = self.editing_state {
            let range = range_utf16
                .as_ref()
                .map(|r| state.range_from_utf16(r))
                .unwrap_or_else(|| state.selected_range());
            state.replace_range(range, new_text);
            cx.notify();
        }
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
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
    text_box_width: f32,
    selected: bool,
    text: String,
}

impl Render for DrawTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_editing = self.canvas_state.editing().is_some();
        let editing_text = self
            .editing_state
            .as_ref()
            .map(|s| s.content().to_string());
        let editing_index = self.canvas_state.editing();

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
                    text_box_width: s.text_box_width(),
                    selected: self.canvas_state.selected() == Some(i),
                    text: if editing_index == Some(i) {
                        editing_text.clone().unwrap_or_default()
                    } else {
                        s.text().to_string()
                    },
                }
            })
            .collect();

        let entity = cx.entity().clone();
        let focus = self.focus_handle.clone();
        let cursor = if is_editing {
            CursorStyle::IBeam
        } else {
            CursorStyle::Arrow
        };

        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .cursor(cursor)
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::close_window))
            .on_action(cx.listener(Self::new_oval))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .on_action(cx.listener(Self::on_stop_editing))
            .on_action(cx.listener(Self::on_backspace))
            .on_action(cx.listener(Self::on_delete))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(
                canvas(
                    move |_bounds, _window, _cx| {},
                    move |bounds, _, window, cx| {
                        if is_editing {
                            window.handle_input(
                                &focus,
                                ElementInputHandler::new(bounds, entity.clone()),
                                cx,
                            );
                        }
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
                                let run = TextRun {
                                    len: shape.text.len(),
                                    font: style.font(),
                                    color: style.color,
                                    background_color: None,
                                    underline: None,
                                    strikethrough: None,
                                };
                                let display_text: SharedString = shape.text.clone().into();
                                let wrap_width = px(shape.text_box_width);

                                if let Ok(lines) = window.text_system().shape_text(
                                    display_text,
                                    font_size,
                                    &[run],
                                    Some(wrap_width),
                                    None,
                                ) {
                                    let line_height = window.line_height();
                                    let total_height: Pixels = lines
                                        .iter()
                                        .map(|l| l.size(line_height).height)
                                        .sum();

                                    let text_origin = point(
                                        center.x - wrap_width / 2.0,
                                        center.y - total_height / 2.0,
                                    );

                                    let mut y = text_origin.y;
                                    let text_bounds = Bounds::new(
                                        text_origin,
                                        size(wrap_width, total_height),
                                    );
                                    for line in &lines {
                                        let line_origin = point(text_origin.x, y);
                                        line.paint(
                                            line_origin,
                                            line_height,
                                            TextAlign::Center,
                                            Some(text_bounds),
                                            window,
                                            cx,
                                        )
                                        .ok();
                                        y += line.size(line_height).height;
                                    }
                                }
                            }
                        }
                    },
                )
                .size_full(),
            )
    }
}

fn editing_key_bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("escape", StopEditing, None),
        KeyBinding::new("backspace", Backspace, None),
        KeyBinding::new("delete", Delete, None),
    ]
}

fn open_draw_window(cx: &mut App) {
    let window = cx
        .open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| DrawTestView {
                focus_handle: cx.focus_handle(),
                canvas_state: CanvasState::new(),
                dragging: false,
                drag_offset: None,
                editing_state: None,
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
        cx.bind_keys(editing_key_bindings());
        setup_menus(cx);

        cx.on_action(|_: &draw_test::NewWindow, cx: &mut App| {
            open_draw_window(cx);
        });

        open_draw_window(cx);
    });
}
