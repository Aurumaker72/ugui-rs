use crate::control::{Button, Control};
use crate::geo::{Point, Rect};
use crate::standard_styler::VisualState::{Active, Disabled, Hover, Normal};
use crate::styler::Styler;
use crate::Input;
use sdl2::pixels::Color;
use sdl2::rect::Point as SdlPoint;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

pub struct StandardStyler<'a> {
    pub current_input: Input,
    pub last_input: Input,
    pub mouse_down_position: Point,
    pub active_control: Option<i64>,
    pub canvas: WindowCanvas,
    pub ttf_context: &'a Sdl2TtfContext,
    pub font: Font<'a, 'static>,
}

#[derive(Eq, PartialEq)]
enum VisualState {
    Normal,
    Hover,
    Active,
    Disabled,
}

impl<'a> StandardStyler<'a> {
    pub fn new(canvas: WindowCanvas, ttf_context: &'a Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font(Path::new("examples/fonts/segoe.ttf"), 12)
            .unwrap();

        Self {
            current_input: Default::default(),
            last_input: Default::default(),
            mouse_down_position: Default::default(),
            active_control: None,
            canvas,
            ttf_context,
            font,
        }
    }

    fn get_visual_state(&mut self, control: Control) -> VisualState {
        if !control.enabled {
            return Disabled;
        }

        if self.active_control.is_some() && self.active_control.unwrap() == control.uid {
            return Active;
        }

        let now_inside = self.current_input.mouse_position.inside(control.rect);
        let down_inside = self.mouse_down_position.inside(control.rect);

        if now_inside && !self.current_input.primary_down {
            return Hover;
        }

        if down_inside && self.current_input.primary_down && !now_inside {
            return Hover;
        }

        if now_inside && self.current_input.primary_down && down_inside {
            return Active;
        }

        return Normal;
    }
}

impl<'a> Styler for StandardStyler<'a> {
    fn begin(
        &mut self,
        current_input: Input,
        last_input: Input,
        mouse_down_position: Point,
        active_control: Option<i64>,
    ) {
        self.current_input = current_input;
        self.last_input = last_input;
        self.mouse_down_position = mouse_down_position;
        self.active_control = active_control;
        self.canvas.set_draw_color(Color::RGB(253, 253, 253));
        self.canvas.clear();
    }

    fn button(&mut self, control: Control, button: Button) {
        let mut back_color = Color::BLACK;
        let mut border_color = Color::BLACK;

        let visual_state = self.get_visual_state(control);

        if visual_state == Normal {
            back_color = Color::RGB(225, 225, 225);
            border_color = Color::RGB(173, 173, 173);
        } else if visual_state == Hover {
            back_color = Color::RGB(229, 241, 251);
            border_color = Color::RGB(0, 120, 215);
        } else if visual_state == Active {
            back_color = Color::RGB(204, 228, 247);
            border_color = Color::RGB(0, 84, 153);
        } else if visual_state == Disabled {
            back_color = Color::RGB(204, 204, 204);
            border_color = Color::RGB(191, 191, 191);
        }

        self.canvas.set_draw_color(border_color);
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();
        self.canvas.set_draw_color(back_color);
        self.canvas
            .fill_rect(control.rect.inflate(-1.0).to_sdl())
            .unwrap();
    }

    fn end(&mut self) {
        self.canvas.present();
    }
}
