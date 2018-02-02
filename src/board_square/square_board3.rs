use super::*;
use super::type_level_integers::*;

impl WallPositionTrait<usize> for [WallPlaced; 2 * 3 * (3 - 1)] {
    fn new() -> Self {
        [WallPlaced::IsEmpty; 2 * 3 * (3 - 1)]
    }
    fn at(&self, row: usize, column: usize, is_left_or_right: bool) -> WallPlaced {
        if is_left_or_right {
            self[column + 3 * row]
        } else {
            self[row + 3 * column + 3 * (3 - 1)]
        }
    }
    fn at_mut(&mut self, row: usize, column: usize, is_left_or_right: bool) -> &mut WallPlaced {
        if is_left_or_right {
            &mut self[column + 3 * row]
        } else {
            &mut self[row + 3 * column + 3 * (3 - 1)]
        }
    }
}

impl WallCrosingTrait<usize> for [WallCrossing; (3 - 1) * (3 - 1)] {
    fn new() -> Self {
        [WallCrossing::IsEmpty; (3 - 1) * (3 - 1)]
    }
    fn at(&self, row: usize, column: usize) -> WallCrossing {
        self[row * (3 - 1) + column]
    }
    fn at_mut(&mut self, row: usize, column: usize) -> &mut WallCrossing {
        &mut self[row * (3 - 1) + column]
    }
}

pub type SquareBoard3 =
    SquareBoard<usize, Usize3, [WallPlaced; 2 * 3 * (3 - 1)], [WallCrossing; (3 - 1) * (3 - 1)]>;
