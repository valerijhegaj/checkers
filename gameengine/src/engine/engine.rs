use crate::board::Board;
use crate::movement::{Coordinate, Move};
use crate::piece::{Piece, Player};

pub fn validate_move(board: &Board, mv: &Move, player: &Player) -> Option<bool> {
    if !board.is_turn(player) {
        return Some(false);
    }

    let moves = get_all_moves(board);
    if moves.len() != 0 {
        return Some(moves.contains(mv));
    }

    None
}

pub fn get_all_moves(board: &Board) -> Vec<Move> {
    let capturing_moves = get_capturing_moves(board);
    if capturing_moves.len() != 0 {
        return capturing_moves;
    }

    get_not_capturing_moves(board)
}

fn collect_moves<F>(board: &Board, get_moves: F) -> Vec<Move>
where
    F: Fn(&Piece, &Board, &Coordinate) -> Vec<Move>,
{
    let mut moves = Vec::new();

    for (coordinate, piece) in board.iter_for_current_player() {
        moves.extend(get_moves(piece, board, &coordinate));
    }

    return moves;
}

fn get_capturing_moves(board: &Board) -> Vec<Move> {
    collect_moves(board, Piece::capturing_moves)
}

fn get_not_capturing_moves(board: &Board) -> Vec<Move> {
    collect_moves(board, Piece::not_capturing_moves)
}
