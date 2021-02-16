use crate::block::Block;
use crate::config;
use crate::config::RED;
use ggez::*;
use rand::Rng;

pub struct Food {
    f: Block,
}

impl Food {
    pub fn new() -> Food {
        let mut rng = rand::thread_rng();
        let _x = rng.gen_range(0..config::WIDTH);
        let _y = rng.gen_range(0..config::HEIGHT);
        Food {
            f: Block::new(_x, _y),
        }
    }
    pub fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        self.f.draw(mb, RED, true)?;
        Ok(())
    }
    pub fn info(&self) -> Block {
        self.f
    }
}
