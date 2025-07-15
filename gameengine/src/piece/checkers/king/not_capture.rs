use crate::piece::{Board, Piece, Player};

use crate::movement::Coordinate;
use crate::movement::Move;
use crate::movement::{DIRECTIONS_EVERY_DIAGONAL, Direction};

impl Piece {
    pub(in crate::piece) fn king_not_capturing_moves(
        &self,
        board: &impl Board,
        from: &Coordinate,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let player = board.get_player();
        let directions = self.king_not_capturing_directions(player);

        for direction in directions {
            let mut iter = board.iter_diagonally(from, direction);
            while let Some((landing, piece)) = iter.next() {
                match piece {
                    Some(_) => break,
                    None => moves.push(Move::new_from(from.clone()) + landing),
                }
            }
        }

        moves
    }

    fn king_not_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        &DIRECTIONS_EVERY_DIAGONAL
    }
}
