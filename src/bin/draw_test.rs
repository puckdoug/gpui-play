use std::ops::Range;
use std::time::Duration;

use gpui::{
    actions, App, Bounds, Context, CursorStyle, ElementInputHandler, EntityInputHandler,
    FocusHandle, Focusable, KeyBinding, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PathBuilder, Pixels, Point, Render, SharedString, TextAlign, TextRun, UTF16Selection, Window,
    WindowOptions, canvas, div, fill, point, prelude::*, px, rgb, rgba, size,
};
use gpui_platform::application;

use gpui_play::draw_test::{self, setup_menus};
use gpui_play::shape::{CanvasState, ResizeHandle, ShapeRenderData};
use gpui_play::text_input::TextInputState;

actions!(
    draw_test_canvas,
    [
        StopEditing,
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
    ]
);

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

const HANDLE_RADIUS: f32 = 5.0;
const HANDLE_SIZE: f32 = 8.0;

struct DrawTestView {
    focus_handle: FocusHandle,
    canvas_state: CanvasState,
    dragging: bool,
    drag_offset: Option<(f32, f32)>,
    editing_state: Option<TextInputState>,
    cursor_visible: bool,
    blink_epoch: usize,
    resizing: Option<ResizeHandle>,
    hover_handle: Option<ResizeHandle>,
    marquee_start: Option<(f32, f32)>,
    marquee_end: Option<(f32, f32)>,
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
        if let Some(ref mut state) = self.editing_state {
            state.undo();
        } else {
            self.canvas_state.undo();
        }
        cx.notify();
    }

    fn redo(&mut self, _: &draw_test::Redo, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.redo();
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
        self.blink_epoch += 1;
    }

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
            return;
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

    // -- Editing action handlers --

    fn on_stop_editing(&mut self, _: &StopEditing, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn on_left(&mut self, _: &Left, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.move_left();
            self.show_cursor(cx);
        }
    }

    fn on_right(&mut self, _: &Right, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.move_right();
            self.show_cursor(cx);
        }
    }

    fn on_select_left(&mut self, _: &SelectLeft, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.select_left();
            self.show_cursor(cx);
        }
    }

    fn on_select_right(&mut self, _: &SelectRight, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.select_right();
            self.show_cursor(cx);
        }
    }

    fn on_select_all(&mut self, _: &SelectAll, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            state.select_all();
            self.show_cursor(cx);
        } else {
            self.canvas_state.select_all();
            cx.notify();
        }
    }

    fn on_select_all_shapes(
        &mut self,
        _: &draw_test::SelectAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editing_state.is_some() {
            // When editing text, let the canvas SelectAll handle it
            return;
        }
        self.canvas_state.select_all();
        cx.notify();
    }

    fn on_copy(&mut self, _: &draw_test::Copy, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref state) = self.editing_state {
            // Text copy when editing
            let range = state.selected_range();
            if !range.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                    state.content()[range].to_string(),
                ));
            }
        } else if let Some(json) = self.canvas_state.copy_selected() {
            // Shape copy when not editing
            cx.write_to_clipboard(
                gpui::ClipboardItem::new_string_with_metadata(json, "gpui-play-shape".into()),
            );
        }
    }

    fn on_cut(&mut self, _: &draw_test::Cut, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut state) = self.editing_state {
            // Text cut when editing
            let range = state.selected_range();
            if !range.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                    state.content()[range].to_string(),
                ));
                state.insert("");
                self.show_cursor(cx);
            }
        } else if let Some(json) = self.canvas_state.copy_selected() {
            // Shape cut: copy then delete
            cx.write_to_clipboard(
                gpui::ClipboardItem::new_string_with_metadata(json, "gpui-play-shape".into()),
            );
            self.canvas_state.delete_selected();
            cx.notify();
        }
    }

    fn on_paste(&mut self, _: &draw_test::Paste, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            // Check for shape metadata first
            if let Some(metadata) = item.metadata() {
                if metadata == "gpui-play-shape" {
                    if let Some(json) = item.text() {
                        self.canvas_state.paste_shapes(&json);
                        cx.notify();
                        return;
                    }
                }
            }
            // Fall back to text paste when editing
            if let Some(ref mut state) = self.editing_state {
                if let Some(text) = item.text() {
                    state.insert(&text);
                    self.show_cursor(cx);
                }
            }
        }
    }

    // -- Text hit testing --

    /// Map a window pixel position to a byte offset in the editing shape's text.
    fn hit_test_text(&self, position: Point<Pixels>, window: &mut Window) -> usize {
        let Some(editing_idx) = self.canvas_state.editing() else {
            return 0;
        };
        let Some(ref editing_state) = self.editing_state else {
            return 0;
        };
        let text = editing_state.content();
        if text.is_empty() {
            return 0;
        }

        let shape = &self.canvas_state.shapes()[editing_idx];
        let (scx, scy) = shape.center();
        let wrap_width = px(shape.text_box_width());

        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let run = TextRun {
            len: text.len(),
            font: style.font(),
            color: style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let display_text: SharedString = text.to_string().into();

        let Ok(lines) = window.text_system().shape_text(
            display_text,
            font_size,
            &[run],
            Some(wrap_width),
            None,
        ) else {
            return 0;
        };

        let total_height: Pixels = lines.iter().map(|l| l.size(line_height).height).sum();
        let text_origin = point(
            px(scx) - wrap_width / 2.0,
            px(scy) - total_height / 2.0,
        );

        // Convert to local coordinates relative to text_origin
        let mut local = point(position.x - text_origin.x, position.y - text_origin.y);

        // Subtract the per-row centering offset so local coords match layout coords
        if let Some(first_line) = lines.first() {
            let rows = row_layout_info(first_line, wrap_width, line_height);
            let clicked_row =
                (f32::from(local.y) / f32::from(line_height)).max(0.0) as usize;
            if let Some(&(_, _, offset)) = rows.get(clicked_row) {
                local.x -= offset;
            } else if let Some(last) = rows.last() {
                local.x -= last.2;
            }

            match first_line.closest_index_for_position(local, line_height) {
                Ok(idx) | Err(idx) => idx,
            }
        } else {
            0
        }
    }

    // -- Mouse event handlers --

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mx = px_to_f32(event.position.x);
        let my = px_to_f32(event.position.y);

        // When currently editing, handle clicks within the editing shape
        if let Some(editing_idx) = self.canvas_state.editing() {
            let on_editing_shape =
                self.canvas_state.shapes()[editing_idx].contains_point(mx, my);

            if on_editing_shape {
                if event.click_count >= 3 {
                    // Triple-click: select all text
                    if let Some(ref mut state) = self.editing_state {
                        state.select_all();
                        self.show_cursor(cx);
                    }
                    return;
                }
                if event.click_count == 2 {
                    // Double-click: select word at click position
                    let offset = self.hit_test_text(event.position, window);
                    if let Some(ref mut state) = self.editing_state {
                        state.select_word_at(offset);
                        self.show_cursor(cx);
                    }
                    return;
                }
                // Single click: position cursor
                let offset = self.hit_test_text(event.position, window);
                if let Some(ref mut state) = self.editing_state {
                    state.move_to(offset);
                    self.show_cursor(cx);
                }
                return;
            }

            // Clicked outside the editing shape — commit editing
            self.commit_editing();
        }

        // Not editing: check resize handles first (priority over shape body)
        if let Some((_idx, handle)) =
            self.canvas_state.hit_test_handle(mx, my, HANDLE_RADIUS)
        {
            self.canvas_state.begin_resize();
            self.resizing = Some(handle);
            self.dragging = false;
            self.drag_offset = None;
            cx.notify();
            return;
        }

        // Double-click enters editing
        if event.click_count == 2 {
            self.canvas_state.select_at(mx, my);
            if let Some(idx) = self.canvas_state.selected() {
                self.start_editing(idx, cx);
                self.dragging = false;
                self.drag_offset = None;
                cx.notify();
                return;
            }
        }

        // Shift-click toggles multi-selection
        if event.modifiers.shift {
            self.canvas_state.toggle_selection_at(mx, my);
            self.dragging = false;
            self.drag_offset = None;
        } else {
            self.canvas_state.select_at(mx, my);

            if let Some(idx) = self.canvas_state.selected() {
                let (shape_cx, shape_cy) = self.canvas_state.shapes()[idx].center();
                self.dragging = true;
                self.drag_offset = Some((mx - shape_cx, my - shape_cy));
            } else {
                // Clicked empty space — start marquee selection
                self.dragging = false;
                self.drag_offset = None;
                self.marquee_start = Some((mx, my));
                self.marquee_end = Some((mx, my));
            }
        }
        cx.notify();
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mx = px_to_f32(event.position.x);
        let my = px_to_f32(event.position.y);

        // Resizing in progress
        if let Some(handle) = self.resizing {
            self.canvas_state.update_resize(handle, mx, my);
            cx.notify();
            return;
        }

        // Dragging shape
        if self.dragging
            && let Some((offset_x, offset_y)) = self.drag_offset
        {
            let new_cx = mx - offset_x;
            let new_cy = my - offset_y;
            self.canvas_state.move_selected(new_cx, new_cy);
            cx.notify();
            return;
        }

        // Marquee drag selection
        if let Some((sx, sy)) = self.marquee_start {
            self.marquee_end = Some((mx, my));
            self.canvas_state.select_in_rect(sx, sy, mx, my);
            cx.notify();
            return;
        }

        // Hover detection for cursor style
        let new_hover = self
            .canvas_state
            .hit_test_handle(mx, my, HANDLE_RADIUS)
            .map(|(_, h)| h);
        if new_hover != self.hover_handle {
            self.hover_handle = new_hover;
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.resizing.is_some() {
            self.canvas_state.commit_resize();
            self.resizing = None;
            cx.notify();
        }
        if self.marquee_start.is_some() {
            self.marquee_start = None;
            self.marquee_end = None;
            cx.notify();
        }
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

/// Compute per-row info from a WrappedLine's wrap boundaries.
/// Returns (row_byte_start, row_width, center_offset) for each visual row.
fn row_layout_info(
    layout: &gpui::WrappedLineLayout,
    wrap_width: Pixels,
    line_height: Pixels,
) -> Vec<(usize, Pixels, Pixels)> {
    let mut row_starts: Vec<usize> = vec![0];
    for wb in layout.wrap_boundaries() {
        let byte_idx = layout.runs()[wb.run_ix].glyphs[wb.glyph_ix].index;
        row_starts.push(byte_idx);
    }
    let text_len = layout.len();

    row_starts
        .iter()
        .enumerate()
        .map(|(i, &start)| {
            let end = row_starts.get(i + 1).copied().unwrap_or(text_len);
            // position_for_index at end gives x = row width (relative to row start)
            let row_width = layout
                .position_for_index(end, line_height)
                .map(|p| p.x)
                .unwrap_or(wrap_width);
            let center_offset = (wrap_width - row_width) / 2.0;
            (start, row_width, center_offset)
        })
        .collect()
}

/// Find the center offset for the row containing the given byte index.
fn center_offset_for_byte(
    rows: &[(usize, Pixels, Pixels)],
    byte_idx: usize,
) -> Pixels {
    for (i, &(start, _, offset)) in rows.iter().enumerate() {
        let next_start = rows.get(i + 1).map(|r| r.0).unwrap_or(usize::MAX);
        if byte_idx >= start && byte_idx < next_start {
            return offset;
        }
    }
    rows.last().map(|r| r.2).unwrap_or(px(0.0))
}

/// Paint text, selection highlight, and cursor for a shape inside the canvas.
fn paint_text_and_cursor(
    shape: &ShapeRenderData,
    center: Point<Pixels>,
    cursor_visible: bool,
    window: &mut Window,
    cx: &mut App,
) {
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

        let Ok(lines) = window.text_system().shape_text(
            display_text,
            font_size,
            &[run],
            Some(wrap_width),
            None,
        ) else {
            return;
        };

        let total_height: Pixels =
            lines.iter().map(|l| l.size(line_height).height).sum();
        let text_origin = point(
            center.x - wrap_width / 2.0,
            center.y - total_height / 2.0,
        );
        let text_bounds = Bounds::new(text_origin, size(wrap_width, total_height));

        // Compute per-row centering info for cursor/selection positioning
        let rows = lines
            .first()
            .map(|l| row_layout_info(l, wrap_width, line_height))
            .unwrap_or_default();

        // Paint selection highlight before text
        if let Some(ref sel) = shape.selected_range
            && !sel.is_empty()
            && let Some(first_line) = lines.first()
        {
            let text_len = first_line.len();

            for (row_idx, &(row_byte_start, row_width, offset)) in
                rows.iter().enumerate()
            {
                let row_byte_end = rows
                    .get(row_idx + 1)
                    .map(|r| r.0)
                    .unwrap_or(text_len);

                if sel.end <= row_byte_start || sel.start >= row_byte_end {
                    continue;
                }

                let row_y = text_origin.y + line_height * row_idx as f32;
                let sel_start_in_row = sel.start.max(row_byte_start);
                let sel_end_in_row = sel.end.min(row_byte_end);

                let left_x = if sel_start_in_row == row_byte_start {
                    px(0.0)
                } else {
                    first_line
                        .position_for_index(sel_start_in_row, line_height)
                        .map(|p| p.x)
                        .unwrap_or(px(0.0))
                };

                let right_x = if sel_end_in_row == row_byte_end {
                    first_line
                        .position_for_index(row_byte_end, line_height)
                        .map(|p| p.x)
                        .unwrap_or(row_width)
                } else {
                    first_line
                        .position_for_index(sel_end_in_row, line_height)
                        .map(|p| p.x)
                        .unwrap_or(row_width)
                };

                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(text_origin.x + left_x + offset, row_y),
                        point(
                            text_origin.x + right_x + offset,
                            row_y + line_height,
                        ),
                    ),
                    rgba(0x3388ff40),
                ));
            }
        }

        // Paint text centered
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

        // Paint cursor (only when no selection active)
        let has_selection = shape
            .selected_range
            .as_ref()
            .is_some_and(|r| !r.is_empty());

        if let Some(offset) = shape.cursor_offset
            && cursor_visible
            && !has_selection
            && let Some(first_line) = lines.first()
            && let Some(cursor_pos) =
                first_line.position_for_index(offset, line_height)
        {
            let offset_x = center_offset_for_byte(&rows, offset);
            window.paint_quad(fill(
                Bounds::new(
                    point(
                        text_origin.x + cursor_pos.x + offset_x,
                        text_origin.y + cursor_pos.y,
                    ),
                    size(px(2.0), line_height),
                ),
                rgb(0x000000),
            ));
        }
    } else if shape.cursor_offset.is_some() {
        // Empty text but editing — show cursor at center
        if cursor_visible {
            window.paint_quad(fill(
                Bounds::new(
                    point(center.x, center.y - line_height / 2.0),
                    size(px(2.0), line_height),
                ),
                rgb(0x000000),
            ));
        }
    }
}

