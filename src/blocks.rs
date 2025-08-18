use crate::{Block, BlockName, block::BlockFlag, item::ItemName};
// @TODO: make this use textures instead of solid colors
// maybe also have LOD textures, as well as quality scales
// or we could just do some dirty algorithms on first launch to write LODs and downscaled textures to a directory
// so that all we have to do is link to some highdef texture (128x128?) and it does the rest
// but that would probably require something like `pub fn get_all_textures` that returns each `block_*`.texture_path
// and definitely texture caching. we can't have, like, 100 I/O operations a frame

pub fn block_dirt(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (139, 69, 19),
        BlockName::Dirt,
        true,
        [Some(BlockFlag::Dig), None, None, None, None, None],
        0,  // dirt is soft
        50, // so it takes litle time
        None,
    )
}

pub fn block_air(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (135, 206, 235),
        BlockName::Air,
        false,
        [None, None, None, None, None, None],
        0,
        0,
        None,
    )
}

pub fn block_void(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (0, 0, 0),
        BlockName::Void,
        false,
        [None, None, None, None, None, None],
        0,
        0,
        None,
    )
}

pub fn block_stone(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (163, 140, 132),
        BlockName::Void,
        true,
        [Some(BlockFlag::Mine), None, None, None, None, None],
        1,
        100,
        Some(ItemName::Stone),
    )
}
