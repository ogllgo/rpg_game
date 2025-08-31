use crate::render::{Rect, draw_rect_with_hole};
use crate::{
    block::Block, camera::Camera, input::Input, player::Player,
    utils::Direction, world::World,
};
use glam::{IVec2, Vec2};
use hecs::World as HecsWorld;
use sdl2::{EventPump, Sdl, VideoSubsystem, render::Canvas, video::Window};
use sdl2::{keyboard::Keycode, mouse::MouseButton};
const FPS: f32 = 60.0;
pub struct Game {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub window: Window,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,

    pub map: World,
    pub ecs: HecsWorld,
    pub input: Input,
    pub camera: Camera,
    pub player: Player,
    pub tick_count: u64,
}

impl Game {
    #[must_use]
    pub fn new(seed: u32, window_dims: Vec2, camera_dims: Vec2) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "silly little game",
                window_dims.x as u32,
                window_dims.y as u32,
            )
            .build()
            .unwrap();
        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let window = canvas.window();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            map: World::new(seed),
            camera: Camera::new(
                Vec2::new(0.0, 40.0),
                camera_dims,
                window_dims,
                0.0,
            ),
            sdl_context,
            video_subsystem,
            window: window.clone(),
            canvas,
            event_pump,
            ecs: HecsWorld::new(),
            input: Input::new(),
            player: Player::default(),
            tick_count: 0,
        }
    }

    pub fn manage_input(&mut self) -> bool {
        self.input.clear_transient();
        for event in self.event_pump.poll_iter() {
            if !self.input.update(&event, &self.camera) {
                return false;
            }
        }
        true
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.tick_count += 1;
        self.map.update_active_chunks(
            self.camera.pos.x,
            self.camera.pos.y,
            self.camera.viewport_dims.x as i32,
            self.camera.viewport_dims.y as i32,
        );
        let blocks: Vec<Block> = self
            .map
            .get_active_chunks()
            .iter()
            .flat_map(|c| c.flatten())
            .collect();
        if self.input.keyboard.held.contains(&Keycode::SPACE) {
            self.player.try_jump(&blocks);
        }
        if self.input.keyboard.held.contains(&Keycode::Left)
            || self.input.keyboard.held.contains(&Keycode::A)
        {
            self.player.try_move(Direction::Left, FPS);
        } else if self.input.keyboard.held.contains(&Keycode::Right)
            || self.input.keyboard.held.contains(&Keycode::D)
        {
            self.player.try_move(Direction::Right, FPS);
        } else {
            self.player.apply_friction(FPS);
        }

        if self.input.mouse.held.contains(&MouseButton::Left) {
            let pos = self.input.mouse.get_global_pos(&self.camera);

            if pos.distance_squared(self.player.pos) <= 5.0 * 5.0 {
                self.player.hit_block(
                    IVec2::new(pos.x.floor() as i32, pos.y.floor() as i32),
                    &mut self.map,
                    self.tick_count,
                );
                // self.map.hit_block(
                //     IVec2::new(pos.x.floor() as i32, pos.y.floor() as i32),
                //     &mut self.player,
                // );
            }
        }

        self.player.apply_gravity(FPS);
        self.player.move_step(&blocks, FPS);

        self.map.generate_around_point(
            self.camera.pos.x,
            self.camera.pos.y,
            self.camera.viewport_dims.x,
            self.camera.viewport_dims.y,
        );

        self.camera.center_around(self.player.pos);
        Ok(())
    }

    /// Runs once after initialisation
    pub fn on_start(&mut self) {}

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.clear();

        let blocks: Vec<Block> = self
            .map
            .get_active_chunks()
            .iter()
            .flat_map(|c| c.flatten())
            .collect();

        for block in blocks {
            // blocks naturally heal, but slowly
            self.map.heal_block(block.pos, 0.1);
            block.render(&mut self.canvas, &self.camera)?;
        }
        self.player.render(&mut self.canvas, &self.camera)?;

        const INVENTORY_ITEM_SIZE: usize = 30;
        const INVENTORY_ITEM_MARGIN: usize = 2;
        const INVENTORY_ITEM_PADDING: usize = 4;
        const INVENTORY_MARGIN: usize = 10;

        for (index, item) in
            self.player.inventory.get_items().iter().enumerate()
        {
            // 10xN grid
            let x = index % 10;
            let y = index / 10;

            let x = x * (INVENTORY_ITEM_SIZE + INVENTORY_ITEM_MARGIN)
                + INVENTORY_MARGIN;
            let y = y * (INVENTORY_ITEM_SIZE + INVENTORY_ITEM_MARGIN)
                + INVENTORY_MARGIN;

            self.canvas.set_draw_color((255, 255, 255));
            Rect::new(
                x as f32,
                y as f32,
                INVENTORY_ITEM_SIZE as f32,
                INVENTORY_ITEM_SIZE as f32,
            )
            .draw(&mut self.canvas)?;

            if let Some(item) = item {
                let fill_x = x + INVENTORY_ITEM_MARGIN;
                let fill_y = y + INVENTORY_ITEM_MARGIN;
                let fill_size =
                    INVENTORY_ITEM_SIZE - 2 * (INVENTORY_ITEM_MARGIN);
                self.canvas.set_draw_color(item.color);
                draw_rect_with_hole(
                    &mut self.canvas,
                    Rect::new(
                        fill_x as f32,
                        fill_y as f32,
                        fill_size as f32,
                        fill_size as f32,
                    ),
                    Rect::new(
                        fill_x as f32 + INVENTORY_ITEM_PADDING as f32,
                        fill_y as f32 + INVENTORY_ITEM_PADDING as f32,
                        fill_size as f32 - 2.0 * INVENTORY_ITEM_PADDING as f32,
                        fill_size as f32 - 2.0 * INVENTORY_ITEM_PADDING as f32,
                    ),
                )?;
            }
        }
        self.canvas.present();
        Ok(())
    }

    pub fn run(&mut self) {
        self.on_start();
        while self.manage_input() {
            match self.tick() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error when ticking: {}", e);
                }
            }
            match self.render() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error when rendering: {}", e);
                }
            }
        }
    }
}
