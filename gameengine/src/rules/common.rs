use crate::model::Board;

pub(super) trait BoardCreator {
    fn new_board(&self) -> impl Board;
}
