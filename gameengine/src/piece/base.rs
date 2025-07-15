use crate::movement::coordinate::Coordinate;
use crate::movement::direction::Direction;

#[derive(PartialEq, Debug)]
pub enum Player {
    White,
    Black,
}

pub enum PieceType {
    Man,
    King,
}

pub trait Board {
    fn get_player(&self) -> &Player;
    fn iter_diagonally(
        &self,
        from: &Coordinate,
        direction: &Direction,
    ) -> impl Iterator<Item = (Coordinate, Option<&Piece>)>;
}

pub struct Piece {
    pub(in crate::piece) player: Player,
    pub(in crate::piece) piece_type: PieceType,
}

impl Piece {
    pub(crate) fn is_owner(&self, player: &Player) -> bool {
        self.player == *player
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}
