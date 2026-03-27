use serde::{Deserialize, Serialize};

use crate::text_input::TextInputState;

const MIN_RADIUS: f32 = 20.0;
const PASTE_OFFSET: f32 = 20.0;
const RECT_TEXT_PADDING: f32 = 16.0;

/// The type of shape on the canvas.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShapeKind {
    Oval,
    Circle,
    Rectangle,
    Square,
}

/// A resize handle on the bounding box of an oval.
/// Corners allow free resize (both axes), midpoints constrain to one axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl ResizeHandle {
    /// All 8 handles in clockwise order starting from TopLeft.
    pub const ALL: [ResizeHandle; 8] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Right,
        Self::BottomRight,
        Self::Bottom,
        Self::BottomLeft,
        Self::Left,
    ];
}

/// A shape on the canvas.
pub struct OvalShape {
    kind: ShapeKind,
    center_x: f32,
    center_y: f32,
    rx: f32,
    ry: f32,
    border_width: f32,
    text: String,
}

impl OvalShape {
    /// Create a new oval at the given center with default size (100x70) and 1pt border.
    pub fn new(cx: f32, cy: f32) -> Self {
        Self {
            kind: ShapeKind::Oval,
            center_x: cx,
            center_y: cy,
            rx: 100.0,
            ry: 70.0,
            border_width: 1.0,
            text: String::new(),
        }
    }

    /// Create a new shape of the given kind at the given center with default size.
    pub fn with_kind(cx: f32, cy: f32, kind: ShapeKind) -> Self {
        let (rx, ry) = match kind {
            ShapeKind::Oval | ShapeKind::Rectangle => (100.0, 70.0),
            ShapeKind::Circle | ShapeKind::Square => (70.0, 70.0),
        };
        Self {
            kind,
            center_x: cx,
            center_y: cy,
            rx,
            ry,
            border_width: 1.0,
            text: String::new(),
        }
    }

    /// Create a new oval with explicit size.
    pub fn with_size(cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        Self {
            kind: ShapeKind::Oval,
            center_x: cx,
            center_y: cy,
            rx,
            ry,
            border_width: 1.0,
            text: String::new(),
        }
    }

    pub fn kind(&self) -> ShapeKind {
        self.kind
    }

    pub fn center(&self) -> (f32, f32) {
        (self.center_x, self.center_y)
    }

    pub fn rx(&self) -> f32 {
        self.rx
    }

    pub fn ry(&self) -> f32 {
        self.ry
    }

    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn move_to(&mut self, cx: f32, cy: f32) {
        self.center_x = cx;
        self.center_y = cy;
    }

    /// Returns the width available for text inside the shape.
    pub fn text_box_width(&self) -> f32 {
        match self.kind {
            ShapeKind::Oval | ShapeKind::Circle => self.rx * std::f32::consts::SQRT_2,
            ShapeKind::Rectangle | ShapeKind::Square => (self.rx * 2.0 - RECT_TEXT_PADDING).max(0.0),
        }
    }

    /// Return the pixel position of a resize handle on this oval's bounding box.
    pub fn handle_position(&self, handle: ResizeHandle) -> (f32, f32) {
        match handle {
            ResizeHandle::TopLeft => (self.center_x - self.rx, self.center_y - self.ry),
            ResizeHandle::Top => (self.center_x, self.center_y - self.ry),
            ResizeHandle::TopRight => (self.center_x + self.rx, self.center_y - self.ry),
            ResizeHandle::Right => (self.center_x + self.rx, self.center_y),
            ResizeHandle::BottomRight => (self.center_x + self.rx, self.center_y + self.ry),
            ResizeHandle::Bottom => (self.center_x, self.center_y + self.ry),
            ResizeHandle::BottomLeft => (self.center_x - self.rx, self.center_y + self.ry),
            ResizeHandle::Left => (self.center_x - self.rx, self.center_y),
        }
    }

    /// Hit-test all 8 resize handles. Returns the first handle within
    /// `handle_radius` pixels of the point, or None.
    pub fn hit_test_handle(&self, px: f32, py: f32, handle_radius: f32) -> Option<ResizeHandle> {
        for handle in ResizeHandle::ALL {
            let (hx, hy) = self.handle_position(handle);
            let dx = px - hx;
            let dy = py - hy;
            if dx * dx + dy * dy <= handle_radius * handle_radius {
                return Some(handle);
            }
        }
        None
    }

