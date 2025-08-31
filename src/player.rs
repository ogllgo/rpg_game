use crate::camera::Camera;
use crate::inventory::{HasInventory, Inventory};
use crate::item::Item;
use crate::world::World;
use crate::{block::Block, utils::Direction};
use glam::{IVec2, Vec2};
use sdl2::rect::FRect;
use sdl2::{render::Canvas, video::Window};

fn aabb_collision(
    px: f32,
    py: f32,
    pw: f32,
    ph: f32,
    bx: f32,
    by: f32,
) -> bool {
    let bw = 1.0;
    let bh = 1.0;

    // Player bbox edges
    let p_left = px;
    let p_right = px + pw;
    let p_top = py;
    let p_bottom = py + ph;

    // Block bbox edges
    let b_left = bx;
    let b_right = bx + bw;
    let b_top = by;
    let b_bottom = by + bh;

    // Check for overlap on x and y axes
    !(p_right <= b_left
        || p_left >= b_right
        || p_bottom <= b_top
        || p_top >= b_bottom)
}

pub const GRAVITY_FORCE: f32 = 30.0;
#[derive(Debug)]
pub struct Player {
    pub pos: Vec2,
    pub look_dir: Direction,
    pub last_tick_block_hit: u64,
    pub block_hit_delay: u32,
    pub velocity: Vec2,
    pub mining_damage: f32,
    pub mining_spread: u32,
    pub health: f32,
    pub max_health: f32,
    pub active_inventory_slot: usize,
    pub stash: Vec<Item>,
    pub inventory: Inventory<40>,
}

impl HasInventory<40> for Player {
    fn inventory(&self) -> &Inventory<40> {
        &self.inventory
    }

    fn inventory_mut(&mut self) -> &mut Inventory<40> {
        &mut self.inventory
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: Inventory::new(),
            pos: Default::default(),
            look_dir: Default::default(),
            velocity: Default::default(),
            mining_damage: 20.0,
            mining_spread: Default::default(),
            health: Default::default(),
            max_health: Default::default(),
            active_inventory_slot: Default::default(),
            stash: Default::default(),
            last_tick_block_hit: Default::default(),
            block_hit_delay: 20,
        }
    }
}

impl Player {
    pub const SIZE: Vec2 = Vec2 { x: 0.8, y: 0.8 };
    const TERMINAL_VELOCITY: f32 = 53.0;

