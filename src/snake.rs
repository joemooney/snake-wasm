use std::collections::VecDeque;

use crate::random::random_range;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub direction: Direction,
    next_direction: Direction,
    // head is first item, tail is last
    pub snake: VecDeque<Position>,
    pub food: Position,
    pub lost: bool,
}


#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: vec![((width - 1).max(0), (height / 2).max(0))].into_iter().collect(),
            direction: Direction::Left,
            next_direction: Direction::Left,
            food: (2.min(width -1), height / 2),
            lost: false,
        }

    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.lost {
            return;
        }
        match (&self.direction, &direction) {
            (Direction::Up, Direction::Up) |
            (Direction::Up, Direction::Down) |
            (Direction::Down, Direction::Up) |
            (Direction::Down, Direction::Down) |
            (Direction::Left, Direction::Left) |
            (Direction::Left, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Right, Direction::Right) => {}
            _ => self.next_direction = direction
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.lost || self.snake.len() == 0 {
            return;
        }
        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
                Direction::Up => (x, y - 1),
                Direction::Down =>  (x, y + 1),
                Direction::Left =>  (x - 1, y),
                Direction::Right =>  (x + 1, y),
        };

        if ! self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.lost = true;
        } else {
            self.direction = self.next_direction;
            if new_head == self.food {
                if self.snake.len() == self.width * self.height {
                    print!("you won!");
                    return;
                }
                self.snake.push_front(self.food);
                loop {
                    self.food = (random_range(0, self.width), random_range(0, self.height));
                    if ! self.snake.contains(&self.food) {
                        break
                    }
                }
            } else {
                self.snake.pop_back();
                self.snake.push_front(new_head);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        let game = SnakeGame::new(40, 40);
        print!("{:?}", game);
    }
}
