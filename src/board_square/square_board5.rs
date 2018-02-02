use super::*;
use super::type_level_integers::*;

impl WallPositionTrait<usize> for [WallPlaced; 2 * 5 * (5 - 1)] {
    fn new() -> Self {
        [WallPlaced::IsEmpty; 2 * 5 * (5 - 1)]
    }
    fn at(&self, row: usize, column: usize, is_left_or_right: bool) -> WallPlaced {
        if is_left_or_right {
            self[column + 5 * row]
        } else {
            self[row + 5 * column + 5 * (5 - 1)]
        }
    }
    fn at_mut(&mut self, row: usize, column: usize, is_left_or_right: bool) -> &mut WallPlaced {
        if is_left_or_right {
            &mut self[column + 5 * row]
        } else {
            &mut self[row + 5 * column + 5 * (5 - 1)]
        }
    }
}

impl WallCrosingTrait<usize> for [WallCrossing; (5 - 1) * (5 - 1)] {
    fn new() -> Self {
        [WallCrossing::IsEmpty; (5 - 1) * (5 - 1)]
    }
    fn at(&self, row: usize, column: usize) -> WallCrossing {
        self[row * (5 - 1) + column]
    }
    fn at_mut(&mut self, row: usize, column: usize) -> &mut WallCrossing {
        &mut self[row * (5 - 1) + column]
    }
}

pub type SquareBoard5 =
    SquareBoard<usize, Usize5, [WallPlaced; 2 * 5 * (5 - 1)], [WallCrossing; (5 - 1) * (5 - 1)]>;
