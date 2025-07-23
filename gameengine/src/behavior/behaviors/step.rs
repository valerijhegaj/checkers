use std::usize;

use super::*;

#[derive(Clone)]
pub struct OneStep {}

impl StepCountStrategy for OneStep {
    #[inline(always)]
    fn get_standard(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return 1;
    }

    #[inline(always)]
    fn get_before_capture(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return 1;
    }

    #[inline(always)]
    fn get_after_capture(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return 1;
    }
}

#[derive(Clone)]
pub struct MaxStep {}

impl StepCountStrategy for MaxStep {
    #[inline(always)]
    fn get_standard(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return usize::MAX;
    }

    #[inline(always)]
    fn get_before_capture(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return usize::MAX;
    }

    #[inline(always)]
    fn get_after_capture(&self, _: &Player, _: &Piece, _: &Direction) -> usize {
        return usize::MAX;
    }
}
