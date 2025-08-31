use crate::item::{Item, ItemBuilder, ItemName, ItemProps, ItemRarity};

#[must_use]
pub fn item_from_name(item_name: ItemName, amount: usize) -> Item {
    match item_name {
        ItemName::Stone => item_stone(amount),
        ItemName::Dirt => item_dirt(amount),
    }
}

#[must_use]
pub fn item_stone(amount: usize) -> Item {
    ItemBuilder::default()
        .max_stack(64)
        .amount(amount)
        .name(ItemName::Stone)
        .rarity(ItemRarity::Common)
        .color((1, 1, 1))
        .props(ItemProps::None)
        .build()
        .unwrap()
}

#[must_use]
pub fn item_dirt(amount: usize) -> Item {
    ItemBuilder::default()
        .max_stack(64)
        .amount(amount)
        .name(ItemName::Dirt)
        .rarity(ItemRarity::Common)
        .color((160, 82, 45))
        .props(ItemProps::None)
        .build()
        .unwrap()
}
