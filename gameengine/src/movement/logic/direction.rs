use super::*;

use crate::model::Direction;

const DIRECTIONS_EVERY_DIAGONAL: [Direction; 4] = [
    Direction::LeftDOWN,
    Direction::LeftUP,
    Direction::RightDOWN,
    Direction::RightUP,
];

const DIRECTIONS_DIAGONAL_FROM_WHITE_TO_BLACK: [Direction; 2] =
    [Direction::LeftUP, Direction::RightUP];

const DIRECTIONS_DIAGONAL_FROM_BLACK_TO_WHITE: [Direction; 2] =
    [Direction::LeftDOWN, Direction::RightUP];

#[derive(Clone)]
pub struct EveryDiagonal {}

impl MovementDirectionStrategy for EveryDiagonal {
    #[inline(always)]
    fn directions(&self, _: Player) -> impl Iterator<Item = &Direction> {
        DIRECTIONS_EVERY_DIAGONAL.iter()
    }
}

#[derive(Clone)]
pub struct ToPieceOpponentDiagonally {}

impl MovementDirectionStrategy for ToPieceOpponentDiagonally {
    #[inline(always)]
    fn directions(&self, player: Player) -> impl Iterator<Item = &Direction> {
        match player {
            Player::WHITE => DIRECTIONS_DIAGONAL_FROM_WHITE_TO_BLACK.iter(),
            Player::BLACK => DIRECTIONS_DIAGONAL_FROM_BLACK_TO_WHITE.iter(),
        }
    }
}
