pub mod control;
pub mod geo;
pub mod input;
pub mod standard_styler;
pub mod styler;

use crate::control::{Button, Control, Listbox, Scrollbar, Textbox};
use crate::geo::{Point, Rect};
use crate::input::Input;
use crate::styler::Styler;
use std::collections::HashMap;

// state for all types of controls flattened into one struct
#[derive(Copy, Clone, Default)]
struct PersistentControlState {
    scrollbar_value: f32,
    textbox_caret: usize,
    textbox_selection_start: Option<usize>,
    textbox_selection_end: Option<usize>,
}

#[derive(Clone, Default)]
pub struct PersistentState {
    active_control: Option<i64>,
    current_input: Input,
    last_input: Input,
    mouse_down_position: Point,
    clear_active_control_after_mouse_up: bool,
    control_state: HashMap<i64, PersistentControlState>,
}

impl PersistentState {
    pub fn new() -> PersistentState {
        return PersistentState {
            clear_active_control_after_mouse_up: true,
            ..Default::default()
        };
    }
}

pub struct Ugui<T: Styler> {
    pub styler: T,
    pub persistent_state: PersistentState,
}

impl<T: Styler> Ugui<T> {
    fn ensure_control_data_exists(&mut self, uid: i64) {
        self.persistent_state
            .control_state
            .entry(uid)
            .or_insert_with(Default::default);
    }
    fn get_control_data(&mut self, uid: i64) -> PersistentControlState {
        self.ensure_control_data_exists(uid);
        return *self.persistent_state.control_state.get(&uid).unwrap();
    }
    fn update_control_data(
        &mut self,
        uid: i64,
        setter: impl FnOnce(PersistentControlState) -> PersistentControlState,
    ) {
        self.ensure_control_data_exists(uid);

        let data = self.persistent_state.control_state.get(&uid).unwrap();
        let new_data = setter(*data);
        self.persistent_state.control_state.insert(uid, new_data);
    }

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
            self.persistent_state.clear_active_control_after_mouse_up = true;
            return true;
        }

        false
    }
    pub fn button(&mut self, control: Control, button: Button) -> bool {
        let pushed = self.process_push(control);
        self.styler.button(control, button);
        pushed
    }
    pub fn scrollbar(&mut self, control: Control, scrollbar: Scrollbar) -> f32 {
        let pushed = self.process_push(control);
        let is_horizontal = control.rect.w > control.rect.h;
        let mut value = scrollbar.value;

        if pushed
            || self
                .persistent_state
                .active_control
                .is_some_and(|x| x == control.uid)
        {
            let relative_mouse = self
                .persistent_state
                .current_input
                .mouse_position
                .sub(control.rect.top_left());
            let relative_mouse_down = self
                .persistent_state
                .mouse_down_position
                .sub(control.rect.top_left());

            let current: f32;
            let start: f32;

            if is_horizontal {
                current = relative_mouse.x / control.rect.w;
                start = relative_mouse_down.x / control.rect.w;
            } else {
                current = relative_mouse.y / control.rect.h;
                start = relative_mouse_down.y / control.rect.h;
            }

            value = start + (current - start);
        }

        value = value.clamp(0.0, 1.0);
        self.update_control_data(control.uid, |x| PersistentControlState {
            scrollbar_value: value,
            ..x
        });

        self.styler.scrollbar(control, scrollbar);

        value
    }
    pub fn listbox(&mut self, mut control: Control, listbox: Listbox) -> Option<usize> {
        let horizontal_scrollbar = Control {
            uid: control.uid + 1,
            enabled: control.enabled,
            rect: Rect {
                x: control.rect.x,
                y: control.rect.bottom() - 16.0,
                w: control.rect.w - 16.0,
                h: 16.0,
            },
        };

        let vertical_scrollbar = Control {
            uid: control.uid + 2,
            enabled: control.enabled,
            rect: Rect {
                x: control.rect.right() - 16.0,
                y: control.rect.y,
                w: 16.0,
                h: control.rect.h,
            },
        };

        let horizontal_scrollbar_value = self
            .get_control_data(horizontal_scrollbar.uid)
            .scrollbar_value;
        let vertical_scrollbar_value = self
            .get_control_data(vertical_scrollbar.uid)
            .scrollbar_value;

        let content_size = self.styler.listbox_get_content_size(control, listbox);

        let content_ratio = Point {
            x: content_size.x / control.rect.w,
            y: content_size.y / control.rect.h,
        };

        let mut scroll = Point::default();

        // For horizontal overflow, shrink control bounds and place a horizontal scrollbar
        if content_ratio.x > 1.0 {
            control.rect.h -= 16.0;
            self.scrollbar(
                horizontal_scrollbar,
                Scrollbar {
                    value: horizontal_scrollbar_value,
                    ratio: content_ratio.x,
                },
            );

            scroll.x = horizontal_scrollbar_value;
        }

        // For vertical overflow, shrink control bounds and place a vertical scrollbar
        if content_ratio.y > 1.0 {
            control.rect.w -= 16.0;
            self.scrollbar(
                vertical_scrollbar,
                Scrollbar {
                    value: vertical_scrollbar_value,
                    ratio: content_ratio.y,
                },
            );

            scroll.y = vertical_scrollbar_value;
        }

        let pushed = self.process_push(control);
        let mut index = listbox.index;

        if pushed
            || self
                .persistent_state
                .active_control
                .is_some_and(|x| x == control.uid)
        {
            index = self.styler.listbox_index_at_point(
                control,
                listbox,
                scroll,
                self.persistent_state
                    .current_input
                    .mouse_position
                    .sub(control.rect.top_left()),
            );
        }

        self.styler.listbox(control, listbox, scroll);

        index
    }

    pub fn textbox(&mut self, mut control: Control, textbox: Textbox) -> String {
        let horizontal_scrollbar = Control {
            uid: control.uid + 1,
            enabled: control.enabled,
            rect: Rect {
                x: control.rect.x,
                y: control.rect.bottom() - 16.0,
                w: control.rect.w - 16.0,
                h: 16.0,
            },
        };

        let vertical_scrollbar = Control {
            uid: control.uid + 2,
            enabled: control.enabled,
            rect: Rect {
                x: control.rect.right() - 16.0,
                y: control.rect.y,
                w: 16.0,
                h: control.rect.h,
            },
        };

        let horizontal_scrollbar_value = self
            .get_control_data(horizontal_scrollbar.uid)
            .scrollbar_value;
        let vertical_scrollbar_value = self
            .get_control_data(vertical_scrollbar.uid)
            .scrollbar_value;

        let content_size = self.styler.textbox_get_content_size(control, textbox);

        let content_ratio = Point {
            x: content_size.x / control.rect.w,
            y: content_size.y / control.rect.h,
        };

        let mut scroll = Point::default();

        // For horizontal overflow, shrink control bounds and place a horizontal scrollbar
        if content_ratio.x > 1.0 {
            control.rect.h -= 16.0;
            self.scrollbar(
                horizontal_scrollbar,
                Scrollbar {
                    value: horizontal_scrollbar_value,
                    ratio: content_ratio.x,
                },
            );

            scroll.x = horizontal_scrollbar_value;
        }

        // For vertical overflow, shrink control bounds and place a vertical scrollbar
        if content_ratio.y > 1.0 {
            control.rect.w -= 16.0;
            self.scrollbar(
                vertical_scrollbar,
                Scrollbar {
                    value: vertical_scrollbar_value,
                    ratio: content_ratio.y,
                },
            );

            scroll.y = vertical_scrollbar_value;
        }

        let pushed = self.process_push(control);
        let mut text = textbox.text;

        // Since this is a textbox, we shouldn't clear the active control after releasing the mouse
        // The active control will be cleared manually in the next step
        if pushed {
            self.persistent_state.clear_active_control_after_mouse_up = false;
        }

        if self
            .persistent_state
            .active_control
            .is_some_and(|x| x == control.uid)
            && !self
                .persistent_state
                .current_input
                .mouse_position
                .inside(control.rect)
            && (self.persistent_state.current_input.primary_down
                && !self.persistent_state.last_input.primary_down)
        {
            self.persistent_state.active_control = None;
        }

        if (pushed
            || self
                .persistent_state
                .active_control
                .is_some_and(|x| x == control.uid))
            && self.persistent_state.current_input.primary_down
        {
            let index = self.styler.textbox_index_at_point(
                control,
                textbox,
                scroll,
                self.persistent_state.current_input.mouse_position,
            );
            self.update_control_data(control.uid, |x| PersistentControlState {
                textbox_caret: index.unwrap(),
                ..x
            });
        }

        self.styler.textbox(control, textbox, scroll);

        text.clone()
    }

    pub fn begin(&mut self, input: Input) {
        self.persistent_state.last_input = self.persistent_state.current_input;
        self.persistent_state.current_input = input;

        if self.persistent_state.current_input.primary_down
            && !self.persistent_state.last_input.primary_down
        {
            self.persistent_state.mouse_down_position =
                self.persistent_state.current_input.mouse_position;
        }

        self.styler.begin(self.persistent_state.clone());
    }

    pub fn end(&mut self) {
        self.styler.end();

        // As soon as we let go of the primary mouse button, the active control should be cleared
        if self.persistent_state.active_control.is_some()
            && !self.persistent_state.current_input.primary_down
            && self.persistent_state.clear_active_control_after_mouse_up
        {
            self.persistent_state.active_control = None;
        }
    }
}
