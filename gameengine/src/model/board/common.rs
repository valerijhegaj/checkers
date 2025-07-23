use super::{Cell, PieceContainer};
use crate::model::{Coordinate, Direction, Piece, Player};

pub(super) fn iter_for<'a, R, C>(
    data: &'a R,
    player: &Player,
) -> impl Iterator<Item = (Coordinate, &'a Piece)>
where
    &'a R: IntoIterator<Item = C>,
    C: IntoIterator<Item = &'a Cell> + 'a,
{
    data.into_iter()
        .enumerate()
        .flat_map(move |(row, row_cells)| {
            row_cells
                .into_iter()
                .enumerate()
                .filter_map(move |(col, cell)| {
                    cell.as_ref().and_then(|piece| {
                        if piece.is_owner(player) {
                            return Some((Coordinate::new(row, col), piece));
                        }
                        return None;
                    })
                })
        })
}

pub(super) fn iter<'a, R, C>(data: &'a R) -> impl Iterator<Item = (Coordinate, &'a Piece)>
where
    &'a R: IntoIterator<Item = C>,
    C: IntoIterator<Item = &'a Cell> + 'a,
{
    data.into_iter()
        .enumerate()
        .flat_map(move |(row, row_cells)| {
            row_cells
                .into_iter()
                .enumerate()
                .filter_map(move |(col, cell)| {
                    cell.as_ref()
                        .and_then(|piece| Some((Coordinate::new(row, col), piece)))
                })
        })
}

pub(super) fn iter_on_direction<'a>(
    container: &'a impl PieceContainer,
    from: &Coordinate,
    direction: &Direction,
) -> impl Iterator<Item = (Coordinate, Option<&'a Piece>)> {
    let mut iter = from.clone();

    std::iter::from_fn(move || {
        if container.is_direction_out_of_border(&iter, direction) {
            return None;
        }

        iter += direction;

        Some((iter.clone(), container.at(&iter)))
    })
}
