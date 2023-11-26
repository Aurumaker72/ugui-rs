use crate::control::Textbox;
use crate::*;

pub trait Styler {
    fn begin(&mut self, persistent_state: PersistentState);
    fn button(&mut self, control: Control, button: Button);
    fn scrollbar(&mut self, control: Control, scrollbar: Scrollbar);
    fn listbox(&mut self, control: Control, listbox: Listbox, scroll: Point);
    fn listbox_get_content_size(&self, control: Control, listbox: Listbox) -> Point;
    fn listbox_index_at_point(
        &mut self,
        control: Control,
        listbox: Listbox,
        scroll: Point,
        point: Point,
    ) -> Option<usize>;
    fn textbox(&mut self, control: Control, textbox: Textbox, scroll: Point);

    fn end(&mut self);
}
