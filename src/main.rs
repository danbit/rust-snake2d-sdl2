extern crate sdl2;
mod snake_game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub const FPS: u32 = 30;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Snake 2D", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut game = snake_game::SnakeGame::new(canvas);

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame : u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if frame >= FPS {
            //game.update();
            frame = 0;
        }

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
    
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        canvas.fill_rect(Rect::new(10, 10, 16, 16))?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
