use super::*;

use crate::model::board::Board;
use crate::movement::RecursiveCapture;

const KING: [Piece; 1] = [Piece::King];

#[derive(Clone)]
pub struct PromoteToKing<'a, Coordinate, MV: Move<Coordinate>, B: Board<Coordinate, MV>> {
    board: &'a B,
    _marker: std::marker::PhantomData<(Coordinate, MV)>,
}

impl<Coordinate, MV: Move<Coordinate>, B: Board<Coordinate, MV>> PromotionRule<Coordinate>
    for PromoteToKing<'_, Coordinate, MV, B>
{
    #[inline(always)]
    fn should_promote(&self, position: &Coordinate, player: Player) -> bool {
        self.board.is_opposite_row(position, &player)
    }

    #[inline(always)]
    fn promoted_types(&self) -> &[Piece] {
        &KING
    }
}

#[derive(Clone)]
pub struct PromoteToKingInOppositeInMove<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> {
    pub recursive_capture: P,
    board: &'a B,
    _marker: std::marker::PhantomData<(Coordinate, MV)>,
}

impl<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> PromotionRule<Coordinate> for PromoteToKingInOppositeInMove<'a, Coordinate, MV, P, B>
{
    #[inline(always)]
    fn should_promote(&self, position: &Coordinate, player: Player) -> bool {
        self.board.is_opposite_row(position, &player)
    }

    #[inline(always)]
    fn promoted_types(&self) -> &[Piece] {
        &KING
    }
}

impl<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> MultiCapturePromotionRule<'a, Coordinate, MV>
    for PromoteToKingInOppositeInMove<'a, Coordinate, MV, P, B>
{
    #[inline(always)]
    fn can_promote_in_move(&self) -> bool {
        true
    }

    #[inline(always)]
    fn strategy(&self, _: Piece) -> &impl RecursiveCapture<'a, Coordinate, MV> {
        &self.recursive_capture
    }
}

#[derive(Clone)]
pub struct PromoteToKingInOppositeAfterMove<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> {
    pub recursive_capture: P,
    board: &'a B,
    _marker: std::marker::PhantomData<(Coordinate, MV)>,
}

impl<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> PromotionRule<Coordinate> for PromoteToKingInOppositeAfterMove<'a, Coordinate, MV, P, B>
{
    #[inline(always)]
    fn should_promote(&self, position: &Coordinate, player: Player) -> bool {
        self.board.is_opposite_row(position, &player)
    }

    #[inline(always)]
    fn promoted_types(&self) -> &[Piece] {
        &KING
    }
}

impl<
    'a,
    Coordinate,
    MV: Move<Coordinate>,
    P: RecursiveCapture<'a, Coordinate, MV>,
    B: Board<Coordinate, MV>,
> MultiCapturePromotionRule<'a, Coordinate, MV>
    for PromoteToKingInOppositeAfterMove<'a, Coordinate, MV, P, B>
{
    #[inline(always)]
    fn can_promote_in_move(&self) -> bool {
        false
    }

    #[inline(always)]
    fn strategy(&self, _: Piece) -> &impl RecursiveCapture<'a, Coordinate, MV> {
        &self.recursive_capture
    }
}

#[derive(Clone)]
pub struct NoPromote {}

// Band-aid solution #2
// source of it is Band-aid solution #1
// strategy should called newer
impl<'a, Coordinate, MV: Move<Coordinate>> RecursiveCapture<'a, Coordinate, MV> for NoPromote {
    fn recursive_moves(&'a self, _: MV) -> impl Iterator<Item = Option<MV>> + 'a
    where
        MV: 'a,
    {
        std::iter::empty()
    }
}

impl<Coordinate> PromotionRule<Coordinate> for NoPromote {
    #[inline(always)]
    fn should_promote(&self, position: &Coordinate, _: Player) -> bool {
        false
    }

    #[inline(always)]
    fn promoted_types(&self) -> &[Piece] {
        &KING
    }
}

impl<'a, Coordinate, MV: Move<Coordinate>> MultiCapturePromotionRule<'a, Coordinate, MV>
    for NoPromote
{
    #[inline(always)]
    fn can_promote_in_move(&self) -> bool {
        false
    }

    fn strategy(&self, _: Piece) -> &impl RecursiveCapture<Coordinate, MV> {
        self
    }
}
