use itertools::Either;

use crate::model::{Board, Move, Piece, Player};

use super::logic::{
    CaptureRule, MoveFilter, MovementDirectionStrategy, MultiCapturePromotionRule, PromotionRule,
    StepCountStrategy,
};
use super::*;

#[derive(Clone)]
pub struct Capture<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate> + Clone,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate> + MultiCapturePromotionRule<'a, Coordinate, MV>,
    F: MoveFilter<Coordinate, MV>,
    C: CaptureRule,
    B: Board<Coordinate, MV>,
> {
    movement_direction_strategy: D,
    step_count_strategy: S,
    promotion_rule: P,
    move_filter: F,
    capture_rule: C,
    board: &'a B,
    piece: Piece,
    _marker: std::marker::PhantomData<(Coordinate, MV)>,
}

impl<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate> + Clone,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate> + MultiCapturePromotionRule<'a, Coordinate, MV>,
    F: MoveFilter<Coordinate, MV>,
    C: CaptureRule,
    B: Board<Coordinate, MV>,
> Capture<'a, Coordinate, MV, D, S, P, F, C, B>
{
    pub fn new(
        movement_direction_strategy: D,
        step_count_strategy: S,
        promotion_rule: P,
        move_filter: F,
        capture_rule: C,
        board: &'a B,
        piece: Piece,
    ) -> Self {
        Capture {
            movement_direction_strategy,
            step_count_strategy,
            promotion_rule,
            move_filter,
            capture_rule,
            board,
            piece,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate> + Clone,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate> + MultiCapturePromotionRule<'a, Coordinate, MV>,
    F: MoveFilter<Coordinate, MV>,
    C: CaptureRule,
    B: Board<Coordinate, MV>,
> Capturer<'a, Coordinate, MV> for Capture<'a, Coordinate, MV, D, S, P, F, C, B>
{
    fn capture_moves(&'a self, player: Player, from: Coordinate) -> impl Iterator<Item = MV> + 'a {
        let prev_move = MV::new_from(from, &self.piece, player);

        let moves = self.recursive_moves(prev_move).filter_map(|mv| mv);

        self.move_filter.filter(moves)
    }
}

impl<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate> + Clone,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate> + MultiCapturePromotionRule<'a, Coordinate, MV>,
    F: MoveFilter<Coordinate, MV>,
    C: CaptureRule,
    B: Board<Coordinate, MV>,
> RecursiveCapture<'a, Coordinate, MV> for Capture<'a, Coordinate, MV, D, S, P, F, C, B>
{
    fn recursive_moves(&'a self, prev_move: MV) -> impl Iterator<Item = Option<MV>> + 'a {
        let directions = self
            .movement_direction_strategy
            .directions(prev_move.get_player());

        directions
            .filter_map(move |direction| {
                let mut mv = prev_move.clone();
                let cap = mv.clone();
                let player = prev_move.get_player();

                let mut cell = self.board.iter_direction(mv.get_landing(), direction);

                let Some((capture, piece)) = cell
                    .by_ref()
                    .take(self.step_count_strategy.get_before_capture())
                    .take_while(move |caption| {
                        if self.board.is_empty(caption) {
                            return true;
                        }

                        if cap.captured(caption) {
                            return self.capture_rule.allow_pass_trough_captured();
                        }

                        return false;
                    })
                    .find_map(move |caption| {
                        let enemy_piece = self.board.get_enemy(&caption, &player)?;
                        Some((caption, enemy_piece))
                    })
                else {
                    return None;
                };

                mv.add_capture(&capture, &piece);

                Some((/*capture,*/ cell, mv))
            })
            .flat_map(move |(/*_,*/ cell, mv)| {
                let loc_mv = mv.clone();
                let mv2 = mv.clone();

                let moves = cell
                    .take(self.step_count_strategy.get_after_capture())
                    .take_while(move |landing| {
                        let loc_mv = loc_mv.clone();

                        if self.board.is_empty(landing) {
                            return true;
                        }

                        if loc_mv.captured(landing) {
                            return self.capture_rule.allow_pass_trough_captured();
                        }

                        return false;
                    })
                    .flat_map(move |landing| {
                        let mut loc_mv = mv2.clone();
                        loc_mv.add_landing(landing);

                        let next_self = self.promotion_rule.strategy(loc_mv.get_piece());

                        let mut rec_mvs = next_self.recursive_moves(loc_mv.clone()).peekable();

                        if rec_mvs.peek().is_none()
                            || self.capture_rule.allow_non_maximal_multi_capture()
                        {
                            rec_mvs.chain(std::iter::once(Some(loc_mv)))
                        } else {
                            rec_mvs.chain(std::iter::once(None))
                        }
                    });

                moves
            })
    }
}
