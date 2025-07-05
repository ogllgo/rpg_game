use std::collections::HashMap;

use crate::{blocks::*, Block};

type TilePos = (i32, i32);

pub struct World {
    tiles: HashMap<TilePos, Block>,
}

impl World {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    fn generate_tile(&self, x: i32, y: i32) -> Block {
        if y > 40 {
            if y == 41 && x % 2 == 0 {
                block_stone(x, y)
            } else {
                block_dirt(x, y)
            }
        } else {
            block_air(x, y)
        }
    }

    pub fn update_around_player(&mut self, player_x: f32, player_y: f32) {
        let center_x = player_x.floor() as i32;
        let center_y = player_y.floor() as i32;

        let half_width = 80 / 2;
        let half_height = 60 / 2;

        for y in (center_y - half_height)..=(center_y + half_height) {
            for x in (center_x - half_width)..=(center_x + half_width) {
                let pos = (x, y);
                if !self.tiles.contains_key(&pos) {
                    let new_block = self.generate_tile(x, y);
                    self.tiles.insert(pos, new_block);
                }
            }
        }
    }

    pub fn get_blocks_around_player(&self, player_x: f32, player_y: f32, radius_x: i32, radius_y: i32) -> Vec<Block> {
        let center_x = player_x.floor() as i32;
        let center_y = player_y.floor() as i32;

        let mut blocks = Vec::new();

        for y in (center_y - radius_y)..=(center_y + radius_y) {
            for x in (center_x - radius_x)..=(center_x + radius_x) {
                if let Some(block) = self.tiles.get(&(x, y)) {
                    blocks.push(block.clone());
                }
            }
        }

        blocks
    }

    // Optional helper to get block at world position
    pub fn get_block(&self, x: i32, y: i32) -> Option<&Block> {
        self.tiles.get(&(x, y))
    }

    pub fn hit_block(&mut self, x: i32, y: i32, damage: f32, damage_level: i32) {
        let pos = (x, y);
        if let Some(block) = self.tiles.get_mut(&pos) {
            if block.can_be_hit() {
                let mut damage = damage;
                if damage_level < block.required_level {
                    damage /= 2.0;
                }
                block.health -= damage;
            }
            if block.health <= 0.0 {
                self.tiles.insert(pos, block_air(x, y));
                // @IMPLEMENT: item drops
            }
        }
    }
    pub fn remove_block(&mut self, x: i32, y: i32) {
        self.tiles.insert((x, y), block_air(x, y));
    }
}