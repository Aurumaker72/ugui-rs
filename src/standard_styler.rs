use crate::control::{Button, Control, Listbox, Scrollbar, Textbox};
use crate::geo::{remap, Point, Rect};
use crate::standard_styler::VisualState::{Active, Disabled, Hover, Normal};
use crate::styler::Styler;
use crate::PersistentState;
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;
use std::path::Path;

fn hex(str: &str) -> Color {
    let r = &str[1..3];
    let g = &str[3..5];
    let b = &str[5..7];
    Color::RGB(
        u8::from_str_radix(r, 16).unwrap(),
        u8::from_str_radix(g, 16).unwrap(),
        u8::from_str_radix(b, 16).unwrap(),
    )
}
#[derive(Eq, PartialEq, Hash, Debug)]
enum VisualState {
    Normal,
    Hover,
    Active,
    Disabled,
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Alignment {
    Start,
    Center,
    End,
}
const LISTBOX_ITEM_PADDING: f32 = 4.0;
const LISTBOX_ITEM_HEIGHT: f32 = 20.0;
const FONT_SIZE: f32 = 12.0;
const LINE_HEIGHT: f32 = 16.0;

pub struct StandardStyler<'a> {
    canvas: WindowCanvas,
    _ttf_context: &'a Sdl2TtfContext,
    font: Font<'a, 'static>,
    persistent_state: PersistentState,
    button_back_colors: HashMap<VisualState, Color>,
    button_border_colors: HashMap<VisualState, Color>,
    button_text_colors: HashMap<VisualState, Color>,
    listbox_back_colors: HashMap<VisualState, Color>,
    listbox_border_colors: HashMap<VisualState, Color>,
    listbox_item_back_colors: HashMap<VisualState, Color>,
    listbox_item_text_colors: HashMap<VisualState, Color>,
    textbox_back_colors: HashMap<VisualState, Color>,
    textbox_border_colors: HashMap<VisualState, Color>,
    textbox_text_colors: HashMap<VisualState, Color>,
    scrollbar_back_colors: HashMap<VisualState, Color>,
    scrollbar_thumb_colors: HashMap<VisualState, Color>,
}

