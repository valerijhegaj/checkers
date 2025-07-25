use itertools::Either;

use crate::model::{Board, Move, Piece, Player};

use super::logic::{MovementDirectionStrategy, PromotionRule, StepCountStrategy, Steps};
use super::*;

pub struct Movement<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate>,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate>,
    B: Board<Coordinate, MV>,
> {
    movement_direction_strategy: D,
    step_count_strategy: S,
    promotion_rule: P,
    board: &'a B,
    piece: Piece,
    _marker: std::marker::PhantomData<(Coordinate, MV)>,
}

impl<
    'a,
    Coordinate: Clone,
    MV: Move<Coordinate>,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate>,
    B: Board<Coordinate, MV>,
> Movement<'a, Coordinate, MV, D, S, P, B>
{
    pub fn new(
        movement_direction_strategy: D,
        step_count_strategy: S,
        promotion_rule: P,
        board: &'a B,
        piece: Piece,
    ) -> Self {
        Movement {
            movement_direction_strategy,
            step_count_strategy,
            promotion_rule,
            board,
            piece,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<
    Coordinate: Clone,
    MV: Move<Coordinate> + Clone,
    D: MovementDirectionStrategy,
    S: StepCountStrategy,
    P: PromotionRule<Coordinate>,
    B: Board<Coordinate, MV>,
> Mover<Coordinate, MV> for Movement<'_, Coordinate, MV, D, S, P, B>
{
    fn moves<'a>(&'a self, player: Player, from: Coordinate) -> impl Iterator<Item = MV> + 'a {
        let directions = self.movement_direction_strategy.directions(player.clone());

        directions.flat_map(move |direction| {
            let fr = from.clone();
            let pl = player.clone();

            self.board
                .iter_direction(from.clone(), direction)
                .take(self.step_count_strategy.get_standard())
                .take_while(|landing| self.board.is_empty(landing))
                .flat_map(move |landing| {
                    let mv = MV::new_simple(fr.clone(), landing.clone(), &self.piece, pl.clone());

                    if self.promotion_rule.should_promote(&landing, pl.clone()) {
                        let promos = self.promotion_rule.promoted_types();

                        Either::Left(promos.into_iter().map(move |promotion| {
                            let mut mv = mv.clone();
                            mv.promote(&promotion);
                            mv
                        }))
                    } else {
                        Either::Right(std::iter::once(MV::new_simple(
                            fr.clone(),
                            landing.clone(),
                            &self.piece,
                            pl.clone(),
                        )))
                    }
                })
        })
    }
}
