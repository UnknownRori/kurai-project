use std::collections::VecDeque;

use super::movement::Movement;

#[derive(Debug)]
pub struct MovementQueue {
    pub target_move: VecDeque<Movement>,
}

impl MovementQueue {
    pub fn new(target_move: Vec<Movement>) -> Self {
        Self {
            target_move: target_move.into(),
        }
    }

    pub fn pop(&mut self) {
        self.target_move.pop_front();
    }
}
