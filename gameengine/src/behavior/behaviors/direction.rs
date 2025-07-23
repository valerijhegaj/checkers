use super::*;

use crate::model::{
    DIRECTIONS_DIAGONAL_FROM_BLACK_TO_WHITE, DIRECTIONS_DIAGONAL_FROM_WHITE_TO_BLACK,
    DIRECTIONS_EVERY_DIAGONAL,
};

#[derive(Clone)]
pub struct EveryDiagonal {}

impl MovementDirectionStrategy for EveryDiagonal {
    #[inline(always)]
    fn directions(&self, _: &Player, _: &Piece) -> &[Direction] {
        return &DIRECTIONS_EVERY_DIAGONAL;
    }
}

#[derive(Clone)]
pub struct ToPieceOpponentDiagonally {}

impl MovementDirectionStrategy for ToPieceOpponentDiagonally {
    #[inline(always)]
    fn directions(&self, _: &Player, piece: &Piece) -> &[Direction] {
        match *piece.get_player() {
            Player::White => &DIRECTIONS_DIAGONAL_FROM_WHITE_TO_BLACK,
            Player::Black => &DIRECTIONS_DIAGONAL_FROM_BLACK_TO_WHITE,
        }
    }
}
