use crate::model::{Direction, Move, Piece, Player};

use super::Board;

pub type Coordinate = u32;

pub struct BitBoardMove {
    from: Coordinate,
    to: Coordinate,

    // нужно только для анимаций
    path_seq: u64, // 5 бит на клетку: 12 позиций максимум
    path_len: u8,

    captured_men: u32,
    captured_kings: u32,

    promote: bool,
    was_king: bool,
    player: Player,
}

// pub trait Move<Coordinate> {
//     fn new_simple(from: Coordinate, to: Coordinate, piece: &Piece, player: Player) -> Self;
//     fn new_from(from: Coordinate, piece: &Piece, player: Player) -> Self;

//     fn add_landing(&mut self, landing: Coordinate);
//     fn add_capture(&mut self, capture: Coordinate);
//     fn promote(&mut self, piece: &Piece);
//     fn captured(&self, at: &Coordinate) -> bool;

//     fn get_player(&self) -> Player;
//     fn get_piece(&self) -> Piece;
//     fn get_landing(&self) -> Coordinate;
// }

impl Move<Coordinate> for BitBoardMove {
    fn new_simple(from: Coordinate, to: Coordinate, from_piece: &Piece, player: Player) -> Self {
        let was_king = *from_piece == Piece::King;

        BitBoardMove {
            from,
            to,
            path_seq: 0,
            path_len: 0,
            captured_men: 0,
            captured_kings: 0,
            promote: false,
            was_king,
            player,
        }
    }

    fn new_from(from: Coordinate, piece: &Piece, player: Player) -> Self {
        BitBoardMove {
            from,
            to: from,
            path_seq: 0,
            path_len: 0,
            captured_men: 0,
            captured_kings: 0,
            promote: false,
            was_king: *piece == Piece::King,
            player,
        }
    }

    fn add_landing(&mut self, landing: Coordinate) {
        if self.to != self.from {
            let compact_to = self.to.trailing_zeros() as u64;
            self.path_seq |= compact_to << (self.path_len * 5);
            self.path_len += 1;
        }
        self.to = landing;
    }

    fn add_capture(&mut self, capture: &Coordinate, piece: &Piece) {
        match *piece {
            Piece::King => self.captured_kings |= capture,
            Piece::Man => self.captured_men |= capture,
        }
    }

    fn promote(&mut self, _: &Piece) {
        self.promote = true;
    }

    fn captured(&self, at: &Coordinate) -> bool {
        self.captured_kings | self.captured_men & *at != 0
    }

    fn get_player(&self) -> Player {
        self.player.clone()
    }

    fn get_piece(&self) -> Piece {
        if self.was_king {
            Piece::King
        } else {
            Piece::Man
        }
    }

    fn get_landing(&self) -> Coordinate {
        self.to
    }
}

pub struct BitBoard {
    white_man: u32,
    white_king: u32,

    black_man: u32,
    black_king: u32,
}

impl Board<Coordinate, BitBoardMove> for BitBoard {
    fn apply(&mut self, mv: &BitBoardMove) {
        let (own_man, own_king, opp_man, opp_king) = match mv.player {
            Player::WHITE => (
                &mut self.white_man,
                &mut self.white_king,
                &mut self.black_man,
                &mut self.black_king,
            ),
            Player::BLACK => (
                &mut self.black_man,
                &mut self.black_king,
                &mut self.white_man,
                &mut self.white_king,
            ),
        };

        *opp_man &= !mv.captured_men;
        *opp_king &= !mv.captured_kings;

        if mv.was_king {
            *own_king ^= mv.from;
            *own_king ^= mv.to;
        } else {
            *own_man ^= mv.from;

            if mv.promote {
                *own_king ^= mv.to;
            } else {
                *own_man ^= mv.to;
            }
        }
    }

    fn unapply(&mut self, mv: &BitBoardMove) {
        let (own_man, own_king, opp_man, opp_king) = match mv.player {
            Player::WHITE => (
                &mut self.white_man,
                &mut self.white_king,
                &mut self.black_man,
                &mut self.black_king,
            ),
            Player::BLACK => (
                &mut self.black_man,
                &mut self.black_king,
                &mut self.white_man,
                &mut self.white_king,
            ),
        };

        if mv.was_king {
            *own_king ^= mv.from;
            *own_king ^= mv.to;
        } else {
            *own_man ^= mv.from;

            if mv.promote {
                *own_king ^= mv.to;
            } else {
                *own_man ^= mv.to;
            }
        }

        *opp_man |= mv.captured_men;
        *opp_king |= mv.captured_kings;
    }

    fn shift(&self, from: Coordinate, d: &Direction) -> Option<Coordinate> {
        let to = match *d {
            Direction::RightUP => shift_right_up(from),
            Direction::LeftUP => shift_left_up(from),
            Direction::RightDOWN => shift_right_down(from),
            Direction::LeftDOWN => shift_left_down(from),
        };

        if to == 0 {
            return None;
        }

        Some(to)
    }

