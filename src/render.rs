use sdl2::{rect::FRect, render::Canvas, video::Window};

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn max_x(&self) -> f32 {
        self.x + self.w
    }

    pub fn max_y(&self) -> f32 {
        self.y + self.h
    }

    pub fn area(&self) -> f32 {
        self.w.max(0.0) * self.h.max(0.0)
    }

    pub fn clamp(&self, clamp: Self) -> Self {
        let x1 = self.x.max(clamp.x);
        let y1 = self.y.max(clamp.y);
        let x2 = self.max_x().min(clamp.max_x());
        let y2 = self.max_y().min(clamp.max_y());

        let w = (x2 - x1).max(0.0);
        let h = (y2 - y1).max(0.0);

        Rect::new(x1, y1, w, h)
    }

    pub fn as_frect(&self) -> FRect {
        FRect::new(self.x, self.y, self.w, self.h)
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.fill_frect::<FRect>((*self).into())?;
        Ok(())
    }
}

impl From<Rect> for FRect {
    fn from(value: Rect) -> Self {
        Self::new(value.x, value.y, value.w, value.h)
    }
}
/// Returns if the two rects overlap
pub fn overlaps(a: Rect, b: Rect) -> bool {
    a.x <= b.max_x() && a.max_x() >= b.x && a.y <= b.max_y() && a.max_y() >= b.y
}

pub fn draw_rect_with_hole(
    canvas: &mut Canvas<Window>,
    rect: Rect,
    hole: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    // if they don't collide,
    // or the hole has w/h of 0/0
    // then just draw the rect
    if !overlaps(rect, hole) || hole.area() == 0.0 {
        canvas.fill_frect(rect.as_frect())?;
        return Ok(());
    }

    let hx1 = hole.x.max(rect.x);
    let hy1 = hole.y.max(rect.y);
    let hx2 = hole.max_x().min(rect.max_x());
    let hy2 = hole.max_y().min(rect.max_y());

    // Left piece
    if hx1 > rect.x {
        Rect::new(rect.x, rect.y, hx1 - rect.x, rect.h).draw(canvas)?;
    }

    // Right piece
    if hx2 < rect.max_x() {
        Rect::new(hx2, rect.y, rect.max_x() - hx2, rect.h).draw(canvas)?;
    }

    // Top piece
    if hy1 > rect.y {
        Rect::new(hx1, rect.y, hx2 - hx1, hy1 - rect.y).draw(canvas)?;
    }

    // Bottom piece
    if hy2 < rect.max_y() {
        Rect::new(hx1, hy2, hx2 - hx1, rect.max_y() - hy2).draw(canvas)?;
    }

    Ok(())
}
