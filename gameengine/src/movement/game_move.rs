use super::coordinate::Coordinate;

#[derive(PartialEq, Debug, Clone)]
pub struct Move {
    from: Coordinate,
    to: Vec<Coordinate>,
}

impl std::ops::Add<Coordinate> for Move {
    type Output = Move;

    fn add(mut self, rhs: Coordinate) -> Self::Output {
        self.to.push(rhs);
        self
    }
}

impl Move {
    pub fn new_from(from: Coordinate) -> Self {
        Move {
            from: from,
            to: Vec::new(),
        }
    }

    pub fn get_landing(&self) -> &Coordinate {
        match self.to.last() {
            Some(to) => to,
            None => &self.from,
        }
    }

    pub fn get_from(&self) -> &Coordinate {
        &self.from
    }

    pub fn get_to(&self) -> &Vec<Coordinate> {
        &self.to
    }
}

pub fn print_moves(moves: &Vec<Move>) {
    for mv in moves {
        print!("From ({})", mv.from.to_string());
        for coord in &mv.to {
            print!(" -> ({})", coord.to_string());
        }
        println!();
    }
}
