use std::ops::{Add, SubAssign};
use num_traits::One;
use super::*;
use super::type_level_integers::*;

#[derive(Clone, Copy, PartialEq)]
pub struct FieldIndexSquare<T: Clone + Copy + PartialEq> {
    pub column: T,
    pub row: T,
}
impl<T: Clone + Copy + PartialEq> FieldIndexTrait for FieldIndexSquare<T> {}

#[derive(Clone, Copy, PartialEq)]
pub enum DirectionsSquare {
    Up,
    Down,
    Left,
    Right,
}

use std::slice::Iter;
impl DirectionsTrait for DirectionsSquare {
    const DIRECTIONS_COUNT: usize = 4;
    type DirectionsArray = Iter<'static, DirectionsSquare>;
    fn get_directions_array() -> Self::DirectionsArray {
        use self::DirectionsSquare::*;
        [Up, Down, Left, Right].into_iter()
    }
}

pub struct PlayerDataSquare<T: Clone + Copy + PartialEq> {
    current_field: FieldIndexSquare<T>,
    wall_count: T,
    shortest_paths: Vec<Vec<(FieldIndexSquare<T>, DirectionsSquare)>>,
}
impl<T> PlayerDataTrait for PlayerDataSquare<T>
where
    T: Clone + Copy + PartialEq + One + SubAssign<T>,
{
    type FieldIndexType = FieldIndexSquare<T>;
    type WallCountType = T;
    type DirectionsType = DirectionsSquare;
    fn get_current_field(&self) -> FieldIndexSquare<T> {
        self.current_field
    }
    fn change_current_field(&mut self, new_field: FieldIndexSquare<T>) {
        self.current_field = new_field;
    }
    fn get_wall_count(&self) -> Self::WallCountType {
        self.wall_count
    }
    fn reduce_wall_count_by_one(&mut self) {
        self.wall_count -= One::one();
    }
    fn get_shortest_paths(&self) -> &Vec<Vec<(FieldIndexSquare<T>, DirectionsSquare)>> {
        &self.shortest_paths
    }
    fn change_shortest_paths(
        &mut self,
        new_list: Vec<Vec<(FieldIndexSquare<T>, DirectionsSquare)>>,
    ) {
        self.shortest_paths = new_list;
    }
}

#[derive(Copy, Clone)]
pub enum WallDirections {
    Left,
    Right,
}

pub trait WallPositionTrait<I> {
    fn new() -> Self;
    fn at(&self, row: I, column: I, is_left_or_right: bool) -> WallPlaced;
    fn at_mut(&mut self, row: I, column: I, is_left_or_right: bool) -> &mut WallPlaced;
}
#[derive(Clone, Copy, PartialEq)]
pub enum WallCrossing {
    IsWallCrossing,
    IsEmpty,
}
pub trait WallCrosingTrait<I> {
    fn new() -> Self;
    fn at(&self, row: I, column: I) -> WallCrossing;
    fn at_mut(&mut self, row: I, column: I) -> &mut WallCrossing;
}

use std::marker::PhantomData;
pub struct SquareBoard<
    T: Clone + Copy + PartialEq,
    SizeType,
    WallDataType: WallPositionTrait<T>,
    WallCrosingType: WallCrosingTrait<T>,
> {
    player_data: [PlayerDataSquare<T>; 2],
    _size: PhantomData<SizeType>,
    wall_positions: WallDataType,
    wall_crossing_positions: WallCrosingType,
}

impl<
    T: Clone + Copy + PartialEq + Add<T, Output = T> + One,
    SizeType,
    WallDataType: WallPositionTrait<T>,

    WallCrosingType: WallCrosingTrait<T>,
