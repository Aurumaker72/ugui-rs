pub mod control;
pub mod geo;
pub mod input;
pub mod standard_styler;
pub mod styler;

use crate::control::{Button, Control};
use crate::geo::Point;
use crate::input::Input;
use crate::styler::Styler;

#[derive(Copy, Clone)]
pub struct PersistentState {
    active_control: Option<i64>,
    current_input: Input,
    last_input: Input,
    mouse_down_position: Point,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            active_control: None,
            current_input: Default::default(),
            last_input: Default::default(),
            mouse_down_position: Default::default(),
        }
    }
}

pub struct Ugui<T: Styler> {
    pub styler: T,
    pub persistent_state: PersistentState,
}

impl<T: Styler> Ugui<T> {
    pub fn button(&mut self, control: Control, button: Button) -> bool {
        self.styler.button(control, button);
        true
    }

    pub fn begin(&mut self, input: Input) {
        self.persistent_state.last_input = self.persistent_state.current_input;
        self.persistent_state.current_input = input;

        if input.primary_down && !self.persistent_state.last_input.primary_down {
            self.persistent_state.mouse_down_position =
                self.persistent_state.current_input.mouse_position;
        }

        self.styler.begin(self.persistent_state);
    }

    pub fn end(&mut self) {
        self.styler.end();
    }
}
