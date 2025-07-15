use crate::movement::{Coordinate, Direction};
use crate::piece::{Board as BoardTrait, Piece, PieceType, Player};

type Cell = Option<Piece>;

pub struct Board {
    pub data: [[Cell; 8]; 8],
    pub current_player: Player,
}

impl Board {
    pub fn is_turn(&self, player: &Player) -> bool {
        return *player == self.current_player;
    }

    pub fn iter_for_current_player(&self) -> impl Iterator<Item = (Coordinate, &Piece)> {
        self.iter_for(&self.current_player)
    }
}

impl BoardTrait for Board {
    fn get_player(&self) -> &Player {
        return &self.current_player;
    }

    fn iter_diagonally(
        &self,
        from: &Coordinate,
        direction: &Direction,
    ) -> impl Iterator<Item = (Coordinate, Option<&Piece>)> {
        let mut iter = from.clone();

        std::iter::from_fn(move || {
            if self.is_direction_out_of_border(&iter, direction) {
                return None;
            }

            iter += direction;

            Some((iter.clone(), self.at(&iter)))
        })
    }
}

impl Board {
    fn is_direction_out_of_border(&self, coordinate: &Coordinate, direction: &Direction) -> bool {
        (coordinate.get_row() == 0 && direction.get_row() < 0)
            || (coordinate.get_row() == 7 && direction.get_row() > 0)
            || (coordinate.get_col() == 0 && direction.get_col() < 0)
            || (coordinate.get_col() == 7 && direction.get_col() > 0)
    }

    fn new_standard() -> Board {
        let mut data: [[Option<Piece>; 8]; 8] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    data[row][col] = Some(Piece::white_man());
                }
            }
        }

        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    data[row][col] = Some(Piece::black_man());
                }
            }
        }

        return Board {
            data: data,
            current_player: Player::White,
        };
    }

    fn print(&self) {
        println!("turn: {:?}", self.current_player);
        println!("  0 1 2 3 4 5 6 7 col");
        for (row_idx, row) in self.data.iter().enumerate() {
            print!("{} ", row_idx);
            for cell in row {
                match cell {
                    Some(piece) => match (piece.get_player(), piece.get_piece_type()) {
                        (Player::White, PieceType::Man) => print!("w "),
                        (Player::White, PieceType::King) => print!("W "),
                        (Player::Black, PieceType::Man) => print!("b "),
                        (Player::Black, PieceType::King) => print!("B "),
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
        println!("row");
    }

    fn at(&self, x: &Coordinate) -> Option<&Piece> {
        return self.data[x.get_row()][x.get_col()].as_ref();
    }

    fn iter_for(&self, player: &Player) -> impl Iterator<Item = (Coordinate, &Piece)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(row, row_cells)| {
                row_cells.iter().enumerate().filter_map(move |(col, cell)| {
                    cell.as_ref().and_then(|piece| {
                        if piece.is_owner(player) {
                            return Some((Coordinate::new(row, col), piece));
                        }
                        return None;
                    })
                })
            })
    }

    fn is_on_border(&self, coordinate: &Coordinate) -> bool {
        coordinate.get_row() == 0
            || coordinate.get_row() >= 7
            || coordinate.get_col() == 0
            || coordinate.get_col() >= 7
    }
}