    /// Resize the shape by dragging a handle to a new position.
    /// Corner handles change both rx and ry (free resize).
    /// Midpoint handles change only the relevant axis.
    /// Circle and Square enforce rx == ry.
    pub fn resize(&mut self, handle: ResizeHandle, px: f32, py: f32) {
        match handle {
            ResizeHandle::Right => {
                self.rx = (px - self.center_x).abs().max(MIN_RADIUS);
            }
            ResizeHandle::Left => {
                self.rx = (self.center_x - px).abs().max(MIN_RADIUS);
            }
            ResizeHandle::Bottom => {
                self.ry = (py - self.center_y).abs().max(MIN_RADIUS);
            }
            ResizeHandle::Top => {
                self.ry = (self.center_y - py).abs().max(MIN_RADIUS);
            }
            ResizeHandle::TopLeft | ResizeHandle::TopRight
            | ResizeHandle::BottomLeft | ResizeHandle::BottomRight => {
                self.rx = (px - self.center_x).abs().max(MIN_RADIUS);
                self.ry = (py - self.center_y).abs().max(MIN_RADIUS);
            }
        }
        // Enforce equal radii for circle and square
        match self.kind {
            ShapeKind::Circle | ShapeKind::Square => {
                let r = self.rx.max(self.ry);
                self.rx = r;
                self.ry = r;
            }
            _ => {}
        }
    }

    /// Point on the shape boundary at the given angle (radians).
    pub fn point_on_border(&self, angle: f32) -> (f32, f32) {
        match self.kind {
            ShapeKind::Oval | ShapeKind::Circle => (
                self.center_x + self.rx * angle.cos(),
                self.center_y + self.ry * angle.sin(),
            ),
            ShapeKind::Rectangle | ShapeKind::Square => {
                // Ray-rectangle intersection
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let tx = if cos_a.abs() > 1e-6 { self.rx / cos_a.abs() } else { f32::INFINITY };
                let ty = if sin_a.abs() > 1e-6 { self.ry / sin_a.abs() } else { f32::INFINITY };
                let t = tx.min(ty);
                (self.center_x + t * cos_a, self.center_y + t * sin_a)
            }
        }
    }

    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        match self.kind {
            ShapeKind::Oval | ShapeKind::Circle => {
                let dx = (px - self.center_x) / self.rx;
                let dy = (py - self.center_y) / self.ry;
                (dx * dx + dy * dy) <= 1.0
            }
            ShapeKind::Rectangle | ShapeKind::Square => {
                (px - self.center_x).abs() <= self.rx
                    && (py - self.center_y).abs() <= self.ry
            }
        }
    }

    fn clone_data(&self) -> OvalShapeData {
        OvalShapeData {
            kind: self.kind,
            center_x: self.center_x,
            center_y: self.center_y,
            rx: self.rx,
            ry: self.ry,
            border_width: self.border_width,
            text: self.text.clone(),
        }
    }

    fn restore_from(&mut self, data: &OvalShapeData) {
        self.kind = data.kind;
        self.center_x = data.center_x;
        self.center_y = data.center_y;
        self.rx = data.rx;
        self.ry = data.ry;
        self.border_width = data.border_width;
        self.text = data.text.clone();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.clone_data()).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let data: OvalShapeData = serde_json::from_str(json).ok()?;
        let mut shape = OvalShape::with_kind(data.center_x, data.center_y, data.kind);
        shape.restore_from(&data);
        Some(shape)
    }
}

/// A curved connector line between two shapes.
#[derive(Clone, Debug)]
pub struct Connector {
    source: usize,
    target: usize,
    curvature: f32,
}

