use crate::{Block, Player, block::BlockFlag, blocks::*, items::*};
use glam::{IVec2, Vec2};
use hecs::World as HecsWorld;
use noise::{NoiseFn, Perlin};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Chunk {
    tiles: [[Block; Chunk::SIZE]; Chunk::SIZE],
    pub x: i32,
    pub y: i32,
}
pub struct World {
    chunks: HashMap<IVec2, Chunk>,
    perlin: Perlin,
}

pub struct KeyboardInput {
    pub held: HashSet<Keycode>,
    pub released: HashSet<Keycode>,
    pub pressed: HashSet<Keycode>,
}

pub struct MouseInput {
    pub held: HashSet<MouseButton>,
    pub released: HashSet<MouseButton>,
    pub pressed: HashSet<MouseButton>,
    pub pos: Vec2,
}

pub struct Input {
    pub keyboard: KeyboardInput,
    pub mouse: MouseInput,
}

pub struct Game {
    pub map: World,
    pub ecs: HecsWorld,
    pub input: Input,
}

impl Chunk {
    const SIZE: usize = 16; // 16x16 chunks
    const SIZE_I: i32 = Chunk::SIZE as i32;
    pub fn world_to_chunk(x: i32, y: i32) -> (i32, i32) {
        (x / Chunk::SIZE_I, y / Chunk::SIZE_I)
    }
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
            let noise = perlin.get([x as f64 * scale, y as f64 * scale]);
            if noise < 0.5 {
                block_dirt(x, y)
            } else {
                block_stone(x, y)
            }
        } else {
            block_air(x, y)
        };
        if block.x % Chunk::SIZE as i32 == 0
            || block.y % Chunk::SIZE as i32 == 0
        {
            block.add_flag(BlockFlag::Highlight);
        }
        block
    }
    fn new(chunk_x: i32, chunk_y: i32, perlin: Perlin) -> Self {
        let mut tiles: [[Block; Chunk::SIZE]; Chunk::SIZE] =
            [[block_void(0, 0); Chunk::SIZE]; Chunk::SIZE];
        for x in 0..Chunk::SIZE {
            for y in 0..Chunk::SIZE {
                let (world_x, world_y) =
                    Chunk::chunk_to_world(chunk_x, chunk_y, x as i32, y as i32);
                let block =
                    Chunk::generate_block(world_x, world_y, perlin, 0.3);
                tiles[x][y] = block;
            }
        }
        Self {
            x: chunk_x,
            y: chunk_y,
            tiles: tiles,
        }
    }
    pub fn flatten(&self) -> Vec<Block> {
        // @TODO: when Block implements copy, this should not use .cloned() {super slow!}
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .collect()
    }
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            perlin: Perlin::new(seed),
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
            ((load_width as f32 / Chunk::SIZE_I as f32) / 2.0).ceil() as i32;
        let half_height_chunks =
            ((load_height as f32 / Chunk::SIZE_I as f32) / 2.0).ceil() as i32;

        for chunk_y in (center_chunk_y - half_height_chunks)
            ..=(center_chunk_y + half_height_chunks)
        {
            for chunk_x in (center_chunk_x - half_width_chunks)
                ..=(center_chunk_x + half_width_chunks)
            {
                if !self.chunks.contains_key(&IVec2::new(chunk_x, chunk_y)) {
                    self.chunks.insert(
                        IVec2::new(chunk_x, chunk_y),
                        Chunk::new(chunk_x, chunk_y, self.perlin),
                    );
                }
            }
        }
    }

    // @TODO: make this `get_chunks_around_point`, they can do the rest
    pub fn get_chunks_around_point(
        &self,
        x: f32,
        y: f32,
        get_width: i32,
        get_height: i32,
    ) -> Vec<&Chunk> {
        let center_chunk_x = (x.floor() as i32).div_euclid(Chunk::SIZE_I);
        let center_chunk_y = (y.floor() as i32).div_euclid(Chunk::SIZE_I);

        // Convert width/height in blocks to chunks (round up maybe)
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
                if let Some(chunk) =
                    self.chunks.get(&IVec2::new(chunk_x, chunk_y))
                {
                    chunks.push(chunk);
                }
            }
        }

        chunks
    }

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
            if player.mining_spread < block.required_level.try_into().unwrap() {
                damage /= 2.0;
            }
            block.health -= damage;
            if block.health <= 0.0 {
                if let Some(item) = block.drop_item {
                    player.add_item(item_from_name(item, 1));
                }
                self.remove_block(x, y);
                println!("{:?}", player.inventory);
            }
        }
    }
    pub fn remove_block(&mut self, x: i32, y: i32) {
        let block = self.get_block_mut(x, y).expect(&format!(
            "There should be a block at ({}, {}), but there isn't",
            x, y
        ));
        *block = block_air(x, y);
    }
}

impl KeyboardInput {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            released: HashSet::new(),
            pressed: HashSet::new(),
        }
    }
}

impl MouseInput {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            released: HashSet::new(),
            pressed: HashSet::new(),
            pos: Vec2::ZERO,
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardInput::new(),
            mouse: MouseInput::new(),
        }
    }
    fn update(&mut self, event: &Event) {
        self.keyboard.pressed.clear();
        self.keyboard.released.clear();

        self.mouse.pressed.clear();
        self.mouse.released.clear();
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                self.keyboard.pressed.insert(*key);
                self.keyboard.held.insert(*key);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                self.keyboard.released.insert(*key);
                self.keyboard.held.remove(key);
            }
            _ => {}
        }
    }
}

impl Game {
    pub fn new(seed: u32) -> Self {
        Self {
            map: World::new(seed),
            ecs: HecsWorld::new(),
            input: Input::new(),
        }
    }
    pub fn manage_input(&mut self, event: &Event) {
        self.input.update(event);
    }
    pub fn tick() {}
}
