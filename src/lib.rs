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
    fn process_push(&mut self, control: Control) -> bool {
        if !control.enabled {
            return false;
        }

        if self.persistent_state.current_input.primary_down
            && !self.persistent_state.last_input.primary_down
            && self
                .persistent_state
                .mouse_down_position
                .inside(control.rect)
        {
            self.persistent_state.active_control = Some(control.uid);
            return true;
        }

        return false;
    }
    pub fn button(&mut self, control: Control, button: Button) -> bool {
        let pushed = self.process_push(control);
        self.styler.button(control, button);
        pushed
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

        // As soon as we let go of the primary mouse button, the active control should be cleared
        if self.persistent_state.active_control.is_some()
            && !self.persistent_state.current_input.primary_down
        {
            self.persistent_state.active_control = None;
        }
    }
}
