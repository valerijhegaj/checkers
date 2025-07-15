pub mod base;
pub use base::{Board, Piece, PieceType, Player};
pub mod checkers;

use crate::movement::Move;
use crate::movement::coordinate::Coordinate;

impl Piece {
    pub fn capturing_moves(&self, board: &impl Board, coordinate: &Coordinate) -> Vec<Move> {
        match self.piece_type {
            PieceType::Man => self.man_capturing_moves(board, coordinate),
            PieceType::King => self.king_capturing_moves(board, coordinate),
        }
    }

    pub fn not_capturing_moves(&self, board: &impl Board, coordinate: &Coordinate) -> Vec<Move> {
        match self.piece_type {
            PieceType::Man => self.man_not_capturing_moves(board, coordinate),
            PieceType::King => self.king_not_capturing_moves(board, coordinate),
        }
    }

    pub fn white_man() -> Piece {
        Piece {
            player: Player::White,
            piece_type: PieceType::Man,
        }
    }

    pub fn white_king() -> Piece {
        Piece {
            player: Player::White,
            piece_type: PieceType::King,
        }
    }

    pub fn black_man() -> Piece {
        Piece {
            player: Player::Black,
            piece_type: PieceType::Man,
        }
    }

    pub fn black_king() -> Piece {
        Piece {
            player: Player::Black,
            piece_type: PieceType::King,
        }
    }
}
