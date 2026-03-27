use gpui_play::shape::{CanvasState, Connector, OvalShape, ResizeHandle, ShapeKind};
use gpui_play::text_input::TextInputState;

// -- Oval creation --

#[test]
fn test_oval_default_values() {
    let oval = OvalShape::new(100.0, 200.0);
    assert_eq!(oval.center(), (100.0, 200.0));
    assert_eq!(oval.rx(), 100.0); // default horizontal radius
    assert_eq!(oval.ry(), 70.0); // default vertical radius
    assert_eq!(oval.border_width(), 1.0);
    assert_eq!(oval.text(), "");
}

#[test]
fn test_oval_custom_size() {
    let oval = OvalShape::with_size(50.0, 50.0, 80.0, 40.0);
    assert_eq!(oval.center(), (50.0, 50.0));
    assert_eq!(oval.rx(), 80.0);
    assert_eq!(oval.ry(), 40.0);
}

// -- Hit testing --

#[test]
fn test_hit_inside_oval() {
    let oval = OvalShape::new(100.0, 100.0);
    assert!(oval.contains_point(100.0, 100.0)); // center
    assert!(oval.contains_point(110.0, 100.0)); // slightly right
    assert!(oval.contains_point(100.0, 110.0)); // slightly down
}

#[test]
fn test_hit_outside_oval() {
    let oval = OvalShape::new(100.0, 100.0);
    assert!(!oval.contains_point(250.0, 100.0)); // far right
    assert!(!oval.contains_point(100.0, 250.0)); // far below
    assert!(!oval.contains_point(0.0, 0.0)); // origin
}

#[test]
fn test_hit_on_boundary() {
    let oval = OvalShape::with_size(0.0, 0.0, 100.0, 50.0);
    // Point exactly on the ellipse boundary: (100, 0) for rx=100
    assert!(oval.contains_point(100.0, 0.0));
    // Point exactly on the ellipse boundary: (0, 50) for ry=50
    assert!(oval.contains_point(0.0, 50.0));
}

#[test]
fn test_hit_elongated_oval() {
    let oval = OvalShape::with_size(0.0, 0.0, 200.0, 20.0);
    assert!(oval.contains_point(150.0, 0.0)); // inside wide but narrow
    assert!(!oval.contains_point(0.0, 25.0)); // outside vertically
}

// -- Move shape --

#[test]
fn test_move_shape() {
    let mut oval = OvalShape::new(100.0, 100.0);
    oval.move_to(200.0, 300.0);
    assert_eq!(oval.center(), (200.0, 300.0));
}

#[test]
fn test_move_preserves_size() {
    let mut oval = OvalShape::with_size(0.0, 0.0, 80.0, 40.0);
    oval.move_to(500.0, 500.0);
    assert_eq!(oval.rx(), 80.0);
    assert_eq!(oval.ry(), 40.0);
}

// -- Shape text --

#[test]
fn test_set_text() {
    let mut oval = OvalShape::new(0.0, 0.0);
    oval.set_text("Hello");
    assert_eq!(oval.text(), "Hello");
}

#[test]
fn test_text_default_empty() {
    let oval = OvalShape::new(0.0, 0.0);
    assert_eq!(oval.text(), "");
}

// -- Canvas state --

#[test]
fn test_canvas_starts_empty() {
    let canvas = CanvasState::new();
    assert_eq!(canvas.shape_count(), 0);
    assert!(canvas.selected().is_none());
}

#[test]
fn test_add_oval() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_add_multiple_ovals() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    assert_eq!(canvas.shape_count(), 2);
}

// -- Selection --

#[test]
fn test_select_shape_at_point() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    assert_eq!(canvas.selected(), Some(0));
}

#[test]
fn test_select_empty_space_deselects() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0); // select it
    canvas.select_at(500.0, 500.0); // click empty space
    assert!(canvas.selected().is_none());
}

#[test]
fn test_select_correct_shape_among_multiple() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(400.0, 400.0);
    assert_eq!(canvas.selected(), Some(1));
}

#[test]
fn test_select_topmost_when_overlapping() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(120.0, 100.0); // overlapping
                                   // Should select the topmost (last added)
    canvas.select_at(110.0, 100.0);
    assert_eq!(canvas.selected(), Some(1));
}

// -- Undo/Redo --

#[test]
fn test_undo_add_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    assert_eq!(canvas.shape_count(), 1);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_redo_add_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 0);
    canvas.redo();
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_undo_move() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    canvas.move_selected_by(100.0, 200.0);
    assert_eq!(canvas.shapes()[0].center(), (200.0, 300.0));
    canvas.undo();
    assert_eq!(canvas.shapes()[0].center(), (100.0, 100.0));
}

#[test]
fn test_undo_past_beginning_noop() {
    let mut canvas = CanvasState::new();
    canvas.undo(); // nothing to undo
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_new_action_clears_redo() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.undo();
    canvas.add_oval(200.0, 200.0); // clears redo
    canvas.redo(); // should be noop
    assert_eq!(canvas.shape_count(), 1);
    assert_eq!(canvas.shapes()[0].center(), (200.0, 200.0));
}

// -- Editing state --

#[test]
fn test_editing_initially_none() {
    let canvas = CanvasState::new();
    assert!(canvas.editing().is_none());
}

#[test]
fn test_start_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    assert_eq!(canvas.editing(), Some(0));
}

#[test]
fn test_start_editing_selects_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    assert_eq!(canvas.selected(), Some(0));
}

#[test]
fn test_stop_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    canvas.stop_editing();
    assert!(canvas.editing().is_none());
}

