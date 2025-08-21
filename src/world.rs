use crate::{
    Block, Player,
    block::BlockFlag,
    blocks::{block_air, block_dirt, block_stone},
    items::item_from_name,
};
use glam::IVec2;
use noise::{NoiseFn, Perlin};
use std::{array, collections::HashMap};

#[derive(Clone)]
pub struct Chunk {
    tiles: [[Block; Chunk::SIZE]; Chunk::SIZE],
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct World {
    chunks: HashMap<IVec2, Chunk>,
    perlin: Perlin,
    active_chunks: Vec<IVec2>,
}

impl Chunk {
    const SIZE: usize = 16; // 16x16 chunks
    const SIZE_I: i32 = Chunk::SIZE as i32;
    #[must_use]
    pub fn world_to_chunk(x: i32, y: i32) -> (i32, i32) {
        (x / Chunk::SIZE_I, y / Chunk::SIZE_I)
    }
    #[must_use]
    pub fn chunk_to_world(
        chunk_x: i32,
        chunk_y: i32,
        x: i32,
        y: i32,
    ) -> (i32, i32) {
        (chunk_x * Chunk::SIZE_I + x, chunk_y * Chunk::SIZE_I + y)
    }
    // @TODO: make this actually use RNG
    // like perlin noise or whatever
    fn generate_block(x: i32, y: i32, perlin: Perlin, scale: f64) -> Block {
        assert!(
            scale != 1.0,
            "Scale is equal to 1! This will result in uniform terrain"
        );
        let mut block: Block = if y >= 40 {
            let noise =
                perlin.get([f64::from(x) * scale, f64::from(y) * scale]);
            if noise < 0.5 {
                block_dirt(IVec2::new(x, y))
            } else {
                block_stone(IVec2::new(x, y))
            }
        } else {
            block_air(IVec2::new(x, y))
        };
        if block.pos.x % Chunk::SIZE as i32 == 0
            || block.pos.y % Chunk::SIZE as i32 == 0
        {
            block.add_flag(BlockFlag::Highlight);
        }
        block
    }
    fn new(chunk_x: i32, chunk_y: i32, perlin: Perlin) -> Self {
        let tiles: [[Block; Chunk::SIZE]; Chunk::SIZE] = array::from_fn(|x| {
            array::from_fn(|y| {
                let (world_x, world_y) =
                    Chunk::chunk_to_world(chunk_x, chunk_y, x as i32, y as i32);
                Chunk::generate_block(world_x, world_y, perlin, 0.3)
            })
        });
        Self {
            x: chunk_x,
            y: chunk_y,
            tiles,
        }
    }
    #[must_use]
    pub fn flatten(&self) -> Vec<Block> {
        // @TODO: when Block implements copy, this should not use .cloned() {super slow!}
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .collect()
    }
}

impl World {
    #[must_use]
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            perlin: Perlin::new(seed),
            active_chunks: Default::default(),
        }
    }

    pub fn update_around_point(
        &mut self,
        x: f32,
        y: f32,
        load_width: f32,
        load_height: f32,
    ) {
        let center_chunk_x = (x.floor() as i32).div_euclid(Chunk::SIZE_I);
        let center_chunk_y = (y.floor() as i32).div_euclid(Chunk::SIZE_I);

        let half_width_chunks =
            ((load_width / Chunk::SIZE_I as f32) / 2.0).ceil() as i32;
        let half_height_chunks =
            ((load_height / Chunk::SIZE_I as f32) / 2.0).ceil() as i32;

        for chunk_y in (center_chunk_y - half_height_chunks)
            ..=(center_chunk_y + half_height_chunks)
        {
            for chunk_x in (center_chunk_x - half_width_chunks)
                ..=(center_chunk_x + half_width_chunks)
            {
                if let std::collections::hash_map::Entry::Vacant(e) =
                    self.chunks.entry(IVec2::new(chunk_x, chunk_y))
                {
                    e.insert(Chunk::new(chunk_x, chunk_y, self.perlin));
                }
            }
        }
    }

    pub fn update_active_chunks(
        &mut self,
        x: f32,
        y: f32,
        get_width: i32,
        get_height: i32,
    ) {
        let center_chunk_x = (x.floor() as i32).div_euclid(Chunk::SIZE_I);
        let center_chunk_y = (y.floor() as i32).div_euclid(Chunk::SIZE_I);

        let half_chunks_x =
            (get_width as f32 / Chunk::SIZE_I as f32 / 2.0).ceil() as i32;
        let half_chunks_y =
            (get_height as f32 / Chunk::SIZE_I as f32 / 2.0).ceil() as i32;

        let mut chunks = Vec::new();

        for chunk_y in
            (center_chunk_y - half_chunks_y)..=(center_chunk_y + half_chunks_y)
        {
            for chunk_x in (center_chunk_x - half_chunks_x)
                ..=(center_chunk_x + half_chunks_x)
            {
                if let Some(_) = self.chunks.get(&IVec2::new(chunk_x, chunk_y))
                {
                    chunks.push(IVec2::new(chunk_x, chunk_y));
                }
            }
        }
        self.active_chunks = chunks;
    }

    pub fn get_active_chunks(&self) -> Vec<&Chunk> {
        let mut chunks: Vec<&Chunk> = Vec::new();
        for chunk_index in self.active_chunks.iter() {
            chunks.push(self.chunks.get(chunk_index).unwrap());
        }
        chunks
    }

    #[must_use]
    pub fn get_block(&self, x: i32, y: i32) -> Option<&Block> {
        let chunk_x = x.div_euclid(Chunk::SIZE_I);
        let chunk_y = y.div_euclid(Chunk::SIZE_I);

        let chunk = self.chunks.get(&IVec2::new(chunk_x, chunk_y))?;
        let local_x = x.rem_euclid(Chunk::SIZE_I);
        let local_y = y.rem_euclid(Chunk::SIZE_I);

        chunk.tiles.get(local_x as usize)?.get(local_y as usize)
    }
    pub fn get_block_mut(&mut self, x: i32, y: i32) -> Option<&mut Block> {
        let chunk_x = x.div_euclid(Chunk::SIZE_I);
        let chunk_y = y.div_euclid(Chunk::SIZE_I);

        let chunk: &mut Chunk =
            self.chunks.get_mut(&IVec2::new(chunk_x, chunk_y))?;
        let local_x = x.rem_euclid(Chunk::SIZE_I);
        let local_y = y.rem_euclid(Chunk::SIZE_I);

        chunk
            .tiles
            .get_mut(local_x as usize)?
            .get_mut(local_y as usize)
    }
    pub fn hit_block(&mut self, x: i32, y: i32, player: &mut Player) {
        let block = self.get_block_mut(x, y).unwrap();
        if block.can_be_hit() {
            let mut damage = player.calculate_mining_speed();
            if player.mining_spread < block.required_level {
                damage /= 2.0;
            }
            block.health -= damage;
            if block.health <= 0.0 {
                if let Some(item) = block.drop_item {
                    player.add_item(item_from_name(item, 1));
                }
                self.remove_block(x, y);
            }
        }
    }
    pub fn remove_block(&mut self, x: i32, y: i32) {
        self.get_block_mut(x, y).map(|block| {
            *block = block_air(IVec2::new(x, y));
        });
    }
}