impl<'a> StandardStyler<'a> {
    pub fn new(canvas: WindowCanvas, ttf_context: &'a Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font(Path::new("examples/fonts/segoe.ttf"), FONT_SIZE as u16)
            .unwrap();

        let mut button_back_colors = HashMap::new();
        button_back_colors.insert(Normal, hex("#E1E1E1"));
        button_back_colors.insert(Hover, hex("#E5F1FB"));
        button_back_colors.insert(Active, hex("#CCE4F7"));
        button_back_colors.insert(Disabled, hex("#CCCCCC"));

        let mut button_border_colors = HashMap::new();
        button_border_colors.insert(Normal, hex("#ADADAD"));
        button_border_colors.insert(Hover, hex("#0078D7"));
        button_border_colors.insert(Active, hex("#005499"));
        button_border_colors.insert(Disabled, hex("#BFBFBF"));

        let mut button_text_colors = HashMap::new();
        button_text_colors.insert(Normal, hex("#000000"));
        button_text_colors.insert(Hover, hex("#000000"));
        button_text_colors.insert(Active, hex("#000000"));
        button_text_colors.insert(Disabled, hex("#A0A0A0"));

        let mut listbox_back_colors = HashMap::new();
        listbox_back_colors.insert(Normal, hex("#FFFFFF"));
        listbox_back_colors.insert(Hover, hex("#FFFFFF"));
        listbox_back_colors.insert(Active, hex("#FFFFFF"));
        listbox_back_colors.insert(Disabled, hex("#FFFFFF"));

        let mut listbox_border_colors = HashMap::new();
        listbox_border_colors.insert(Normal, hex("#7A7A7A"));
        listbox_border_colors.insert(Hover, hex("#7A7A7A"));
        listbox_border_colors.insert(Active, hex("#7A7A7A"));
        listbox_border_colors.insert(Disabled, hex("#7A7A7A"));

        let mut listbox_item_back_colors = HashMap::new();
        listbox_item_back_colors.insert(Normal, hex("#FFFFFF"));
        listbox_item_back_colors.insert(Hover, hex("#FFFFFF"));
        listbox_item_back_colors.insert(Active, hex("#0078D7"));
        listbox_item_back_colors.insert(Disabled, hex("#FFFFFF"));

        let mut listbox_item_text_colors = HashMap::new();
        listbox_item_text_colors.insert(Normal, hex("#000000"));
        listbox_item_text_colors.insert(Hover, hex("#000000"));
        listbox_item_text_colors.insert(Active, hex("#FFFFFF"));
        listbox_item_text_colors.insert(Disabled, hex("#A0A0A0"));

        let mut textbox_back_colors = HashMap::new();
        textbox_back_colors.insert(Normal, hex("#FFFFFF"));
        textbox_back_colors.insert(Hover, hex("#FFFFFF"));
        textbox_back_colors.insert(Active, hex("#FFFFFF"));
        textbox_back_colors.insert(Disabled, hex("#FFFFFF"));

        let mut textbox_border_colors = HashMap::new();
        textbox_border_colors.insert(Normal, hex("#7A7A7A"));
        textbox_border_colors.insert(Hover, hex("#171717"));
        textbox_border_colors.insert(Active, hex("#0078D7"));
        textbox_border_colors.insert(Disabled, hex("#CCCCCC"));

        let mut textbox_text_colors = HashMap::new();
        textbox_text_colors.insert(Normal, hex("#000000"));
        textbox_text_colors.insert(Hover, hex("#000000"));
        textbox_text_colors.insert(Active, hex("#000000"));
        textbox_text_colors.insert(Disabled, hex("#CCCCCC"));

        let mut scrollbar_back_colors = HashMap::new();
        scrollbar_back_colors.insert(Normal, hex("#F0F0F0"));
        scrollbar_back_colors.insert(Hover, hex("#F0F0F0"));
        scrollbar_back_colors.insert(Active, hex("#F0F0F0"));
        scrollbar_back_colors.insert(Disabled, hex("#F0F0F0"));

        let mut scrollbar_thumb_colors = HashMap::new();
        scrollbar_thumb_colors.insert(Normal, hex("#CDCDCD"));
        scrollbar_thumb_colors.insert(Hover, hex("#A6A6A6"));
        scrollbar_thumb_colors.insert(Active, hex("#606060"));
        scrollbar_thumb_colors.insert(Disabled, hex("#C0C0C0"));

        Self {
            persistent_state: Default::default(),
            canvas,
            _ttf_context: ttf_context,
            font,
            button_back_colors,
            button_border_colors,
            button_text_colors,
            listbox_back_colors,
            listbox_border_colors,
            listbox_item_back_colors,
            listbox_item_text_colors,
            textbox_back_colors,
            textbox_border_colors,
            textbox_text_colors,
            scrollbar_back_colors,
            scrollbar_thumb_colors,
        }
    }

    fn quad(&mut self, rect: Rect, back_color: Color, border_color: Color) {
        self.canvas.set_draw_color(border_color);
        self.canvas.fill_rect(rect.to_sdl()).unwrap();
        self.canvas.set_draw_color(back_color);
        self.canvas.fill_rect(rect.inflate(-1.0).to_sdl()).unwrap();
    }

    fn index_in_singleline_string(&mut self, text: &str, x: f32) -> usize {
        let mut positions: Vec<f32> = Vec::new();
        for j in 0..text.len() {
            positions.push(self.font.size_of(&text[0..j]).unwrap().0 as f32);
        }
        for j in (0..positions.len()).rev() {
            if x > positions[j] {
                return j.clamp(0, positions.len() + 1);
            }
        }
        0
    }

    fn get_multiline_string_positions(&mut self, text: &str) -> Vec<(usize, Point)> {
        let mut positions: Vec<(usize, Point)> = Vec::new();
        let mut char_count = 0;
        let lines = text.split("\n").collect::<Vec<&str>>();
        for i in 0..lines.len() {
            let mut line = lines[i].replace("\n", "");

            // SDL freaks out when performing operations on 0-width strings
            if line.len() == 0 {
                line = " ".to_string();
            }

            // Compute bounds of current line
            let size = self.font.size_of(&line).unwrap();
            let text_size = Point {
                x: size.0 as f32,
                y: size.1 as f32,
            };
            let line_rect = Rect {
                x: 0.0,
                y: LINE_HEIGHT * i as f32,
                w: text_size.x,
                h: LINE_HEIGHT,
            };

            for j in 0..line.len() {
                positions.push((
                    char_count,
                    Point {
                        x: self.font.size_of(&line[0..j]).unwrap().0 as f32,
                        y: line_rect.y,
                    },
                ));
                char_count += 1;
            }
        }
        return positions;
    }

