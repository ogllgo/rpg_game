use rpg_game::Block;
use rpg_game::Player;
use rpg_game::utils::Direction;
use rpg_game::world::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::FRect;
use std::time::{Duration, Instant};
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 800.0;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("window", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::new(0.0, 0.0);

    // Initial camera size (tiles shown)
    let camera_width = 40.0;
    let camera_height = 30.0;
    let mut camera = FRect::new(
        player.x + Player::WIDTH / 2.0 - camera_width / 2.0,
        player.y + Player::HEIGHT / 2.0 - camera_height / 2.0,
        camera_width,
        camera_height,
    );

    let mut game = Game::new(6589);

    // Scale based on camera and window size
    let scale_x: f32 = WINDOW_WIDTH / camera.w;
    let scale_y: f32 = WINDOW_HEIGHT / camera.h;
    let scale = scale_x.min(scale_y);

    let fps: f32 = 60.0;
    let frame_duration = Duration::from_secs_f32(1.0 / fps);
    let mut last_frame_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_frame_time).as_secs_f32();
        last_frame_time = now;
        game.input.clear_transient();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
            game.manage_input(&event);
        }

        let (mouse_x, mouse_y) =
            (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        let mouse_world_x = camera.x + (mouse_x as f32) / scale;
        let mouse_world_y = camera.y + (mouse_y as f32) / scale;
        player.look_at(mouse_world_x, mouse_world_y);

        if event_pump.mouse_state().left() {
            let center_x = player.x + Player::WIDTH / 2.0;
            let center_y = player.y + Player::HEIGHT / 2.0;

            let (target_x, target_y) = match &player.look_dir {
                Direction::Right => (center_x + 1.0, center_y),
                Direction::Left => (center_x - 1.0, center_y),
                Direction::Up => (center_x, center_y - 1.0),
                Direction::Down => (center_x, center_y + 1.0),
                Direction::None => (center_x, center_y),
            };

            game.map.hit_block(
                target_x.floor() as i32,
                target_y.floor() as i32,
                &mut player,
            );
        }
        let blocks: Vec<Block> = game
            .map
            .get_chunks_around_point(
                player.x,
                player.y,
                camera.w as i32,
                camera.h as i32,
            )
            .iter()
            .flat_map(|c| c.flatten())
            .collect();
        if game.input.keyboard.pressed.contains(&Keycode::Space) {
            player.try_jump(&blocks);
        }
        if game.input.keyboard.held.contains(&Keycode::Left)
            || game.input.keyboard.held.contains(&Keycode::A)
        {
            player.try_move(Direction::Left, dt);
        } else if game.input.keyboard.held.contains(&Keycode::Right)
            || game.input.keyboard.held.contains(&Keycode::D)
        {
            player.try_move(Direction::Right, dt);
        } else {
            player.apply_friction(dt);
        }

        player.apply_gravity(dt);
        player.move_step(&blocks, dt);

        game.map.update_around_point(
            player.x,
            player.y,
            camera_width * 2.0,
            camera_height * 2.0,
        );

        // Update camera centered on player
        camera.x = player.x + Player::WIDTH / 2.0 - camera.w / 2.0;
        camera.y = player.y + Player::HEIGHT / 2.0 - camera.h / 2.0;

        canvas.set_draw_color((0, 0, 0));
        canvas.clear();

        for block in blocks {
            block.render(&mut canvas, &camera, scale);
        }

        player.render(&mut canvas, &camera, scale);
        canvas.present();

        let elapsed = now.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
