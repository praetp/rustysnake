use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

use super::shared::{Position, UnitVector};

extern crate rand;

#[derive(Debug)]
pub struct Snake {
    segments: VecDeque<Position>, /* head of the snake is always first element */
    current: UnitVector,
    next: Option<UnitVector>

}

#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
    RUNNING,
    FINISHED
}

#[derive(Debug)]
pub struct SnakeGame {
    bait: Position,
    snake: Snake,
    state: GameState
}

#[derive(PartialEq, Eq, Hash)]
pub enum MoveResult {
    COLLISION,
    BAIT,
    NORMAL,
    NONE
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        let mut snake_game = SnakeGame {
            bait: Position::new(),
            snake: Snake {
                segments: VecDeque::from(vec![Position::new()]),
                current: UnitVector::new(),
                next: None
            },
            state: GameState::RUNNING
        };

        snake_game.place_new_bait();

        return snake_game;
    }

    pub fn get_bait(&self) -> &Position {
        return &self.bait;
    }

    pub fn get_snake_mut(&mut self) -> &mut Snake {
        return &mut self.snake;
    }

    pub fn get_snake(&self) -> &Snake {
        return &self.snake;
    }

    pub fn get_state(&self) -> &GameState {
        return &self.state;
    }

    pub fn get_score(&self) -> usize {
        self.snake.segments.len() - 1
    }

    fn place_new_bait(&mut self) {
        let mut newbait = Position::new();
        while Snake::check_collision(&self.snake, &newbait) {
            newbait = Position::new();
        }
        self.bait = newbait;
    }

    pub fn update(&mut self) -> MoveResult {
        if self.state == GameState::FINISHED {
            return MoveResult::NONE;
        }

        let bait;
        {
            bait = self.bait.clone()
        }
        let result = Snake::do_move(self.get_snake_mut(), bait);
        if result == MoveResult::BAIT {
            SnakeGame::place_new_bait(self);
        } else if result == MoveResult::COLLISION {
            self.state = GameState::FINISHED;
        }
//        println!("State {:?}", self);

        return result;
    }
}

impl Snake {

    pub fn get_segments(&self) -> Iter<Position> {
        return self.segments.iter();
    }

    fn get_new_front(&self) -> Position {
        let front: &Position = self.segments.front().unwrap();
        return front.add(&self.current);
    }

    pub fn check_collision(&self, newfront: &Position) -> bool {
        self.segments.contains(newfront)
    }

    pub fn do_move(&mut self, bait: Position) -> MoveResult {
        if self.next.is_some() {
            let new_current = &self.next;
            self.current = new_current.clone().unwrap();
        }

        self.next = None;
        let newfront: Position = Snake::get_new_front(self);
        if Snake::check_collision(self, &newfront) {
            return MoveResult::COLLISION;
        }

        let mut result = MoveResult::NORMAL;
        if newfront == bait {
            result = MoveResult::BAIT;
        }

        self.segments.push_front(newfront);

        if result != MoveResult::BAIT {
            self.segments.pop_back();
        }
        return result;
    }

    pub fn set_next(&mut self, new_next: UnitVector) {
//        /* don't allow snake reversal */

        if self.current.dist_squared(&new_next) != 4 {
            self.next = Some(new_next);
        }
    }
}