    fn index_in_multiline_string(&mut self, text: &str, point: Point) -> usize {
        let positions = self.get_multiline_string_positions(text);

        let closest_match = positions
            .iter()
            .min_by(|x, y| x.1.dist(point).total_cmp(&y.1.dist(point)));

        closest_match.unwrap().0
    }

    fn position_in_multiline_string(&mut self, text: &str, index: usize) -> Point {
        let positions = self.get_multiline_string_positions(text);

        return positions.iter().find(|x| x.0 == index).unwrap().1;
    }

    fn draw_text(
        &mut self,
        text: &str,
        rect: Rect,
        color: Color,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
    ) {
        let texture_creator = self.canvas.texture_creator();

        let lines = text.split("\n").collect::<Vec<&str>>();

        for i in 0..lines.len() {
            let mut line = lines[i].replace("\n", "");

            // SDL freaks out when performing operations on 0-width strings
            if line.len() == 0 {
                line = " ".to_string();
            }

            let surface = self
                .font
                .render(&line)
                .blended(color)
                .map_err(|e| e.to_string())
                .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            let size = self.font.size_of(&line).unwrap();
            let text_size = Point {
                x: size.0 as f32,
                y: size.1 as f32,
            };
            let mut line_rect = Rect {
                x: rect.x,
                y: rect.y + (i as f32 * LINE_HEIGHT),
                w: rect.w,
                h: LINE_HEIGHT,
            };
            if lines.len() == 1 {
                // Single-line string: line rect is just the regular rect
                line_rect = rect;
            }
            if horizontal_alignment == Alignment::Center {
                line_rect.x += line_rect.w / 2.0 - text_size.x / 2.0;
            }
            if horizontal_alignment == Alignment::End {
                line_rect.x += line_rect.w - text_size.x;
            }
            if vertical_alignment == Alignment::Center {
                line_rect.y += line_rect.h / 2.0 - text_size.y / 2.0;
            }
            if vertical_alignment == Alignment::End {
                line_rect.y += line_rect.h - text_size.y;
            }
            line_rect.w = text_size.x;
            line_rect.h = text_size.y;
            self.canvas
                .copy(&texture, None, Some(line_rect.to_sdl()))
                .unwrap();
        }
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
        let mut visual_state = Normal;
        if !enabled {
            visual_state = Disabled;
        }
        if selected {
            visual_state = Active;
        }

        let back_color = self
            .listbox_item_back_colors
            .get(&visual_state)
            .unwrap()
            .clone();
        let text_color = self
            .listbox_item_text_colors
            .get(&visual_state)
            .unwrap()
            .clone();

        self.quad(rect, back_color, back_color);

        self.draw_text(
            item,
            rect.inflate(-LISTBOX_ITEM_PADDING),
            text_color,
            Alignment::Start,
            Alignment::Center,
        );
    }