impl Connector {
    pub fn new(source: usize, target: usize) -> Self {
        Self {
            source,
            target,
            curvature: 0.0,
        }
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn curvature(&self) -> f32 {
        self.curvature
    }

    pub fn set_curvature(&mut self, curvature: f32) {
        self.curvature = curvature;
    }

    /// Quadratic bezier control point: midpoint of centers + perpendicular offset.
    pub fn control_point(&self, shapes: &[OvalShape]) -> (f32, f32) {
        let (ax, ay) = shapes[self.source].center();
        let (bx, by) = shapes[self.target].center();
        let mx = (ax + bx) / 2.0;
        let my = (ay + by) / 2.0;
        // Perpendicular to the center-to-center line (rotated 90° CCW)
        let dx = bx - ax;
        let dy = by - ay;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let perp_x = -dy / len;
        let perp_y = dx / len;
        (mx + self.curvature * perp_x, my + self.curvature * perp_y)
    }

    /// Start point: on the source oval border, angled toward the control point.
    pub fn start_point(&self, shapes: &[OvalShape]) -> (f32, f32) {
        let (cx, cy) = shapes[self.source].center();
        let (cpx, cpy) = self.control_point(shapes);
        let angle = (cpy - cy).atan2(cpx - cx);
        shapes[self.source].point_on_border(angle)
    }

    /// End point: on the target oval border, angled toward the control point.
    pub fn end_point(&self, shapes: &[OvalShape]) -> (f32, f32) {
        let (cx, cy) = shapes[self.target].center();
        let (cpx, cpy) = self.control_point(shapes);
        let angle = (cpy - cy).atan2(cpx - cx);
        shapes[self.target].point_on_border(angle)
    }

    /// Visual midpoint of the quadratic bezier at t=0.5.
    /// For quadratic: P(0.5) = 0.25*start + 0.5*control + 0.25*end
    pub fn midpoint(&self, shapes: &[OvalShape]) -> (f32, f32) {
        let (sx, sy) = self.start_point(shapes);
        let (cx, cy) = self.control_point(shapes);
        let (ex, ey) = self.end_point(shapes);
        (
            0.25 * sx + 0.5 * cx + 0.25 * ex,
            0.25 * sy + 0.5 * cy + 0.25 * ey,
        )
    }

    fn clone_data(&self) -> ConnectorData {
        ConnectorData {
            source: self.source,
            target: self.target,
            curvature: self.curvature,
        }
    }
}

#[derive(Clone, Debug)]
struct ConnectorData {
    source: usize,
    target: usize,
    curvature: f32,
}

fn default_oval_kind() -> ShapeKind {
    ShapeKind::Oval
}

/// Snapshot of a shape for undo/redo and serialization.
#[derive(Clone, Serialize, Deserialize)]
struct OvalShapeData {
    #[serde(default = "default_oval_kind")]
    kind: ShapeKind,
    center_x: f32,
    center_y: f32,
    rx: f32,
    ry: f32,
    border_width: f32,
    text: String,
}

/// Clipboard format for copy/paste including shapes and connectors.
#[derive(Clone, Serialize, Deserialize)]
struct ClipboardData {
    shapes: Vec<OvalShapeData>,
    connectors: Vec<ClipboardConnector>,
}

/// Connector reference in clipboard data, with indices relative to the copied shape set.
#[derive(Clone, Serialize, Deserialize)]
struct ClipboardConnector {
    source: usize,
    target: usize,
    curvature: f32,
}

/// An undo entry for canvas operations.
#[derive(Clone)]
enum UndoAction {
    AddShape {
        index: usize,
        data: OvalShapeData,
    },
    MoveShape {
        index: usize,
        old_data: OvalShapeData,
    },
    ResizeShape {
        index: usize,
        old_data: OvalShapeData,
    },
    PasteShapes {
        start_index: usize,
        shapes_data: Vec<OvalShapeData>,
        conn_start_index: usize,
        connectors_data: Vec<ConnectorData>,
    },
    DeleteShapes {
        /// (original_index, data) sorted by index descending for correct re-insertion
        shapes: Vec<(usize, OvalShapeData)>,
        /// Connectors removed as part of the delete (for undo restoration)
        removed_connectors: Vec<(usize, ConnectorData)>,
    },
    AddConnector {
        index: usize,
    },
    RemoveConnector {
        index: usize,
        data: ConnectorData,
    },
}

/// State for a drawing canvas containing shapes.
pub struct CanvasState {
    shapes: Vec<OvalShape>,
    connectors: Vec<Connector>,
    selected: Vec<usize>,
    selected_connectors: Vec<usize>,
    editing: Option<usize>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
    resize_pre_data: Option<(usize, OvalShapeData)>,
}

impl CanvasState {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            connectors: Vec::new(),
            selected: Vec::new(),
            selected_connectors: Vec::new(),
            editing: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            resize_pre_data: None,
        }
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    pub fn shapes(&self) -> &[OvalShape] {
        &self.shapes
    }

