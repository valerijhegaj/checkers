use crate::model::{Direction, Move, Piece, Player};

use super::RecursiveCapture;

pub trait MovementDirectionStrategy {
    fn directions(&self, player: Player) -> impl Iterator<Item = &Direction>;
}

pub type Steps = usize;

const STEPS_INFINITY: Steps = usize::MAX;
const STEPS_ONE: Steps = 1;

pub trait StepCountStrategy {
    fn get_standard(&self) -> Steps;
    fn get_before_capture(&self) -> Steps;
    fn get_after_capture(&self) -> Steps;
}

pub trait PromotionRule<Coordinate> {
    fn should_promote(&self, position: &Coordinate, player: Player) -> bool;
    fn promoted_types(&self) -> &[Piece];
}

// Band-aid solution #1
// ufortunally trait `RecursiveCapture` is not dyn compatible
// to fix it we need brake zero cost abstractions to dyn
// it won't be problem, because several promotions fortunately doesn't exists in checkers
// and fortunately in chess we don't have multi capturing
// `CapturePromotionBehavior` need only in  multi capturing
// TODO: research
pub trait MultiCapturePromotionRule<'a, Coordinate, MV: Move<Coordinate>> {
    fn can_promote_in_move(&self) -> bool;
    fn strategy(&'a self, piece: Piece) -> &'a impl RecursiveCapture<Coordinate, MV>;
}

// pub trait MovePermissionPolicy {
//     fn can_move(&self, player: &Player, piece: &Piece) -> bool;
//     fn can_capture(&self, player: &Player, captor: &Piece, captured: &Piece) -> bool;
// }

pub trait CaptureRule {
    fn allow_landing_on_captured(&self) -> bool;
    fn allow_multi_capture(&self) -> bool;
    fn allow_pass_trough_captured(&self) -> bool;
    fn allow_non_maximal_multi_capture(&self) -> bool;
}

pub trait MoveFilter<Coordinate, MV: Move<Coordinate>> {
    fn filter(&self, moves: impl Iterator<Item = MV>) -> impl Iterator<Item = MV>;
}

mod capture;
mod direction;
mod movefilter;
mod permission;
mod promotion;
mod step;
