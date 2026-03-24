use gpui_play::shape::{CanvasState, OvalShape, ShapeRenderData};
use gpui_play::text_input::TextInputState;

// -- Oval creation --

#[test]
fn test_oval_default_values() {
    let oval = OvalShape::new(100.0, 200.0);
    assert_eq!(oval.center(), (100.0, 200.0));
    assert_eq!(oval.rx(), 100.0);  // default horizontal radius
    assert_eq!(oval.ry(), 70.0);   // default vertical radius
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
    assert!(!oval.contains_point(0.0, 0.0));     // origin
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
    assert!(oval.contains_point(150.0, 0.0));   // inside wide but narrow
    assert!(!oval.contains_point(0.0, 25.0));    // outside vertically
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
    canvas.move_selected(200.0, 300.0);
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

    let mut editing = TextInputState::new("");
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
