use crate::item::ItemName;
use sdl2::{rect::FRect, render::Canvas, video::Window};

pub const BLOCK_SIZE: i32 = 10;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum BlockName {
    Air,
    Dirt,
    Void,
    Stone,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum BlockFlag {
    Mine,
    Dig,
    Chop,
    Highlight,
}

#[derive(Clone, Debug, Copy)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: (u8, u8, u8),
    pub block_type: BlockName,
    pub can_collide: bool,
    flags: [Option<BlockFlag>; 6],
    flag_count: usize,
    pub required_level: u32,
    pub health: f32,
    pub max_health: i32,
    pub drop_item: Option<ItemName>,
}

impl Block {
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &FRect,
        scale: f32,
    ) {
        let screen_x = (self.x as f32 - camera.x) * scale;
        let screen_y = (self.y as f32 - camera.y) * scale;

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
            let highlight_screen_x = (self.x as f32 - camera.x + 0.2) * scale;
            let highlight_screen_y = (self.y as f32 - camera.y + 0.2) * scale;

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
    pub fn new(
        x: i32,
        y: i32,
        color: (u8, u8, u8),
        block_type: BlockName,
        can_collide: bool,
        flags: [Option<BlockFlag>; 6],
        resist: u32,
        max_health: i32,
        drop_item: Option<ItemName>,
    ) -> Self {
        let mut flags_count: usize = 1;
        for i in 0..6 {
            if let Some(_) = flags[i] {
                flags_count += 1;
            }
        }
        Self {
            x,
            y,
            color,
            block_type,
            can_collide,
            flags,
            flag_count: flags_count,
            required_level: resist,
            max_health,
            health: max_health as f32,
            drop_item,
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
