use crate::{
    blocks::BLOCK_COLOR_AIR, camera::Camera, item::ItemName, render::Rect,
};
use derive_builder::Builder;
use glam::{IVec2, Vec2};
use sdl2::{render::Canvas, video::Window};

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
    flags: [Option<BlockFlag>; 6],
    flag_count: usize,
    pub last_hit_tick: u64,
}

fn color_interp(
    col1: (u8, u8, u8),
    col2: (u8, u8, u8),
    perc: f32,
) -> (u8, u8, u8) {
    let p = perc.clamp(0.0, 1.0);

    let r = col1.0 as f32 + (col2.0 as f32 - col1.0 as f32) * p;
    let g = col1.1 as f32 + (col2.1 as f32 - col1.1 as f32) * p;
    let b = col1.2 as f32 + (col2.2 as f32 - col1.2 as f32) * p;

    (r.round() as u8, g.round() as u8, b.round() as u8)
}

impl Block {
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &Camera,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let screen_pos = camera.global_to_screen(self.pos.as_vec2());
        let screen_dims = camera.scale_global_to_screen(Vec2::ONE);

        let health_percent = 1.0
            - if self.max_health == 0.0 {
                1.0
            } else {
                self.health / self.max_health
            };

        canvas.set_draw_color(color_interp(
            self.color,
            BLOCK_COLOR_AIR,
            health_percent,
        ));
        Rect::new(screen_pos.x, screen_pos.y, screen_dims.x, screen_dims.y)
            .draw(canvas)?;
        Ok(())
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
