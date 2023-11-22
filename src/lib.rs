pub mod control;
pub mod geo;
pub mod input;
pub mod standard_styler;
pub mod styler;

use crate::control::{Button, Control, Listbox, Scrollbar};
use crate::geo::{Point, Rect};
use crate::input::Input;
use crate::styler::Styler;
use std::collections::HashMap;

// state for all types of controls flattened into one struct
#[derive(Copy, Clone, Default)]
struct PersistentControlState {
    scrollbar_value: f32,
}

#[derive(Clone, Default)]
pub struct PersistentState {
    active_control: Option<i64>,
    current_input: Input,
    last_input: Input,
    mouse_down_position: Point,
    control_state: HashMap<i64, PersistentControlState>,
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

            let current = relative_mouse.y / control.rect.h;
            let start = relative_mouse_down.y / control.rect.h;
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
        control.rect.w -= 16.0;

        let scrollbar_control = Control {
            uid: control.uid + 1,
            enabled: control.enabled,
            rect: Rect {
                x: control.rect.right(),
                y: control.rect.y,
                w: 16.0,
                h: control.rect.h,
            },
        };

        let scrollbar_control_data = self.get_control_data(scrollbar_control.uid);

        self.scrollbar(
            scrollbar_control,
            Scrollbar {
                value: scrollbar_control_data.scrollbar_value,
                ratio: 1.5,
            },
        );

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
                Point {
                    x: 0.0,
                    y: scrollbar_control_data.scrollbar_value,
                },
                self.persistent_state
                    .current_input
                    .mouse_position
                    .sub(control.rect.top_left()),
            );
        }

        self.styler.listbox(
            control,
            listbox,
            Point {
                x: 0.0,
                y: scrollbar_control_data.scrollbar_value,
            },
        );

        index
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
        {
            self.persistent_state.active_control = None;
        }
    }
}
