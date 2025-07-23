use super::*;

#[derive(Clone)]
pub struct EveryMove {}

impl MoveFilter for EveryMove {
    #[inline(always)]
    fn filter(
        &self,
        board: &impl PieceContainer,
        player: &Player,
        moves: Vec<MoveWithEffect>,
    ) -> Vec<MoveWithEffect> {
        moves
    }
}
