use super::*;

#[derive(Clone)]
pub struct EveryMove {}

impl<Coordinate, MV: Move<Coordinate>> MoveFilter<Coordinate, MV> for EveryMove {
    #[inline(always)]
    fn filter(&self, moves: impl Iterator<Item = MV>) -> impl Iterator<Item = MV> {
        moves
    }
}
