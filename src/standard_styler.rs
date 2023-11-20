use crate::control::{Button, Control};
use crate::styler::Styler;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;
use std::rc::Rc;

pub struct StandardStyler<'a> {
    pub canvas: WindowCanvas,
    pub ttf_context: &'a Sdl2TtfContext,
    pub font: Font<'a, 'static>,
}

impl<'a> StandardStyler<'a> {
    pub fn new(canvas: WindowCanvas, ttf_context: &'a Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font(Path::new("examples/fonts/segoe.ttf"), 12)
            .unwrap();

        Self {
            canvas,
            ttf_context,
            font,
        }
    }
}

impl<'a> Styler for StandardStyler<'a> {
    fn begin(&mut self) {
        self.canvas.set_draw_color(Color::RGB(253, 253, 253));
        self.canvas.clear();
    }

    fn button(&mut self, control: Control, button: Button) {
        self.canvas.set_draw_color(Color::RGB(173, 173, 173));
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();

        self.canvas.set_draw_color(Color::RGB(225, 225, 225));
        self.canvas
            .fill_rect(control.rect.inflate(-1).to_sdl())
            .unwrap();
    }

    fn end(&mut self) {
        self.canvas.present();
    }
}
