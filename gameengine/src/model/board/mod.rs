pub mod bitboard;

use crate::model::{Direction, Piece, Player};

pub trait Board<Coordinate, MV: Move<Coordinate>> {
    fn apply(&mut self, mv: &MV);
    fn unapply(&mut self, mv: &MV);

    fn shift(&self, from: Coordinate, d: &Direction) -> Option<Coordinate>;

    fn is_opposite_row(&self, to: &Coordinate, player: &Player) -> bool;

    fn iter_direction(&self, from: Coordinate, d: &Direction) -> impl Iterator<Item = Coordinate>;

    fn is_empty(&self, to: &Coordinate) -> bool;
    fn get_enemy(&self, to: &Coordinate, player: &Player) -> Option<Piece>;
}

pub trait Move<Coordinate> {
    fn new_simple(from: Coordinate, to: Coordinate, piece: &Piece, player: Player) -> Self;
    fn new_from(from: Coordinate, piece: &Piece, player: Player) -> Self;

    fn add_landing(&mut self, landing: Coordinate);
    fn add_capture(&mut self, capture: &Coordinate, piece: &Piece);
    fn promote(&mut self, piece: &Piece);
    fn captured(&self, at: &Coordinate) -> bool;

    fn get_player(&self) -> Player;
    fn get_piece(&self) -> Piece;
    fn get_landing(&self) -> Coordinate;
}
