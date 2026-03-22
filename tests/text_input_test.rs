use gpui_play::text_input::TextInputState;

// -- Construction --

#[test]
fn test_new_with_content() {
    let state = TextInputState::new("hello");
    assert_eq!(state.content(), "hello");
    assert_eq!(state.cursor_offset(), 0);
    assert!(state.selected_range().is_empty());
}

#[test]
fn test_new_empty() {
    let state = TextInputState::new("");
    assert_eq!(state.content(), "");
    assert_eq!(state.cursor_offset(), 0);
}

// -- Cursor movement --

#[test]
fn test_move_right() {
    let mut state = TextInputState::new("abc");
    state.move_right();
    assert_eq!(state.cursor_offset(), 1);
    state.move_right();
    assert_eq!(state.cursor_offset(), 2);
    state.move_right();
    assert_eq!(state.cursor_offset(), 3);
    // Should not move past end
    state.move_right();
    assert_eq!(state.cursor_offset(), 3);
}

#[test]
fn test_move_left() {
    let mut state = TextInputState::new("abc");
    state.move_to(3);
    state.move_left();
    assert_eq!(state.cursor_offset(), 2);
    state.move_left();
    assert_eq!(state.cursor_offset(), 1);
    state.move_left();
    assert_eq!(state.cursor_offset(), 0);
    // Should not move past start
    state.move_left();
    assert_eq!(state.cursor_offset(), 0);
}

#[test]
fn test_move_to_home_end() {
    let mut state = TextInputState::new("hello");
    state.move_to_end();
    assert_eq!(state.cursor_offset(), 5);
    state.move_to_home();
    assert_eq!(state.cursor_offset(), 0);
}

#[test]
fn test_move_right_with_multibyte() {
    let mut state = TextInputState::new("café");
    // c-a-f-é (é is 2 bytes in UTF-8)
    state.move_right(); // past 'c'
    assert_eq!(state.cursor_offset(), 1);
    state.move_right(); // past 'a'
    assert_eq!(state.cursor_offset(), 2);
    state.move_right(); // past 'f'
    assert_eq!(state.cursor_offset(), 3);
    state.move_right(); // past 'é' (2 bytes)
    assert_eq!(state.cursor_offset(), 5);
}

// -- Text insertion --

#[test]
fn test_insert_at_cursor() {
    let mut state = TextInputState::new("");
    state.insert("hello");
    assert_eq!(state.content(), "hello");
    assert_eq!(state.cursor_offset(), 5);
}

#[test]
fn test_insert_in_middle() {
    let mut state = TextInputState::new("hllo");
    state.move_right(); // after 'h'
    state.insert("e");
    assert_eq!(state.content(), "hello");
    assert_eq!(state.cursor_offset(), 2);
}

#[test]
fn test_insert_replaces_selection() {
    let mut state = TextInputState::new("hello world");
    state.move_to(0);
    state.select_to(5); // select "hello"
    state.insert("goodbye");
    assert_eq!(state.content(), "goodbye world");
    assert_eq!(state.cursor_offset(), 7);
}

// -- Backspace / Delete --

#[test]
fn test_backspace() {
    let mut state = TextInputState::new("hello");
    state.move_to_end();
    state.backspace();
    assert_eq!(state.content(), "hell");
    assert_eq!(state.cursor_offset(), 4);
}

#[test]
fn test_backspace_at_start() {
    let mut state = TextInputState::new("hello");
    state.backspace(); // cursor at 0, should do nothing
    assert_eq!(state.content(), "hello");
}

#[test]
fn test_backspace_deletes_selection() {
    let mut state = TextInputState::new("hello world");
    state.select_to(5); // select "hello"
    state.backspace();
    assert_eq!(state.content(), " world");
    assert_eq!(state.cursor_offset(), 0);
}

#[test]
fn test_delete_forward() {
    let mut state = TextInputState::new("hello");
    state.delete();
    assert_eq!(state.content(), "ello");
    assert_eq!(state.cursor_offset(), 0);
}

#[test]
fn test_delete_at_end() {
    let mut state = TextInputState::new("hello");
    state.move_to_end();
    state.delete(); // should do nothing
    assert_eq!(state.content(), "hello");
}

// -- Selection --

#[test]
fn test_select_right() {
    let mut state = TextInputState::new("hello");
    state.select_right();
    assert_eq!(state.selected_range(), 0..1);
    state.select_right();
    assert_eq!(state.selected_range(), 0..2);
}

#[test]
fn test_select_left() {
    let mut state = TextInputState::new("hello");
    state.move_to_end();
    state.select_left();
    assert_eq!(state.selected_range(), 4..5);
}

#[test]
fn test_select_all() {
    let mut state = TextInputState::new("hello");
    state.select_all();
    assert_eq!(state.selected_range(), 0..5);
}

#[test]
fn test_move_collapses_selection() {
    let mut state = TextInputState::new("hello");
    state.select_all();
    state.move_right();
    assert!(state.selected_range().is_empty());
    assert_eq!(state.cursor_offset(), 5);
}

#[test]
fn test_move_left_collapses_to_start() {
    let mut state = TextInputState::new("hello");
    state.select_all();
    state.move_left();
    assert!(state.selected_range().is_empty());
    assert_eq!(state.cursor_offset(), 0);
}

// -- UTF-16 conversion --

#[test]
fn test_utf16_offset_ascii() {
    let state = TextInputState::new("hello");
    assert_eq!(state.offset_to_utf16(3), 3);
    assert_eq!(state.offset_from_utf16(3), 3);
}

#[test]
fn test_utf16_offset_multibyte() {
    // '€' is 3 bytes in UTF-8, 1 unit in UTF-16
    let state = TextInputState::new("€");
    assert_eq!(state.offset_to_utf16(3), 1); // 3 UTF-8 bytes -> 1 UTF-16 unit
    assert_eq!(state.offset_from_utf16(1), 3); // 1 UTF-16 unit -> 3 UTF-8 bytes
}

#[test]
fn test_utf16_offset_emoji() {
    // '😀' is 4 bytes in UTF-8, 2 units in UTF-16 (surrogate pair)
    let state = TextInputState::new("😀");
    assert_eq!(state.offset_to_utf16(4), 2);
    assert_eq!(state.offset_from_utf16(2), 4);
}
