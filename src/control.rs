use crate::geo::Rect;

#[derive(Copy, Clone)]
pub struct Control {
    pub uid: i64,
    pub enabled: bool,
    pub rect: Rect,
}

impl Default for Control {
    fn default() -> Self {
        Control {
            enabled: true,
            uid: 0,
            rect: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Button<'a> {
    pub text: &'a str,
}

#[derive(Copy, Clone)]
pub struct Scrollbar {
    pub value: f32,
    pub ratio: f32,
}

#[derive(Copy, Clone)]
pub struct Listbox<'a> {
    pub items: &'a Vec<&'a str>,
    pub index: Option<usize>,
}

#[derive(Copy, Clone)]
pub struct Textbox<'a> {
    pub text: &'a String,
}
