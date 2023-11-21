use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn inside(&self, rect: Rect) -> bool {
        self.x > rect.x && self.x < rect.right() && self.y > rect.y && self.y < rect.bottom()
    }
}
impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn inflate(&self, v: f32) -> Rect {
        Rect {
            x: self.x - v,
            y: self.y - v,
            w: self.w + v * 2.0,
            h: self.h + v * 2.0,
        }
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}
