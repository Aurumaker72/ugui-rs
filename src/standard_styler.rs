use crate::control::{Button, Control};
use crate::geo::{Point, Rect};
use crate::standard_styler::VisualState::{Active, Disabled, Hover, Normal};
use crate::styler::Styler;
use crate::{Input, PersistentState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SdlPoint;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

pub struct StandardStyler<'a> {
    canvas: WindowCanvas,
    ttf_context: &'a Sdl2TtfContext,
    font: Font<'a, 'static>,
    persistent_state: PersistentState,
}

#[derive(Eq, PartialEq)]
enum VisualState {
    Normal,
    Hover,
    Active,
    Disabled,
}

#[derive(Eq, PartialEq)]
enum Alignment {
    Start,
    Center,
    End,
}

impl<'a> StandardStyler<'a> {
    pub fn new(canvas: WindowCanvas, ttf_context: &'a Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font(Path::new("examples/fonts/segoe.ttf"), 12)
            .unwrap();

        Self {
            persistent_state: Default::default(),
            canvas,
            ttf_context,
            font,
        }
    }

    fn draw_text(
        &mut self,
        text: &str,
        rect: SdlRect,
        color: Color,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
    ) {
        let texture_creator = self.canvas.texture_creator();

        let surface = self
            .font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let text_rect = self.font.size_of(text).unwrap();

        let mut computed_rect = SdlRect::new(rect.x, rect.y, text_rect.0, text_rect.1);

        if horizontal_alignment == Alignment::End {
            computed_rect.x = rect.x + rect.w - text_rect.0 as i32;
        }
        if horizontal_alignment == Alignment::Center {
            computed_rect.x = rect.x + rect.w / 2 - text_rect.0 as i32 / 2;
        }
        if vertical_alignment == Alignment::End {
            computed_rect.y = rect.y + rect.h - text_rect.1 as i32;
        }
        if vertical_alignment == Alignment::Center {
            computed_rect.y = rect.y + rect.h / 2 - text_rect.1 as i32 / 2;
        }

        self.canvas
            .copy(&texture, None, Some(computed_rect))
            .unwrap();
    }

    fn get_visual_state(&mut self, control: Control) -> VisualState {
        if !control.enabled {
            return Disabled;
        }

        if self.persistent_state.active_control.is_some()
            && self.persistent_state.active_control.unwrap() == control.uid
        {
            return Active;
        }

        let now_inside = self
            .persistent_state
            .current_input
            .mouse_position
            .inside(control.rect);
        let down_inside = self
            .persistent_state
            .mouse_down_position
            .inside(control.rect);

        if now_inside && !self.persistent_state.current_input.primary_down {
            return Hover;
        }

        if down_inside && self.persistent_state.current_input.primary_down && !now_inside {
            return Hover;
        }

        if now_inside && self.persistent_state.current_input.primary_down && down_inside {
            return Active;
        }

        return Normal;
    }
}

impl<'a> Styler for StandardStyler<'a> {
    fn begin(&mut self, persistent_state: PersistentState) {
        self.persistent_state = persistent_state;
        self.canvas.set_draw_color(Color::RGB(253, 253, 253));
        self.canvas.clear();
    }

    fn button(&mut self, control: Control, button: Button) {
        let mut back_color = Color::BLACK;
        let mut border_color = Color::BLACK;
        let mut text_color = Color::BLACK;

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
            text_color = Color::RGB(160, 160, 160);
        }

        self.canvas.set_draw_color(border_color);
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();
        self.canvas.set_draw_color(back_color);
        self.canvas
            .fill_rect(control.rect.inflate(-1.0).to_sdl())
            .unwrap();
        self.draw_text(
            button.text,
            control.rect.to_sdl(),
            text_color,
            Alignment::Center,
            Alignment::Center,
        );
    }

    fn end(&mut self) {
        self.canvas.present();
    }
}
