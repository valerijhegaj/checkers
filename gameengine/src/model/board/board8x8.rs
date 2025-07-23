use super::{Cell, PieceContainer, iter, iter_for, iter_on_direction};
use crate::model::{Board, Coordinate, Direction, Piece, Player};

pub struct Board8x8 {
    pub data: [[Cell; 8]; 8],
}

impl Board for Board8x8 {
    fn iter(&self) -> impl Iterator<Item = (Coordinate, &Piece)> {
        iter(&self.data)
    }

    fn iter_on_direction(
        &self,
        from: &Coordinate,
        direction: &Direction,
    ) -> impl Iterator<Item = (Coordinate, Option<&Piece>)> {
        iter_on_direction(self, from, direction)
    }

    fn is_on_opponent_border(&self, coordinate: &Coordinate, player: &Player) -> bool {
        match *player {
            Player::White => coordinate.get_row() == 7,
            Player::Black => coordinate.get_row() == 0,
        }
    }

    fn iter_for(&self, player: &Player) -> impl Iterator<Item = (Coordinate, &Piece)> {
        iter_for(&self.data, player)
    }
}

impl PieceContainer for Board8x8 {
    fn is_direction_out_of_border(&self, coordinate: &Coordinate, direction: &Direction) -> bool {
        (coordinate.get_row() == 0 && direction.get_row() < 0)
            || (coordinate.get_row() == 7 && direction.get_row() > 0)
            || (coordinate.get_col() == 0 && direction.get_col() < 0)
            || (coordinate.get_col() == 7 && direction.get_col() > 0)
    }

    fn at(&self, x: &Coordinate) -> Option<&Piece> {
        return self.data[x.get_row()][x.get_col()].as_ref();
    }
}