    /// Returns the first selected index (backwards-compatible with single-select callers).
    pub fn selected(&self) -> Option<usize> {
        self.selected.first().copied()
    }

    /// Returns all selected shape indices.
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected
    }

    /// Returns all selected connector indices.
    pub fn selected_connector_indices(&self) -> &[usize] {
        &self.selected_connectors
    }

    /// Mutable access to selected connector indices.
    pub fn selected_connectors_mut(&mut self) -> &mut Vec<usize> {
        &mut self.selected_connectors
    }

    pub fn editing(&self) -> Option<usize> {
        self.editing
    }

    /// Start editing the shape at the given index. Also selects only it.
    /// Ignored if the index is out of bounds.
    pub fn start_editing(&mut self, index: usize) {
        if index < self.shapes.len() {
            self.editing = Some(index);
            self.selected = vec![index];
        }
    }

    /// Stop editing the current shape.
    pub fn stop_editing(&mut self) {
        self.editing = None;
    }

    /// Set the text of a shape at the given index.
    pub fn set_shape_text(&mut self, index: usize, text: &str) {
        if index < self.shapes.len() {
            self.shapes[index].set_text(text);
        }
    }

    pub fn add_shape(&mut self, cx: f32, cy: f32, kind: ShapeKind) {
        let shape = OvalShape::with_kind(cx, cy, kind);
        let data = shape.clone_data();
        let index = self.shapes.len();
        self.shapes.push(shape);
        self.undo_stack.push(UndoAction::AddShape { index, data });
        self.redo_stack.clear();
    }

    pub fn add_oval(&mut self, cx: f32, cy: f32) {
        self.add_shape(cx, cy, ShapeKind::Oval);
    }

    /// Select the topmost shape at the given point, or deselect all.
    /// Replaces any existing selection. Clears editing state if selection changes.
    pub fn select_at(&mut self, px: f32, py: f32) {
        let new_selected = self
            .shapes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, shape)| shape.contains_point(px, py))
            .map(|(i, _)| i);
        if new_selected != self.editing {
            self.editing = None;
        }
        self.selected = new_selected.into_iter().collect();
        self.selected_connectors.clear();
    }

    /// Select all shapes and connectors on the canvas.
    pub fn select_all(&mut self) {
        self.selected = (0..self.shapes.len()).collect();
        self.selected_connectors = (0..self.connectors.len()).collect();
        self.editing = None;
    }

    /// Select all shapes and connectors that overlap the given rectangle.
    pub fn select_in_rect(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let left = x0.min(x1);
        let right = x0.max(x1);
        let top = y0.min(y1);
        let bottom = y0.max(y1);
        self.selected = self
            .shapes
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                let (cx, cy) = s.center();
                let s_left = cx - s.rx();
                let s_right = cx + s.rx();
                let s_top = cy - s.ry();
                let s_bottom = cy + s.ry();
                s_left < right && s_right > left && s_top < bottom && s_bottom > top
            })
            .map(|(i, _)| i)
            .collect();
        // Select connectors whose midpoint is in the rect
        self.selected_connectors = self
            .connectors
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                let (mx, my) = c.midpoint(&self.shapes);
                mx >= left && mx <= right && my >= top && my <= bottom
            })
            .map(|(i, _)| i)
            .collect();
        self.editing = None;
    }

    /// Select a connector near the given point, or clear connector selection.
    /// Uses point-to-curve distance by sampling the bezier.
    pub fn select_connector_at(&mut self, px: f32, py: f32, tolerance: f32) -> Option<usize> {
        let tol_sq = tolerance * tolerance;
        for (ci, c) in self.connectors.iter().enumerate().rev() {
            let s = c.start_point(&self.shapes);
            let cp = c.control_point(&self.shapes);
            let e = c.end_point(&self.shapes);
            // Sample quadratic bezier at 20 points
            for step in 0..=20 {
                let t = step as f32 / 20.0;
                let u = 1.0 - t;
                let x = u * u * s.0 + 2.0 * u * t * cp.0 + t * t * e.0;
                let y = u * u * s.1 + 2.0 * u * t * cp.1 + t * t * e.1;
                let dx = px - x;
                let dy = py - y;
                if dx * dx + dy * dy <= tol_sq {
                    return Some(ci);
                }
            }
        }
        None
    }

    /// Toggle a shape at the given point in/out of the selection (shift-click).
    pub fn toggle_selection_at(&mut self, px: f32, py: f32) {
        let hit = self
            .shapes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, shape)| shape.contains_point(px, py))
            .map(|(i, _)| i);
        if let Some(idx) = hit {
            if let Some(pos) = self.selected.iter().position(|&i| i == idx) {
                self.selected.remove(pos);
            } else {
                self.selected.push(idx);
                self.selected.sort();
            }
        }
        self.editing = None;
    }

    /// Hit-test resize handles on the selected shape (single-select only).
    /// Returns `Some((shape_index, handle))` if a handle is hit, or `None`.
    pub fn hit_test_handle(&self, px: f32, py: f32, handle_radius: f32) -> Option<(usize, ResizeHandle)> {
        if self.selected.len() != 1 {
            return None;
        }
        let index = self.selected[0];
        let handle = self.shapes[index].hit_test_handle(px, py, handle_radius)?;
        Some((index, handle))
    }

    /// Begin a resize operation. Snapshots the selected shape for undo.
    pub fn begin_resize(&mut self) {
        if let Some(&index) = self.selected.first() {
            self.resize_pre_data = Some((index, self.shapes[index].clone_data()));
        }
    }

    /// Update the resize in progress (mutates shape, no undo entry).
    pub fn update_resize(&mut self, handle: ResizeHandle, px: f32, py: f32) {
        if let Some((index, _)) = &self.resize_pre_data {
            self.shapes[*index].resize(handle, px, py);
        }
    }

    /// Commit the resize, pushing a single undo entry.
    pub fn commit_resize(&mut self) {
        if let Some((index, old_data)) = self.resize_pre_data.take() {
            self.undo_stack.push(UndoAction::ResizeShape { index, old_data });
            self.redo_stack.clear();
        }
    }

    /// Move all selected shapes by a delta.
    pub fn move_selected_by(&mut self, dx: f32, dy: f32) {
        for &index in &self.selected {
            let old_data = self.shapes[index].clone_data();
            let (cx, cy) = self.shapes[index].center();
            self.shapes[index].move_to(cx + dx, cy + dy);
            self.undo_stack.push(UndoAction::MoveShape { index, old_data });
            self.redo_stack.clear();
        }
    }

    /// Serialize all selected shapes and connectors between them as JSON.
    pub fn copy_selected(&self) -> Option<String> {
        if self.selected.is_empty() {
            return None;
        }
        let shapes: Vec<OvalShapeData> = self
            .selected
            .iter()
            .map(|&i| self.shapes[i].clone_data())
            .collect();
        // Include connectors where both source and target are in the selection
        let connectors: Vec<ClipboardConnector> = self
            .connectors
            .iter()
            .filter(|c| self.selected.contains(&c.source) && self.selected.contains(&c.target))
            .map(|c| {
                // Remap indices to positions within the selected set
                let source_pos = self.selected.iter().position(|&i| i == c.source).unwrap();
                let target_pos = self.selected.iter().position(|&i| i == c.target).unwrap();
                ClipboardConnector {
                    source: source_pos,
                    target: target_pos,
                    curvature: c.curvature,
                }
            })
            .collect();
        let clipboard = ClipboardData { shapes, connectors };
        serde_json::to_string(&clipboard).ok()
    }

    /// Paste shapes and connectors from JSON. Adds them with an offset,
    /// preserving relative spacing. Selects the newly pasted shapes.
    pub fn paste_shapes(&mut self, json: &str) {
        // Try new format first, fall back to old Vec<OvalShapeData> format
        let (shapes_data, connector_data) = if let Ok(clipboard) =
            serde_json::from_str::<ClipboardData>(json)
        {
            (clipboard.shapes, clipboard.connectors)
        } else if let Ok(shapes) = serde_json::from_str::<Vec<OvalShapeData>>(json) {
            (shapes, Vec::new())
        } else {
            return;
        };
        if shapes_data.is_empty() {
            return;
        }
        let start_index = self.shapes.len();
        for data in &shapes_data {
            let mut shape = OvalShape::with_kind(
                data.center_x + PASTE_OFFSET,
                data.center_y + PASTE_OFFSET,
                data.kind,
            );
            shape.rx = data.rx;
            shape.ry = data.ry;
            shape.border_width = data.border_width;
            shape.text = data.text.clone();
            self.shapes.push(shape);
        }
        let count = shapes_data.len();
        self.selected = (start_index..start_index + count).collect();

        // Paste connectors with remapped indices
        let conn_start = self.connectors.len();
        for cc in &connector_data {
            let mut conn = Connector::new(start_index + cc.source, start_index + cc.target);
            conn.curvature = cc.curvature;
            self.connectors.push(conn);
        }
        let conn_count = connector_data.len();
        self.selected_connectors = (conn_start..conn_start + conn_count).collect();

        // Store data for undo/redo
        let pasted_data: Vec<OvalShapeData> = (start_index..start_index + count)
            .map(|i| self.shapes[i].clone_data())
            .collect();
        let pasted_connectors: Vec<ConnectorData> = (conn_start..conn_start + conn_count)
            .map(|i| self.connectors[i].clone_data())
            .collect();
        self.undo_stack.push(UndoAction::PasteShapes {
            start_index,
            shapes_data: pasted_data,
            conn_start_index: conn_start,
            connectors_data: pasted_connectors,
        });
        self.redo_stack.clear();
    }

    /// Delete all selected shapes. Single undo entry restores them all.
    /// Also removes any connectors referencing deleted shapes and reindexes remaining.
    pub fn delete_selected(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        let mut indices = self.selected.clone();
        indices.sort();

        // Remove connectors that are selected or reference deleted shapes (in reverse)
        let mut removed_connectors: Vec<(usize, ConnectorData)> = Vec::new();
        for ci in (0..self.connectors.len()).rev() {
            let c = &self.connectors[ci];
            if self.selected_connectors.contains(&ci)
                || indices.contains(&c.source)
                || indices.contains(&c.target)
            {
                let data = self.connectors[ci].clone_data();
                self.connectors.remove(ci);
                removed_connectors.push((ci, data));
            }
        }

        // Reindex remaining connectors: decrement source/target for each deleted index below them
        for c in &mut self.connectors {
            for &del_idx in indices.iter().rev() {
                if c.source > del_idx {
                    c.source -= 1;
                }
                if c.target > del_idx {
                    c.target -= 1;
                }
            }
        }

        // Remove shapes in reverse order to preserve indices
        let mut removed: Vec<(usize, OvalShapeData)> = Vec::new();
        for &idx in indices.iter().rev() {
            let data = self.shapes[idx].clone_data();
            self.shapes.remove(idx);
            removed.push((idx, data));
        }
        self.selected.clear();
        self.selected_connectors.clear();
        self.editing = None;
        self.undo_stack.push(UndoAction::DeleteShapes {
            shapes: removed,
            removed_connectors,
        });
        self.redo_stack.clear();
    }

    // -- Connector management --

    pub fn connector_count(&self) -> usize {
        self.connectors.len()
    }

    pub fn connectors(&self) -> &[Connector] {
        &self.connectors
    }

    pub fn add_connector(&mut self, source: usize, target: usize) {
        let index = self.connectors.len();
        self.connectors.push(Connector::new(source, target));
        self.undo_stack.push(UndoAction::AddConnector { index });
        self.redo_stack.clear();
    }

    pub fn remove_connector(&mut self, index: usize) {
        let data = self.connectors[index].clone_data();
        self.connectors.remove(index);
        self.undo_stack.push(UndoAction::RemoveConnector { index, data });
        self.redo_stack.clear();
    }

    pub fn set_connector_curvature(&mut self, index: usize, curvature: f32) {
        if index < self.connectors.len() {
            self.connectors[index].set_curvature(curvature);
        }
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                UndoAction::AddShape { index, .. } => {
                    self.shapes.remove(*index);
                    self.selected.retain(|&i| i != *index);
                }
                UndoAction::MoveShape { index, old_data }
                | UndoAction::ResizeShape { index, old_data } => {
                    let current_data = self.shapes[*index].clone_data();
                    let redo_action = match &action {
                        UndoAction::MoveShape { .. } => UndoAction::MoveShape {
                            index: *index,
                            old_data: current_data,
                        },
                        UndoAction::ResizeShape { .. } => UndoAction::ResizeShape {
                            index: *index,
                            old_data: current_data,
                        },
                        _ => unreachable!(),
                    };
                    self.shapes[*index].restore_from(old_data);
                    self.redo_stack.push(redo_action);
                    return;
                }
                UndoAction::PasteShapes { start_index, shapes_data, conn_start_index, connectors_data } => {
                    // Remove pasted connectors first
                    for _ in 0..connectors_data.len() {
                        if *conn_start_index < self.connectors.len() {
                            self.connectors.remove(*conn_start_index);
                        }
                    }
                    // Remove pasted shapes
                    for _ in 0..shapes_data.len() {
                        self.shapes.remove(*start_index);
                    }
                    // Reindex connectors that referenced pasted shapes
                    self.connectors.retain(|c| c.source < self.shapes.len() && c.target < self.shapes.len());
                    self.selected.clear();
                    self.selected_connectors.clear();
                }
                UndoAction::DeleteShapes { shapes, removed_connectors } => {
                    // Re-insert shapes in forward order (stored in reverse)
                    for (idx, data) in shapes.iter().rev() {
                        let mut oval = OvalShape::new(data.center_x, data.center_y);
                        oval.restore_from(data);
                        self.shapes.insert(*idx, oval);
                    }
                    // Undo connector reindexing: increment back
                    let mut sorted_indices: Vec<usize> = shapes.iter().map(|(i, _)| *i).collect();
                    sorted_indices.sort();
                    for c in &mut self.connectors {
                        for &del_idx in &sorted_indices {
                            if c.source >= del_idx {
                                c.source += 1;
                            }
                            if c.target >= del_idx {
                                c.target += 1;
                            }
                        }
                    }
                    // Re-insert removed connectors (stored in reverse removal order)
                    for (ci, cd) in removed_connectors.iter().rev() {
                        let mut conn = Connector::new(cd.source, cd.target);
                        conn.curvature = cd.curvature;
                        self.connectors.insert(*ci, conn);
                    }
                }
                UndoAction::AddConnector { index } => {
                    self.connectors.remove(*index);
                }
                UndoAction::RemoveConnector { index, data } => {
                    let mut conn = Connector::new(data.source, data.target);
                    conn.curvature = data.curvature;
                    self.connectors.insert(*index, conn);
                }
            }
            self.redo_stack.push(action);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                UndoAction::AddShape { index, data } => {
                    let mut oval = OvalShape::new(data.center_x, data.center_y);
                    oval.restore_from(data);
                    self.shapes.insert(*index, oval);
                }
                UndoAction::MoveShape { index, old_data }
                | UndoAction::ResizeShape { index, old_data } => {
                    let current_data = self.shapes[*index].clone_data();
                    let undo_action = match &action {
                        UndoAction::MoveShape { .. } => UndoAction::MoveShape {
                            index: *index,
                            old_data: current_data,
                        },
                        UndoAction::ResizeShape { .. } => UndoAction::ResizeShape {
                            index: *index,
                            old_data: current_data,
                        },
                        _ => unreachable!(),
                    };
                    self.shapes[*index].restore_from(old_data);
                    self.undo_stack.push(undo_action);
                    return;
                }
                UndoAction::PasteShapes { start_index, shapes_data, conn_start_index, connectors_data } => {
                    for (i, data) in shapes_data.iter().enumerate() {
                        let mut oval = OvalShape::new(data.center_x, data.center_y);
                        oval.restore_from(data);
                        self.shapes.insert(*start_index + i, oval);
                    }
                    for (i, cd) in connectors_data.iter().enumerate() {
                        let mut conn = Connector::new(cd.source, cd.target);
                        conn.curvature = cd.curvature;
                        self.connectors.insert(*conn_start_index + i, conn);
                    }
                    self.selected = (*start_index..*start_index + shapes_data.len()).collect();
                    self.selected_connectors = (*conn_start_index..*conn_start_index + connectors_data.len()).collect();
                }
                UndoAction::DeleteShapes { shapes, removed_connectors } => {
                    // Re-apply connector removal and reindexing
                    let mut sorted_indices: Vec<usize> = shapes.iter().map(|(i, _)| *i).collect();
                    sorted_indices.sort();
                    // Remove connectors in reverse order
                    for (ci, _) in removed_connectors.iter().rev() {
                        if *ci < self.connectors.len() {
                            self.connectors.remove(*ci);
                        }
                    }
                    // Reindex connectors
                    for c in &mut self.connectors {
                        for &del_idx in sorted_indices.iter().rev() {
                            if c.source > del_idx {
                                c.source -= 1;
                            }
                            if c.target > del_idx {
                                c.target -= 1;
                            }
                        }
                    }
                    // Remove shapes in reverse order
                    for (idx, _) in shapes.iter() {
                        self.shapes.remove(*idx);
                    }
                    self.selected.clear();
                }
                UndoAction::AddConnector { index } => {
                    // Redo add = re-add (but we don't store data... need to fix)
                    // This is handled by push to undo_stack below
                    _ = index;
                }
                UndoAction::RemoveConnector { index, .. } => {
                    if *index < self.connectors.len() {
                        self.connectors.remove(*index);
                    }
                }
            }
            self.undo_stack.push(action);
        }
    }
}

