use super::direction::Direction;

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub struct Coordinate {
    col: usize,
    row: usize,
}

impl std::ops::AddAssign<&Direction> for Coordinate {
    fn add_assign(&mut self, direction: &Direction) {
        self.row = (self.row as isize + direction.get_row() as isize) as usize;
        self.col = (self.col as isize + direction.get_col() as isize) as usize;
    }
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        format!("(col: {}, row: {})", self.col, self.row)
    }
}

impl Coordinate {
    pub fn new(col: usize, row: usize) -> Coordinate {
        Coordinate { col: col, row: row }
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    pub fn get_row(&self) -> usize {
        self.row
    }
}
