use crate::text_input::TextInputState;

/// An oval shape on the canvas.
pub struct OvalShape {
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
            center_x: cx,
            center_y: cy,
            rx: 100.0,
            ry: 70.0,
            border_width: 1.0,
            text: String::new(),
        }
    }

    /// Create a new oval with explicit size.
    pub fn with_size(cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        Self {
            center_x: cx,
            center_y: cy,
            rx,
            ry,
            border_width: 1.0,
            text: String::new(),
        }
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

    /// Test whether a point is inside this oval using the ellipse equation:
    /// ((px - cx) / rx)² + ((py - cy) / ry)² <= 1
    /// Returns the width of the largest inscribed rectangle in the oval.
    /// Used as the wrap width for text rendering: `rx * √2`.
    pub fn text_box_width(&self) -> f32 {
        self.rx * std::f32::consts::SQRT_2
    }

    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let dx = (px - self.center_x) / self.rx;
        let dy = (py - self.center_y) / self.ry;
        (dx * dx + dy * dy) <= 1.0
    }

    fn clone_data(&self) -> OvalShapeData {
        OvalShapeData {
            center_x: self.center_x,
            center_y: self.center_y,
            rx: self.rx,
            ry: self.ry,
            border_width: self.border_width,
            text: self.text.clone(),
        }
    }

    fn restore_from(&mut self, data: &OvalShapeData) {
        self.center_x = data.center_x;
        self.center_y = data.center_y;
        self.rx = data.rx;
        self.ry = data.ry;
        self.border_width = data.border_width;
        self.text = data.text.clone();
    }
}

/// Snapshot of an oval for undo/redo.
#[derive(Clone)]
struct OvalShapeData {
    center_x: f32,
    center_y: f32,
    rx: f32,
    ry: f32,
    border_width: f32,
    text: String,
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
}

/// State for a drawing canvas containing shapes.
pub struct CanvasState {
    shapes: Vec<OvalShape>,
    selected: Option<usize>,
    editing: Option<usize>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
}

impl CanvasState {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            selected: None,
            editing: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    pub fn shapes(&self) -> &[OvalShape] {
        &self.shapes
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn editing(&self) -> Option<usize> {
        self.editing
    }

    /// Start editing the shape at the given index. Also selects it.
    /// Ignored if the index is out of bounds.
    pub fn start_editing(&mut self, index: usize) {
        if index < self.shapes.len() {
            self.editing = Some(index);
            self.selected = Some(index);
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

    pub fn add_oval(&mut self, cx: f32, cy: f32) {
        let oval = OvalShape::new(cx, cy);
        let data = oval.clone_data();
        let index = self.shapes.len();
        self.shapes.push(oval);
        self.undo_stack.push(UndoAction::AddShape { index, data });
        self.redo_stack.clear();
    }

    /// Select the topmost shape at the given point, or deselect if none.
    /// Clears editing state if selection changes.
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
        self.selected = new_selected;
    }

    /// Move the selected shape to a new center position.
    pub fn move_selected(&mut self, cx: f32, cy: f32) {
        if let Some(index) = self.selected {
            let old_data = self.shapes[index].clone_data();
            self.shapes[index].move_to(cx, cy);
            self.undo_stack.push(UndoAction::MoveShape { index, old_data });
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                UndoAction::AddShape { index, .. } => {
                    self.shapes.remove(*index);
                    if self.selected == Some(*index) {
                        self.selected = None;
                    }
                }
                UndoAction::MoveShape { index, old_data } => {
                    let current_data = self.shapes[*index].clone_data();
                    self.shapes[*index].restore_from(old_data);
                    self.redo_stack.push(UndoAction::MoveShape {
                        index: *index,
                        old_data: current_data,
                    });
                    return;
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
                UndoAction::MoveShape { index, old_data } => {
                    let current_data = self.shapes[*index].clone_data();
                    self.shapes[*index].restore_from(old_data);
                    self.undo_stack.push(UndoAction::MoveShape {
                        index: *index,
                        old_data: current_data,
                    });
                    return;
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
    pub cx: f32,
    pub cy: f32,
    pub rx: f32,
    pub ry: f32,
    pub border_width: f32,
    pub text_box_width: f32,
    pub selected: bool,
    pub text: String,
    pub cursor_offset: Option<usize>,
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
                    cx,
                    cy,
                    rx: s.rx(),
                    ry: s.ry(),
                    border_width: s.border_width(),
                    text_box_width: s.text_box_width(),
                    selected: self.selected == Some(i),
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
