use glam::IVec2;

use crate::{
    Block, BlockName,
    block::{BlockBuilder, BlockFlag},
    item::ItemName,
};

#[must_use]
pub fn block_dirt(pos: IVec2) -> Block {
    BlockBuilder::new()
        .pos(pos)
        .color((139, 69, 19))
        .block_type(BlockName::Dirt)
        .can_collide(true)
        .add_flag(BlockFlag::Dig)
        .required_level(0)
        .max_health(50)
        .build()
}

#[must_use]
pub fn block_air(pos: IVec2) -> Block {
    BlockBuilder::new()
        .pos(pos)
        .color((135, 206, 235))
        .block_type(BlockName::Air)
        .can_collide(false)
        .required_level(0)
        .max_health(0)
        .build()
}

#[must_use]
pub fn block_stone(pos: IVec2) -> Block {
    BlockBuilder::new()
        .pos(pos)
        .color((163, 140, 132))
        .block_type(BlockName::Stone)
        .can_collide(true)
        .add_flag(BlockFlag::Mine)
        .required_level(1)
        .max_health(100)
        .drop_item(Some(ItemName::Stone))
        .build()
}
