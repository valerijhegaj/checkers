use crate::piece::{Board, Piece, Player};

use crate::movement::Coordinate;
use crate::movement::Move;
use crate::movement::{DIRECTIONS_EVERY_DIAGONAL, Direction};

impl Piece {
    pub(in crate::piece) fn king_capturing_moves(
        &self,
        board: &impl Board,
        from: &Coordinate,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut eaten = std::collections::HashSet::new();
        eaten.insert(from.clone());

        self.king_capturing_moves_recursive(
            board,
            &Move::new_from(from.clone()),
            &mut moves,
            &mut eaten,
        );
        return moves;
    }

    fn king_capturing_moves_recursive(
        &self,
        board: &impl Board,
        prev_move: &Move,
        moves: &mut Vec<Move>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) {
        let player = board.get_player();
        let directions = self.king_capturing_directions(player);
        let prev_landing = prev_move.get_landing();

        for direction in directions {
            let mut iter = board.iter_diagonally(prev_landing, direction);

            let Some(captured) = self.king_find_capture(&mut iter, player, eaten) else {
                continue;
            };

            let landings = self.king_find_after_capture_landings(&mut iter, eaten);

            eaten.insert(captured.clone());

            for landing in landings {
                let mv = prev_move.clone() + landing;

                self.king_capturing_moves_recursive(board, &mv, moves, eaten);

                moves.push(mv);
            }

            eaten.remove(&captured);
        }
    }

    fn king_find_capture<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Option<Coordinate> {
        while let Some((coordinate, piece)) = iter.next() {
            let Some(piece) = piece else {
                continue;
            };

            if eaten.contains(&coordinate) {
                continue;
            }

            if piece.is_owner(player) {
                return None;
            }

            return Some(coordinate);
        }

        return None;
    }

    fn king_find_after_capture_landings<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<Coordinate> {
        let mut landing = Vec::new();

        while let Some((coordinate, piece)) = iter.next() {
            let Some(_) = piece else {
                landing.push(coordinate);
                continue;
            };

            if eaten.contains(&coordinate) {
                landing.push(coordinate);
                continue;
            }

            return landing;
        }

        return landing;
    }

    fn king_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        return &DIRECTIONS_EVERY_DIAGONAL;
    }
}