#[test]
fn test_start_editing_invalid_index_ignored() {
    let mut canvas = CanvasState::new();
    canvas.start_editing(5); // no shapes exist
    assert!(canvas.editing().is_none());
}

#[test]
fn test_deselect_stops_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    canvas.select_at(500.0, 500.0); // click empty space
    assert!(canvas.editing().is_none());
}

// -- Re-editing --

#[test]
fn test_re_edit_after_stop() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    canvas.set_shape_text(0, "hello");
    canvas.stop_editing();
    // Re-enter editing on same shape
    canvas.start_editing(0);
    assert_eq!(canvas.editing(), Some(0));
    assert_eq!(canvas.shapes()[0].text(), "hello");
}

#[test]
fn test_select_same_shape_preserves_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    // Clicking the same shape that is being edited should not clear editing
    canvas.select_at(100.0, 100.0);
    assert_eq!(canvas.editing(), Some(0));
}

#[test]
fn test_select_different_shape_stops_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.start_editing(0);
    canvas.select_at(400.0, 400.0); // click different shape
    assert!(canvas.editing().is_none());
}

// -- Text box width --

#[test]
fn test_text_box_width_default_oval() {
    let oval = OvalShape::new(0.0, 0.0);
    // Inscribed rectangle width = rx * sqrt(2) ≈ 100 * 1.414 ≈ 141.42
    let expected = 100.0 * std::f32::consts::SQRT_2;
    assert!((oval.text_box_width() - expected).abs() < 0.01);
}

#[test]
fn test_text_box_width_custom_size() {
    let oval = OvalShape::with_size(0.0, 0.0, 200.0, 50.0);
    let expected = 200.0 * std::f32::consts::SQRT_2;
    assert!((oval.text_box_width() - expected).abs() < 0.01);
}

// -- Render data with cursor --

#[test]
fn test_render_data_no_cursor_when_not_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);

    let data = canvas.render_data(None);
    assert_eq!(data.len(), 1);
    assert!(data[0].cursor_offset.is_none());
}

#[test]
fn test_render_data_has_cursor_when_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);

    let editing = TextInputState::new("");
    let data = canvas.render_data(Some(&editing));
    assert_eq!(data[0].cursor_offset, Some(0));
}

#[test]
fn test_render_data_cursor_at_end_after_input() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);

    let mut editing = TextInputState::new("hello");
    editing.move_to_end();
    let data = canvas.render_data(Some(&editing));
    assert_eq!(data[0].cursor_offset, Some(5));
}

#[test]
fn test_render_data_cursor_only_on_editing_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    canvas.start_editing(0);

    let editing = TextInputState::new("hi");
    let data = canvas.render_data(Some(&editing));
    assert_eq!(data[0].cursor_offset, Some(0)); // editing shape has cursor
    assert!(data[1].cursor_offset.is_none()); // other shape does not
}

#[test]
fn test_render_data_includes_editing_text() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.set_shape_text(0, "old");
    canvas.start_editing(0);

    let editing = TextInputState::new("new text");
    let data = canvas.render_data(Some(&editing));
    // Editing shape should show live editing text, not saved shape text
    assert_eq!(data[0].text, "new text");
}

#[test]
fn test_render_data_no_selected_range_when_not_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    let data = canvas.render_data(None);
    assert!(data[0].selected_range.is_none());
}

#[test]
fn test_render_data_has_selected_range_when_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);

    let mut editing = TextInputState::new("hello world");
    editing.select_word_at(3); // selects "hello" → 0..5
    let data = canvas.render_data(Some(&editing));
    assert_eq!(data[0].selected_range, Some(0..5));
}

#[test]
fn test_render_data_collapsed_selected_range() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);

    let editing = TextInputState::new("hello");
    let data = canvas.render_data(Some(&editing));
    // Cursor at 0 with no selection → collapsed range
    assert_eq!(data[0].selected_range, Some(0..0));
}

#[test]
fn test_render_data_no_selected_range_on_non_editing_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    canvas.start_editing(0);

    let mut editing = TextInputState::new("hello");
    editing.select_all();
    let data = canvas.render_data(Some(&editing));
    assert!(data[1].selected_range.is_none());
}

// -- Resize handle positions (8 handles: 4 corners + 4 midpoints) --
// Default oval: rx=100, ry=70. Bounding box: (cx-rx, cy-ry) to (cx+rx, cy+ry)

#[test]
fn test_handle_position_top_left() {
    let oval = OvalShape::new(100.0, 200.0);
    // Top-left corner of bounding box: (cx-rx, cy-ry) = (0, 130)
    assert_eq!(
        oval.handle_position(ResizeHandle::TopLeft),
        (0.0, 130.0)
    );
}

#[test]
fn test_handle_position_top() {
    let oval = OvalShape::new(100.0, 200.0);
    // Top midpoint: (cx, cy-ry) = (100, 130)
    assert_eq!(
        oval.handle_position(ResizeHandle::Top),
        (100.0, 130.0)
    );
}

#[test]
fn test_handle_position_top_right() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx+rx, cy-ry) = (200, 130)
    assert_eq!(
        oval.handle_position(ResizeHandle::TopRight),
        (200.0, 130.0)
    );
}

#[test]
fn test_handle_position_right() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx+rx, cy) = (200, 200)
    assert_eq!(
        oval.handle_position(ResizeHandle::Right),
        (200.0, 200.0)
    );
}

