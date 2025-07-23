use crate::model::PieceType;

use super::Coordinate;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Move {
    path: Vec<Coordinate>, // >= 1 ever
}

impl std::ops::Add<Coordinate> for Move {
    type Output = Move;

    fn add(mut self, rhs: Coordinate) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Move {
    pub fn new_start(start: Coordinate) -> Self {
        Move { path: vec![start] }
    }

    pub fn new(start: Coordinate, landing: Coordinate) -> Self {
        Move {
            path: vec![start, landing],
        }
    }

    pub fn get_landing(&self) -> &Coordinate {
        match self.path.last() {
            Some(landing) => landing,
            None => panic!("empty move"),
        }
    }

    pub fn get_path(&self) -> &Vec<Coordinate> {
        &self.path
    }
}

pub fn print_moves(moves: &Vec<Move>) {
    for mv in moves {
        println!(
            "From {}",
            mv.path
                .iter()
                .map(|coord| format!("({})", coord.to_string()))
                .collect::<Vec<_>>()
                .join(" -> ")
        );
    }
}

#[derive(Clone)]
pub struct Promotion {
    landing: Coordinate,
    piece_type: PieceType,
}

#[derive(Clone)]
pub struct MoveEffect {
    captured: Vec<Coordinate>,
    promotion: Option<Promotion>,
}

impl MoveEffect {
    pub fn no_effect() -> Self {
        MoveEffect {
            captured: Vec::new(),
            promotion: None,
        }
    }

    pub fn capture(mut self, captured: Coordinate) -> Self {
        self.captured.push(captured);

        self
    }

    pub fn promote(mut self, landing: Coordinate, piece_type: PieceType) -> Self {
        self.promotion = Some(Promotion {
            landing: landing,
            piece_type: piece_type,
        });

        self
    }

    pub fn get_captured(&self) -> &Vec<Coordinate> {
        &self.captured
    }

    pub fn get_promotion(&self) -> &Option<Promotion> {
        &self.promotion
    }
}

pub type MoveWithEffect = (Move, MoveEffect);
