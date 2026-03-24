use std::ops::Range;
use std::time::Duration;

use gpui::{
    actions, App, Bounds, Context, CursorStyle, ElementInputHandler, EntityInputHandler,
    FocusHandle, Focusable, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PathBuilder, Pixels, Point, Render, SharedString, TextAlign, TextRun, UTF16Selection, Window,
    WindowOptions, canvas, div, fill, point, prelude::*, px, rgb, size,
};
use gpui_platform::application;

use gpui_play::draw_test::{self, setup_menus};
use gpui_play::shape::CanvasState;
use gpui_play::text_input::TextInputState;

actions!(draw_test_canvas, [StopEditing, Backspace, Delete]);

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

struct DrawTestView {
    focus_handle: FocusHandle,
    canvas_state: CanvasState,
    dragging: bool,
    drag_offset: Option<(f32, f32)>,
    editing_state: Option<TextInputState>,
    cursor_visible: bool,
    blink_epoch: usize,
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

    fn start_editing(&mut self, index: usize, cx: &mut Context<Self>) {
        let text = self.canvas_state.shapes()[index].text().to_string();
        let mut state = TextInputState::new(&text);
        state.move_to_end();
        self.editing_state = Some(state);
        self.canvas_state.start_editing(index);
        self.show_cursor(cx);
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
        self.blink_epoch += 1; // cancel any pending blink task
    }

    /// Show cursor and restart blink cycle (call after user input).
    fn show_cursor(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self.blink_epoch += 1;
        let epoch = self.blink_epoch;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(CURSOR_BLINK_INTERVAL).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.blink_cursor(epoch, cx));
            }
        })
        .detach();
        cx.notify();
    }

    fn blink_cursor(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch != self.blink_epoch || self.editing_state.is_none() {
            return; // stale epoch or no longer editing
        }
        self.cursor_visible = !self.cursor_visible;
        cx.notify();

        let interval = CURSOR_BLINK_INTERVAL;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(interval).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.blink_cursor(epoch, cx));
            }
        })
        .detach();
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
            self.show_cursor(cx);
        }
    }

    fn on_delete(&mut self, _: &Delete, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.delete();
            self.show_cursor(cx);
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
                self.start_editing(idx, cx);
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
            self.show_cursor(cx);
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
        if let Some(ref mut state) = self.editing_state {
            let range = range_utf16
                .as_ref()
                .map(|r| state.range_from_utf16(r))
                .unwrap_or_else(|| state.selected_range());
            state.replace_range(range, new_text);
            self.show_cursor(cx);
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

impl Render for DrawTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_editing = self.canvas_state.editing().is_some();
        let shapes = self
            .canvas_state
            .render_data(self.editing_state.as_ref());
        let cursor_visible = self.cursor_visible;

        let entity = cx.entity().clone();
        let focus = self.focus_handle.clone();
        let cursor_style = if is_editing {
            CursorStyle::IBeam
        } else {
            CursorStyle::Arrow
        };

        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .cursor(cursor_style)
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

                            // Text and cursor rendering
                            let style = window.text_style();
                            let font_size = style.font_size.to_pixels(window.rem_size());
                            let line_height = window.line_height();
                            let wrap_width = px(shape.text_box_width);

                            if !shape.text.is_empty() {
                                let run = TextRun {
                                    len: shape.text.len(),
                                    font: style.font(),
                                    color: style.color,
                                    background_color: None,
                                    underline: None,
                                    strikethrough: None,
                                };
                                let display_text: SharedString = shape.text.clone().into();

                                if let Ok(lines) = window.text_system().shape_text(
                                    display_text,
                                    font_size,
                                    &[run],
                                    Some(wrap_width),
                                    None,
                                ) {
                                    let total_height: Pixels = lines
                                        .iter()
                                        .map(|l| l.size(line_height).height)
                                        .sum();

                                    let text_origin = point(
                                        center.x - wrap_width / 2.0,
                                        center.y - total_height / 2.0,
                                    );

                                    let text_bounds = Bounds::new(
                                        text_origin,
                                        size(wrap_width, total_height),
                                    );

                                    let mut y = text_origin.y;
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

                                    // Paint cursor if editing this shape
                                    if let Some(offset) = shape.cursor_offset {
                                        if cursor_visible {
                                            if let Some(first_line) = lines.first() {
                                                if let Some(cursor_pos) =
                                                    first_line.position_for_index(
                                                        offset, line_height,
                                                    )
                                                {
                                                    let cursor_x = text_origin.x
                                                        + cursor_pos.x
                                                        + (wrap_width
                                                            - first_line
                                                                .width()
                                                                .min(wrap_width))
                                                            / 2.0;
                                                    let cursor_y =
                                                        text_origin.y + cursor_pos.y;

                                                    window.paint_quad(fill(
                                                        Bounds::new(
                                                            point(cursor_x, cursor_y),
                                                            size(px(2.0), line_height),
                                                        ),
                                                        rgb(0x000000),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if let Some(_offset) = shape.cursor_offset {
                                // Empty text but editing — show cursor at center
                                if cursor_visible {
                                    let cursor_x = center.x;
                                    let cursor_y = center.y - line_height / 2.0;
                                    window.paint_quad(fill(
                                        Bounds::new(
                                            point(cursor_x, cursor_y),
                                            size(px(2.0), line_height),
                                        ),
                                        rgb(0x000000),
                                    ));
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
                cursor_visible: false,
                blink_epoch: 0,
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