#[test]
fn test_handle_position_bottom_right() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx+rx, cy+ry) = (200, 270)
    assert_eq!(
        oval.handle_position(ResizeHandle::BottomRight),
        (200.0, 270.0)
    );
}

#[test]
fn test_handle_position_bottom() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx, cy+ry) = (100, 270)
    assert_eq!(
        oval.handle_position(ResizeHandle::Bottom),
        (100.0, 270.0)
    );
}

#[test]
fn test_handle_position_bottom_left() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx-rx, cy+ry) = (0, 270)
    assert_eq!(
        oval.handle_position(ResizeHandle::BottomLeft),
        (0.0, 270.0)
    );
}

#[test]
fn test_handle_position_left() {
    let oval = OvalShape::new(100.0, 200.0);
    // (cx-rx, cy) = (0, 200)
    assert_eq!(
        oval.handle_position(ResizeHandle::Left),
        (0.0, 200.0)
    );
}

// -- Resize handle hit testing --

#[test]
fn test_hit_test_handle_corner() {
    let oval = OvalShape::new(100.0, 100.0);
    // TopRight corner at (200, 30). Click near it.
    assert_eq!(
        oval.hit_test_handle(198.0, 32.0, 5.0),
        Some(ResizeHandle::TopRight)
    );
}

#[test]
fn test_hit_test_handle_midpoint() {
    let oval = OvalShape::new(100.0, 100.0);
    // Right midpoint at (200, 100). Click near it.
    assert_eq!(
        oval.hit_test_handle(198.0, 100.0, 5.0),
        Some(ResizeHandle::Right)
    );
}

#[test]
fn test_hit_test_handle_miss() {
    let oval = OvalShape::new(100.0, 100.0);
    // Point far from all handles (center of oval)
    assert_eq!(oval.hit_test_handle(100.0, 100.0, 5.0), None);
}

#[test]
fn test_hit_test_handle_on_bounding_box_edge_miss() {
    let oval = OvalShape::new(100.0, 100.0);
    // On the top edge of bounding box, between TopLeft and Top handles
    // Not close enough to any handle
    assert_eq!(oval.hit_test_handle(50.0, 30.0, 5.0), None);
}

// -- Resize: midpoint handles (axis-constrained) --

#[test]
fn test_resize_right_changes_rx_only() {
    let mut oval = OvalShape::new(100.0, 100.0);
    let orig_ry = oval.ry();
    oval.resize(ResizeHandle::Right, 250.0, 150.0);
    assert_eq!(oval.rx(), 150.0); // rx = |250 - 100|
    assert_eq!(oval.ry(), orig_ry); // ry unchanged
}

#[test]
fn test_resize_left_changes_rx_only() {
    let mut oval = OvalShape::new(100.0, 100.0);
    let orig_ry = oval.ry();
    oval.resize(ResizeHandle::Left, -50.0, 150.0);
    assert_eq!(oval.rx(), 150.0); // rx = |100 - (-50)|
    assert_eq!(oval.ry(), orig_ry); // ry unchanged
}

#[test]
fn test_resize_top_changes_ry_only() {
    let mut oval = OvalShape::new(100.0, 100.0);
    let orig_rx = oval.rx();
    oval.resize(ResizeHandle::Top, 150.0, -50.0);
    assert_eq!(oval.ry(), 150.0); // ry = |100 - (-50)|
    assert_eq!(oval.rx(), orig_rx); // rx unchanged
}

#[test]
fn test_resize_bottom_changes_ry_only() {
    let mut oval = OvalShape::new(100.0, 100.0);
    let orig_rx = oval.rx();
    oval.resize(ResizeHandle::Bottom, 150.0, 200.0);
    assert_eq!(oval.ry(), 100.0); // ry = |200 - 100|
    assert_eq!(oval.rx(), orig_rx); // rx unchanged
}

// -- Resize: corner handles (free resize, both axes) --

#[test]
fn test_resize_top_right_changes_both() {
    let mut oval = OvalShape::new(100.0, 100.0);
    oval.resize(ResizeHandle::TopRight, 250.0, -50.0);
    assert_eq!(oval.rx(), 150.0); // rx = |250 - 100|
    assert_eq!(oval.ry(), 150.0); // ry = |100 - (-50)|
}

#[test]
fn test_resize_bottom_left_changes_both() {
    let mut oval = OvalShape::new(100.0, 100.0);
    oval.resize(ResizeHandle::BottomLeft, -50.0, 200.0);
    assert_eq!(oval.rx(), 150.0); // rx = |100 - (-50)|
    assert_eq!(oval.ry(), 100.0); // ry = |200 - 100|
}

#[test]
fn test_resize_top_left_changes_both() {
    let mut oval = OvalShape::new(100.0, 100.0);
    oval.resize(ResizeHandle::TopLeft, 50.0, 50.0);
    assert_eq!(oval.rx(), 50.0);  // rx = |100 - 50|
    assert_eq!(oval.ry(), 50.0);  // ry = |100 - 50|
}

#[test]
fn test_resize_bottom_right_changes_both() {
    let mut oval = OvalShape::new(100.0, 100.0);
    oval.resize(ResizeHandle::BottomRight, 300.0, 250.0);
    assert_eq!(oval.rx(), 200.0); // rx = |300 - 100|
    assert_eq!(oval.ry(), 150.0); // ry = |250 - 100|
}

// -- Resize constraints --

#[test]
fn test_resize_enforces_minimum_radius() {
    let mut oval = OvalShape::new(100.0, 100.0);
    // Drag right handle past center
    oval.resize(ResizeHandle::Right, 100.0, 100.0);
    assert!(oval.rx() >= 20.0); // minimum radius enforced
}

