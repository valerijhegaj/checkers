use super::*;

#[derive(Clone)]
pub struct PromoteToKing {}

impl PromotionRule for PromoteToKing {
    #[inline(always)]
    fn should_promote(
        &self,
        board: &impl Board,
        position: &Coordinate,
        player: &Player,
        piece: &Piece,
    ) -> bool {
        board.is_on_opponent_border(position, piece.get_player())
    }

    #[inline(always)]
    fn promoted_types(&self, player: &Player, piece: &Piece) -> &[PieceType] {
        &[PieceType::King]
    }
}

#[derive(Clone)]
pub struct PromoteToKingInOppositeInMove<P: RecursiveCapture> {
    pub recursive_capture: P,
}

impl<P: RecursiveCapture> PromotionRule for PromoteToKingInOppositeInMove<P> {
    #[inline(always)]
    fn should_promote(
        &self,
        board: &impl Board,
        position: &Coordinate,
        player: &Player,
        piece: &Piece,
    ) -> bool {
        board.is_on_opponent_border(position, piece.get_player())
    }

    #[inline(always)]
    fn promoted_types(&self, player: &Player, piece: &Piece) -> &[PieceType] {
        &[PieceType::King]
    }
}

impl<P: RecursiveCapture> MultiCapturePromotionRule for PromoteToKingInOppositeInMove<P> {
    fn can_promote_in_move(&self) -> bool {
        true
    }
    fn strategy(&self, player: &Player, piece: &Piece) -> &impl RecursiveCapture {
        &self.recursive_capture
    }
}

#[derive(Clone)]
pub struct PromoteToKingInOppositeAfterMove<P: RecursiveCapture> {
    pub recursive_capture: P,
}

impl<P: RecursiveCapture> PromotionRule for PromoteToKingInOppositeAfterMove<P> {
    #[inline(always)]
    fn should_promote(
        &self,
        board: &impl Board,
        position: &Coordinate,
        player: &Player,
        piece: &Piece,
    ) -> bool {
        board.is_on_opponent_border(position, piece.get_player())
    }

    #[inline(always)]
    fn promoted_types(&self, player: &Player, piece: &Piece) -> &[PieceType] {
        &[PieceType::King]
    }
}

impl<P: RecursiveCapture> MultiCapturePromotionRule for PromoteToKingInOppositeAfterMove<P> {
    fn can_promote_in_move(&self) -> bool {
        false
    }
    fn strategy(&self, player: &Player, piece: &Piece) -> &impl RecursiveCapture {
        &self.recursive_capture
    }
}

#[derive(Clone)]
pub struct NoPromote {}

// Band-aid solution #2
// source of it is Band-aid solution #1
// strategy should called newer
impl RecursiveCapture for NoPromote {
    fn recursive_moves(
        &self,
        board: &impl Board,
        player: &Player,
        piece: &Piece,
        prev_move: &MoveWithEffect,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<MoveWithEffect> {
        Vec::new()
    }
}

impl PromotionRule for NoPromote {
    #[inline(always)]
    fn should_promote(
        &self,
        board: &impl Board,
        position: &Coordinate,
        player: &Player,
        piece: &Piece,
    ) -> bool {
        false
    }

    #[inline(always)]
    fn promoted_types(&self, player: &Player, piece: &Piece) -> &[PieceType] {
        &[PieceType::King]
    }
}

impl MultiCapturePromotionRule for NoPromote {
    fn can_promote_in_move(&self) -> bool {
        false
    }
    fn strategy(&self, player: &Player, piece: &Piece) -> &impl RecursiveCapture {
        self
    }
}
