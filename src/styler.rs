use crate::*;

pub trait Styler {
    fn begin(&mut self, current_input: Input, last_input: Input, mouse_down_position: Point, active_control: Option<i64>);
    fn button(&mut self, control: Control, button: Button);
    fn end(&mut self);
}