#[test]
fn test_resize_corner_enforces_minimum_both_axes() {
    let mut oval = OvalShape::new(100.0, 100.0);
    // Drag corner to center
    oval.resize(ResizeHandle::BottomRight, 100.0, 100.0);
    assert!(oval.rx() >= 20.0);
    assert!(oval.ry() >= 20.0);
}

#[test]
fn test_resize_preserves_center() {
    let mut oval = OvalShape::new(100.0, 200.0);
    oval.resize(ResizeHandle::TopRight, 250.0, 100.0);
    assert_eq!(oval.center(), (100.0, 200.0));
}

// -- Canvas resize with undo/redo --

#[test]
fn test_canvas_hit_test_handle_on_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    // Right midpoint handle at (200, 100)
    let result = canvas.hit_test_handle(198.0, 100.0, 5.0);
    assert_eq!(result, Some((0, ResizeHandle::Right)));
}

#[test]
fn test_canvas_hit_test_handle_corner() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    // BottomRight corner at (200, 170)
    let result = canvas.hit_test_handle(198.0, 168.0, 5.0);
    assert_eq!(result, Some((0, ResizeHandle::BottomRight)));
}

#[test]
fn test_canvas_hit_test_handle_when_none_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    assert_eq!(canvas.hit_test_handle(200.0, 100.0, 5.0), None);
}

#[test]
fn test_undo_resize() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    let orig_rx = canvas.shapes()[0].rx();
    canvas.begin_resize();
    canvas.update_resize(ResizeHandle::Right, 250.0, 100.0);
    canvas.commit_resize();
    assert_eq!(canvas.shapes()[0].rx(), 150.0);
    canvas.undo();
    assert_eq!(canvas.shapes()[0].rx(), orig_rx);
}

#[test]
fn test_redo_resize() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    canvas.begin_resize();
    canvas.update_resize(ResizeHandle::Right, 250.0, 100.0);
    canvas.commit_resize();
    canvas.undo();
    canvas.redo();
    assert_eq!(canvas.shapes()[0].rx(), 150.0);
}

#[test]
fn test_undo_corner_resize_restores_both_axes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    let orig_rx = canvas.shapes()[0].rx();
    let orig_ry = canvas.shapes()[0].ry();
    canvas.begin_resize();
    canvas.update_resize(ResizeHandle::BottomRight, 300.0, 250.0);
    canvas.commit_resize();
    canvas.undo();
    assert_eq!(canvas.shapes()[0].rx(), orig_rx);
    assert_eq!(canvas.shapes()[0].ry(), orig_ry);
}

// -- Render data with resize handles (8 handle positions) --

#[test]
fn test_render_data_includes_handles_for_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    let data = canvas.render_data(None);
    let handles = data[0].resize_handles.as_ref().unwrap();
    assert_eq!(handles.len(), 8); // 4 corners + 4 midpoints
}

#[test]
fn test_render_data_handle_positions_match_bounding_box() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0); // rx=100, ry=70
    canvas.select_at(100.0, 100.0);
    let data = canvas.render_data(None);
    let handles = data[0].resize_handles.as_ref().unwrap();
    // Verify corners and midpoints of bounding box
    // TopLeft=(0,30), Top=(100,30), TopRight=(200,30), Right=(200,100),
    // BottomRight=(200,170), Bottom=(100,170), BottomLeft=(0,170), Left=(0,100)
    assert_eq!(handles[0], (0.0, 30.0));     // TopLeft
    assert_eq!(handles[1], (100.0, 30.0));   // Top
    assert_eq!(handles[2], (200.0, 30.0));   // TopRight
    assert_eq!(handles[3], (200.0, 100.0));  // Right
    assert_eq!(handles[4], (200.0, 170.0));  // BottomRight
    assert_eq!(handles[5], (100.0, 170.0));  // Bottom
    assert_eq!(handles[6], (0.0, 170.0));    // BottomLeft
    assert_eq!(handles[7], (0.0, 100.0));    // Left
}

#[test]
fn test_render_data_no_handles_when_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.start_editing(0);
    let editing = TextInputState::new("");
    let data = canvas.render_data(Some(&editing));
    assert!(data[0].resize_handles.is_none());
}

#[test]
fn test_render_data_no_handles_for_unselected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    canvas.select_at(100.0, 100.0);
    let data = canvas.render_data(None);
    assert!(data[1].resize_handles.is_none());
}

// -- Shape serialization --

#[test]
fn test_shape_to_json_roundtrip() {
    let mut oval = OvalShape::new(100.0, 200.0);
    oval.set_text("hello world");
    let json = oval.to_json();
    let restored = OvalShape::from_json(&json).unwrap();
    assert_eq!(restored.center(), (100.0, 200.0));
    assert_eq!(restored.rx(), 100.0);
    assert_eq!(restored.ry(), 70.0);
    assert_eq!(restored.text(), "hello world");
}

#[test]
fn test_shape_to_json_preserves_custom_size() {
    let oval = OvalShape::with_size(50.0, 60.0, 150.0, 80.0);
    let json = oval.to_json();
    let restored = OvalShape::from_json(&json).unwrap();
    assert_eq!(restored.rx(), 150.0);
    assert_eq!(restored.ry(), 80.0);
}

#[test]
fn test_shape_from_json_invalid_returns_none() {
    assert!(OvalShape::from_json("not valid json").is_none());
}

// -- Multi-select --

