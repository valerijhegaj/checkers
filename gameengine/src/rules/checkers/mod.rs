use crate::{
    behavior::Mover,
    model::{Board, Coordinate, MoveWithEffect, Piece, PieceContainer, PieceType, Player},
};

use super::{BoardCreator, Rule};

mod russian;

struct CheckersRule<
    ManMovement: Mover,
    KingMovement: Mover,
    ManCapture: Mover,
    KingCapture: Mover,
    B: BoardCreator,
> {
    man_movement: ManMovement,
    king_movement: KingMovement,
    man_capture: ManCapture,
    king_capture: KingCapture,
    board_creator: B,
    first_player: Player,
}

impl<
    ManMovement: Mover,
    KingMovement: Mover,
    ManCapture: Mover,
    KingCapture: Mover,
    B: BoardCreator,
> Rule for CheckersRule<ManMovement, KingMovement, ManCapture, KingCapture, B>
{
    fn moves(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect> {
        match *piece.get_piece_type() {
            PieceType::Man => self.man_movement.moves(board, player, piece, from),
            PieceType::King => self.king_movement.moves(board, player, piece, from),
        }
    }

    fn captures(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect> {
        match *piece.get_piece_type() {
            PieceType::Man => self.man_capture.moves(board, player, piece, from),
            PieceType::King => self.king_capture.moves(board, player, piece, from),
        }
    }

    fn get_init_board(&self) -> impl Board {
        self.board_creator.new_board()
    }

    fn get_first_player(&self) -> Player {
        self.first_player.clone()
    }
}
