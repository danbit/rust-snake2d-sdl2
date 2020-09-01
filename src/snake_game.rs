use std::rc::Rc;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const PIXEL_SIZE: u32 = 16;
pub const GAME_WIDTH: u32 = 39;
pub const GAME_HEIGHT: u32 = 29;

#[derive(Copy, Clone)]
pub enum GameState {
    Paused,
    Playing,
    GameOver,
    GameWin,
}

pub struct Snake {
    direction: Point,
    width: u32,
    height: u32,
    positions: Vec<Point>,
}

pub struct SnakeGame {
    canvas: Rc<Canvas<Window>>,
    state: GameState,
}

impl SnakeGame {
    pub fn new(canvas: Canvas<Window>) -> SnakeGame {
        SnakeGame {
            canvas: Rc::new(canvas),
            state: GameState::Paused,
        }
    }

    pub fn update() {
        println!("SnakeGame.update");
    }
}
