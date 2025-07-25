pub mod board;

pub use board::{Board, Move};

pub enum Direction {
    LeftDOWN,
    RightDOWN,
    LeftUP,
    RightUP,
}

#[derive(Clone)]
pub enum Player {
    WHITE,
    BLACK,
}

impl Player {
    fn to_opposite(&mut self) {
        *self = match *self {
            Player::BLACK => Player::WHITE,
            Player::WHITE => Player::BLACK,
        };
    }
}

#[derive(PartialEq, Clone)]
pub enum Piece {
    Man,
    King,
}
