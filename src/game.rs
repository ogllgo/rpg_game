use std::time::{Duration, Instant};

use crate::Block;
use crate::input::Input;
use crate::utils::Direction;
use crate::{Player, camera::Camera, world::World};
use glam::Vec2;
use hecs::World as HecsWorld;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl, VideoSubsystem, render::Canvas, video::Window};
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
}

impl Game {
    #[must_use]
    pub fn new(seed: u32, window_dims: Vec2, camera_dims: Vec2) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let (window, canvas) = {
            let win = video_subsystem
                .window("window", window_dims.x as u32, window_dims.y as u32)
                .build()
                .unwrap();
            (win.clone(), win.into_canvas().build().unwrap())
        };

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            map: World::new(seed),
            camera: Camera::new(Vec2::ZERO, camera_dims, window_dims, 0.0),
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            window: window,
            canvas: canvas,
            event_pump: event_pump,
            ecs: HecsWorld::new(),
            input: Input::new(),
            player: Player::default(),
        }
    }

    pub fn manage_input(&mut self) -> bool {
        self.input.clear_transient();
        for event in self.event_pump.poll_iter() {
            if !self.input.update(&event, &self.camera) {
                return false;
            }
        }
        return true;
    }

    pub fn tick(&mut self) {
        self.map.update_active_chunks(
            self.player.pos.x,
            self.player.pos.y,
            self.camera.viewport_dims.x as i32,
            self.camera.viewport_dims.y as i32,
        );
        let blocks: Vec<Block> = self
            .map
            .get_active_chunks()
            .iter()
            .flat_map(|c| c.flatten())
            .collect();
        if self.input.keyboard.pressed.contains(&Keycode::SPACE) {
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

        self.player.apply_gravity(FPS);
        self.player.move_step(&blocks, FPS);

        self.map.update_around_point(
            self.player.pos.x,
            self.player.pos.y,
            self.camera.viewport_dims.x,
            self.camera.viewport_dims.y,
        );

        self.camera.center_around(self.player.pos);
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.clear();

        let blocks: Vec<Block> = self
            .map
            .get_active_chunks()
            .iter()
            .flat_map(|c| c.flatten())
            .collect();

        for block in blocks {
            block.render(
                &mut self.canvas,
                &self.camera,
                self.camera.get_scale().x,
            );
        }
        self.player.render(
            &mut self.canvas,
            &self.camera,
            self.camera.get_scale().x,
        );
        self.canvas.present();
    }

    pub fn run(&mut self) {
        let frame_duration = Duration::from_secs_f32(1.0 / FPS);

        while self.manage_input() {
            let now = Instant::now();

            self.tick();
            self.render();

            let elapsed = now.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }
}