    fn scrollbar_get_thumb(control: Control, scrollbar: Scrollbar) -> Rect {
        if control.rect.w > control.rect.h {
            let scrollbar_width = control.rect.w * (1.0 / scrollbar.ratio);
            let scrollbar_x = remap(
                scrollbar.value,
                0.0,
                1.0,
                0.0,
                control.rect.w - scrollbar_width,
            );

            Rect {
                x: control.rect.x + scrollbar_x,
                y: control.rect.y,
                w: scrollbar_width,
                h: control.rect.h,
            }
        } else {
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

    fn get_visible_range(
        &self,
        control: Control,
        listbox: Listbox,
        scroll: Point,
    ) -> (usize, usize) {
        let content_height = listbox.items.len() as f32 * LISTBOX_ITEM_HEIGHT;

        let mut index_begin =
            ((scroll.y * (content_height - control.rect.h)) / LISTBOX_ITEM_HEIGHT) as usize;
        let mut index_end = ((control.rect.h + (scroll.y * (content_height - control.rect.h)))
            / LISTBOX_ITEM_HEIGHT) as usize
            + 1;

        return (
            index_begin.clamp(0, listbox.items.len()),
            index_end.clamp(0, listbox.items.len()),
        );
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
        let visual_state = self.get_visual_state(control);
        let back_color = self.button_back_colors.get(&visual_state).unwrap().clone();
        let border_color = self
            .button_border_colors
            .get(&visual_state)
            .unwrap()
            .clone();
        let text_color = self.button_text_colors.get(&visual_state).unwrap().clone();

        self.quad(control.rect, back_color, border_color);
        self.draw_text(
            button.text,
            control.rect,
            text_color,
            Alignment::Center,
            Alignment::Center,
        );
    }

    fn scrollbar(&mut self, control: Control, scrollbar: Scrollbar) {
        let thumb_rect = StandardStyler::scrollbar_get_thumb(control, scrollbar);

        // We need visual state of thumb, not scrollbar, since thats the interactable part
        let visual_state = self.get_visual_state(Control {
            rect: thumb_rect,
            ..control
        });

        let back_color = self
            .scrollbar_back_colors
            .get(&visual_state)
            .unwrap()
            .clone();
        let thumb_color = self
            .scrollbar_thumb_colors
            .get(&visual_state)
            .unwrap()
            .clone();

        self.quad(control.rect, back_color, back_color);
        self.quad(thumb_rect, thumb_color, thumb_color);
    }

    fn listbox(&mut self, control: Control, listbox: Listbox, scroll: Point) {
        let visual_state = self.get_visual_state(control);
        let back_color = self.listbox_back_colors.get(&visual_state).unwrap().clone();
        let border_color = self
            .listbox_border_colors
            .get(&visual_state)
            .unwrap()
            .clone();

        self.quad(control.rect, back_color, border_color);

        let prev_clip_rect = self.canvas.clip_rect();
        self.canvas
            .set_clip_rect(control.rect.inflate(-1.0).to_sdl());

        let visible_range = self.get_visible_range(control, listbox, scroll);
        let content_size = self.listbox_get_content_size(control, listbox);

        let x_offset = ((content_size.x - control.rect.w) * scroll.x).max(0.0);

        for i in visible_range.0..visible_range.1 {
            let base_y = LISTBOX_ITEM_HEIGHT * i as f32;
            let moved_y = scroll.y * (content_size.y - control.rect.h);
            let final_y = base_y - moved_y;

            let rect = Rect::new(
                control.rect.x - x_offset,
                control.rect.y + final_y,
                content_size.x.max(control.rect.w),
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
        let content_size = self.listbox_get_content_size(control, listbox);

        let index = (((point.y + (scroll.y * (content_size.y - control.rect.h)))
            / LISTBOX_ITEM_HEIGHT)
            .ceil()
            - 1.0) as usize;

        Some(index.clamp(0, listbox.items.len() - 1))
    }

    fn listbox_get_content_size(&self, control: Control, listbox: Listbox) -> Point {
        // Width is measured by getting max width of all items
        // TODO: Optimize, as this is very slow on large data sets
        let item_widths = listbox
            .items
            .iter()
            .map(|x| self.font.size_of(x).unwrap().0);

        return Point {
            // We add the padding back in because it's off otherwise
            x: item_widths.max().unwrap() as f32 + LISTBOX_ITEM_PADDING,
            y: listbox.items.len() as f32 * LISTBOX_ITEM_HEIGHT,
        };
    }

    fn textbox(&mut self, control: Control, textbox: Textbox, scroll: Point) {
        let visual_state = self.get_visual_state(control);
        let back_color = self.textbox_back_colors.get(&visual_state).unwrap().clone();
        let text_color = self.textbox_text_colors.get(&visual_state).unwrap().clone();
        let border_color = self
            .textbox_border_colors
            .get(&visual_state)
            .unwrap()
            .clone();

        self.quad(control.rect, back_color, border_color);

        self.draw_text(
            textbox.text,
            control.rect,
            text_color,
            Alignment::Start,
            Alignment::Center,
        );

        // Now we draw the caret overlay + selection
        if let Some(control_state) = self.persistent_state.control_state.get(&control.uid) {
            let caret_position = self
                .position_in_multiline_string(textbox.text, control_state.textbox_caret)
                .add(control.rect.top_left());

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.draw_line(
                caret_position.to_sdl(),
                caret_position
                    .add(Point {
                        x: 0.0,
                        y: LINE_HEIGHT,
                    })
                    .to_sdl(),
            );
        }
    }

    fn textbox_get_content_size(&self, control: Control, textbox: Textbox) -> Point {
        Default::default()
    }

    fn textbox_index_at_point(
        &mut self,
        control: Control,
        textbox: Textbox,
        scroll: Point,
        point: Point,
    ) -> Option<usize> {
        return Some(
            self.index_in_multiline_string(textbox.text, point.sub(control.rect.top_left())),
        );
    }
}
