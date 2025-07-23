use crate::model::{Board, MoveWithEffect, Piece, PieceContainer, Player};

pub enum GameStatus {
    Continue,
    Draw,
    Winner(Player),
}

pub trait GameStatusManager {
    fn set_move(self, mv: &MoveWithEffect) -> Self;
    fn cancel_move(self, mv: &MoveWithEffect) -> Self;
    fn get_game_status(&self) -> GameStatus;
}

pub trait PieceStatistics {
    fn set_move(self, mv: &MoveWithEffect) -> Self;
    fn cancel_move(self, mv: &MoveWithEffect) -> Self;
    fn count(&self, piece: &Piece) -> usize;
}

// not blazing fast
struct BoardSpectator<'a, B: Board> {
    board: &'a B,
}

impl<'a, B: Board> PieceStatistics for BoardSpectator<'a, B> {
    fn set_move(self, _: &MoveWithEffect) -> Self {
        self
    }

    fn cancel_move(self, _: &MoveWithEffect) -> Self {
        self
    }

    fn count(&self, piece: &Piece) -> usize {
        let mut iter = self.board.iter_for(piece.get_player());
        let mut counter = 0;
        while let Some((_, curr)) = iter.next() {
            if *curr.get_piece_type() == *piece.get_piece_type() {
                counter += 1;
            }
        }

        counter
    }
}

struct CommonStateManager<S: PieceStatistics> {
    piece_statistics: S,
}

impl<S: PieceStatistics> GameStatusManager for CommonStateManager<S> {
    fn set_move(mut self, mv: &MoveWithEffect) -> Self {
        self.piece_statistics = self.piece_statistics.set_move(mv);
        self
    }

    fn cancel_move(mut self, mv: &MoveWithEffect) -> Self {
        self.piece_statistics = self.piece_statistics.cancel_move(mv);
        self
    }

    fn get_game_status(&self) -> GameStatus {
        GameStatus::Continue
    }
}
