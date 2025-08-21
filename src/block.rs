use crate::{camera::Camera, item::ItemName};
use derive_builder::Builder;
use glam::IVec2;
use sdl2::{rect::FRect, render::Canvas, video::Window};

pub const BLOCK_SIZE_PIXELS: i32 = 10;
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

#[derive(Clone, Debug, Copy, Builder)]
pub struct Block {
    pub pos: IVec2,
    pub color: (u8, u8, u8),
    pub block_type: BlockName,
    pub can_collide: bool,
    pub required_level: u32,
    pub health: f32,
    pub max_health: f32,
    pub drop_item: Option<ItemName>,
    pub is_solid: bool,
    #[builder(private)]
    flags: [Option<BlockFlag>; 6],
    #[builder(private)]
    flag_count: usize,
}

impl Block {
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &Camera,
        scale: f32,
    ) {
        let screen_x = (self.pos.x as f32 - camera.pos.x) * scale;
        let screen_y = (self.pos.y as f32 - camera.pos.y) * scale;
        // println!("screen pos: ({}, {})", screen_x, screen_y);
        canvas.set_draw_color(self.color);
        canvas
            .fill_frect(FRect::new(screen_x, screen_y, scale, scale))
            .unwrap();
    }
    #[must_use]
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
    pub fn add_flag(&mut self, flag: BlockFlag) -> &mut Self {
        if self.flag_count.unwrap_or(0) >= 6 {
            // already full, do nothing
            return self;
        }

        // initialize flags if not already
        if self.flags.is_none() {
            self.flags = Some([None; 6]);
        }

        let mut flags = self.flags.take().unwrap();
        let count = self.flag_count.unwrap_or(0);
        flags[count] = Some(flag);
        self.flag_count = Some(count + 1);
        self.flags = Some(flags);

        self
    }
}