/// Shape data prepared for rendering. Owns all data so it can be
/// passed into 'static canvas paint closures.
#[derive(Clone, Debug)]
pub struct ShapeRenderData {
    pub kind: ShapeKind,
    pub cx: f32,
    pub cy: f32,
    pub rx: f32,
    pub ry: f32,
    pub border_width: f32,
    pub text_box_width: f32,
    pub selected: bool,
    pub text: String,
    pub cursor_offset: Option<usize>,
    pub selected_range: Option<std::ops::Range<usize>>,
    pub resize_handles: Option<Vec<(f32, f32)>>,
}

impl CanvasState {
    /// Build render data for all shapes. When editing, the editing shape
    /// uses live text and cursor offset from the `TextInputState`.
    pub fn render_data(&self, editing_state: Option<&TextInputState>) -> Vec<ShapeRenderData> {
        self.shapes
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let (cx, cy) = s.center();
                let is_editing = self.editing == Some(i);
                ShapeRenderData {
                    kind: s.kind(),
                    cx,
                    cy,
                    rx: s.rx(),
                    ry: s.ry(),
                    border_width: s.border_width(),
                    text_box_width: s.text_box_width(),
                    selected: self.selected.contains(&i),
                    text: if is_editing {
                        editing_state
                            .map(|s| s.content().to_string())
                            .unwrap_or_default()
                    } else {
                        s.text().to_string()
                    },
                    cursor_offset: if is_editing {
                        Some(editing_state.map(|s| s.cursor_offset()).unwrap_or(0))
                    } else {
                        None
                    },
                    selected_range: if is_editing {
                        editing_state.map(|s| s.selected_range())
                    } else {
                        None
                    },
                    resize_handles: if self.selected.contains(&i) && !is_editing {
                        Some(
                            ResizeHandle::ALL
                                .iter()
                                .map(|h| s.handle_position(*h))
                                .collect(),
                        )
                    } else {
                        None
                    },
                }
            })
            .collect()
    }
}

