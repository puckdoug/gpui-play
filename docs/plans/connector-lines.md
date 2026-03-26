# Curved Connector Lines Between Ovals

## Design

Option-click-drag from one oval to another draws a curved connector line. The line:
- Starts/ends on the oval borders (not centers)
- Uses a quadratic bezier (rendered as cubic via GPUI's `cubic_bezier_to`)
- Has a midpoint drag handle to adjust curvature
- Has an arrowhead at the target end
- Has a "+" label just behind the arrow, slightly above the line
- Option-clicking the "+" toggles it to "-"

## Data Model (src/shape.rs)

```
Connector { source: usize, target: usize, curvature: f32, label: ConnectorLabel }
ConnectorLabel { Plus, Minus }
```

- `CanvasState::connectors: Vec<Connector>`
- `OvalShape::point_on_border(angle) -> (f32, f32)` — parametric ellipse point
- `Connector::control_point(shapes)` — midpoint + perpendicular offset by curvature
- `Connector::start_point/end_point(shapes)` — border points angled toward control point
- `Connector::midpoint(shapes)` — bezier t=0.5 point for drag handle
- `ConnectorRenderData` — all precomputed positions for the paint closure

## Geometry

- Border point at angle θ: `(cx + rx*cos(θ), cy + ry*sin(θ))`
- Control point: midpoint of centers + curvature * perpendicular to center line
- Quadratic → cubic: `ctrl_a = start + 2/3*(Q - start)`, `ctrl_b = end + 2/3*(Q - end)`
- Arrowhead: two wings at ±25° from tangent direction at endpoint
- Label position: offset behind arrow tip, above the tangent line

## Mouse Interaction (src/bin/draw_test.rs)

- Option+mousedown on oval → start connector creation
- Option+mousemove → preview line to cursor
- Option+mouseup on different oval → create connector
- Click midpoint handle → drag to adjust curvature
- Option+click on "+"/"-" label → toggle label

## Undo/Redo

- `AddConnector`, `RemoveConnector`, `ModifyConnector` undo variants
- `DeleteShapes` must cascade: remove connectors referencing deleted shapes, reindex remaining

## TDD Tests (20 tests)

### Connector data (4)
1. Creation stores source/target
2. Default curvature is 0.0
3. Default label is Plus
4. Toggle switches Plus↔Minus

### Border geometry (4)
5. Angle 0 → right edge
6. Angle -π/2 → top edge
7. Angle π/4 → correct diagonal point
8. Connector endpoints are on oval borders

### Bezier control point (3)
9. Zero curvature → midpoint of centers
10. Positive curvature → perpendicular offset one way
11. Negative curvature → perpendicular offset other way

### Canvas management (5)
12. Add connector increments count
13. Add connector is undoable
14. Remove connector decrements count
15. Multiple connectors between same shapes allowed
16. Delete shape removes connected connectors and reindexes

### Midpoint + render data (4)
17. Midpoint at t=0.5 for zero curvature
18. Midpoint changes with curvature
19. ConnectorRenderData produced for each connector
20. Render data has correct endpoints and control points
