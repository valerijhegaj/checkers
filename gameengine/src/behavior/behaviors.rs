mod capture;
pub use capture::CaptureRuleBuilder;

mod direction;
pub use direction::{EveryDiagonal, ToPieceOpponentDiagonally};

mod movefilter;
pub use movefilter::EveryMove;

mod permission;
pub use permission::MoveOwnedCaptureOther;

mod promotion;
pub use promotion::{
    NoPromote, PromoteToKing, PromoteToKingInOppositeAfterMove, PromoteToKingInOppositeInMove,
};

mod step;
pub use step::{MaxStep, OneStep};

use super::RecursiveCapture;
use crate::model::{
    Board, Coordinate, Direction, MoveWithEffect, Piece, PieceContainer, PieceType, Player,
};

pub trait MovementDirectionStrategy {
    fn directions(&self, player: &Player, piece: &Piece) -> &[Direction];
}

pub trait StepCountStrategy {
    fn get_standard(&self, player: &Player, piece: &Piece, direction: &Direction) -> usize;
    fn get_before_capture(&self, player: &Player, piece: &Piece, direction: &Direction) -> usize;
    fn get_after_capture(&self, player: &Player, piece: &Piece, direction: &Direction) -> usize;
}

pub trait PromotionRule {
    fn should_promote(
        &self,
        board: &impl Board,
        position: &Coordinate,
        player: &Player,
        piece: &Piece,
    ) -> bool;
    fn promoted_types(&self, player: &Player, piece: &Piece) -> &[PieceType];
}

// Band-aid solution #1
// ufortunally trait `RecursiveCapture` is not dyn compatible
// to fix it we need brake zero cost abstractions to dyn
// it won't be problem, because several promotions fortunately doesn't exists in checkers
// and fortunately in chess we don't have multi capturing
// `CapturePromotionBehavior` need only in  multi capturing
// TODO: research
pub trait MultiCapturePromotionRule {
    fn can_promote_in_move(&self) -> bool;
    fn strategy(&self, player: &Player, piece: &Piece) -> &impl RecursiveCapture;
}

pub trait MovePermissionPolicy {
    fn can_move(&self, player: &Player, piece: &Piece) -> bool;
    fn can_capture(&self, player: &Player, captor: &Piece, captured: &Piece) -> bool;
}

pub trait CaptureRule {
    fn allow_landing_on_captured(&self) -> bool;
    fn allow_multi_capture(&self) -> bool;
    fn allow_pass_trough_captured(&self) -> bool;
    fn allow_non_maximal_multi_capture(&self) -> bool;
}

pub trait MoveFilter {
    fn filter(
        &self,
        board: &impl PieceContainer,
        player: &Player,
        moves: Vec<MoveWithEffect>,
    ) -> Vec<MoveWithEffect>;
}
