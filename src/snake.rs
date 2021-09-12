use std::collections::LinkedList;
use piston_window::{
    Context,
    G2d,
    types::{
        Color
    }
};
use drawing::draw_block;
use game::Direction;

const SNAKE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block {
            x: x + 2,
            y: y
        });
        body.push_back(Block {
            x: x + 1,
            y: y
        });
        body.push_back(Block {
            x: x,
            y: y
        });

        Snake {
            direction: Direction::Right,
            body: body,
            last_removed_block: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for b in &self.body {
            draw_block(SNAKE_COLOR, b.x, b.y, con, g);
        }
    }

    pub fn moving_direction(&self) -> Direction {
        return self.direction;
    }

    pub fn move_foward(&mut self, dir: Option<Direction>) {
        if dir.is_some() {
            self.direction = dir.unwrap();
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            }
        };

        self.body.push_front(new_block);
        let removed = self.body.pop_back();
        self.last_removed_block = removed;
    }

    pub fn restore_last_removed(&mut self) {
        if self.last_removed_block.is_none() {
            return;
        }

        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn next_position(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let mut moving_dir: Direction = self.direction;

        if dir.is_some() {
            moving_dir = dir.unwrap();
        };

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn is_overlap_except_tail(&self, x: i32, y: i32) -> bool {
        let mut checked = 0;
        for b in &self.body {
            if x == b.x && y == b.y {
                return true;
            }

            checked += 1;
            if checked == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}