use crate::control::{Button, Control};
use crate::styler::Styler;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct StandardStyler {
    pub canvas: WindowCanvas,
}

impl Styler for StandardStyler {
    fn begin(&mut self) {
        self.canvas.set_draw_color(Color::RGB(253, 253, 253));
        self.canvas.clear();
    }

    fn button(&mut self, control: Control, button: Button) {
        self.canvas.set_draw_color(Color::RGB(173, 173, 173));
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();

        self.canvas.set_draw_color(Color::RGB(225, 225, 225));
        self.canvas.fill_rect(control.rect.inflate(-1).to_sdl()).unwrap();

    }

    fn end(&mut self) {
        self.canvas.present();
    }
}
