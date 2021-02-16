use crate::block::Block;
use crate::config::{HEIGHT, WHITE, WIDTH};
use ggez::*;
use std::vec::Vec;

pub struct BackGround {
    map: [[Block; WIDTH]; HEIGHT],
}

impl BackGround {
    pub fn new() -> BackGround {
        let mut mm: [[Block; WIDTH]; HEIGHT] = [[Block::new(0, 0); WIDTH]; HEIGHT];
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                mm[row][col].set_axis(col, row);
            }
        }
        BackGround { map: mm }
    }
    pub fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                self.map[row][col].draw(mb, WHITE, true)?;
            }
        }
        Ok(())
    }
}
