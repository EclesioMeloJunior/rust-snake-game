use piston_window;
use snake::{
    Snake
};
use drawing::{draw_block, draw_rectangle};
use rand::{thread_rng, Rng};

const FOOD_COLOR: piston_window::types::Color = [0.90, 0.49, 0.13, 1.0];
const BORDER_COLOR: piston_window::types::Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: piston_window::types::Color = [0.91, 0.30, 0.24, 0.5];

const MOVING_PERIOD: f64 = 1.0;
const RESTART_TIME: f64 = 1.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction{
    Up, Down, Left, Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Game {
    snake: Snake,
    
    food_exist: bool,
    food_y: i32,
    food_x: i32,

    width: i32,
    height: i32,

    is_game_over: bool,
    waiting_time: f64,

    moving_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: true,
            food_x: 5,
            food_y: 3,
            width: width,
            height: height,
            is_game_over: false,
            moving_time: MOVING_PERIOD,
        }
    }

    pub fn key_pressed(&mut self, key: piston_window::Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            piston_window::Key::Up => Some(Direction::Up),
            piston_window::Key::Right => Some(Direction::Right),
            piston_window::Key::Left => Some(Direction::Left),
            piston_window::Key::Down => Some(Direction::Down),
            _ => None,
        };

        if dir.is_none() {
            return;
        }

        if self.snake.moving_direction().opposite() == dir.unwrap() {
            return;
        }

        self.update_snake(dir);
    }

    fn snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_head_x, next_head_y) = self.snake.next_position(dir);
        
        if self.snake.is_overlap_except_tail(next_head_x, next_head_y) {
            return false;
        }

        next_head_x > 0 && next_head_y > 0 && next_head_x < self.width - 1 && next_head_y < self.height - 1
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();

            if self.moving_time > 0.1 {
                self.moving_time = self.moving_time - 0.050
            }
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.snake_alive(dir) {
            self.snake.move_foward(dir);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height -1);

        while self.snake.is_overlap_except_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height -1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }

    pub fn draw(&self, con: &piston_window::Context, g: &mut piston_window::G2d) {
        self.snake.draw(con, g);
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.is_game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.is_game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }

            return;
        }

        if !self.food_exist {
            self.add_food();
        }

        if self.waiting_time > self.moving_time {
            self.update_snake(None);
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exist = true;
        self.food_x = 5;
        self.food_y = 3;
        self.is_game_over = false;
        self.moving_time = MOVING_PERIOD;
    }
}