use crate::geo::Point;

#[derive(Clone, Copy)]
pub struct Input {
    pub mouse_position: Point,
    pub primary_down: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            mouse_position: Default::default(),
            primary_down: false,
        }
    }
}
