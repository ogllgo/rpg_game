use crate::item::{Item, ItemBuilder, ItemName, ItemProps, ItemRarity};

#[must_use]
pub fn item_from_name(item_name: ItemName, amount: usize) -> Item {
    match item_name {
        ItemName::Stone => item_stone(amount),
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
        .unwrap() // safe because we used the Default, meaning all field are init'd
}
