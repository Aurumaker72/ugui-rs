use crate::*;

pub trait Styler {
    fn begin(&mut self, persistent_state: PersistentState);
    fn button(&mut self, control: Control, button: Button);
    fn end(&mut self);
}