#[test]
fn test_toggle_selection_adds_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    assert_eq!(canvas.selected_indices(), &[0, 1]);
}

#[test]
fn test_toggle_selection_removes_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    canvas.toggle_selection_at(100.0, 100.0); // deselect first
    assert_eq!(canvas.selected_indices(), &[1]);
}

#[test]
fn test_select_at_clears_multi_selection() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    // Normal click clears multi-selection, selects only clicked shape
    canvas.select_at(400.0, 400.0);
    assert_eq!(canvas.selected_indices(), &[1]);
}

#[test]
fn test_selected_returns_single_from_multi() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    // selected() returns first for backwards compatibility
    assert_eq!(canvas.selected(), Some(0));
}

// -- Select all --

#[test]
fn test_select_all_selects_every_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    canvas.add_oval(500.0, 500.0);
    canvas.select_all();
    assert_eq!(canvas.selected_indices(), &[0, 1, 2]);
}

#[test]
fn test_select_all_empty_canvas() {
    let mut canvas = CanvasState::new();
    canvas.select_all();
    assert!(canvas.selected_indices().is_empty());
}

#[test]
fn test_select_all_clears_editing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(300.0, 300.0);
    canvas.start_editing(0);
    canvas.select_all();
    assert!(canvas.editing().is_none());
    assert_eq!(canvas.selected_indices(), &[0, 1]);
}

// -- Marquee (drag) selection --

#[test]
fn test_select_in_rect_selects_overlapping_shapes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0); // bounding box: (0,30) to (200,170)
    canvas.add_oval(300.0, 300.0); // bounding box: (200,230) to (400,370)
    canvas.add_oval(600.0, 600.0); // bounding box: (500,530) to (700,670)
    // Rect that covers first two shapes but not the third
    canvas.select_in_rect(0.0, 0.0, 450.0, 400.0);
    assert_eq!(canvas.selected_indices(), &[0, 1]);
}

#[test]
fn test_select_in_rect_empty_area() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    // Rect that doesn't overlap any shape
    canvas.select_in_rect(500.0, 500.0, 600.0, 600.0);
    assert!(canvas.selected_indices().is_empty());
}

#[test]
fn test_select_in_rect_partial_overlap_selects() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0); // bounding box: (0,30) to (200,170)
    // Rect that partially overlaps the bounding box
    canvas.select_in_rect(150.0, 0.0, 300.0, 200.0);
    assert_eq!(canvas.selected_indices(), &[0]);
}

#[test]
fn test_select_in_rect_replaces_previous_selection() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(500.0, 500.0);
    canvas.select_at(500.0, 500.0);
    assert_eq!(canvas.selected_indices(), &[1]);
    // Marquee over first shape replaces selection
    canvas.select_in_rect(0.0, 0.0, 250.0, 200.0);
    assert_eq!(canvas.selected_indices(), &[0]);
}

// -- Copy selected shapes --

#[test]
fn test_copy_single_selected_returns_json() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.set_shape_text(0, "test");
    canvas.select_at(100.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    assert!(json.contains("test"));
}

#[test]
fn test_copy_selected_returns_none_when_no_selection() {
    let canvas = CanvasState::new();
    assert!(canvas.copy_selected().is_none());
}

#[test]
fn test_copy_multiple_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.set_shape_text(0, "first");
    canvas.set_shape_text(1, "second");
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    let json = canvas.copy_selected().unwrap();
    assert!(json.contains("first"));
    assert!(json.contains("second"));
}

// -- Paste shapes --

#[test]
fn test_paste_single_shape() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.set_shape_text(0, "original");
    canvas.select_at(100.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shape_count(), 2);
    assert_eq!(canvas.shapes()[1].text(), "original");
}

#[test]
fn test_paste_offsets_position() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    let (cx, cy) = canvas.shapes()[1].center();
    assert!(cx != 100.0 || cy != 100.0);
}

#[test]
fn test_paste_multiple_shapes_preserves_spacing() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 200.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 200.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shape_count(), 4);
    // Original spacing: dx=300, dy=100 between the two shapes
    let (cx0, cy0) = canvas.shapes()[2].center();
    let (cx1, cy1) = canvas.shapes()[3].center();
    let dx = cx1 - cx0;
    let dy = cy1 - cy0;
    assert_eq!(dx, 300.0);
    assert_eq!(dy, 100.0);
}

#[test]
fn test_paste_multiple_preserves_sizes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0); // default 100x70
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    // Resize first shape
    canvas.begin_resize();
    canvas.update_resize(ResizeHandle::Right, 250.0, 100.0);
    canvas.commit_resize();
    canvas.toggle_selection_at(400.0, 400.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    // Pasted shapes preserve original sizes
    assert_eq!(canvas.shapes()[2].rx(), 150.0); // resized
    assert_eq!(canvas.shapes()[3].rx(), 100.0); // default
}

#[test]
fn test_paste_multiple_preserves_text() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.set_shape_text(0, "hello");
    canvas.set_shape_text(1, "world");
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shapes()[2].text(), "hello");
    assert_eq!(canvas.shapes()[3].text(), "world");
}

#[test]
fn test_paste_selects_new_shapes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    // Only the pasted shapes should be selected
    assert_eq!(canvas.selected_indices(), &[2, 3]);
}

#[test]
fn test_paste_is_undoable() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shape_count(), 4);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 2);
}

#[test]
fn test_paste_invalid_json_is_noop() {
    let mut canvas = CanvasState::new();
    canvas.paste_shapes("not json");
    assert_eq!(canvas.shape_count(), 0);
}

