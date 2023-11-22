use crate::control::{Button, Control, Listbox, Scrollbar};
use crate::geo::{remap, Point, Rect};
use crate::standard_styler::VisualState::{Active, Disabled, Hover, Normal};
use crate::styler::Styler;
use crate::PersistentState;

use sdl2::pixels::Color;

use sdl2::rect::Rect as SdlRect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

pub struct StandardStyler<'a> {
    canvas: WindowCanvas,
    _ttf_context: &'a Sdl2TtfContext,
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
const LISTBOX_ITEM_PADDING: f32 = 4.0;
const LISTBOX_ITEM_HEIGHT: f32 = 20.0;

impl<'a> StandardStyler<'a> {
    pub fn new(canvas: WindowCanvas, ttf_context: &'a Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font(Path::new("examples/fonts/segoe.ttf"), 12)
            .unwrap();

        Self {
            persistent_state: Default::default(),
            canvas,
            _ttf_context: ttf_context,
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

        Normal
    }

    fn listbox_item(&mut self, item: &str, enabled: bool, selected: bool, rect: Rect) {
        let mut back_color = Color::WHITE;
        let mut text_color = Color::BLACK;

        if selected {
            back_color = Color::RGB(0, 120, 215);
            text_color = Color::WHITE;
        }
        if !enabled {
            back_color = Color::RGB(204, 204, 204);
            text_color = Color::RGB(160, 160, 160);
        }

        self.canvas.set_draw_color(back_color);
        self.canvas.fill_rect(rect.to_sdl()).unwrap();

        self.draw_text(
            item,
            rect.inflate(-LISTBOX_ITEM_PADDING).to_sdl(),
            text_color,
            Alignment::Start,
            Alignment::Center,
        );
    }

    fn scrollbar_get_thumb(control: Control, scrollbar: Scrollbar) -> Rect {
        let scrollbar_height = control.rect.h * (1.0 / scrollbar.ratio);
        let scrollbar_y = remap(
            scrollbar.value,
            0.0,
            1.0,
            0.0,
            control.rect.h - scrollbar_height,
        );

        Rect {
            x: control.rect.x,
            y: control.rect.y + scrollbar_y,
            w: control.rect.w,
            h: scrollbar_height,
        }
    }
}

impl<'a> Styler for StandardStyler<'a> {
    fn begin(&mut self, persistent_state: PersistentState) {
        self.persistent_state = persistent_state;
        self.canvas.set_draw_color(Color::RGB(253, 253, 253));
        self.canvas.clear();
    }
    fn end(&mut self) {
        self.canvas.present();
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

    fn scrollbar(&mut self, control: Control, scrollbar: Scrollbar) {
        let back_color = Color::RGB(240, 240, 240);
        let mut thumb_color = Color::RGB(205, 205, 205);
        let thumb_rect = StandardStyler::scrollbar_get_thumb(control, scrollbar);

        // We need visual state of thumb, not scrollbar, since thats the interactable part
        let visual_state = self.get_visual_state(Control {
            rect: thumb_rect,
            ..control
        });

        if visual_state == Hover {
            thumb_color = Color::RGB(166, 166, 166);
        } else if visual_state == Active {
            thumb_color = Color::RGB(96, 96, 96);
        } else if visual_state == Disabled {
            thumb_color = Color::RGB(192, 192, 192);
        }

        self.canvas.set_draw_color(back_color);
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();

        self.canvas.set_draw_color(thumb_color);
        self.canvas.fill_rect(thumb_rect.to_sdl()).unwrap();
    }

    fn listbox(&mut self, control: Control, listbox: Listbox, scroll: Point) {
        let back_color = Color::RGB(255, 255, 255);
        let border_color = Color::RGB(130, 135, 144);

        self.canvas.set_draw_color(border_color);
        self.canvas.fill_rect(control.rect.to_sdl()).unwrap();
        self.canvas.set_draw_color(back_color);
        self.canvas
            .fill_rect(control.rect.inflate(-1.0).to_sdl())
            .unwrap();

        let prev_clip_rect = self.canvas.clip_rect();
        self.canvas
            .set_clip_rect(control.rect.inflate(-1.0).to_sdl());

        let content_height = listbox.items.len() as f32 * LISTBOX_ITEM_HEIGHT;

        for i in 0..listbox.items.len() {
            let base_y = (LISTBOX_ITEM_HEIGHT * i as f32);
            let moved_y = scroll.y * (content_height - control.rect.h);
            let final_y = base_y - moved_y;

            let rect = Rect::new(
                control.rect.x,
                control.rect.y + final_y,
                control.rect.w,
                LISTBOX_ITEM_HEIGHT,
            )
            .inflate(-1.0);
            self.listbox_item(
                listbox.items[i],
                control.enabled,
                listbox.index.is_some_and(|x| x == i),
                rect,
            );
        }

        self.canvas.set_clip_rect(prev_clip_rect);
    }

    fn listbox_index_at_point(
        &mut self,
        control: Control,
        listbox: Listbox,
        scroll: Point,
        point: Point,
    ) -> Option<usize> {
        if listbox.items.is_empty() {
            return listbox.index;
        }

        let content_height = listbox.items.len() as f32 * LISTBOX_ITEM_HEIGHT;

        let index = (((point.y + (scroll.y * (content_height - control.rect.h)))
            / LISTBOX_ITEM_HEIGHT)
            .ceil()
            - 1.0) as usize;

        Some(index.clamp(0, listbox.items.len() - 1))
    }
}
