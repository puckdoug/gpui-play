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
    pub fn new(_cx: f32, _cy: f32) -> Self {
        todo!()
    }

    pub fn with_size(_cx: f32, _cy: f32, _rx: f32, _ry: f32) -> Self {
        todo!()
    }

    pub fn center(&self) -> (f32, f32) {
        todo!()
    }

    pub fn rx(&self) -> f32 {
        todo!()
    }

    pub fn ry(&self) -> f32 {
        todo!()
    }

    pub fn border_width(&self) -> f32 {
        todo!()
    }

    pub fn text(&self) -> &str {
        todo!()
    }

    pub fn set_text(&mut self, _text: &str) {
        todo!()
    }

    pub fn move_to(&mut self, _cx: f32, _cy: f32) {
        todo!()
    }

    pub fn contains_point(&self, _px: f32, _py: f32) -> bool {
        todo!()
    }
}

/// State for a drawing canvas containing shapes.
pub struct CanvasState {
    shapes: Vec<OvalShape>,
    selected: Option<usize>,
}

impl CanvasState {
    pub fn new() -> Self {
        todo!()
    }

    pub fn shape_count(&self) -> usize {
        todo!()
    }

    pub fn shapes(&self) -> &[OvalShape] {
        todo!()
    }

    pub fn selected(&self) -> Option<usize> {
        todo!()
    }

    pub fn add_oval(&mut self, _cx: f32, _cy: f32) {
        todo!()
    }

    pub fn select_at(&mut self, _px: f32, _py: f32) {
        todo!()
    }

    pub fn move_selected(&mut self, _cx: f32, _cy: f32) {
        todo!()
    }

    pub fn undo(&mut self) {
        todo!()
    }

    pub fn redo(&mut self) {
        todo!()
    }
}