> SquareBoard<T, SizeType, WallDataType, WallCrosingType>
{
    fn place_wall_unsafe(
        &mut self,
        lower_left_field: FieldIndexSquare<T>,
        is_left_or_right: bool,
    ) -> Option<WallPlacmentError> {
        // check first wall
        use self::WallPlaced::*;
        if self.wall_positions.at(
            lower_left_field.column,
            lower_left_field.row,
            is_left_or_right,
        ) == IsWall
        {
            return Some(WallPlacmentError::WallAlreadyPlaced);
        };
        if match is_left_or_right {
            true => self.wall_positions.at(
                lower_left_field.column,
                lower_left_field.row + One::one(),
                is_left_or_right,
            ),
            false => self.wall_positions.at(
                lower_left_field.column + One::one(),
                lower_left_field.row,
                is_left_or_right,
            ),
        } == IsWall
        {
            return Some(WallPlacmentError::WallAlreadyPlaced);
        };
        if self.wall_crossing_positions
            .at(lower_left_field.column, lower_left_field.row)
            == WallCrossing::IsWallCrossing
        {
            return Some(WallPlacmentError::WallsAlreadyCrossing);
        }
        // wall can be placed
        *self.wall_positions.at_mut(
            lower_left_field.column,
            lower_left_field.row,
            is_left_or_right,
        ) = WallPlaced::IsWall;
        match is_left_or_right {
            true => {
                *self.wall_positions.at_mut(
                    lower_left_field.column,
                    lower_left_field.row + One::one(),
                    is_left_or_right,
                ) = WallPlaced::IsWall
            }
            false => {
                *self.wall_positions.at_mut(
                    lower_left_field.column + One::one(),
                    lower_left_field.row,
                    is_left_or_right,
                ) = WallPlaced::IsWall
            }
        };
        *self.wall_crossing_positions
            .at_mut(lower_left_field.column, lower_left_field.row) = WallCrossing::IsWallCrossing;
        None
    }
    fn place_wall_unsafe_redo(
        &mut self,
        lower_left_field: FieldIndexSquare<T>,
        is_left_or_right: bool,
    ) {
        *self.wall_positions.at_mut(
            lower_left_field.column,
            lower_left_field.row,
            is_left_or_right,
        ) = WallPlaced::IsEmpty;
        match is_left_or_right {
            true => {
                *self.wall_positions.at_mut(
                    lower_left_field.column,
                    lower_left_field.row + One::one(),
                    is_left_or_right,
                ) = WallPlaced::IsEmpty
            }
            false => {
                *self.wall_positions.at_mut(
                    lower_left_field.column + One::one(),
                    lower_left_field.row,
                    is_left_or_right,
                ) = WallPlaced::IsEmpty
            }
        };
        *self.wall_crossing_positions
            .at_mut(lower_left_field.column, lower_left_field.row) = WallCrossing::IsEmpty;
    }
    pub fn wall_lookup_unsafe(
        &self,
        lower_left_field: FieldIndexSquare<T>,
        is_left_or_right: bool,
    ) -> WallPlaced {
        self.wall_positions.at(
            lower_left_field.column,
            lower_left_field.row,
            is_left_or_right,
        )
    }
    pub fn croosing_lookup_unsafe(&self, lower_left_field: FieldIndexSquare<T>) -> WallCrossing {
        self.wall_crossing_positions
            .at(lower_left_field.column, lower_left_field.row)
    }
}

impl<
    SizeType,
    WallDataType: WallPositionTrait<usize>,
    WallCrosingType: WallCrosingTrait<usize>,
