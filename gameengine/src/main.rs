#[derive(PartialEq, Debug)]
enum Player {
    White,
    Black,
}

enum PieceType {
    Man,
    King,
}

struct Piece {
    player: Player,
    piece_type: PieceType,
}

impl Piece {
    fn capturing_moves(&self, board: &Board, coordinate: &Coordinate) -> Vec<Move> {
        match self.piece_type {
            PieceType::Man => self.man_capturing_moves(board, coordinate),
            PieceType::King => self.king_capturing_moves(board, coordinate),
        }
    }

    fn man_capturing_moves(&self, board: &Board, from: &Coordinate) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut eaten = std::collections::HashSet::new();
        eaten.insert(from.clone());

        self.man_capturing_moves_recursive(
            board,
            &Move {
                from: from.clone(),
                to: Vec::new(),
            },
            &mut moves,
            &mut eaten,
        );
        return moves;
    }

    fn man_capturing_moves_recursive(
        &self,
        board: &Board,
        prev_move: &Move,
        moves: &mut Vec<Move>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) {
        let player = board.get_player();
        let directions = self.man_capturing_directions(player);
        let prev_landing = prev_move.get_landing();

        for direction in directions {
            let mut iter = board.iter_diagonally(prev_landing, direction);

            let Some(captured) = self.man_find_capture(&mut iter, player, eaten) else {
                continue;
            };

            let landings = self.man_find_after_capture_landings(&mut iter, eaten);

            eaten.insert(captured.clone());

            for landing in landings {
                let mut to = prev_move.to.clone();
                to.push(landing.clone());

                let mv = Move {
                    from: prev_move.from.clone(),
                    to,
                };

                self.man_capturing_moves_recursive(board, &mv, moves, eaten);

                moves.push(mv);
            }

            eaten.remove(&captured);
        }
    }

    fn man_find_capture<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Option<Coordinate> {
        let Some((captured, Some(enemy_piece))) = iter.next() else {
            return None;
        };

        if enemy_piece.player == *player || eaten.contains(&captured) {
            return None;
        }

        return Some(captured);
    }

    fn man_find_after_capture_landings<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<Coordinate> {
        let Some((landing, piece)) = iter.next() else {
            return Vec::new();
        };
        let Some(_) = piece else {
            return vec![landing];
        };
        if eaten.contains(&landing) {
            return vec![landing];
        }

        return Vec::new();
    }

    fn man_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        return &DIRECTIONS_EVERY_DIAGONAL;
    }

    fn king_capturing_moves(&self, board: &Board, from: &Coordinate) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut eaten = std::collections::HashSet::new();
        eaten.insert(from.clone());

        self.king_capturing_moves_recursive(
            board,
            &Move {
                from: from.clone(),
                to: Vec::new(),
            },
            &mut moves,
            &mut eaten,
        );
        return moves;
    }

    fn king_capturing_moves_recursive(
        &self,
        board: &Board,
        prev_move: &Move,
        moves: &mut Vec<Move>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) {
        let player = board.get_player();
        let directions = self.man_capturing_directions(player);
        let prev_landing = prev_move.get_landing();

        for direction in directions {
            let mut iter = board.iter_diagonally(prev_landing, direction);

            let Some(captured) = self.king_find_capture(&mut iter, player, eaten) else {
                continue;
            };

            let landings = self.king_find_after_capture_landings(&mut iter, eaten);

            eaten.insert(captured.clone());

            for landing in landings {
                let mut to = prev_move.to.clone();
                to.push(landing.clone());

                let mv = Move {
                    from: prev_move.from.clone(),
                    to,
                };

                self.king_capturing_moves_recursive(board, &mv, moves, eaten);

                moves.push(mv);
            }

            eaten.remove(&captured);
        }
    }

    fn king_find_capture<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        player: &Player,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Option<Coordinate> {
        while let Some((coordinate, piece)) = iter.next() {
            let Some(piece) = piece else {
                continue;
            };

            if eaten.contains(&coordinate) {
                continue;
            }

            if piece.player == *player {
                return None;
            }

            return Some(coordinate);
        }

        return None;
    }

    fn king_find_after_capture_landings<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (Coordinate, Option<&'a Piece>)>,
        eaten: &mut std::collections::HashSet<Coordinate>,
    ) -> Vec<Coordinate> {
        let mut landing = Vec::new();

        while let Some((coordinate, piece)) = iter.next() {
            let Some(_) = piece else {
                landing.push(coordinate);
                continue;
            };

            if eaten.contains(&coordinate) {
                landing.push(coordinate);
                continue;
            }

            return landing;
        }

        return landing;
    }

    fn king_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        return &DIRECTIONS_EVERY_DIAGONAL;
    }

    fn not_capturing_moves(&self, board: &Board, coordinate: &Coordinate) -> Vec<Move> {
        match self.piece_type {
            PieceType::Man => self.man_not_capturing_moves(board, coordinate),
            PieceType::King => self.king_not_capturing_moves(board, coordinate),
        }
    }

    fn man_not_capturing_moves(&self, board: &Board, from: &Coordinate) -> Vec<Move> {
        let mut moves = Vec::new();
        let player = board.get_player();
        let directions = self.man_not_capturing_directions(player);

        for direction in directions {
            let mut iter = board.iter_diagonally(from, direction);
            if let Some((to, None)) = iter.next() {
                moves.push(Move {
                    from: from.clone(),
                    to: vec![to],
                });
            }
        }

        moves
    }

    fn man_not_capturing_directions(&self, player: &Player) -> &[Direction; 2] {
        match *player {
            Player::White => &DIRECTIONS_FROM_WHITE_TO_BLACK,
            Player::Black => &DIRECTIONS_FROM_BLACK_TO_WHITE,
        }
    }

    fn king_not_capturing_moves(&self, board: &Board, from: &Coordinate) -> Vec<Move> {
        let mut moves = Vec::new();
        let player = board.get_player();
        let directions = self.king_not_capturing_directions(player);

        for direction in directions {
            let mut iter = board.iter_diagonally(from, direction);
            while let Some((to, piece)) = iter.next() {
                match piece {
                    Some(_) => break,
                    None => moves.push(Move {
                        from: from.clone(),
                        to: vec![to],
                    }),
                }
            }
        }

        moves
    }

    fn king_not_capturing_directions(&self, _: &Player) -> &[Direction; 4] {
        &DIRECTIONS_EVERY_DIAGONAL
    }
}

