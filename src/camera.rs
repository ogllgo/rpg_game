use glam::Vec2;
use std::fmt;

fn rotate(v: Vec2, theta: f32) -> Vec2 {
    let (s, c) = theta.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Camera {
    pub pos: Vec2,
    pub viewport_dims: Vec2,
    pub rot: f32,
    window_dims: Vec2, // private
}

impl Camera {
    #[must_use]
    pub fn new(
        pos: Vec2,
        viewport_dims: Vec2,
        window_dims: Vec2,
        rot: f32,
    ) -> Self {
        Self {
            pos,
            viewport_dims,
            rot,
            window_dims, // keep for transforms
        }
    }

    /// Convert world/global position -> screen space
    #[must_use]
    pub fn global_to_screen(&self, world_pos: Vec2) -> Vec2 {
        // world - self
        // rotate(-rot)
        // * scale
        // + (dims / 2)
        let mut local = world_pos - self.pos;
        local = rotate(local, -self.rot);
        local *= self.pixels_per_unit();
        local + self.window_dims * 0.5
    }

    /// Convert screen space -> world/global position
    #[must_use]
    pub fn screen_to_global(&self, screen_pos: Vec2) -> Vec2 {
        // - (dims / 2)
        // / scale
        // rotate(rot)
        // + self
        let mut local = screen_pos - self.window_dims / 2.0;
        local /= self.pixels_per_unit();
        local = rotate(local, self.rot);
        local + self.pos
    }

    pub fn scale_global_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let new = rotate(world_pos * self.pixels_per_unit(), -self.rot);
        new
    }

    pub fn pixels_per_unit(&self) -> Vec2 {
        self.window_dims / self.viewport_dims
    }

    #[must_use]
    pub fn window_size(&self) -> Vec2 {
        self.window_dims
    }

    pub fn set_window_dims(&mut self, new_dims: Vec2) {
        self.window_dims = new_dims;
    }

    pub fn center_around(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Camera {{ pos: ({:.2}, {:.2}), viewport: ({:.2}, {:.2}), window: ({:.2}, {:.2}), rot: {:.2}° }}",
            self.pos.x,
            self.pos.y,
            self.viewport_dims.x,
            self.viewport_dims.y,
            self.window_dims.x,
            self.window_dims.y,
            self.rot.to_degrees(),
        )
    }
}
