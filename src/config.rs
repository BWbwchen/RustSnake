use ggez::graphics;
pub const WIDTH: usize = 40;
pub const HEIGHT: usize = 30;
pub const BLOCK_SIZE: usize = 20;
pub const SCREEN_OFFSET: usize = 0;
pub const WHITE: graphics::Color = graphics::WHITE;
pub const BLACK: graphics::Color = graphics::BLACK;
pub const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
pub const MODE: u32 = 10;

pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}
