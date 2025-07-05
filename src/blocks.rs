use crate::{ block::BlockType, Block, BlockName };

pub fn block_dirt(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (139, 69, 19),
        BlockName::Dirt,
        true,
        vec![BlockType::Dirt],
        0, // dirt is soft
        50, // so it takes litle time
    )
}

pub fn block_air(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (135, 206, 235),
        BlockName::Air,
        false,
        vec![],
        0,
        0,
    )
}

pub fn block_void(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (0, 0, 0),
        BlockName::Void,
        false,
        vec![],
        0,
        0,
    )
}
pub fn block_stone(x: i32, y: i32) -> Block {
    Block::new(
        x,
        y,
        (163, 140, 132),
        BlockName::Void,
        true,
        vec![BlockType::Rock],
        1, 
        100, 
    )
}