use glam::Vec2;
use rpg_game::game::Game;

const WINDOW_DIMS: Vec2 = Vec2 { x: 800.0, y: 600.0 };

pub fn main() {
    Game::new(6589, WINDOW_DIMS, WINDOW_DIMS / 20.0).run();
}