const DIRECTIONS_EVERY_DIAGONAL: [Direction; 4] = [
    Direction { drow: 1, dcol: 1 },
    Direction { drow: 1, dcol: -1 },
    Direction { drow: -1, dcol: 1 },
    Direction { drow: -1, dcol: -1 },
];

const DIRECTIONS_FROM_WHITE_TO_BLACK: [Direction; 2] = [
    Direction { drow: 1, dcol: 1 },
    Direction { drow: 1, dcol: -1 },
];

const DIRECTIONS_FROM_BLACK_TO_WHITE: [Direction; 2] = [
    Direction { drow: -1, dcol: 1 },
    Direction { drow: -1, dcol: -1 },
];

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl std::ops::AddAssign<&Direction> for Coordinate {
    fn add_assign(&mut self, direction: &Direction) {
        self.row = (self.row as isize + direction.drow as isize) as usize;
        self.col = (self.col as isize + direction.dcol as isize) as usize;
    }
}

struct Direction {
    pub drow: i8,
    pub dcol: i8,
}

type Cell = Option<Piece>;

struct Board {
    pub data: [[Cell; 8]; 8],
    pub current_player: Player,
}

impl TurnChecker for Board {
    fn is_turn(&self, player: &Player) -> bool {
        return *player == self.current_player;
    }
}

