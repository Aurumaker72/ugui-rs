use crate::*;

pub trait Styler {
    fn begin(&mut self, persistent_state: PersistentState);
    fn button(&mut self, control: Control, button: Button);
    fn listbox(&mut self, control: Control, listbox: ListBox);
    fn listbox_index_at_point(
        &mut self,
        control: Control,
        listbox: ListBox,
        point: Point,
    ) -> Option<usize>;
    fn end(&mut self);
}
