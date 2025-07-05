use sdl2::{render::{Canvas}, video::Window};
use sdl2::rect::FRect;

use crate::{utils::Direction, Block};
pub const GRAVITY_FORCE: f32 = 30.0;
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub look_dir: Direction,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub mining_speed: i32,
    
}

fn aabb_collision(
    px: f32, py: f32, pw: f32, ph: f32,
    bx: i32, by: i32
) -> bool {
    let bw = 1.0;
    let bh = 1.0;

    // Player bbox edges
    let p_left = px;
    let p_right = px + pw;
    let p_top = py;
    let p_bottom = py + ph;

    // Block bbox edges
    let b_left = bx as f32;
    let b_right = bx as f32 + bw;
    let b_top = by as f32;
    let b_bottom = by as f32 + bh;

    // Check for overlap on x and y axes
    !(p_right <= b_left || p_left >= b_right || p_bottom <= b_top || p_top >= b_bottom)
}

impl Player {
    pub const WIDTH: f32 = 0.8;
    pub const HEIGHT: f32 = 0.8;
    const TERMINAL_VELOCITY: f32 = 53.0;

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
            look_dir: Direction::None,
            velocity_x: 0.0,
            velocity_y: 0.0,
            mining_speed: 6000,
        }
    }
    pub fn apply_gravity(&mut self, dt: f32) {
        self.velocity_y = (self.velocity_y + GRAVITY_FORCE * dt).min(Self::TERMINAL_VELOCITY);
    }

    pub fn move_step(&mut self, blocks: &Vec<Block>, dt: f32) {
        // Total movement vector based on velocity and delta time
        let dx = self.velocity_x * dt;
        let dy = self.velocity_y * dt;

        // Determine number of sub-steps to break movement into
        let steps = dx.abs().max(dy.abs()).ceil() as usize;
        let steps = steps.max(1); // Avoid zero steps

        // Per-step delta movement
        let step_dx = dx / steps as f32;
        let step_dy = dy / steps as f32;

        // Collision detection closure
        let collides = |x: f32, y: f32| -> bool {
            blocks.iter().any(|block| {
                aabb_collision(x, y, Self::WIDTH, Self::HEIGHT, block.x as i32, block.y as i32)
                    && block.can_collide
            })
        };

        for _ in 0..steps {
            // Try moving along X
            let tentative_x = self.x + step_dx;
            if collides(tentative_x, self.y) {
                // Collision: binary search between current and target X
                let mut lo = 0.0;
                let mut hi = step_dx;
                let mut contact_x = self.x;

                for _ in 0..5 {
                    let mid = lo + (hi - lo) / 2.0;
                    let test_x = self.x + mid;
                    if collides(test_x, self.y) {
                        hi = mid;
                    } else {
                        contact_x = test_x;
                        lo = mid;
                    }
                }

                self.x = contact_x;
                self.velocity_x = 0.0;
            } else {
                self.x = tentative_x;
            }

            // Try moving along Y
            let tentative_y = self.y + step_dy;
            if collides(self.x, tentative_y) {
                // Collision: binary search between current and target Y
                let mut lo = 0.0;
                let mut hi = step_dy;
                let mut contact_y = self.y;

                for _ in 0..5 {
                    let mid = lo + (hi - lo) / 2.0;
                    let test_y = self.y + mid;
                    if collides(self.x, test_y) {
                        hi = mid;
                    } else {
                        contact_y = test_y;
                        lo = mid;
                    }
                }

                self.y = contact_y;
                self.velocity_y = 0.0;
            } else {
                self.y = tentative_y;
            }
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>, camera: &FRect, scale: f32) {
        canvas.set_draw_color((244, 194, 157));

        let screen_x = (self.x - camera.x) * scale;
        let screen_y = (self.y - camera.y) * scale;

        canvas.fill_frect(FRect::new(screen_x,
            screen_y,
            Player::WIDTH * scale,
            Player::HEIGHT * scale)).unwrap();
        
        // Calculate the center of the player
        let player_center_x = self.x + Self::WIDTH / 2.0;
        let player_center_y = self.y + Self::HEIGHT / 2.0;

        // Directional offset to get the block in front
        let (dx, dy) = match self.look_dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::None => (0, 0),
        };
        // Compute block coordinates in world space
        let front_block_x = (player_center_x + dx as f32).floor();
        let front_block_y = (player_center_y + dy as f32).floor();

        // Convert to screen space
        let dot_screen_x = (front_block_x - camera.x) * scale + scale / 2.0;
        let dot_screen_y = (front_block_y - camera.y) * scale + scale / 2.0;

        // Draw red dot (small rectangle, e.g., 6x6 pixels)
        canvas.set_draw_color((255, 0, 0));
        let dot_size = 6.0;
        canvas.fill_frect(FRect::new(
            dot_screen_x - dot_size / 2.0,
            dot_screen_y - dot_size / 2.0,
            dot_size,
            dot_size,
        )).unwrap();
    }
    fn is_on_ground(&self, blocks: &Vec<Block>) -> bool {
        let feet_y = self.y + Self::HEIGHT;

        blocks.iter().any(|block| {
            block.can_collide &&
            // block's top edge is close to player's feet
            (block.y as f32 - feet_y).abs() < 0.05 &&
            // player horizontally overlaps block
            !(self.x + Self::WIDTH <= block.x as f32 || self.x >= (block.x as f32 + 1.0))
        })
    }
    pub fn try_jump(&mut self, blocks: &Vec<Block>) {
        if self.is_on_ground(blocks) {
            println!("on ground");
            self.velocity_y = -20.0; // jump impulse, tune this value to your liking
        }
    }

    pub fn try_move(&mut self, direction: Direction, dt: f32) {
        let acceleration = 60.0; // units per second per second
        let max_speed = 20.0; // max horizontal speed

        match direction {
            Direction::Left => {
                self.velocity_x -= acceleration * dt;
                if self.velocity_x < -max_speed {
                    self.velocity_x = -max_speed;
                }
            },
            Direction::Right => {
                self.velocity_x += acceleration * dt;
                if self.velocity_x > max_speed {
                    self.velocity_x = max_speed;
                }
            },
            _ => println!("When trying to move, direction not horizontal"),
        }
    }

    pub fn apply_friction(&mut self, dt: f32) {
        let friction = 15.0; // units per second², tweak for slow down speed

        if self.velocity_x > 0.0 {
            self.velocity_x -= friction * dt;
            if self.velocity_x < 0.0 {
                self.velocity_x = 0.0;
            }
        } else if self.velocity_x < 0.0 {
            self.velocity_x += friction * dt;
            if self.velocity_x > 0.0 {
                self.velocity_x = 0.0;
            }
        }
    }

    pub fn wrap_board(&mut self, board_x: u32) {
        let width = board_x as f32;
        if self.x < 0.0 {
            self.x += width;
        } else if self.x > width {
            self.x -= width;
        }
    }
        pub fn look_at(&mut self, target_x: f32, target_y: f32) {
        let px = self.x + Self::WIDTH / 2.0;
        let py = self.y + Self::HEIGHT / 2.0;

        let dx = target_x - px;
        let dy = target_y - py;

        self.look_dir = if dx.abs() > dy.abs() {
            if dx > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if dy > 0.0 {
                Direction::Down
            } else {
                Direction::Up
            }
        };
    }
}
