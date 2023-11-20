use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn inflate(&self, v: i32) -> Rect {
        Rect {
            x: self.x - v,
            y: self.y - v,
            w: self.w + v * 2,
            h: self.h + v * 2,
        }
    }
    pub fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x, self.y, self.w as u32, self.h as u32)
    }
}
