use super::*;
use super::type_level_integers::*;

impl WallPositionTrait<usize> for [WallPlaced; 2 * 4 * (4 - 1)] {
    fn new() -> Self {
        [WallPlaced::IsEmpty; 2 * 4 * (4 - 1)]
    }
    fn at(&self, row: usize, column: usize, is_left_or_right: bool) -> WallPlaced {
        if is_left_or_right {
            self[column + 4 * row]
        } else {
            self[row + 4 * column + 4 * (4 - 1)]
        }
    }
    fn at_mut(&mut self, row: usize, column: usize, is_left_or_right: bool) -> &mut WallPlaced {
        if is_left_or_right {
            &mut self[column + 4 * row]
        } else {
            &mut self[row + 4 * column + 4 * (4 - 1)]
        }
    }
}

impl WallCrosingTrait<usize> for [WallCrossing; (4 - 1) * (4 - 1)] {
    fn new() -> Self {
        [WallCrossing::IsEmpty; (4 - 1) * (4 - 1)]
    }
    fn at(&self, row: usize, column: usize) -> WallCrossing {
        self[row * (4 - 1) + column]
    }
    fn at_mut(&mut self, row: usize, column: usize) -> &mut WallCrossing {
        &mut self[row * (4 - 1) + column]
    }
}

pub type SquareBoard4 =
    SquareBoard<usize, Usize4, [WallPlaced; 2 * 4 * (4 - 1)], [WallCrossing; (4 - 1) * (4 - 1)]>;
