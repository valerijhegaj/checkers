use crate::piece::{Board, Piece, Player};

use crate::movement::Coordinate;
use crate::movement::Move;
use crate::movement::{DIRECTIONS_FROM_BLACK_TO_WHITE, DIRECTIONS_FROM_WHITE_TO_BLACK, Direction};

impl Piece {
    pub(in crate::piece) fn man_not_capturing_moves(
        &self,
        board: &impl Board,
        from: &Coordinate,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let player = board.get_player();
        let directions = self.man_not_capturing_directions(player);

        for direction in directions {
            let mut iter = board.iter_diagonally(from, direction);
            if let Some((landing, None)) = iter.next() {
                moves.push(Move::new_from(from.clone()) + landing);
            }
        }

        moves
    }

    fn man_not_capturing_directions(&self, player: &Player) -> &[Direction; 2] {
        match *player {
            Player::White => &DIRECTIONS_FROM_WHITE_TO_BLACK,
            Player::Black => &DIRECTIONS_FROM_BLACK_TO_WHITE,
        }
    }
}
