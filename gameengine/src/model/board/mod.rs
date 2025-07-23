use crate::model::{Coordinate, Direction, Piece, Player};

pub trait Board {
    fn iter_for(&self, player: &Player) -> impl Iterator<Item = (Coordinate, &Piece)>;
    fn iter(&self) -> impl Iterator<Item = (Coordinate, &Piece)>;
    fn iter_on_direction(
        &self,
        from: &Coordinate,
        direction: &Direction,
    ) -> impl Iterator<Item = (Coordinate, Option<&Piece>)>;
    fn is_on_opponent_border(&self, coordinate: &Coordinate, player: &Player) -> bool;
}

pub trait PieceContainer {
    fn is_direction_out_of_border(&self, coordinate: &Coordinate, direction: &Direction) -> bool;
    fn at(&self, x: &Coordinate) -> Option<&Piece>;
}

type Cell = Option<Piece>;

mod common;
use common::{iter, iter_for, iter_on_direction};

mod board8x8;
pub use board8x8::Board8x8;
