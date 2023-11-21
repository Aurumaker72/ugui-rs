use crate::geo::Point;

#[derive(Clone, Copy, Default)]
pub struct Input {
    pub mouse_position: Point,
    pub primary_down: bool,
}
