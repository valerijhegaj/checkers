type Coordinate = u8;

// Move linked with BitBoard
#[derive(Clone, Debug)]
struct Move {
    from: Coordinate,
    to: Coordinate,

    // нужно только для анимаций
    path_seq: u64, // 5 бит на клетку: 12 позиций максимум
    path_len: u8,

    captured: u32,
    captured_kings: u32,

    promote: bool,
    was_king: bool,
    player: Player,
}

impl Move {
    fn add_landing(&mut self, landing: Coordinate) {
        self.to = landing;
    }
}

struct BitBoard {
    white: u32,
    black: u32,
    king: u32,
}

impl BitBoard {
    fn apply(&mut self, mv: &Move) {
        let from = 1 << mv.from;
        let to = 1 << mv.to;

        let (own, opp) = match mv.player {
            WHITE => (&mut self.white, &mut self.black),
            BLACK => (&mut self.black, &mut self.white),
        };

        *opp &= !mv.captured;
        self.king &= !mv.captured;

        *own ^= from;
        *own ^= to;

        if mv.was_king {
            self.king ^= from;
            self.king |= to;
        } else if mv.promote {
            self.king |= to;
        }
    }

    fn unapply(&mut self, mv: &Move) {
        let from = 1 << mv.from;
        let to = 1 << mv.to;

        let (own, opp) = match mv.player {
            WHITE => (&mut self.white, &mut self.black),
            BLACK => (&mut self.black, &mut self.white),
        };

        *own ^= from;
        *own ^= to;

        if mv.was_king {
            self.king ^= to;
            self.king |= from;
        } else if mv.promote {
            self.king ^= to;
        }

        *opp |= mv.captured;
        self.king |= mv.captured_kings;
    }
}

// TODO: bench to compare with enum
type Player = bool;

const WHITE: Player = true;
const BLACK: Player = false;

