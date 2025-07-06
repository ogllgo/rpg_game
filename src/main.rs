use rpg_game::Block;
use rpg_game::utils::Direction;
use rpg_game::{Player, world::World};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{FRect, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::collections::HashSet;
use std::time::{Duration, Instant};

fn render_text_to_canvas<T>(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<T>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(Color::RGBA(0, 0, 0, 255))
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let query = texture.query();
    let target = Rect::new(x, y, query.width, query.height);
    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
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

    let window_width = 800.0;
    let window_height = 600.0;

    let window = video_subsystem
        .window("window", window_width as u32, window_height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::new(6589);

    // Scale based on camera and window size
    let scale_x: f32 = window_width / camera.w;
    let scale_y: f32 = window_height / camera.h;
    let scale = scale_x.min(scale_y);

    let mut pressed_keys = HashSet::new();
    let fps: f32 = 60.0;
    let frame_duration = Duration::from_secs_f32(1.0 / fps);
    let mut last_frame_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_frame_time).as_secs_f32();
        last_frame_time = now;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    pressed_keys.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    pressed_keys.remove(&key);
                }
                _ => {}
            }
        }

        let (mouse_x, mouse_y) =
            (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        let mouse_world_x = camera.x + (mouse_x as f32) / scale;
        let mouse_world_y = camera.y + (mouse_y as f32) / scale;
        player.look_at(mouse_world_x, mouse_world_y);

        if event_pump.mouse_state().left() {
            let center_x = player.x + Player::WIDTH / 2.0;
            let center_y = player.y + Player::HEIGHT / 2.0;

            let (target_x, target_y) = match player.look_dir {
                Direction::Right => (center_x + 1.0, center_y),
                Direction::Left => (center_x - 1.0, center_y),
                Direction::Up => (center_x, center_y - 1.0),
                Direction::Down => (center_x, center_y + 1.0),
                Direction::None => (center_x, center_y),
            };

            world.hit_block(
                target_x.floor() as i32,
                target_y.floor() as i32,
                player.mining_speed as f32 * dt,
                1,
            );
        }
        let blocks: Vec<Block> = world
            .get_chunks_around_point(
                player.x,
                player.y,
                camera.w as i32,
                camera.h as i32,
            )
            .iter()
            .flat_map(|c| c.flatten())
            .collect();
        if pressed_keys.contains(&Keycode::Space) {
            player.try_jump(&blocks);
        }
        if pressed_keys.contains(&Keycode::Left)
            || pressed_keys.contains(&Keycode::A)
        {
            player.try_move(Direction::Left, dt);
        } else if pressed_keys.contains(&Keycode::Right)
            || pressed_keys.contains(&Keycode::D)
        {
            player.try_move(Direction::Right, dt);
        } else {
            player.apply_friction(dt);
        }

        player.apply_gravity(dt);
        player.move_step(&blocks, dt);

        world.update_around_point(
            player.x,
            player.y,
            camera_width * 2.0,
            camera_height * 2.0,
        );

        // Update camera centered on player
        camera.x = player.x + Player::WIDTH / 2.0 - camera.w / 2.0;
        camera.y = player.y + Player::HEIGHT / 2.0 - camera.h / 2.0;

        // No fixed board bounds to clamp to; if you want clamp, do it dynamically here

        canvas.set_draw_color((0, 0, 0));
        canvas.clear();

        for block in blocks {
            block.render(&mut canvas, &camera, scale);
        }

        player.render(&mut canvas, &camera, scale);
        let font = ttf_context
            .load_font("/usr/share/fonts/TTF/Arial.TTF", 24)
            .unwrap();
        render_text_to_canvas(
            &mut canvas,
            &texture_creator,
            &font,
            "I love SDL!",
            0,
            0,
        )
        .unwrap();
        canvas.present();

        let elapsed = Instant::now() - now;
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