    fn is_opposite_row(&self, to: &Coordinate, player: &Player) -> bool {
        match *player {
            Player::WHITE => *to & UP_EDGE != 0,
            Player::BLACK => *to & DOWN_EDGE != 0,
        }
    }

    fn iter_direction(
        &self,
        mut from: Coordinate,
        d: &Direction,
    ) -> impl Iterator<Item = Coordinate> {
        std::iter::from_fn(move || {
            if let Some(to) = self.shift(from, d) {
                from = to;
                Some(to)
            } else {
                None
            }
        })
    }

    fn is_empty(&self, to: &Coordinate) -> bool {
        self.black_king | self.black_man | self.white_king | self.white_man & to != 0
    }

    fn get_enemy(&self, to: &Coordinate, player: &Player) -> Option<Piece> {
        match *player {
            Player::WHITE => {
                if self.black_man & to != 0 {
                    Some(Piece::Man)
                } else if self.black_king & to != 0 {
                    Some(Piece::King)
                } else {
                    None
                }
            }
            Player::BLACK => {
                if self.white_man & to != 0 {
                    Some(Piece::Man)
                } else if self.white_king & to != 0 {
                    Some(Piece::King)
                } else {
                    None
                }
            }
        }
    }
}

// movement

const RIGHT_EDGE: u32 = 0b_0000_1000_0000_1000_0000_1000_0000_1000;
const LEFT_EDGE: u32 = 0b_0001_0000_0001_0000_0001_0000_0001_0000;
const UP_EDGE: u32 = 0b_0000_0000_0000_0000_0000_0000_0000_1111;
const DOWN_EDGE: u32 = 0b_1111_0000_0000_0000_0000_0000_0000_0000;

const NOT_RIGHT_EDGE: u32 = !RIGHT_EDGE;
const NOT_LEFT_EDGE: u32 = !LEFT_EDGE;
const NOT_UP_EDGE: u32 = !UP_EDGE;
const NOT_DOWN_EDGE: u32 = !DOWN_EDGE;

const EVEN_ROW: u32 = 0b_0000_1111_0000_1111_0000_1111_0000_1111;
const NOT_EVEN_ROW: u32 = !EVEN_ROW;

#[inline(always)]
fn shift_left_down(bb: Coordinate) -> Coordinate {
    (bb & EVEN_ROW) << 4 | (bb & NOT_LEFT_EDGE & NOT_EVEN_ROW) << 3
}

#[inline(always)]
fn shift_right_down(bb: Coordinate) -> Coordinate {
    (bb & NOT_RIGHT_EDGE & EVEN_ROW) << 5 | (bb & NOT_EVEN_ROW) << 4
}

#[inline(always)]
fn shift_left_up(bb: Coordinate) -> Coordinate {
    (bb & EVEN_ROW) >> 4 | (bb & NOT_LEFT_EDGE & NOT_EVEN_ROW) >> 5
}

#[inline(always)]
fn shift_right_up(bb: Coordinate) -> Coordinate {
    (bb & NOT_RIGHT_EDGE & EVEN_ROW) >> 3 | (bb & NOT_EVEN_ROW) >> 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_left_down() {
        assert_eq!(shift_left_down(1 << 3), 1 << 7);
        assert_eq!(shift_left_down(1 << 7), 1 << 10);
        assert_eq!(shift_left_down(1 << 0), 1 << 4);
        assert_eq!(shift_left_down(1 << 4), 0);
        assert_eq!(shift_left_down(1 << 28), 0);
    }

    #[test]
    fn test_shift_right_down() {
        assert_eq!(shift_right_down(1 << 0), 1 << 5);
        assert_eq!(shift_right_down(1 << 3), 0);
        assert_eq!(shift_right_down(1 << 4), 1 << 8);
        assert_eq!(shift_right_down(1 << 5), 1 << 9);
        assert_eq!(shift_right_down(1 << 28), 0);
    }

    #[test]
    fn test_shift_left_up() {
        assert_eq!(shift_left_up(1 << 29), 1 << 24);
        assert_eq!(shift_left_up(1 << 24), 1 << 20);
        assert_eq!(shift_left_up(1 << 31), 1 << 26);
        assert_eq!(shift_left_up(1 << 20), 0);
        assert_eq!(shift_left_up(1 << 0), 0);
    }

    #[test]
    fn test_shift_right_up() {
        assert_eq!(shift_right_up(1 << 28), 1 << 24);
        assert_eq!(shift_right_up(1 << 31), 1 << 27);
        assert_eq!(shift_right_up(1 << 24), 1 << 21);
        assert_eq!(shift_right_up(1 << 27), 0);
        assert_eq!(shift_right_up(1 << 0), 0);
    }
}
