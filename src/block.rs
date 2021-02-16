use crate::config::{BLOCK_SIZE, HEIGHT, SCREEN_OFFSET, WIDTH};
use ggez::*;
use mint::Point2;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Block {
    x: usize,
    y: usize,
}

impl Block {
    pub fn new(_x: usize, _y: usize) -> Block {
        Block { x: _x, y: _y }
    }
    pub fn new_by_block(_template: Block) -> Block {
        Block {
            x: _template.x,
            y: _template.y,
        }
    }
    pub fn draw(
        &mut self,
        mb: &mut graphics::MeshBuilder,
        _color: graphics::Color,
        stroked: bool,
    ) -> GameResult {
        let mut bb = graphics::Rect::new(0.0, 0.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32);
        bb.move_to(Point2 {
            x: (self.x * BLOCK_SIZE + SCREEN_OFFSET) as f32,
            y: (self.y * BLOCK_SIZE) as f32,
        });
        mb.rectangle(
            graphics::DrawMode::fill(),
            bb,
            //graphics::Color::new(0.0, 0.0, 0.0, 0.8),
            _color,
        );
        if stroked {
            mb.rectangle(
                graphics::DrawMode::stroke(2.0),
                bb,
                graphics::Color::new(0.0, 0.0, 0.0, 0.2),
            );
        }
        Ok(())
    }
    pub fn set_axis(&mut self, _x: usize, _y: usize) {
        self.x = _x;
        self.y = _y;
    }
    pub fn up(&mut self) -> bool {
        if self.y == 0 {
            return false;
        }
        self.y -= 1;
        true
    }
    pub fn down(&mut self) -> bool {
        if self.y == HEIGHT - 1 {
            return false;
        }
        self.y += 1;
        true
    }
    pub fn right(&mut self) -> bool {
        if self.x == WIDTH - 1 {
            return false;
        }
        self.x += 1;
        true
    }
    pub fn left(&mut self) -> bool {
        if self.x == 0 {
            return false;
        }
        self.x -= 1;
        true
    }
    pub fn check(self) -> bool {
        if self.x < 0 || self.x >= WIDTH || self.y < 0 || self.y >= HEIGHT {
            return false;
        }
        true
    }
}