/// Connector data prepared for rendering.
#[derive(Clone, Debug)]
pub struct ConnectorRenderData {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub control_a: (f32, f32),
    pub control_b: (f32, f32),
    pub midpoint: (f32, f32),
    pub selected: bool,
    /// Bounding box of the curve: (min_x, min_y, max_x, max_y)
    pub bounds: (f32, f32, f32, f32),
}

impl CanvasState {
    pub fn connector_render_data(&self) -> Vec<ConnectorRenderData> {
        self.connectors
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let start = c.start_point(&self.shapes);
                let end = c.end_point(&self.shapes);
                let cp = c.control_point(&self.shapes);
                let midpoint = c.midpoint(&self.shapes);

                // Quadratic → cubic control points
                let control_a = (
                    start.0 + 2.0 / 3.0 * (cp.0 - start.0),
                    start.1 + 2.0 / 3.0 * (cp.1 - start.1),
                );
                let control_b = (
                    end.0 + 2.0 / 3.0 * (cp.0 - end.0),
                    end.1 + 2.0 / 3.0 * (cp.1 - end.1),
                );

                // Compute bounding box from key points
                let pts = [start, end, control_a, control_b, midpoint];
                let min_x = pts.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
                let min_y = pts.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
                let max_x = pts.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
                let max_y = pts.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);

                ConnectorRenderData {
                    start,
                    end,
                    control_a,
                    control_b,
                    midpoint,
                    selected: self.selected_connectors.contains(&i),
                    bounds: (min_x - 4.0, min_y - 4.0, max_x + 4.0, max_y + 4.0),
                }
            })
            .collect()
    }
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::new()
    }
}
