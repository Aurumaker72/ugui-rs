use crate::geo::Point;

#[derive(Clone, Copy, Default)]
pub struct Input {
    pub mouse_position: Point,
    pub mouse_wheel: i8,
    pub primary_down: bool,
}

impl Input {
    fn wheel_up(&self) -> bool {
        return self.mouse_wheel < 0;
    }
    fn wheel_down(&self) -> bool {
        return self.mouse_wheel > 0;
    }
}
