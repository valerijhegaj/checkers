use crate::model::{Board, Coordinate, Move, MoveEffect, MoveWithEffect, Piece, Player};

use super::{
    MovePermissionPolicy, MovementDirectionStrategy, Mover, PromotionRule, StepCountStrategy,
};

pub struct Movement<
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule,
    A: MovePermissionPolicy,
> {
    movement_direction_strategy: D,
    step_count_strategy: S,
    promotion_rule: P,
    move_permission_policy: A,
}

impl<D: MovementDirectionStrategy, S: StepCountStrategy, P: PromotionRule, A: MovePermissionPolicy>
    Movement<D, S, P, A>
{
    pub fn new(
        movement_direction_strategy: D,
        step_count_strategy: S,
        promotion_rule: P,
        move_permission_policy: A,
    ) -> Self {
        Movement {
            movement_direction_strategy,
            step_count_strategy,
            promotion_rule,
            move_permission_policy,
        }
    }
}

impl<D: MovementDirectionStrategy, S: StepCountStrategy, P: PromotionRule, A: MovePermissionPolicy>
    Mover for Movement<D, S, P, A>
{
    fn moves(
        &self,
        board: &impl Board,
        player: &Player,
        piece: &Piece,
        from: &Coordinate,
    ) -> Vec<MoveWithEffect> {
        let mut moves = Vec::new();

        if !self.move_permission_policy.can_move(player, piece) {
            return moves;
        }

        let directions = self.movement_direction_strategy.directions(player, piece);

        for direction in directions {
            let mut steps_left = self
                .step_count_strategy
                .get_standard(player, piece, direction);
            let mut iter = board.iter_on_direction(from, direction);

            while steps_left > 0 {
                steps_left -= 1;

                let Some((landing, landing_piece)) = iter.next() else {
                    break;
                };

                if landing_piece.is_some() {
                    break;
                }

                let mv = Move::new(from.clone(), landing.clone());

                if self
                    .promotion_rule
                    .should_promote(board, &landing, player, piece)
                {
                    let promotions = self.promotion_rule.promoted_types(player, piece);

                    for promotion in promotions {
                        let effect =
                            MoveEffect::no_effect().promote(landing.clone(), promotion.clone());
                        moves.push((mv.clone(), effect));
                    }
                } else {
                    moves.push((mv, MoveEffect::no_effect()));
                }
            }
        }

        moves
    }
}
