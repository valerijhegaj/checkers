mod checkers;
mod common;

use crate::{
    behavior::Mover,
    model::{Board, Coordinate, MoveWithEffect, Piece, PieceContainer, PieceType, Player},
};
use common::*;

trait Rule {
    fn moves(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect>;

    fn captures(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect>;

    fn get_init_board(&self) -> impl Board;
    fn get_first_player(&self) -> Player;
}
