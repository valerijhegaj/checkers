pub struct Direction {
    drow: i8,
    dcol: i8,
}

impl Direction {
    pub fn get_row(&self) -> i8 {
        self.drow
    }

    pub fn get_col(&self) -> i8 {
        self.dcol
    }
}

pub const DIRECTIONS_EVERY_DIAGONAL: [Direction; 4] = [
    Direction { drow: 1, dcol: 1 },
    Direction { drow: 1, dcol: -1 },
    Direction { drow: -1, dcol: 1 },
    Direction { drow: -1, dcol: -1 },
];

pub const DIRECTIONS_FROM_WHITE_TO_BLACK: [Direction; 2] = [
    Direction { drow: 1, dcol: 1 },
    Direction { drow: 1, dcol: -1 },
];

pub const DIRECTIONS_FROM_BLACK_TO_WHITE: [Direction; 2] = [
    Direction { drow: -1, dcol: 1 },
    Direction { drow: -1, dcol: -1 },
];