// -- Delete selected shapes --

#[test]
fn test_delete_single_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 0);
    assert!(canvas.selected().is_none());
}

#[test]
fn test_delete_multiple_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.add_oval(700.0, 700.0);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 1);
    assert_eq!(canvas.shapes()[0].center(), (700.0, 700.0));
}

#[test]
fn test_delete_selected_noop_when_none_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 1);
}

#[test]
fn test_delete_selected_is_undoable() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.set_shape_text(0, "hello");
    canvas.select_at(100.0, 100.0);
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 0);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 1);
    assert_eq!(canvas.shapes()[0].text(), "hello");
}

#[test]
fn test_delete_multiple_is_undoable() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 400.0);
    canvas.set_shape_text(0, "first");
    canvas.set_shape_text(1, "second");
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 400.0);
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 0);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 2);
    assert_eq!(canvas.shapes()[0].text(), "first");
    assert_eq!(canvas.shapes()[1].text(), "second");
}

#[test]
fn test_delete_selected_redo() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.select_at(100.0, 100.0);
    canvas.delete_selected();
    canvas.undo();
    assert_eq!(canvas.shape_count(), 1);
    canvas.redo();
    assert_eq!(canvas.shape_count(), 0);
}

// -- Connector data model --

#[test]
fn test_connector_creation() {
    let conn = Connector::new(0, 1);
    assert_eq!(conn.source(), 0);
    assert_eq!(conn.target(), 1);
}

#[test]
fn test_connector_default_curvature() {
    let conn = Connector::new(0, 1);
    assert_eq!(conn.curvature(), 0.0);
}


// -- Oval border point geometry --

#[test]
fn test_point_on_oval_border_right() {
    let oval = OvalShape::new(100.0, 100.0); // rx=100, ry=70
    let (px, py) = oval.point_on_border(0.0);
    assert!((px - 200.0).abs() < 0.01); // cx + rx
    assert!((py - 100.0).abs() < 0.01); // cy
}

#[test]
fn test_point_on_oval_border_top() {
    let oval = OvalShape::new(100.0, 100.0);
    let (px, py) = oval.point_on_border(-std::f32::consts::FRAC_PI_2);
    assert!((px - 100.0).abs() < 0.01); // cx
    assert!((py - 30.0).abs() < 0.01);  // cy - ry
}

#[test]
fn test_point_on_oval_border_diagonal() {
    let oval = OvalShape::with_size(0.0, 0.0, 100.0, 50.0);
    let angle = std::f32::consts::FRAC_PI_4;
    let (px, py) = oval.point_on_border(angle);
    let expected_x = 100.0 * angle.cos();
    let expected_y = 50.0 * angle.sin();
    assert!((px - expected_x).abs() < 0.01);
    assert!((py - expected_y).abs() < 0.01);
}

#[test]
fn test_connector_endpoints_on_oval_borders() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let conn = Connector::new(0, 1);
    let (sx, sy) = conn.start_point(&shapes);
    let (ex, ey) = conn.end_point(&shapes);
    // Start should be on source oval border (ellipse equation ≈ 1.0)
    let s = &shapes[0];
    let dx = (sx - s.center().0) / s.rx();
    let dy = (sy - s.center().1) / s.ry();
    assert!((dx * dx + dy * dy - 1.0).abs() < 0.01);
    // End should be on target oval border
    let t = &shapes[1];
    let dx = (ex - t.center().0) / t.rx();
    let dy = (ey - t.center().1) / t.ry();
    assert!((dx * dx + dy * dy - 1.0).abs() < 0.01);
}

// -- Bezier control point --

#[test]
fn test_control_point_zero_curvature() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let conn = Connector::new(0, 1);
    let (cpx, cpy) = conn.control_point(&shapes);
    // Zero curvature: control point at midpoint of centers
    assert!((cpx - 250.0).abs() < 0.01);
    assert!((cpy - 100.0).abs() < 0.01);
}

#[test]
fn test_control_point_positive_curvature() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let mut conn = Connector::new(0, 1);
    conn.set_curvature(50.0);
    let (cpx, cpy) = conn.control_point(&shapes);
    // Positive curvature: perpendicular offset from center line
    assert!((cpx - 250.0).abs() < 0.01); // still at midpoint x
    assert!((cpy - 100.0).abs() > 10.0); // offset from the center line
}

#[test]
fn test_control_point_negative_curvature() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let conn_pos = {
        let mut c = Connector::new(0, 1);
        c.set_curvature(50.0);
        c.control_point(&shapes)
    };
    let conn_neg = {
        let mut c = Connector::new(0, 1);
        c.set_curvature(-50.0);
        c.control_point(&shapes)
    };
    // Negative curvature offsets opposite direction from positive
    assert!((conn_pos.1 - 100.0).abs() > 10.0);
    assert!((conn_neg.1 - 100.0).abs() > 10.0);
    // They should be on opposite sides of the center line
    assert!((conn_pos.1 - 100.0).signum() != (conn_neg.1 - 100.0).signum());
}

// -- Canvas connector management --

#[test]
fn test_add_connector() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    assert_eq!(canvas.connector_count(), 1);
}

#[test]
fn test_add_connector_is_undoable() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    assert_eq!(canvas.connector_count(), 1);
    canvas.undo();
    assert_eq!(canvas.connector_count(), 0);
}

#[test]
fn test_remove_connector() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.remove_connector(0);
    assert_eq!(canvas.connector_count(), 0);
}

#[test]
fn test_multiple_connectors_between_same_shapes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.add_connector(0, 1);
    assert_eq!(canvas.connector_count(), 2);
}

