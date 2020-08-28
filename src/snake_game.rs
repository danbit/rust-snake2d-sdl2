mod snake_game {
    pub const PIXEL_SIZE: u32 = 16;
    pub const GAME_WIDTH: u32 = 39;
    pub const GAME_HEIGHT: u32 = 29;

    #[derive(Copy, Clone)]
    pub enum GameState {        
        Paused,
        Playing,
        GameOver,
        GameWin
    }
    
    pub struct SnakeGame {
        state: State,
    }

    impl SnakeGame {
        pub fn new() -> SnakeGame {

        }

        //  fn draw_border()
    }
}