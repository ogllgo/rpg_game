use glam::IVec2;

use crate::{
    Block, BlockName,
    block::{BlockBuilder, BlockFlag},
    item::ItemName,
};
// @TODO: make this use textures instead of solid colors
// maybe also have LOD textures, as well as quality scales
// or we could just do some dirty algorithms on first launch to write LODs and downscaled textures to a directory
// so that all we have to do is link to some highdef texture (128x128?) and it does the rest
// but that would probably require something like `pub fn get_all_textures` that returns each `block_*`.texture_path
// and definitely texture caching. we can't have, like, 100 I/O operations a frame

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