#[test]
fn test_delete_shape_removes_connected_connectors() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0); // 0
    canvas.add_oval(400.0, 100.0); // 1
    canvas.add_oval(700.0, 100.0); // 2
    canvas.add_connector(0, 1); // connected to shape 0 and 1
    canvas.add_connector(1, 2); // connected to shape 1 and 2
    canvas.select_at(400.0, 100.0); // select shape 1
    canvas.delete_selected();
    // Shape 1 deleted: connector 0→1 and 1→2 both removed
    assert_eq!(canvas.connector_count(), 0);
    assert_eq!(canvas.shape_count(), 2);
}

// -- Connector midpoint --

#[test]
fn test_connector_midpoint_zero_curvature() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let conn = Connector::new(0, 1);
    let (mx, my) = conn.midpoint(&shapes);
    // With zero curvature, midpoint is on the straight line between endpoints
    // Should be approximately at the center between the two ovals
    assert!((mx - 250.0).abs() < 5.0);
    assert!((my - 100.0).abs() < 5.0);
}

#[test]
fn test_connector_midpoint_changes_with_curvature() {
    let shapes = vec![
        OvalShape::new(100.0, 100.0),
        OvalShape::new(400.0, 100.0),
    ];
    let mut conn = Connector::new(0, 1);
    let (_, my_straight) = conn.midpoint(&shapes);
    conn.set_curvature(80.0);
    let (_, my_curved) = conn.midpoint(&shapes);
    // Curved midpoint should be offset from the straight midpoint
    assert!((my_curved - my_straight).abs() > 10.0);
}

// -- Connector render data --

#[test]
fn test_connector_render_data_count() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    let data = canvas.connector_render_data();
    assert_eq!(data.len(), 1);
}

#[test]
fn test_connector_render_data_has_endpoints() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    let data = canvas.connector_render_data();
    let d = &data[0];
    // Start point should be near right edge of first oval
    assert!(d.start.0 > 150.0); // past center of first oval
    // End point should be near left edge of second oval
    assert!(d.end.0 < 350.0); // before center of second oval
    // Control points should be between start and end
    assert!(d.control_a.0 > d.start.0);
    assert!(d.control_b.0 < d.end.0);
}

// -- Connector selection --

#[test]
fn test_select_all_includes_connectors() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    assert_eq!(canvas.selected_indices(), &[0, 1]);
    assert_eq!(canvas.selected_connector_indices(), &[0]);
}

#[test]
fn test_select_in_rect_includes_connectors() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);  // bbox (0,30)-(200,170)
    canvas.add_oval(400.0, 100.0);  // bbox (300,30)-(500,170)
    canvas.add_connector(0, 1);
    // Rect covers both shapes and the connector midpoint between them
    canvas.select_in_rect(0.0, 0.0, 500.0, 200.0);
    assert_eq!(canvas.selected_indices(), &[0, 1]);
    assert_eq!(canvas.selected_connector_indices(), &[0]);
}

#[test]
fn test_select_at_clears_connector_selection() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    assert_eq!(canvas.selected_connector_indices(), &[0]);
    canvas.select_at(100.0, 100.0); // click one shape
    assert!(canvas.selected_connector_indices().is_empty());
}

// -- Copy/paste with connectors --

#[test]
fn test_copy_paste_includes_connectors_between_selected() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    // Original 2 shapes + 2 pasted shapes
    assert_eq!(canvas.shape_count(), 4);
    // Original connector + pasted connector
    assert_eq!(canvas.connector_count(), 2);
}

#[test]
fn test_copy_paste_connector_references_new_shapes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    // The pasted connector should reference shapes 2 and 3 (not 0 and 1)
    let pasted_conn = &canvas.connectors()[1];
    assert_eq!(pasted_conn.source(), 2);
    assert_eq!(pasted_conn.target(), 3);
}

#[test]
fn test_copy_only_connectors_between_selected_shapes() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);  // 0
    canvas.add_oval(400.0, 100.0);  // 1
    canvas.add_oval(700.0, 100.0);  // 2
    canvas.add_connector(0, 1);
    canvas.add_connector(1, 2);
    // Select only shapes 0 and 1
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    // Only the 0→1 connector should be pasted (not 1→2)
    assert_eq!(canvas.connector_count(), 3); // 2 original + 1 pasted
}

// -- Cut/delete with connectors --

#[test]
fn test_delete_selected_connectors() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    canvas.delete_selected();
    assert_eq!(canvas.shape_count(), 0);
    assert_eq!(canvas.connector_count(), 0);
}

#[test]
fn test_undo_delete_restores_connectors() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    canvas.add_oval(400.0, 100.0);
    canvas.add_connector(0, 1);
    canvas.select_all();
    canvas.delete_selected();
    assert_eq!(canvas.connector_count(), 0);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 2);
    assert_eq!(canvas.connector_count(), 1);
    assert_eq!(canvas.connectors()[0].source(), 0);
    assert_eq!(canvas.connectors()[0].target(), 1);
}

// -- ShapeKind and creation --

#[test]
fn test_shape_kind_default_is_oval() {
    let s = OvalShape::new(100.0, 200.0);
    assert_eq!(s.kind(), ShapeKind::Oval);
}

#[test]
fn test_create_circle() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Circle);
    assert_eq!(s.kind(), ShapeKind::Circle);
    assert_eq!(s.rx(), s.ry()); // equal radii
}

