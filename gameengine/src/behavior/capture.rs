use crate::model::{
    Board, Coordinate, Direction, Move, MoveEffect, MoveWithEffect, Piece, PieceContainer, Player,
};

use super::{
    CaptureRule, MoveFilter, MovePermissionPolicy, MovementDirectionStrategy, Mover,
    MultiCapturePromotionRule, PromotionRule, RecursiveCapture, StepCountStrategy,
};

#[derive(Clone)]
pub struct Capture<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule + MultiCapturePromotionRule,
    MP: MovePermissionPolicy,
    F: MoveFilter,
    C: CaptureRule,
> {
    movement_direction_strategy: D,
    step_count_strategy: S,
    promotion_rule: P,
    move_permission_policy: MP,
    move_filter: F,
    capture_rule: C,
}

impl<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule + MultiCapturePromotionRule,
    MP: MovePermissionPolicy,
    F: MoveFilter,
    C: CaptureRule,
> Capture<D, S, P, MP, F, C>
{
    pub fn new(
        movement_direction_strategy: D,
        step_count_strategy: S,
        promotion_rule: P,
        move_permission_policy: MP,
        move_filter: F,
        capture_rule: C,
    ) -> Self {
        Capture {
            movement_direction_strategy,
            step_count_strategy,
            promotion_rule,
            move_permission_policy,
            move_filter,
            capture_rule,
        }
    }
}

impl<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule + MultiCapturePromotionRule,
    MP: MovePermissionPolicy,
    F: MoveFilter,
    C: CaptureRule,
> Mover for Capture<D, S, P, MP, F, C>
{
    fn moves(
        &self,
        board: &(impl Board + PieceContainer),
        player: &Player,
        captor: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect> {
        if !self.move_permission_policy.can_move(player, captor) {
            return Vec::new();
        }

        let prev_move = (Move::new_start(from.clone()), MoveEffect::no_effect());
        let mut eaten = std::collections::HashSet::new();
        eaten.insert(from.clone());

        let moves = self.recursive_moves(board, player, captor, &prev_move, &mut eaten);

        self.move_filter.filter(board, player, moves)
    }
}

impl<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule + MultiCapturePromotionRule,
    MP: MovePermissionPolicy,
    F: MoveFilter,
    C: CaptureRule,
> RecursiveCapture for Capture<D, S, P, MP, F, C>
{
    fn recursive_moves(
        &self,
        board: &impl Board,
        player: &Player,
        captor: &Piece,
        prev_move: &MoveWithEffect,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<MoveWithEffect> {
        let mut moves = Vec::new();

        let from = prev_move.0.get_landing();
        let directions = self.movement_direction_strategy.directions(player, captor);

        for direction in directions {
            let mut iter = board.iter_on_direction(from, direction);

            let Some(capture_position) =
                self.find_capture(&mut iter, player, captor, eaten, &direction)
            else {
                continue;
            };

            let landings = self.find_landing(
                &mut iter,
                player,
                captor,
                eaten,
                direction,
                &capture_position,
            );

            eaten.insert(capture_position.clone());

            for landing in landings {
                let next_moves = self.find_next_moves(
                    board,
                    player,
                    captor,
                    prev_move,
                    eaten,
                    landing,
                    &capture_position,
                ); // may recursive or promote call

                moves.extend(next_moves);
            }

            eaten.remove(&capture_position);
        }

        moves
    }
}

impl<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule + MultiCapturePromotionRule,
    MP: MovePermissionPolicy,
    F: MoveFilter,
    C: CaptureRule,
> Capture<D, S, P, MP, F, C>
{
    fn find_capture<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        captor: &Piece,
        eaten: &mut std::collections::HashSet<Coordinate>,
        direction: &Direction,
    ) -> Option<Coordinate> {
        let mut steps_before_capture = self
            .step_count_strategy
            .get_before_capture(player, captor, direction);

        while steps_before_capture > 0 {
            steps_before_capture -= 1;

            let Some((coordinate, piece)) = iter.next() else {
                return None;
            };

            let Some(captured) = piece else {
                continue;
            };

            if !self.capture_rule.allow_pass_trough_captured() {
                return None;
            }

            if eaten.contains(&coordinate) {
                continue;
            }

            if self
                .move_permission_policy
                .can_capture(player, captor, captured)
            {
                return None;
            }

            return Some(coordinate);
        }

        return None;
    }

    fn find_landing<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        captor: &Piece,
        eaten: &mut std::collections::HashSet<Coordinate>,
        direction: &Direction,
        capture_position: &Coordinate,
    ) -> Vec<Coordinate> {
        let mut landings = Vec::new();

        if self.capture_rule.allow_landing_on_captured() {
            landings = vec![capture_position.clone()];
        }

        let mut steps_after_capture = self
            .step_count_strategy
            .get_after_capture(player, captor, direction);

        while steps_after_capture > 0 {
            steps_after_capture -= 1;

            let Some((coordinate, piece)) = iter.next() else {
                return landings;
            };

            let Some(_) = piece else {
                landings.push(coordinate);
                continue;
            };

            if !self.capture_rule.allow_pass_trough_captured() {
                return landings;
            }

            if !eaten.contains(&coordinate) {
                return landings;
            }

            landings.push(coordinate);
        }

        return landings;
    }

    fn find_next_moves(
        &self,
        board: &impl Board,
        player: &Player,
        captor: &Piece,
        prev_move: &MoveWithEffect,
        eaten: &mut std::collections::HashSet<Coordinate>,
        landing: Coordinate,
        capture_position: &Coordinate,
    ) -> Vec<MoveWithEffect> {
        let mut mv = prev_move.clone();
        mv.0 = mv.0 + landing.clone();
        mv.1 = mv.1.capture(capture_position.clone());

        let mut next_moves = Vec::new();

        if self.promotion_rule.can_promote_in_move()
            && self
                .promotion_rule
                .should_promote(board, &landing, player, captor)
        {
            let promote_types = self.promotion_rule.promoted_types(player, captor);
            mv.1 = mv.1.promote(landing.clone(), promote_types[0].clone()); // Band-aid solution #1

            if self.capture_rule.allow_multi_capture() {
                let next_self = self.promotion_rule.strategy(player, captor);
                next_moves = next_self.recursive_moves(board, player, captor, &mv, eaten);
            }
        } else {
            if self.capture_rule.allow_multi_capture() {
                next_moves = self.recursive_moves(board, player, captor, &mv, eaten);
            }
        }

        if next_moves.len() == 0 || self.capture_rule.allow_non_maximal_multi_capture() {
            if !self.promotion_rule.can_promote_in_move()
                && self
                    .promotion_rule
                    .should_promote(board, &landing, player, captor)
            {
                let promote_types = self.promotion_rule.promoted_types(player, captor);
                mv.1 = mv.1.promote(landing.clone(), promote_types[0].clone()); // Band-aid solution #1
            }

            next_moves.push(mv);
        }

        next_moves
    }
}
