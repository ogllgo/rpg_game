#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}