#[test]
fn test_create_rectangle() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    assert_eq!(s.kind(), ShapeKind::Rectangle);
    assert_eq!(s.rx(), 100.0);
    assert_eq!(s.ry(), 70.0);
}

#[test]
fn test_create_square() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Square);
    assert_eq!(s.kind(), ShapeKind::Square);
    assert_eq!(s.rx(), s.ry());
}

// -- Contains point per shape kind --

#[test]
fn test_rectangle_contains_point_inside() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    assert!(s.contains_point(100.0, 100.0)); // center
    assert!(s.contains_point(150.0, 130.0)); // inside bbox
}

#[test]
fn test_rectangle_contains_point_corner() {
    // Corner of bounding box IS inside a rectangle (unlike an oval)
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    assert!(s.contains_point(199.0, 169.0)); // near corner
}

#[test]
fn test_rectangle_contains_point_outside() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    assert!(!s.contains_point(250.0, 100.0)); // beyond rx
}

#[test]
fn test_oval_contains_point_corner_is_outside() {
    // Regression: corner of bounding box is OUTSIDE an oval
    let oval = OvalShape::new(100.0, 100.0);
    assert!(!oval.contains_point(199.0, 169.0));
}

// -- Resize constraints --

#[test]
fn test_circle_resize_enforces_equal_radii() {
    let mut s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Circle);
    s.resize(ResizeHandle::BottomRight, 250.0, 200.0);
    assert_eq!(s.rx(), s.ry());
}

#[test]
fn test_square_resize_enforces_equal_radii() {
    let mut s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Square);
    s.resize(ResizeHandle::BottomRight, 250.0, 200.0);
    assert_eq!(s.rx(), s.ry());
}

#[test]
fn test_rectangle_resize_unconstrained() {
    let mut s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    s.resize(ResizeHandle::BottomRight, 250.0, 200.0);
    assert_ne!(s.rx(), s.ry());
}

// -- Text box width --

#[test]
fn test_rectangle_text_box_width() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    // Rectangle text width: full width minus padding
    assert!(s.text_box_width() > 100.0 * std::f32::consts::SQRT_2);
}

// -- Canvas add shapes --

#[test]
fn test_canvas_add_circle() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Circle);
    assert_eq!(canvas.shape_count(), 1);
    assert_eq!(canvas.shapes()[0].kind(), ShapeKind::Circle);
}

#[test]
fn test_canvas_add_square() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Square);
    assert_eq!(canvas.shapes()[0].kind(), ShapeKind::Square);
}

#[test]
fn test_canvas_add_rectangle() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Rectangle);
    assert_eq!(canvas.shapes()[0].kind(), ShapeKind::Rectangle);
}

#[test]
fn test_canvas_add_oval_still_works() {
    let mut canvas = CanvasState::new();
    canvas.add_oval(100.0, 100.0);
    assert_eq!(canvas.shapes()[0].kind(), ShapeKind::Oval);
}

// -- Serialization with kind --

#[test]
fn test_shape_json_includes_kind() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Circle);
    let json = s.to_json();
    assert!(json.contains("Circle"));
}

#[test]
fn test_shape_json_roundtrip_with_kind() {
    let s = OvalShape::with_kind(100.0, 100.0, ShapeKind::Rectangle);
    let json = s.to_json();
    let restored = OvalShape::from_json(&json).unwrap();
    assert_eq!(restored.kind(), ShapeKind::Rectangle);
}

#[test]
fn test_shape_json_without_kind_defaults_oval() {
    // Backward compat: old JSON without kind field
    let json = r#"{"center_x":100,"center_y":100,"rx":100,"ry":70,"border_width":1,"text":""}"#;
    let restored = OvalShape::from_json(json).unwrap();
    assert_eq!(restored.kind(), ShapeKind::Oval);
}

// -- Copy/paste preserves kind --

#[test]
fn test_copy_paste_preserves_shape_kind() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Circle);
    canvas.select_at(100.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shapes()[1].kind(), ShapeKind::Circle);
}

#[test]
fn test_copy_paste_mixed_kinds() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Oval);
    canvas.add_shape(400.0, 100.0, ShapeKind::Rectangle);
    canvas.select_at(100.0, 100.0);
    canvas.toggle_selection_at(400.0, 100.0);
    let json = canvas.copy_selected().unwrap();
    canvas.paste_shapes(&json);
    assert_eq!(canvas.shapes()[2].kind(), ShapeKind::Oval);
    assert_eq!(canvas.shapes()[3].kind(), ShapeKind::Rectangle);
}

// -- Undo/redo with kind --

#[test]
fn test_undo_add_circle() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Circle);
    canvas.undo();
    assert_eq!(canvas.shape_count(), 0);
}

#[test]
fn test_redo_add_circle() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Circle);
    canvas.undo();
    canvas.redo();
    assert_eq!(canvas.shapes()[0].kind(), ShapeKind::Circle);
}

// -- Render data includes kind --

#[test]
fn test_render_data_includes_shape_kind() {
    let mut canvas = CanvasState::new();
    canvas.add_shape(100.0, 100.0, ShapeKind::Rectangle);
    let data = canvas.render_data(None);
    assert_eq!(data[0].kind, ShapeKind::Rectangle);
}

// -- Connector has no label --

#[test]
fn test_connector_no_label() {
    // Connector should only have source, target, curvature — no label
    let conn = Connector::new(0, 1);
    assert_eq!(conn.source(), 0);
    assert_eq!(conn.target(), 1);
    assert_eq!(conn.curvature(), 0.0);
    // No label() or toggle_label() methods should exist
}
