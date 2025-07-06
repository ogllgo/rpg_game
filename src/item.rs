pub enum ItemRarity {
    Common,
}
pub enum ItemName {
    Stone,
}
pub struct Item {
    max_stack: usize,
    rarity: ItemRarity,
    name: ItemName,
}
