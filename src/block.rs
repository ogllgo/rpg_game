use crate::item::ItemName;
use glam::IVec2;
use sdl2::{rect::FRect, render::Canvas, video::Window};

pub const BLOCK_SIZE: i32 = 10;
#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub enum BlockName {
    #[default]
    Air,
    Dirt,
    Stone,
}

#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub enum BlockFlag {
    #[default]
    Mine,
    Dig,
    Chop,
    Highlight,
}

#[derive(Clone, Debug, Copy)]
pub struct Block {
    pub pos: IVec2,
    pub color: (u8, u8, u8),
    pub block_type: BlockName,
    pub can_collide: bool,
    pub required_level: u32,
    pub health: f32,
    pub max_health: i32,
    pub drop_item: Option<ItemName>,
    pub is_solid: bool,
    flags: [Option<BlockFlag>; 6],
    flag_count: usize,
}

#[derive(Default)]
pub struct BlockBuilder {
    pos: IVec2,
    color: (u8, u8, u8),
    block_type: BlockName,
    can_collide: bool,
    required_level: u32,
    health: f32,
    max_health: i32,
    drop_item: Option<ItemName>,
    is_solid: bool,
    flags: [Option<BlockFlag>; 6],
}

impl Block {
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &FRect,
        scale: f32,
    ) {
        let screen_x = (self.pos.x as f32 - camera.x) * scale;
        let screen_y = (self.pos.y as f32 - camera.y) * scale;

        canvas.set_draw_color(self.color);
        canvas
            .fill_frect(FRect::new(screen_x, screen_y, scale, scale))
            .unwrap();
        if self.flags.iter().any(|f| {
            if let Some(bf) = f {
                *bf == BlockFlag::Highlight
            } else {
                false
            }
        }) {
            canvas.set_draw_color((
                255 - self.color.0,
                255 - self.color.1,
                255 - self.color.2,
            ));
            let highlight_screen_x =
                (self.pos.x as f32 - camera.x + 0.2) * scale;
            let highlight_screen_y =
                (self.pos.y as f32 - camera.y + 0.2) * scale;

            canvas
                .fill_frect(FRect::new(
                    highlight_screen_x,
                    highlight_screen_y,
                    scale * 0.6,
                    scale * 0.6,
                ))
                .unwrap();
        }
    }
    pub fn can_be_hit(&self) -> bool {
        self.flags.iter().any(|b| {
            if let Some(bt) = b {
                matches!(bt, BlockFlag::Mine | BlockFlag::Chop | BlockFlag::Dig)
            } else {
                false
            }
        })
    }
    pub fn add_flag(&mut self, flag: BlockFlag) {
        // if we are full on flags, don't try to add another
        if self.flag_count >= self.flags.len() {
            return;
        }
        // if the flag already exists, don't add it again
        for i in 0..self.flag_count {
            if let Some(f) = self.flags[i] {
                if f == flag {
                    return;
                }
            }
        }
        // add the flag
        self.flags[self.flag_count] = Some(flag);
        self.flag_count += 1;
    }
    pub fn remove_flag(&mut self, flag: BlockFlag) {
        for i in 0..self.flag_count {
            if let Some(f) = self.flags[i] {
                if f == flag {
                    self.remove_flag_by_index(i);
                    break;
                }
            }
        }
    }
    pub fn remove_flag_by_index(&mut self, index: usize) {
        // add 1 to convert index to item count
        if index >= self.flag_count {
            return;
        }
        self.flags[index] = None;
        for i in index + 1..self.flag_count {
            self.flags[i - 1] = self.flags[i];
        }
        // clear the last slot; it should have been shifted down
        self.flags[self.flag_count - 1] = None;

        // Decrement count without going negative
        if self.flag_count > 0 {
            self.flag_count -= 1;
        }
    }
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            pos: IVec2::ZERO,
            color: (255, 255, 255),
            block_type: BlockName::default(),
            can_collide: false,
            required_level: 0,
            health: 0.0,
            max_health: 0,
            drop_item: None,
            is_solid: false,
            flags: [None; 6],
        }
    }

    pub fn pos(mut self, pos: IVec2) -> Self {
        self.pos = pos;
        self
    }

    pub fn color(mut self, color: (u8, u8, u8)) -> Self {
        self.color = color;
        self
    }

    pub fn block_type(mut self, block_type: BlockName) -> Self {
        self.block_type = block_type;
        self
    }

    pub fn can_collide(mut self, can_collide: bool) -> Self {
        self.can_collide = can_collide;
        self
    }

    pub fn required_level(mut self, level: u32) -> Self {
        self.required_level = level;
        self
    }

    pub fn health(mut self, health: f32) -> Self {
        self.health = health;
        self
    }

    pub fn max_health(mut self, max_health: i32) -> Self {
        self.max_health = max_health;
        self
    }

    pub fn drop_item(mut self, item: Option<ItemName>) -> Self {
        self.drop_item = item;
        self
    }

    pub fn is_solid(mut self, solid: bool) -> Self {
        self.is_solid = solid;
        self
    }

    pub fn add_flag(mut self, flag: BlockFlag) -> Self {
        for slot in &mut self.flags {
            if slot.is_none() {
                *slot = Some(flag);
                break;
            }
        }
        self
    }

    pub fn build(self) -> Block {
        let flag_count = self.flags.iter().filter(|f| f.is_some()).count();

        Block {
            pos: self.pos,
            color: self.color,
            block_type: self.block_type,
            can_collide: self.can_collide,
            required_level: self.required_level,
            health: if self.health == 0.0 {
                self.max_health as f32
            } else {
                self.health
            },
            max_health: self.max_health,
            drop_item: self.drop_item,
            is_solid: self.is_solid,
            flags: self.flags,
            flag_count,
        }
    }
}