    #[must_use]
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }
    pub fn apply_gravity(&mut self, fps: f32) {
        self.velocity.y = (self.velocity.y + GRAVITY_FORCE / fps)
            .min(Self::TERMINAL_VELOCITY);
    }

    pub fn move_step(&mut self, blocks: &[Block], fps: f32) {
        let dx = self.velocity.x / fps;
        let dy = self.velocity.y / fps;

        // Determine number of sub-steps to break movement into
        let steps = dx.abs().max(dy.abs()).ceil() as usize;
        let steps = steps.max(1); // Avoid zero steps

        // Per-step delta movement
        let step_dx = dx / steps as f32;
        let step_dy = dy / steps as f32;

        // Collision detection closure
        let collides = |x: f32, y: f32| -> bool {
            blocks.iter().any(|block| {
                aabb_collision(
                    x,
                    y,
                    Self::SIZE.x,
                    Self::SIZE.y,
                    block.pos.x as f32,
                    block.pos.y as f32,
                ) && block.can_collide
            })
        };

        for _ in 0..steps {
            // Try moving along X
            let tentative_x = self.pos.x + step_dx;
            if collides(tentative_x, self.pos.y) {
                // Collision: binary search between current and target X
                let mut lo = 0.0;
                let mut hi = step_dx;
                let mut contact_x = self.pos.x;

                for _ in 0..5 {
                    let mid = lo + (hi - lo) / 2.0;
                    let test_x = self.pos.x + mid;
                    if collides(test_x, self.pos.y) {
                        hi = mid;
                    } else {
                        contact_x = test_x;
                        lo = mid;
                    }
                }

                self.pos.x = contact_x;
                self.velocity.x = 0.0;
            } else {
                self.pos.x = tentative_x;
            }

            // Try moving along Y
            let tentative_y = self.pos.y + step_dy;
            if collides(self.pos.x, tentative_y) {
                // Collision: binary search between current and target Y
                let mut lo = 0.0;
                let mut hi = step_dy;
                let mut contact_y = self.pos.y;

                for _ in 0..5 {
                    let mid = lo + (hi - lo) / 2.0;
                    let test_y = self.pos.y + mid;
                    if collides(self.pos.x, test_y) {
                        hi = mid;
                    } else {
                        contact_y = test_y;
                        lo = mid;
                    }
                }
                self.pos.y = contact_y;
                self.velocity.y = 0.0;
            } else {
                self.pos.y = tentative_y;
            }
        }
    }
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &Camera,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.set_draw_color((244, 194, 157));

        let screen_pos = camera.global_to_screen(self.pos);
        let screen_dims = camera.scale_global_to_screen(Self::SIZE);

        let rect = FRect::new(
            screen_pos.x,
            screen_pos.y,
            screen_dims.x,
            screen_dims.y,
        );
        // if we want not subpixel-perfect rendering, then use
        // let rect = FRect::new(
        //     screen_pos.x.round(),
        //     screen_pos.y.round(),
        //     screen_dims.x.round(),
        //     screen_dims.y.round(),
        // );

        canvas.fill_frect(rect)?;
        Ok(())
    }
    fn is_on_ground(&self, blocks: &[Block]) -> bool {
        let feet_y = self.pos.y + Self::SIZE.y;

        blocks.iter().any(|block| {
            block.can_collide &&
            // block's top edge is close to player's feet
            (block.pos.y as f32 - feet_y).abs() < 0.05 &&
            // player horizontally overlaps block
            !(self.pos.x + Self::SIZE.x <= block.pos.x as f32 || self.pos.x >= (block.pos.x as f32 + 1.0))
        })
    }
    pub fn try_jump(&mut self, blocks: &[Block]) {
        if self.is_on_ground(blocks) {
            self.velocity.y = -20.0; // @TODO: magic number
        }
    }

    pub fn try_move(&mut self, direction: Direction, fps: f32) {
        let acceleration = 60.0 / fps; // units per second per second
        let max_speed = 20.0; // max horizontal speed

        match direction {
            Direction::Left => {
                self.velocity.x -= acceleration;
                if self.velocity.x < -max_speed {
                    self.velocity.x = -max_speed;
                }
            }
            Direction::Right => {
                self.velocity.x += acceleration;
                if self.velocity.x > max_speed {
                    self.velocity.x = max_speed;
                }
            }
            _ => {}
        }
    }

    pub fn apply_friction(&mut self, fps: f32) {
        let friction = 15.0 / fps; // units per second², tweak for slow down speed

        if self.velocity.x > 0.0 {
            self.velocity.x -= friction;
            if self.velocity.x < 0.0 {
                self.velocity.x = 0.0;
            }
        } else if self.velocity.x < 0.0 {
            self.velocity.x += friction;
            if self.velocity.x > 0.0 {
                self.velocity.x = 0.0;
            }
        }
    }

    pub fn wrap_board(&mut self, board_x: u32) {
        let width = board_x as f32;
        if self.pos.x < 0.0 {
            self.pos.x += width;
        } else if self.pos.x > width {
            self.pos.x -= width;
        }
    }
    pub fn look_at(&mut self, target_x: f32, target_y: f32) {
        let px = self.pos.x + Self::SIZE.x / 2.0;
        let py = self.pos.y + Self::SIZE.y / 2.0;

        let dx = target_x - px;
        let dy = target_y - py;

        self.look_dir = if dx.abs() > dy.abs() {
            if dx > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if dy > 0.0 {
            Direction::Down
        } else {
            Direction::Up
        };
    }

    #[must_use]
    pub fn calculate_mining_speed(&self) -> f32 {
        let mul = 1.0;
        self.mining_damage * mul
    }

    pub fn hit_block(&mut self, pos: IVec2, map: &mut World, tick: u64) {
        if self.last_tick_block_hit + self.block_hit_delay as u64 <= tick {
            self.last_tick_block_hit = tick;

            map.hit_block(pos, self);
        }
    }
}
