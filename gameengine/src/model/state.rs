use crate::model::{Board, Move, PieceContainer, Player};

struct StateContainer<B: Board + PieceContainer, S> {
    pub board: B,
    pub current_player: Player,
    pub moves: Vec<Move>,
    pub state_manager: S,
}

impl<B: Board + PieceContainer, S> StateContainer<B, S> {
    fn add_move(&mut self, mv: Move) -> &Self {
        self.moves.push(mv);
        self
    }

    fn change_player(&mut self) -> &Self {
        self.current_player = self.current_player.opponent();
        self
    }

    fn change_state_manager(&mut self) -> &mut S {
        &mut self.state_manager
    }

    fn get_state_manager(&self) -> &S {
        &self.state_manager
    }
}
