use crate::geo::Rect;

pub struct Control {
    pub uid: u64,
    pub enabled: bool,
    pub rect: Rect,
}

pub struct Button {
    pub text: String,
}
