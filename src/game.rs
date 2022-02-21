use std::io::repeat;

use piston_window::*;
use piston_window::types::Color;
use rand::{Rng, thread_rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAME_OVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,

    gameboard_width: i32,
    gameboard_height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    /// Initialises a new game with a `width`x`height` gameboard
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            gameboard_width: width,
            gameboard_height: height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(dir);
    }

    /// Draws a new gameboard with initial ``Snake``, borders, and the first food
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.gameboard_width, 1, con, g); // Upper border
        draw_rectangle(BORDER_COLOR, 0, self.gameboard_height - 1, self.gameboard_width, 1, con, g); // Lower border
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.gameboard_height, con, g); // left border
        draw_rectangle(BORDER_COLOR, self.gameboard_width - 1, 0, 1, self.gameboard_height, con, g); // Right border

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.gameboard_width, self.gameboard_height, con, g);
        }
    }

    /// Adds a food randomly on the gameboard
    pub fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_food_x: i32;
        let mut new_food_y: i32;
        loop {
            new_food_x = rng.gen_range(1..self.gameboard_width - 1);
            new_food_y = rng.gen_range(1..self.gameboard_height - 1);
            if !self.snake.overlap_tail(new_food_x, new_food_y) { break; }
        }
        self.food_x = new_food_x;
        self.food_y = new_food_y;
        self.food_exists = true;
    }

    pub fn eating_food(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists {
            if self.food_x == head_x && self.food_y == head_y {
                self.food_exists = false;
                self.snake.add_block_tail()
            }
        }
    }

    /// Verifies if the snake is still alive, which means he has not eaten its tail or outbounded the gameboard.
    pub fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.gameboard_width - 1 && next_y < self.gameboard_height - 1
    }

    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.eating_food();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time > RESTART_TIME { self.restart(); }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}