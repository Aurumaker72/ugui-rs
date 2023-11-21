use crate::geo::Rect;

#[derive(Copy, Clone)]
pub struct Control {
    pub uid: i64,
    pub enabled: bool,
    pub rect: Rect,
}

#[derive(Copy, Clone)]
pub struct Button<'a> {
    pub text: &'a String,
}
