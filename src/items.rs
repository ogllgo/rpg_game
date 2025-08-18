use crate::item::{Item, ItemName, ItemProps, ItemRarity};

#[must_use] pub fn item_from_name(item_name: ItemName, amount: usize) -> Item {
    match item_name {
        ItemName::Stone => item_stone(amount),
    }
}

#[must_use] pub fn item_stone(amount: usize) -> Item {
    Item::new(
        64,
        amount,
        ItemRarity::Common,
        ItemName::Stone,
        (1, 1, 1),
        ItemProps::None,
    )
}
