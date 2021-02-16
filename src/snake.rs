use crate::block::Block;
use crate::config;
use crate::config::Direction;
use crate::config::BLACK;
use ggez::*;
use rand::Rng;
use std::collections::VecDeque;
use std::vec::Vec;

pub struct Snake {
    body: VecDeque<Block>,
    direction: Direction,
    add_len: bool,
}

impl Snake {
    pub fn new() -> Snake {
        let mut rng = rand::thread_rng();
        let _x = rng.gen_range(0..config::WIDTH);
        let _y = rng.gen_range(0..config::HEIGHT);
        let mut _body: VecDeque<Block> = VecDeque::new();
        _body.push_front(Block::new(_x, _y));
        Snake {
            body: _body,
            direction: Direction::RIGHT,
            add_len: false,
        }
    }
    pub fn auto_move(&mut self) -> bool {
        // add front block
        let mut temp = Block::new_by_block(self.body[0]);
        let success = match &self.direction {
            Direction::UP => temp.up(),
            Direction::DOWN => temp.down(),
            Direction::RIGHT => temp.right(),
            Direction::LEFT => temp.left(),
        };
        if !success {
            return false;
        }
        self.body.push_front(temp);

        if !self.add_len {
            self.body.pop_back();
        }

        true
    }
    pub fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        for (id, item) in self.body.iter_mut().enumerate() {
            item.draw(mb, BLACK, false)?;
        }
        Ok(())
    }
    pub fn check(&mut self) -> bool {
        // touch the edge
        if !self.body[0].check() {
            return false;
        }
        // touch the body
        for id in 1..self.body.len() {
            if self.body[0] == self.body[id] {
                return false;
            }
        }
        true
    }
    pub fn eat(&mut self, food: Block) -> bool {
        self.add_len = self.body[0] == food;
        self.add_len
    }
    pub fn change_dir(&mut self, _dir: Direction) {
        self.direction = _dir;
    }
}