impl Render for DrawTestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_editing = self.canvas_state.editing().is_some();
        let shapes = self
            .canvas_state
            .render_data(self.editing_state.as_ref());
        let cursor_visible = self.cursor_visible;
        let marquee = match (self.marquee_start, self.marquee_end) {
            (Some((x0, y0)), Some((x1, y1))) => Some((x0, y0, x1, y1)),
            _ => None,
        };

        let entity = cx.entity().clone();
        let focus = self.focus_handle.clone();
        let cursor_style = if is_editing {
            CursorStyle::IBeam
        } else if let Some(handle) = self.hover_handle {
            match handle {
                ResizeHandle::Left | ResizeHandle::Right => CursorStyle::ResizeLeftRight,
                ResizeHandle::Top | ResizeHandle::Bottom => CursorStyle::ResizeUpDown,
                ResizeHandle::TopLeft | ResizeHandle::BottomRight => {
                    CursorStyle::ResizeUpLeftDownRight
                }
                ResizeHandle::TopRight | ResizeHandle::BottomLeft => {
                    CursorStyle::ResizeUpRightDownLeft
                }
            }
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
            .on_action(cx.listener(Self::on_left))
            .on_action(cx.listener(Self::on_right))
            .on_action(cx.listener(Self::on_select_left))
            .on_action(cx.listener(Self::on_select_right))
            .on_action(cx.listener(Self::on_select_all))
            .on_action(cx.listener(Self::on_select_all_shapes))
            .on_action(cx.listener(Self::on_copy))
            .on_action(cx.listener(Self::on_cut))
            .on_action(cx.listener(Self::on_paste))
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

                            // Paint bounding box and resize handles for selected shapes
                            if let Some(ref handles) = shape.resize_handles {
                                // Bounding box: dashed-style rectangle
                                let top_left = point(
                                    center.x - px(shape.rx),
                                    center.y - px(shape.ry),
                                );
                                let bottom_right = point(
                                    center.x + px(shape.rx),
                                    center.y + px(shape.ry),
                                );
                                let bbox = Bounds::from_corners(top_left, bottom_right);
                                let mut bb = PathBuilder::stroke(px(1.0));
                                bb.move_to(bbox.origin);
                                bb.line_to(point(bottom_right.x, bbox.origin.y));
                                bb.line_to(bottom_right);
                                bb.line_to(point(bbox.origin.x, bottom_right.y));
                                bb.close();
                                if let Ok(path) = bb.build() {
                                    window.paint_path(path, rgba(0x4488ff80));
                                }

                                // Resize handle squares
                                let hs = px(HANDLE_SIZE);
                                let half = hs / 2.0;
                                for &(hx, hy) in handles {
                                    // White fill
                                    window.paint_quad(fill(
                                        Bounds::new(
                                            point(px(hx) - half, px(hy) - half),
                                            size(hs, hs),
                                        ),
                                        rgb(0xffffff),
                                    ));
                                    // Blue border
                                    let mut hb = PathBuilder::stroke(px(1.0));
                                    hb.move_to(point(px(hx) - half, px(hy) - half));
                                    hb.line_to(point(px(hx) + half, px(hy) - half));
                                    hb.line_to(point(px(hx) + half, px(hy) + half));
                                    hb.line_to(point(px(hx) - half, px(hy) + half));
                                    hb.close();
                                    if let Ok(path) = hb.build() {
                                        window.paint_path(path, rgb(0x4488ff));
                                    }
                                }
                            }

                            paint_text_and_cursor(
                                shape,
                                center,
                                cursor_visible,
                                window,
                                cx,
                            );
                        }

                        // Paint marquee selection rectangle
                        if let Some((x0, y0, x1, y1)) = marquee {
                            // Fill
                            window.paint_quad(fill(
                                Bounds::from_corners(
                                    point(px(x0), px(y0)),
                                    point(px(x1), px(y1)),
                                ),
                                rgba(0x3388ff10),
                            ));
                            // Border
                            let mut mb = PathBuilder::stroke(px(1.0));
                            mb.move_to(point(px(x0), px(y0)));
                            mb.line_to(point(px(x1), px(y0)));
                            mb.line_to(point(px(x1), px(y1)));
                            mb.line_to(point(px(x0), px(y1)));
                            mb.close();
                            if let Ok(path) = mb.build() {
                                window.paint_path(path, rgba(0x3388ff80));
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
        KeyBinding::new("left", Left, None),
        KeyBinding::new("right", Right, None),
        KeyBinding::new("shift-left", SelectLeft, None),
        KeyBinding::new("shift-right", SelectRight, None),
        KeyBinding::new("cmd-a", SelectAll, None),
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
                resizing: None,
                hover_handle: None,
                marquee_start: None,
                marquee_end: None,
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
