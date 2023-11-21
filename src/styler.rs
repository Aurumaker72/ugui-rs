use crate::geo::Rect;
use crate::*;

pub trait Styler {
    fn begin(&mut self, persistent_state: PersistentState);
    fn button(&mut self, control: Control, button: Button);
    fn scrollbar(&mut self, control: Control, scrollbar: Scrollbar);
    fn listbox(&mut self, control: Control, listbox: Listbox);
    fn listbox_index_at_point(
        &mut self,
        control: Control,
        listbox: Listbox,
        point: Point,
    ) -> Option<usize>;
    fn end(&mut self);
}