impl Board {
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
                if (row + col) % 2 != 1 {
                    continue;
                }
                data[row][col] = Some(Piece {
                    player: Player::White,
                    piece_type: PieceType::Man,
                });
            }
        }

        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 != 1 {
                    continue;
                }
                data[row][col] = Some(Piece {
                    player: Player::Black,
                    piece_type: PieceType::Man,
                });
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
                    Some(piece) => match (&piece.player, &piece.piece_type) {
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

    fn get(&self, x: &Coordinate) -> Option<&Piece> {
        return self.data[x.row][x.col].as_ref();
    }

    fn get_player(&self) -> &Player {
        return &self.current_player;
    }

    fn iter_for(&self, player: &Player) -> impl Iterator<Item = (Coordinate, &Piece)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(row, row_cells)| {
                row_cells.iter().enumerate().filter_map(move |(col, cell)| {
                    cell.as_ref().and_then(|piece| {
                        if piece.player == *player {
                            return Some((Coordinate { row, col }, piece));
                        }
                        return None;
                    })
                })
            })
    }

    fn iter_for_current_player(&self) -> impl Iterator<Item = (Coordinate, &Piece)> {
        self.iter_for(&self.current_player)
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

            Some((iter.clone(), self.get(&iter)))
        })
    }

    fn is_on_border(&self, coordinate: &Coordinate) -> bool {
        coordinate.row == 0 || coordinate.row >= 7 || coordinate.col == 0 || coordinate.col >= 7
    }

    fn is_direction_out_of_border(&self, coordinate: &Coordinate, direction: &Direction) -> bool {
        (coordinate.row == 0 && direction.drow < 0)
            || (coordinate.row == 7 && direction.drow > 0)
            || (coordinate.col == 0 && direction.dcol < 0)
            || (coordinate.col == 7 && direction.dcol > 0)
    }
}

trait TurnChecker {
    fn is_turn(&self, player: &Player) -> bool;
}

#[derive(PartialEq, Debug)]
struct Move {
    from: Coordinate,
    to: Vec<Coordinate>,
}

impl Move {
    fn get_landing(&self) -> &Coordinate {
        match self.to.last() {
            Some(to) => to,
            None => &self.from,
        }
    }
}

fn collect_moves<F>(board: &Board, get_moves: F) -> Vec<Move>
where
    F: Fn(&Piece, &Board, &Coordinate) -> Vec<Move>,
{
    let mut moves = Vec::new();

    for (coordinate, piece) in board.iter_for_current_player() {
        moves.extend(get_moves(piece, board, &coordinate));
    }

    return moves;
}

fn get_capturing_moves(board: &Board) -> Vec<Move> {
    collect_moves(board, Piece::capturing_moves)
}

fn get_not_capturing_moves(board: &Board) -> Vec<Move> {
    collect_moves(board, Piece::not_capturing_moves)
}

fn validate_move(board: &Board, mv: &Move, player: &Player) -> Option<bool> {
    if !board.is_turn(player) {
        return Some(false);
    }

    let capturing_moves = get_capturing_moves(board);
    if capturing_moves.len() != 0 {
        return Some(capturing_moves.contains(mv));
    }

    let not_capturing_moves = get_not_capturing_moves(board);
    if not_capturing_moves.len() != 0 {
        return Some(capturing_moves.contains(mv));
    }

    None
}

fn get_all_moves(board: &Board) -> Vec<Move> {
    let capturing_moves = get_capturing_moves(board);
    if capturing_moves.len() != 0 {
        return capturing_moves;
    }

    get_not_capturing_moves(board)
}

fn print_moves(moves: &Vec<Move>) {
    for mv in moves {
        print!("From (col: {}, row: {})", mv.from.col, mv.from.row);
        for coord in &mv.to {
            print!(" -> (col: {}, row: {})", coord.col, coord.row);
        }
        println!();
    }
}

fn ww() -> Option<Piece> {
    return Some(Piece {
        player: Player::White,
        piece_type: PieceType::Man,
    });
}

fn WW() -> Option<Piece> {
    return Some(Piece {
        player: Player::White,
        piece_type: PieceType::King,
    });
}

fn bb() -> Option<Piece> {
    return Some(Piece {
        player: Player::Black,
        piece_type: PieceType::Man,
    });
}

fn BB() -> Option<Piece> {
    return Some(Piece {
        player: Player::Black,
        piece_type: PieceType::King,
    });
}

fn main() {
    let mut data: [[Option<Piece>; 8]; 8] = [
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, ww(), None, None, None],
        [None, None, None, bb(), None, bb(), None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, bb(), None, bb(), None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
    ];

    let board = Board {
        data: data,
        current_player: Player::White,
    };
    board.print();

    let moves = get_all_moves(&board);
    print_moves(&moves);
}
