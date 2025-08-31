use glam::IVec2;

use crate::{
    block::{Block, BlockName},
    block::{BlockBuilder, BlockFlag},
    item::ItemName,
};

pub const BLOCK_COLOR_AIR: (u8, u8, u8) = (135, 206, 235);

#[must_use]
pub fn block_dirt(pos: IVec2) -> Block {
    BlockBuilder::default()
        .pos(pos)
        .color((139, 69, 19))
        .block_type(BlockName::Dirt)
        .can_collide(true)
        .flags([Some(BlockFlag::Dig), None, None, None, None, None])
        .flag_count(1)
        .required_level(0)
        .max_health(50.0)
        .health(50.0)
        .drop_item(Some(ItemName::Dirt))
        .is_solid(true)
        .last_hit_tick(0)
        .build()
        .unwrap()
}

#[must_use]
pub fn block_air(pos: IVec2) -> Block {
    BlockBuilder::default()
        .pos(pos)
        .color(BLOCK_COLOR_AIR)
        .block_type(BlockName::Air)
        .can_collide(false)
        .required_level(0)
        .max_health(0.0)
        .health(0.0)
        .drop_item(None)
        .is_solid(false)
        .flags([None, None, None, None, None, None])
        .flag_count(0)
        .last_hit_tick(0)
        .build()
        .unwrap()
}

#[must_use]
pub fn block_stone(pos: IVec2) -> Block {
    BlockBuilder::default()
        .pos(pos)
        .color((163, 140, 132))
        .block_type(BlockName::Stone)
        .can_collide(true)
        .flags([Some(BlockFlag::Mine), None, None, None, None, None])
        .flag_count(1)
        .required_level(1)
        .max_health(100.0)
        .health(100.0)
        .drop_item(Some(ItemName::Stone))
        .is_solid(true)
        .last_hit_tick(0)
        .build()
        .unwrap()
}
