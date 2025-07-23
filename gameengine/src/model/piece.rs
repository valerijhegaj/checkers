use crate::model::player;

use super::Player;

#[derive(Clone, PartialEq)]
pub enum PieceType {
    Man,
    King,
}

pub struct Piece {
    player: Player,
    piece_type: PieceType,
}

impl Piece {
    pub fn is_owner(&self, player: &Player) -> bool {
        self.player == *player
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

pub fn wm() -> Option<Piece> {
    Some(Piece {
        player: Player::White,
        piece_type: PieceType::Man,
    })
}

pub fn wk() -> Option<Piece> {
    Some(Piece {
        player: Player::White,
        piece_type: PieceType::King,
    })
}

pub fn bm() -> Option<Piece> {
    Some(Piece {
        player: Player::Black,
        piece_type: PieceType::Man,
    })
}

pub fn bk() -> Option<Piece> {
    Some(Piece {
        player: Player::Black,
        piece_type: PieceType::King,
    })
}
