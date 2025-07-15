use crate::piece::{Board, Piece, Player};

use crate::movement::Coordinate;
use crate::movement::Move;
use crate::movement::{DIRECTIONS_EVERY_DIAGONAL, Direction};

impl Piece {
    pub(in crate::piece) fn man_capturing_moves(
        &self,
        board: &impl Board,
        from: &Coordinate,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut eaten = std::collections::HashSet::new();
        eaten.insert(from.clone());

        self.man_capturing_moves_recursive(
            board,
            &Move::new_from(from.clone()),
            &mut moves,
            &mut eaten,
        );
        return moves;
    }

    fn man_capturing_moves_recursive(
        &self,
        board: &impl Board,
        prev_move: &Move,
        moves: &mut Vec<Move>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) {
        let player = board.get_player();
        let directions = self.man_capturing_directions(player);
        let prev_landing = prev_move.get_landing();

        for direction in directions {
            let mut iter = board.iter_diagonally(prev_landing, direction);

            let Some(captured) = self.man_find_capture(&mut iter, player, eaten) else {
                continue;
            };

            let landings = self.man_find_after_capture_landings(&mut iter, eaten);

            eaten.insert(captured.clone());

            for landing in landings {
                let mv = prev_move.clone() + landing;

                self.man_capturing_moves_recursive(board, &mv, moves, eaten);

                moves.push(mv);
            }

            eaten.remove(&captured);
        }
    }

    fn man_find_capture<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Option<Coordinate> {
        let Some((captured, Some(enemy_piece))) = iter.next() else {
            return None;
        };

        if enemy_piece.is_owner(player) || eaten.contains(&captured) {
            return None;
        }

        return Some(captured);
    }

    fn man_find_after_capture_landings<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<Coordinate> {
        let Some((landing, piece)) = iter.next() else {
            return Vec::new();
        };
        let Some(_) = piece else {
            return vec![landing];
        };
        if eaten.contains(&landing) {
            return vec![landing];
        }

        return Vec::new();
    }

    fn man_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        return &DIRECTIONS_EVERY_DIAGONAL;
    }
}
