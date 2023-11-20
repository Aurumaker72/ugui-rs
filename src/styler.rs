use crate::*;

pub trait Styler {
    fn begin(&mut self);
    fn button(&mut self, control: Control, button: Button);
    fn end(&mut self);
}
