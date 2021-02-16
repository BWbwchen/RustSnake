use ggez::input::keyboard::KeyCode::*;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::*;
use mint::Point2;
use rand::Rng;
use std::vec::Vec;

mod background;
mod block;
mod config;
mod food;
mod snake;

struct Game {
    snake: snake::Snake,
    food: food::Food,
    background: background::BackGround,
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, config::MODE) {
            self.update();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mb = &mut graphics::MeshBuilder::new();

        self.draw(mb)?;

        let mesh = mb.build(ctx)?;
        match graphics::draw(ctx, &mesh, graphics::DrawParam::new()) {
            Ok(_) => (),
            Err(e) => println!("ERROR : {:#?}", e),
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            Escape => event::quit(ctx),
            Up => self.change_dir(config::Direction::UP),
            Down => self.change_dir(config::Direction::DOWN),
            Right => self.change_dir(config::Direction::RIGHT),
            Left => self.change_dir(config::Direction::LEFT),
            _ => println!("key {:#?} is pressed", keycode),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: snake::Snake::new(),
            food: food::Food::new(),
            background: background::BackGround::new(),
        }
    }

    pub fn change_dir(&mut self, _dir: config::Direction) {
        self.snake.change_dir(_dir);
    }

    pub fn update(&mut self) {
        // snake move and check if out of edge or touch body
        if !self.snake.auto_move() || !self.snake.check() {
            // game over
            self.reset();
        }
        if self.snake.eat(self.food.info()) {
            self.food = food::Food::new();
        }
    }
    fn reset(&mut self) {
        self.snake = snake::Snake::new();
        self.food = food::Food::new();
        self.background = background::BackGround::new();
    }
    pub fn draw(&mut self, mb: &mut graphics::MeshBuilder) -> GameResult {
        self.background.draw(mb)?;
        self.snake.draw(mb)?;
        self.food.draw(mb)?;
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Tetris Game", "BWbwchen")
        .conf(c)
        .build()
        .unwrap();

    let my_game = &mut Game::new();

    event::run(ctx, event_loop, my_game).unwrap();
}
