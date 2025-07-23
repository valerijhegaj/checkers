use crate::model::{Board, Coordinate, MoveWithEffect, Piece, PieceContainer, Player};

pub trait RecursiveCapture {
    fn recursive_moves(
        &self,
        board: &impl Board,
        player: &Player,
        piece: &Piece,
        prev_move: &MoveWithEffect,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<MoveWithEffect>;
}

pub trait Mover {
    fn moves(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect>;
}

mod behaviors;
use behaviors::{
    CaptureRule, MoveFilter, MovePermissionPolicy, MovementDirectionStrategy,
    MultiCapturePromotionRule, PromotionRule, StepCountStrategy,
};
pub use behaviors::{
    CaptureRuleBuilder, EveryDiagonal, EveryMove, MaxStep, MoveOwnedCaptureOther, NoPromote,
    OneStep, PromoteToKing, PromoteToKingInOppositeAfterMove, PromoteToKingInOppositeInMove,
    ToPieceOpponentDiagonally,
};

mod endings;

mod ending;

mod movement;
pub use movement::Movement;

mod capture;
pub use capture::Capture;
