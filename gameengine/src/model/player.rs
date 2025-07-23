#[derive(PartialEq, Debug, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match *self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