fn to_opposite(player: Player) -> Player {
    !player
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

enum Direction {
    LeftDOWN,
    RightDOWN,
    LeftUP,
    RightUP,
}

#[inline(always)]
fn shift_left_down(bb: u32) -> u32 {
    (bb & EVEN_ROW) << 4 | (bb & NOT_LEFT_EDGE & NOT_EVEN_ROW) << 3
}

#[inline(always)]
fn shift_right_down(bb: u32) -> u32 {
    (bb & NOT_RIGHT_EDGE & EVEN_ROW) << 5 | (bb & NOT_EVEN_ROW) << 4
}

#[inline(always)]
fn shift_left_up(bb: u32) -> u32 {
    (bb & EVEN_ROW) >> 4 | (bb & NOT_LEFT_EDGE & NOT_EVEN_ROW) >> 5
}

#[inline(always)]
fn shift_right_up(bb: u32) -> u32 {
    (bb & NOT_RIGHT_EDGE & EVEN_ROW) >> 3 | (bb & NOT_EVEN_ROW) >> 4
}

fn shift(from: u32, d: &Direction) -> Option<u32> {
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

fn promote(to: u32, player: Player) -> bool {
    match player {
        WHITE => to & UP_EDGE != 0,
        BLACK => to & DOWN_EDGE != 0,
    }
}

fn simple_man_moves(from: Coordinate, player: Player, occupied: u32, moves: &mut Vec<Move>) {
    let directions: &[Direction] = if player == WHITE {
        &[Direction::RightUP, Direction::LeftUP]
    } else {
        &[Direction::RightDOWN, Direction::LeftDOWN]
    };

    for d in directions {
        if let Some(to) = shift(1 << from, d) {
            if to == 0 {
                continue;
            }

            if (occupied & to) == 0 {
                moves.push(Move {
                    from: from,
                    to: to.trailing_zeros() as u8,
                    path_seq: 0,
                    path_len: 0,
                    captured: 0,
                    captured_kings: 0,
                    promote: promote(to, player),
                    was_king: false,
                    player: player,
                });
            }
        }
    }
}

fn simple_king_moves(from: Coordinate, player: Player, occupied: u32, moves: &mut Vec<Move>) {
    let directions: &[Direction] = &[
        Direction::LeftUP,
        Direction::LeftDOWN,
        Direction::RightUP,
        Direction::RightDOWN,
    ];

    for d in directions {
        let mut it = 1 << from;
        while let Some(to) = shift(it, d) {
            if (occupied & to) != 0 || to == 0 {
                break;
            }
            it = to;

            moves.push(Move {
                from: from,
                to: to.trailing_zeros() as u8,
                path_seq: 0,
                path_len: 0,
                captured: 0,
                captured_kings: 0,
                promote: false,
                was_king: true,
                player: player,
            });
        }
    }
}

fn find_man_captures(board: &BitBoard, occupied: u32, mv: &Move, moves: &mut Vec<Move>) -> bool {
    let directions: &[Direction] = &[
        Direction::LeftUP,
        Direction::LeftDOWN,
        Direction::RightUP,
        Direction::RightDOWN,
    ];

    let player = mv.player;
    let to_capture = match player {
        WHITE => board.black,
        BLACK => board.white,
    };
    let mut found = false;

    for d in directions {
        let Some(capture) = shift(1 << mv.to, d) else {
            continue;
        };

        if to_capture & capture == 0 || capture & mv.captured != 0 {
            continue;
        }

        let Some(landing) = shift(capture, d) else {
            continue;
        };

        if landing & occupied != 0 {
            continue;
        }

        found = true;
        let landing_coordinate = landing.trailing_zeros() as u8;

        let mut new_mv = mv.clone();
        new_mv.add_landing(landing_coordinate);
        new_mv.promote = promote(landing, player);
        new_mv.captured |= capture;
        new_mv.captured_kings |= capture & board.king;

        if new_mv.promote {
            if !find_king_captures(board, occupied, &new_mv, moves) {
                moves.push(new_mv);
            }
        } else {
            if !find_man_captures(board, occupied, &new_mv, moves) {
                moves.push(new_mv);
            }
        }
    }

    found
}

fn find_king_captures(board: &BitBoard, occupied: u32, mv: &Move, moves: &mut Vec<Move>) -> bool {
    let directions: &[Direction] = &[
        Direction::LeftUP,
        Direction::LeftDOWN,
        Direction::RightUP,
        Direction::RightDOWN,
    ];

    let player = mv.player;
    let to_capture = match player {
        WHITE => board.black,
        BLACK => board.white,
    };
    let mut found = false;

    for d in directions {
        let Some(capture) = find_king_capture(occupied, 1 << mv.to, d) else {
            continue;
        };

        if to_capture & capture == 0 || capture & mv.captured != 0 {
            continue;
        }

        let landings = find_landings(occupied, capture, d);

        if landings.len() == 0 {
            continue;
        }

        found = true;

        for landing in landings {
            let landing_coordinate = landing.trailing_zeros() as u8;

            let mut new_mv = mv.clone();
            new_mv.add_landing(landing_coordinate);
            new_mv.captured |= capture;
            new_mv.captured_kings |= capture & board.king;

            if !find_king_captures(board, occupied, &new_mv, moves) {
                moves.push(new_mv);
            }
        }
    }

    found
}

fn find_king_capture(occupied: u32, mut from: u32, d: &Direction) -> Option<u32> {
    while let Some(capture) = shift(from, d) {
        if capture & occupied != 0 {
            return Some(capture);
        }

        from = capture;
    }

    None
}

fn find_landings(occupied: u32, mut from: u32, d: &Direction) -> Vec<u32> {
    let mut landings = Vec::new();

    while let Some(landing) = shift(from, d) {
        if landing & occupied != 0 {
            break;
        }

        landings.push(landing);
        from = landing;
    }

    landings
}

// rules

type TerminalState = u8;

const LOSE: TerminalState = 0;
const DRAW: TerminalState = 1;
const WIN: TerminalState = 2;

struct RussianRule {
    board: BitBoard,
    player: Player,
    // positions: std::collections::HashMap<(BitBoard, Player), u8>,
    counter: u8,
}

impl Node<Move, u8, TerminalState> for RussianRule {
    fn evaluate(&self, player: &Player) -> score {
        let white_men = self.board.white.count_ones() as score;
        let black_men = self.board.black.count_ones() as score;

        let kings = self.board.king;
        let white_kings = (kings & self.board.white).count_ones() as score;
        let black_kings = (kings & self.board.black).count_ones() as score;

        let base_score = (white_men + 2 * white_kings) - (black_men + 2 * black_kings);

        let perspective = if *player == BLACK { -1 } else { 1 };

        base_score * perspective
    }

    fn terminal(&self, moves: &Vec<Move>) -> (bool, TerminalState) {
        let (own, opposite) = match self.player {
            WHITE => (self.board.white, self.board.black),
            BLACK => (self.board.black, self.board.white),
        };

        if own == 0 {
            return (true, LOSE);
        } else if opposite == 0 {
            return (true, WIN);
        }

        if self.counter == 15 {
            return (true, DRAW);
        }

        if moves.len() == 0 {
            return (true, LOSE);
        }

        return (false, 0);
    }

    fn evaluate_terminal(&self, terminal_state: TerminalState, player: &Player) -> score {
        if self.player == *player {
            return match terminal_state {
                LOSE => MIN,
                DRAW => 0,
                WIN => MAX,
                _ => 0,
            };
        }

        match terminal_state {
            LOSE => MAX,
            DRAW => 0,
            WIN => MIN,
            _ => 0,
        }
    }

    fn apply(&mut self, edge: &Move) -> u8 {
        self.player = to_opposite(self.player);
        self.board.apply(edge);

        let counter = self.counter;
        if edge.was_king && edge.captured == 0 {
            self.counter += 1;
        } else {
            self.counter = 0;
        }

        counter
    }

    fn unapply(&mut self, edge: &Move, counter: u8) {
        self.player = to_opposite(self.player);
        self.board.unapply(edge);
        self.counter = counter;
    }

    fn edge_to_children(&self) -> Vec<Move> {
        let (own, ops) = match self.player {
            WHITE => (self.board.white, self.board.black),
            BLACK => (self.board.black, self.board.white),
        };

        let mut moves = Vec::new();

        let mut own_man = own & !self.board.king;
        let mut own_king = own & self.board.king;
        let occupied = self.board.black | self.board.white;

        // logic
        while own_man != 0 {
            let from = own_man.trailing_zeros() as Coordinate;
            own_man &= own_man - 1;

            find_man_captures(
                &self.board,
                occupied ^ (1 << from),
                &Move {
                    from: from,
                    to: from,
                    path_seq: 0,
                    path_len: 0,
                    captured: 0,
                    captured_kings: 0,
                    promote: false,
                    was_king: false,
                    player: self.player,
                },
                &mut moves,
            );
        }

        // --- дамки ---
        while own_king != 0 {
            let from = own_king.trailing_zeros() as Coordinate;
            own_king &= own_king - 1;

            find_king_captures(
                &self.board,
                occupied ^ (1 << from),
                &Move {
                    from: from,
                    to: from,
                    path_seq: 0,
                    path_len: 0,
                    captured: 0,
                    captured_kings: 0,
                    promote: false,
                    was_king: true,
                    player: self.player,
                },
                &mut moves,
            );
        }

        if moves.len() > 0 {
            return moves;
        }

        let mut own_man = own & !self.board.king;
        let mut own_king = own & self.board.king;

        // logic
        while own_man != 0 {
            let from = own_man.trailing_zeros() as Coordinate;
            own_man &= own_man - 1;

            simple_man_moves(from, self.player, occupied, &mut moves);
        }

        // --- дамки ---
        while own_king != 0 {
            let from = own_king.trailing_zeros() as Coordinate;
            own_king &= own_king - 1;

            simple_king_moves(from, self.player, occupied, &mut moves);
        }

        return moves;
    }
}

impl RussianRule {
    fn new() -> Self {
        RussianRule {
            board: BitBoard {
                white: 0b_1111_1111_1111_0000_0000_0000_0000_0000,
                black: 0b_0000_0000_0000_0000_0000_1111_1111_1111,
                king: 0,
            },
            player: WHITE,
            counter: 0,
        }
    }
}

// min max ai

type score = i8;

const MIN: score = i8::MIN;
const MAX: score = i8::MAX;

#[inline(always)]
fn min(left: score, right: score) -> score {
    if left < right {
        return left;
    }
    right
}

#[inline(always)]
fn max(left: score, right: score) -> score {
    if left > right {
        return left;
    }
    right
}

trait Node<E, S, TS> {
    fn evaluate(&self, player: &Player) -> score;

    fn terminal(&self, edges: &Vec<E>) -> (bool, TS);
    fn evaluate_terminal(&self, terminal_state: TS, player: &Player) -> score;

    fn edge_to_children(&self) -> Vec<E>;
    fn apply(&mut self, edge: &E) -> S;
    fn unapply(&mut self, edge: &E, state: S);
}

fn predict_via_alpha_beta_min_max<E, S, TS>(
    node: &mut impl Node<E, S, TS>,
    player: &Player,
    depth: u8,
) -> Option<E> {
    let mut best_edge = None;
    let mut best_score = MIN;

    for edge in node.edge_to_children() {
        let state = node.apply(&edge);
        let value = alpha_beta(node, player, depth - 1, MIN, MAX, false);
        node.unapply(&edge, state);

        if value >= best_score {
            best_score = value;
            best_edge = Some(edge);
        }
    }

    best_edge
}

fn alpha_beta<E, S, TS>(
    node: &mut impl Node<E, S, TS>,
    player: &Player,
    depth: u8,
    mut alpha: score,
    mut beta: score,
    is_maximizing: bool,
) -> score {
    let edges = node.edge_to_children();

    let (terminal, terminal_state) = node.terminal(&edges);
    if terminal {
        return node.evaluate_terminal(terminal_state, player);
    }

    if depth == 0 {
        return node.evaluate(player);
    }

    if is_maximizing {
        let mut value = MIN;
        for edge in edges {
            let state = node.apply(&edge);
            value = max(
                value,
                alpha_beta(node, &player, depth - 1, alpha, beta, false),
            );
            node.unapply(&edge, state);

            alpha = max(alpha, value);
            if beta <= alpha {
                break;
            }
        }

        return value;
    }

    let mut value = MAX;
    for edge in edges {
        let state = node.apply(&edge);
        value = min(
            value,
            alpha_beta(node, &player, depth - 1, alpha, beta, true),
        );
        node.unapply(&edge, state);

        beta = min(beta, value);
        if beta <= alpha {
            break;
        }
    }

    return value;
}

fn print_board(board: &BitBoard) {
    println!("  +-----------------+");

    let mut bit_index = 0;

    for row in 0..8 {
        print!("{} |", 8 - row);

        for col in 0..8 {
            if (row + col) % 2 == 1 {
                let bit = 1 << bit_index;
                bit_index += 1;

                let is_white = board.white & bit != 0;
                let is_black = board.black & bit != 0;
                let is_king = board.king & bit != 0;

                let symbol = match (is_white, is_black, is_king) {
                    (true, false, false) => "w",
                    (true, false, true) => "W",
                    (false, true, false) => "b",
                    (false, true, true) => "B",
                    _ => ".",
                };
                print!(" {}", symbol);
            } else {
                print!("  "); // белая клетка — не игровая
            }
        }

        println!(" |");
    }

    println!("  +-----------------+");
    println!("    A B C D E F G H");
}

fn main() {
    // let mut russian_rule = RussianRule {
    //     board: BitBoard {
    //         white: 1 << 31
    //             | 1 << 28
    //             | 1 << 27
    //             | 1 << 26
    //             | 1 << 25
    //             | 1 << 24
    //             | 1 << 20
    //             | 1 << 15
    //             | 1 << 12,
    //         black: 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 9 | 1 << 14 | 1 << 17,
    //         king: 0,
    //     },
    //     player: WHITE,
    //     counter: 0,
    // };

    // let children = russian_rule.edge_to_children();

    // let child = &children[1];
    // russian_rule.apply(child);

    // let children = russian_rule.edge_to_children();

    // for child in children {
    //     println!("{:?}", child);
    // }

    // print_board(&russian_rule.board);

    // let start = std::time::Instant::now();
    // let Some(best_move) = predict_via_alpha_beta_min_max(&mut russian_rule, &BLACK, 12) else {
    //     panic!("no move")
    // };
    // let duration = start.elapsed();
    // println!("⏱ Выполнено за: {:?}", duration);

    // println!("{:?}", best_move);
    // print_board(&russian_rule.board);

    let mut russian_rule = RussianRule::new();

    loop {
        print_board(&russian_rule.board);
        let children = russian_rule.edge_to_children();

        let (mut is_terminal, mut ts) = russian_rule.terminal(&children);
        if is_terminal {
            match ts {
                LOSE => println!("вы проиграли"),
                DRAW => println!("ничья"),
                WIN => println!("победа"),
                _ => panic!("статус терминал не верен"),
            }
            break;
        }

        println!("Введите from to через пробел:");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка ввода");

        let numbers_opt: Option<Vec<u8>> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .ok();

        let numbers = match numbers_opt {
            Some(num) => num,
            None => continue,
        };

        if numbers.len() != 2 {
            println!("Нужно ввести ровно два числа!");
            return;
        }

        let (from, to) = (numbers[0], numbers[1]);
        println!("Вы ввели: {} и {}", from, to);

        let Some(mv) = children.iter().find(|m| m.from == from && m.to == to) else {
            println!("такого хода нет, возможные ходы {:?}", children);
            continue;
        };

        russian_rule.apply(mv);

        (is_terminal, ts) = russian_rule.terminal(&russian_rule.edge_to_children());
        if is_terminal {
            match ts {
                WIN => println!("вы проиграли"),
                DRAW => println!("ничья"),
                LOSE => println!("победа"),
                _ => panic!("статус терминал не верен"),
            }
            break;
        }

        let start = std::time::Instant::now();
        let Some(best_move) = predict_via_alpha_beta_min_max(&mut russian_rule, &BLACK, 14) else {
            panic!("no move")
        };
        let duration = start.elapsed();
        println!("⏱ Выполнено за: {:?}", duration);

        russian_rule.apply(&best_move);
    }
}
