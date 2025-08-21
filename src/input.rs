use crate::camera::Camera;
use glam::Vec2;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct KeyboardInput {
    pub held: HashSet<Keycode>,
    pub released: HashSet<Keycode>,
    pub pressed: HashSet<Keycode>,
}

#[derive(Clone, Debug)]
pub struct MouseInput {
    pub held: HashSet<MouseButton>,
    pub released: HashSet<MouseButton>,
    pub pressed: HashSet<MouseButton>,
    pub pos: Vec2,
    world_pos: Vec2,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub keyboard: KeyboardInput,
    pub mouse: MouseInput,
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardInput {
    #[must_use]
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            released: HashSet::new(),
            pressed: HashSet::new(),
        }
    }
}

impl Default for MouseInput {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseInput {
    #[must_use]
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            released: HashSet::new(),
            pressed: HashSet::new(),
            pos: Vec2::ZERO,
            world_pos: Vec2::ZERO,
        }
    }

    pub fn get_global_pos(&self) -> Vec2 {
        self.world_pos
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
impl Input {
    #[must_use]
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardInput::new(),
            mouse: MouseInput::new(),
        }
    }
    /// Clear states that should not persist over multiple frames.
    /// This function should only be called once per tick.
    pub fn clear_transient(&mut self) {
        self.keyboard.pressed.clear();
        self.keyboard.released.clear();

        self.mouse.pressed.clear();
        self.mouse.released.clear();
    }

    /// Update input based on a single SDL2 event
    pub fn update(&mut self, event: &Event, camera: &Camera) -> bool {
        match event {
            Event::Quit { .. } => return false,
            // Keyboard
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                self.keyboard.pressed.insert(*key);
                self.keyboard.held.insert(*key);
            }
            Event::KeyUp {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                self.keyboard.released.insert(*key);
                self.keyboard.held.remove(key);
            }

            // Mouse button
            Event::MouseButtonDown { mouse_btn, .. } => {
                self.mouse.pressed.insert(*mouse_btn);
                self.mouse.held.insert(*mouse_btn);
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                self.mouse.released.insert(*mouse_btn);
                self.mouse.held.remove(mouse_btn);
            }

            // Mouse movement
            Event::MouseMotion { x, y, .. } => {
                self.mouse.pos = Vec2::new(*x as f32, *y as f32);
            }

            // idc :shrug:
            _ => {}
        }
        self.mouse.world_pos = camera.screen_to_global(self.mouse.pos);
        return true;
    }
}
