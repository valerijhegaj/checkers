use crate::model::{Move, Player};

pub trait RecursiveCapture<'a, Coordinate, MV: Move<Coordinate>> {
    fn recursive_moves(&'a self, prev_move: MV) -> impl Iterator<Item = Option<MV>> + 'a
    where
        MV: 'a;
}

pub trait Mover<Coordinate, MV: Move<Coordinate>> {
    fn moves<'a>(&'a self, player: Player, from: Coordinate) -> impl Iterator<Item = MV> + 'a;
}

pub trait Capturer<'a, Coordinate, MV: Move<Coordinate>> {
    fn capture_moves(&'a self, player: Player, from: Coordinate) -> impl Iterator<Item = MV> + 'a;
}

pub mod logic;

mod capture;
mod movement;
