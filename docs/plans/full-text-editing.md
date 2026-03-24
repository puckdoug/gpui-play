# Full Text Editing in Canvas Shapes

## Goal
Make text in ovals fully editable: arrow keys, click-to-position, double-click word select, triple-click select all, selection highlighting.

## Phases

### Phase 1: Word Boundary Methods on TextInputState
- Add `word_start(offset)`, `word_end(offset)`, `select_word_at(offset)` using `unicode-segmentation`
- Pure state, unit-testable

### Phase 2: ShapeRenderData Selected Range
- Add `selected_range: Option<Range<usize>>` to `ShapeRenderData`
- Populate from `TextInputState::selected_range()` in `render_data()`

### Phase 3: Arrow Key Bindings and Actions
- Add Left, Right, SelectLeft, SelectRight, SelectAll actions + key bindings
- Wire action handlers on DrawTestView delegating to TextInputState

### Phase 4: Mouse Click-to-Position, Word Select, Select All
- Single click in text while editing → position cursor via `hit_test_text()`
- Double-click word → `select_word_at(offset)`
- Triple-click → `select_all()`
- `hit_test_text()` calls `shape_text()` + `index_for_position()` to map pixel position to byte offset

### Phase 5: Selection Rendering
- Paint highlight rectangles for selected range using `position_for_index()`
- Hide blinking cursor when selection is non-empty

## Mouse Event Dispatch (revised)
```
if editing:
    if click_count == 3 on editing shape → select_all()
    if click_count == 2 on editing shape → select_word_at(hit_test offset)
    if click_count == 1 on editing shape → move_to(hit_test offset)
    else → commit_editing(), fall through to normal handling
if not editing:
    if click_count == 2 on a shape → start_editing()
    else → normal select + drag
```
