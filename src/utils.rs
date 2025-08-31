use crate::item::Item;

#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}

pub fn can_stack(a: &Item, b: &Item) -> bool {
    a.name == b.name && a.rarity == b.rarity && a.props == b.props
}
