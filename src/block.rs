use sdl2::{rect::FRect, render::Canvas, video::Window};

pub const BLOCK_SIZE: i32 = 10;
#[derive(Clone, Debug, PartialEq)]
pub enum BlockName {
    Air,
    Dirt,
    Void,
    Stone
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    Rock,
    Dirt,
    Wood
}

#[derive(Clone, Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: (u8, u8, u8),
    pub block_type: BlockName,
    pub can_collide: bool,
    pub types: Vec<BlockType>,
    pub required_level: i32,
    pub health: f32,
    pub max_health: i32
}

impl Block {
    pub fn render(&self, canvas: &mut Canvas<Window>, camera: &FRect, scale: f32) {
        let screen_x = (self.x as f32 - camera.x) * scale;
        let screen_y = (self.y as f32 - camera.y) * scale;

        canvas.set_draw_color(self.color);
        canvas.fill_frect(FRect::new(
            screen_x,
            screen_y,
            scale,
            scale,
        )).unwrap();
    }
    pub fn new(x: i32, y: i32, color: (u8, u8, u8), block_type: BlockName, can_collide: bool, types: Vec<BlockType>, resist: i32, max_health: i32) -> Self {
        Self {
            x,
            y,
            color,
            block_type,
            can_collide,
            types,
            required_level: resist,
            max_health,
            health: max_health as f32,
        }
    }
    pub fn can_be_hit(&self) -> bool {
        self.types.iter().any(|b| vec![BlockType::Rock, BlockType::Wood, BlockType::Dirt].contains(b))
    }
}