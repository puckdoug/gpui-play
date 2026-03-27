# Multi-Shape Support and Connector Simplification

## Design

### Shape Types
Add `ShapeKind` enum: `Oval`, `Circle`, `Rectangle`, `Square`. Unified `Shape` struct (renamed from `OvalShape`) with a `kind` field. All shapes share center, rx, ry, text, resize handles.

| Kind | Rendering | Resize Constraint | contains_point | text_box_width |
|------|-----------|-------------------|----------------|----------------|
| Oval | arc_to ellipse | none | ellipse eq | rx * sqrt(2) |
| Circle | arc_to ellipse | rx == ry | ellipse eq | rx * sqrt(2) |
| Rectangle | line_to rect | none | AABB | rx * 2 - 16 |
| Square | line_to rect | rx == ry | AABB | rx * 2 - 16 |

### Connector Simplification
- Remove `ConnectorLabel` enum, `label` field, `toggle_label()`
- Remove arrowhead fields and rendering
- Plain curves with midpoint drag handle only

### Menu
Shapes → Add Square, Add Rectangle, Add Oval, Add Circle.
Cmd-Shift-N creates the last-selected shape type (`last_shape_kind` on DrawTestView).

## Migration
Keep `OvalShape` as type alias `pub type OvalShape = Shape;` during transition so existing tests compile.

## TDD Tests (34 tests across 11 batches)
See tests/shape_test.rs for full red-phase tests.
