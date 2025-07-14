#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
/** convert screen coordinates to world coordinates
 *  pos: the point on the screen
 *  camera_pos: the camera's position in worldspace
 *  scale: scaling from camera size to window size
 */
pub fn screen_to_world(pos: Point2, camera_pos: Point2, scale: f32) -> Point2 {
    let global_x = pos.x / scale + camera_pos.x;
    let global_y = pos.y / scale + camera_pos.y;
    Point2::new(global_x, global_y)
}