> BoardTrait for SquareBoard<usize, SizeType, WallDataType, WallCrosingType>
where
    SizeType: IntegerTrait<usize>,
{
    const AVERAGE_BOARD_SIZE: usize = SizeType::SIZE;
    type PlayerIndexType = TwoPlayerIndices;
    type DirectionsType = DirectionsSquare;
    type FieldIndexType = FieldIndexSquare<usize>;
    type PlayerDataType = PlayerDataSquare<usize>;
    type WallDirectionType = WallDirections;

    fn new() -> Self {
        const WALL_COUNT: usize = 5;
        let mut board = SquareBoard {
            _size: PhantomData,
            wall_positions: WallDataType::new(),
            wall_crossing_positions: WallCrosingType::new(),
            player_data: [
                PlayerDataSquare {
                    current_field: FieldIndexSquare {
                        column: Self::AVERAGE_BOARD_SIZE / 2,
                        row: 0,
                    },
                    wall_count: WALL_COUNT,
                    shortest_paths: vec![],
                },
                PlayerDataSquare {
                    current_field: FieldIndexSquare {
                        column: Self::AVERAGE_BOARD_SIZE / 2,
                        row: Self::AVERAGE_BOARD_SIZE - 1,
                    },
                    wall_count: WALL_COUNT,
                    shortest_paths: vec![],
                },
            ],
        };
        for &player in TwoPlayerIndices::get_player_index_array() {
            let shortest_paths = board.compute_shortest_paths(player);
            let mut data = board.get_player_data_mut(player);
            data.shortest_paths = shortest_paths;
        }
        board
    }
    fn get_field_in_direction(
        field: FieldIndexSquare<usize>,
        direction: DirectionsSquare,
    ) -> Option<(FieldIndexSquare<usize>, DirectionsSquare)> {
        use self::DirectionsSquare::*;
        if (direction == Right && field.column == Self::AVERAGE_BOARD_SIZE - 1)
            || (direction == Left && field.column == 0)
            || (direction == Up && field.row == Self::AVERAGE_BOARD_SIZE - 1)
            || (direction == Down && field.row == 0)
        {
            None
        } else {
            match direction {
                Up => Some((
                    FieldIndexSquare {
                        column: field.column,
                        row: field.row + 1,
                    },
                    direction,
                )),
                Down => Some((
                    FieldIndexSquare {
                        column: field.column,
                        row: field.row - 1,
                    },
                    direction,
                )),
                Left => Some((
                    FieldIndexSquare {
                        column: field.column - 1,
                        row: field.row,
                    },
                    direction,
                )),
                Right => Some((
                    FieldIndexSquare {
                        column: field.column + 1,
                        row: field.row,
                    },
                    direction,
                )),
            }
        }
    }

    // this function assumes that the direction is possible
    fn check_for_wall_unsafe(
        &self,
        field: Self::FieldIndexType,
        direction: Self::DirectionsType,
    ) -> WallPlaced {
        use self::DirectionsSquare::*;
        match direction {
            Up => self.wall_positions.at(field.column, field.row, false),
            Down => self.wall_positions.at(field.column, field.row - 1, false),
            Left => self.wall_positions.at(field.column - 1, field.row, true),
            Right => self.wall_positions.at(field.column, field.row, true),
        }
    }
    fn is_final_field(field: FieldIndexSquare<usize>, player: TwoPlayerIndices) -> bool {
        match player {
            TwoPlayerIndices::Black => field.row == Self::AVERAGE_BOARD_SIZE - 1,
            TwoPlayerIndices::White => field.row == 0,
        }
    }
    fn get_player_data(&self, player: Self::PlayerIndexType) -> &Self::PlayerDataType {
        match player {
            TwoPlayerIndices::White => &self.player_data[1],
            TwoPlayerIndices::Black => &self.player_data[0],
        }
    }
    fn get_player_data_mut(&mut self, player: Self::PlayerIndexType) -> &mut Self::PlayerDataType {
        match player {
            TwoPlayerIndices::White => &mut self.player_data[1],
            TwoPlayerIndices::Black => &mut self.player_data[0],
        }
    }

    fn place_wall(
        &mut self,
        player: TwoPlayerIndices,
        first_field: FieldIndexSquare<usize>,
        direction: DirectionsSquare,
        wall_direction: WallDirections,
    ) -> Option<WallPlacmentError> {
        if Self::get_player_data(self, player).wall_count == 0 {
            Some(WallPlacmentError::NoMoreWalls)
        } else {
            if let Some((second_field, _)) = Self::get_field_in_direction(first_field, direction) {
                let second_direction: DirectionsSquare = match (direction, wall_direction) {
                    (DirectionsSquare::Left, WallDirections::Left) => DirectionsSquare::Down,
                    (DirectionsSquare::Left, WallDirections::Right) => DirectionsSquare::Up,
                    (DirectionsSquare::Right, WallDirections::Left) => DirectionsSquare::Up,
                    (DirectionsSquare::Right, WallDirections::Right) => DirectionsSquare::Down,
                    (DirectionsSquare::Up, WallDirections::Left) => DirectionsSquare::Left,
                    (DirectionsSquare::Up, WallDirections::Right) => DirectionsSquare::Right,
                    (DirectionsSquare::Down, WallDirections::Left) => DirectionsSquare::Right,
                    (DirectionsSquare::Down, WallDirections::Right) => DirectionsSquare::Left,
                };
                if let Some((final_field, _)) =
                    Self::get_field_in_direction(second_field, second_direction)
                {
                    let (lower_left_field, is_left_or_right) = match (direction, wall_direction) {
                        (DirectionsSquare::Left, WallDirections::Left) => (final_field, true),
                        (DirectionsSquare::Left, WallDirections::Right) => (second_field, true),
                        (DirectionsSquare::Right, WallDirections::Left) => (first_field, true),
                        (DirectionsSquare::Right, WallDirections::Right) => (
                            FieldIndexSquare {
                                column: first_field.column,
                                row: final_field.row,
                            },
                            true,
                        ),
                        (DirectionsSquare::Up, WallDirections::Left) => (
                            FieldIndexSquare {
                                column: final_field.column,
                                row: first_field.row,
                            },
                            false,
                        ),
                        (DirectionsSquare::Up, WallDirections::Right) => (first_field, false),
                        (DirectionsSquare::Down, WallDirections::Left) => (second_field, false),
                        (DirectionsSquare::Down, WallDirections::Right) => (final_field, false),
                    };
                    match self.place_wall_unsafe(lower_left_field, is_left_or_right) {
                        Some(x) => Some(x),
                        None => {
                            let shortest_paths_white =
                                self.compute_shortest_paths(TwoPlayerIndices::White);
                            if shortest_paths_white.is_empty() {
                                self.place_wall_unsafe_redo(lower_left_field, is_left_or_right);
                                return Some(WallPlacmentError::PlayerBlocked);
                            };
                            let shortest_paths_black =
                                self.compute_shortest_paths(TwoPlayerIndices::Black);
                            if shortest_paths_black.is_empty() {
                                self.place_wall_unsafe_redo(lower_left_field, is_left_or_right);
                                return Some(WallPlacmentError::PlayerBlocked);
                            };
                            self.get_player_data_mut(TwoPlayerIndices::White)
                                .change_shortest_paths(shortest_paths_white);
                            self.get_player_data_mut(TwoPlayerIndices::Black)
                                .change_shortest_paths(shortest_paths_black);
                            self.get_player_data_mut(player).reduce_wall_count_by_one();
                            None
                        }
                    }
                } else {
                    Some(WallPlacmentError::BoardBoundary)
                }
            } else {
                Some(WallPlacmentError::BoardBoundary)
            }
        }
    }
}
