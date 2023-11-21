pub mod control;
pub mod geo;
pub mod input;
pub mod standard_styler;
pub mod styler;

use crate::control::{Button, Control};
use crate::geo::Point;
use crate::input::Input;
use crate::styler::Styler;

pub struct Ugui<T: Styler> {
    pub active_control: Option<i64>,
    pub styler: T,
    pub current_input: Input,
    pub last_input: Input,
    pub mouse_down_position: Point,
}

impl<T: Styler> Ugui<T> {
    pub fn button(&mut self, control: Control, button: Button) -> bool {
        self.styler.button(control, button);
        true
    }

    pub fn begin(&mut self, input: Input) {
        self.last_input = self.current_input;
        self.current_input = input;

        if input.primary_down && !self.last_input.primary_down {
            self.mouse_down_position = self.current_input.mouse_position;
        }

        self.styler.begin(
            self.current_input,
            self.last_input,
            self.mouse_down_position,
            self.active_control,
        );
    }

    pub fn end(&mut self) {
        self.styler.end();
    }
}
