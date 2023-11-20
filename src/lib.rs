pub mod control;
pub mod geo;
pub mod standard_styler;
pub mod styler;

use crate::control::{Button, Control};
use crate::styler::Styler;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(Clone, Copy)]
pub struct Input {
    pub x: f32,
    pub y: f32,
    pub primary_down: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            x: 0f32,
            y: 0f32,
            primary_down: false,
        }
    }
}

pub struct Ugui<T: Styler> {
    pub current_input: Input,
    pub last_input: Input,
    pub styler: T,
}

impl<T: Styler> Ugui<T> {
    pub fn button(&mut self, control: Control, button: Button) -> bool {
        self.styler.button(control, button);
        true
    }

    pub fn begin(&mut self, input: Input) {
        self.last_input = self.current_input.clone();
        self.current_input = input;
        self.styler.begin();
    }

    pub fn end(&mut self) {
        self.styler.end();
    }
}
