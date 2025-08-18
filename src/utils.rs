use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
pub fn screen_to_world(pos: Vec2, camera_pos: Vec2, scale: f32) -> Vec2 {
    let global_x = pos.x / scale + camera_pos.x;
    let global_y = pos.y / scale + camera_pos.y;
    Vec2::new(global_x, global_y)
}
