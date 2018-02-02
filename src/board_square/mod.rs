mod player;
pub use self::player::*;
mod type_level_integers;
mod square_board;
pub use self::square_board::*;
mod square_board5;
pub use self::square_board3::*;
mod square_board3;
pub use self::square_board4::*;
mod square_board4;
pub use self::square_board5::*;

pub trait DirectionsTrait: Clone + Copy + 'static {
    const DIRECTIONS_COUNT: usize;
    type DirectionsArray: Iterator<Item = &'static Self>;
    fn get_directions_array() -> Self::DirectionsArray;
}

pub trait FieldIndexTrait: Clone + Copy + PartialEq {}

pub trait PlayerDataTrait {
    //type PlayerIndexType : PlayerIndexTrait;
    type FieldIndexType: FieldIndexTrait + PartialEq;
    type DirectionsType: DirectionsTrait;
    type WallCountType;
    fn get_current_field(&self) -> Self::FieldIndexType;
    fn change_current_field(&mut self, new_field: Self::FieldIndexType);
    fn get_wall_count(&self) -> Self::WallCountType;
    fn reduce_wall_count_by_one(&mut self);
    fn get_shortest_paths(&self) -> &Vec<Vec<(Self::FieldIndexType, Self::DirectionsType)>>;
    fn change_shortest_paths(
        &mut self,
        new_list: Vec<Vec<(Self::FieldIndexType, Self::DirectionsType)>>,
    );
}

#[derive(PartialEq, Clone, Copy)]
pub enum WallPlaced {
    IsWall,
    IsEmpty,
}
pub enum WallPlacmentError {
    BoardBoundary,
    WallAlreadyPlaced,
    NoMoreWalls,
    PlayerBlocked,
    WallsAlreadyCrossing,
    NotConnected,
}
pub enum MoveError {
    BoardBoundary,
    Wall,
    FieldsNotAdjacent,
}

pub trait BoardTrait {
    const AVERAGE_BOARD_SIZE: usize;
    type PlayerIndexType: PlayerIndexTrait;
    type DirectionsType: DirectionsTrait;
    type FieldIndexType: FieldIndexTrait;
    type PlayerDataType: PlayerDataTrait<
        FieldIndexType = Self::FieldIndexType,
        DirectionsType = Self::DirectionsType,
    >;
    type WallDirectionType;
    fn new() -> Self;
    fn get_field_in_direction(
        field: Self::FieldIndexType,
        direction: Self::DirectionsType,
    ) -> Option<(Self::FieldIndexType, Self::DirectionsType)>;
    fn get_surrounding_fields(
        field: Self::FieldIndexType,
    ) -> Vec<(Self::FieldIndexType, Self::DirectionsType)> {
        Self::DirectionsType::get_directions_array()
            .filter_map(|dir| Self::get_field_in_direction(field, *dir))
            .collect()
    }
    // this function assumes that the direction is possible
    fn check_for_wall_unsafe(
        &self,
        field: Self::FieldIndexType,
        direction: Self::DirectionsType,
    ) -> WallPlaced;
    fn check_for_wall(
        &self,
        field: Self::FieldIndexType,
        direction: Self::DirectionsType,
    ) -> Option<WallPlaced> {
        if Self::get_field_in_direction(field, direction).is_none() {
            None
        } else {
            Some(self.check_for_wall_unsafe(field, direction))
        }
    }
    fn get_surrounding_possible_fields(
        &self,
        field: Self::FieldIndexType,
    ) -> Vec<(Self::FieldIndexType, Self::DirectionsType)> {
        Self::get_surrounding_fields(field)
            .iter()
            .filter(|&&(_, direction)| {
                self.check_for_wall_unsafe(field, direction) == WallPlaced::IsEmpty
            })
            .map(|x| *x)
            .collect()
    }
    fn is_final_field(field: Self::FieldIndexType, player: Self::PlayerIndexType) -> bool;

    fn get_player_data(&self, player: Self::PlayerIndexType) -> &Self::PlayerDataType;
    fn get_player_data_mut(&mut self, player: Self::PlayerIndexType) -> &mut Self::PlayerDataType;
    fn get_current_field(&self, player: Self::PlayerIndexType) -> Self::FieldIndexType {
        self.get_player_data(player).get_current_field()
    }
    fn move_player(
        &mut self,
        player: Self::PlayerIndexType,
        direction: Self::DirectionsType,
    ) -> Option<MoveError> {
        let current_field = self.get_current_field(player);
        // check if fields is at board boundary
        if let Some((next_field, _)) = Self::get_field_in_direction(current_field, direction) {
            // check if wall is in between
            if self.check_for_wall_unsafe(current_field, direction) == WallPlaced::IsWall {
                Some(MoveError::Wall)
            } else {
                self.get_player_data_mut(player)
                    .change_current_field(next_field);
                let new_paths = self.compute_shortest_paths(player).clone();
                self.get_player_data_mut(player)
                    .change_shortest_paths(new_paths);
                None
            }
        } else {
            Some(MoveError::BoardBoundary)
        }
    }
    fn compute_shortest_paths(
        &self,
        player: Self::PlayerIndexType,
    ) -> Vec<Vec<(Self::FieldIndexType, Self::DirectionsType)>> {
        let mut visited_fields =
            Vec::with_capacity(Self::AVERAGE_BOARD_SIZE * Self::AVERAGE_BOARD_SIZE);
        let start_field = self.get_current_field(player);
        visited_fields.push(start_field);
        let mut shortest_paths = Vec::with_capacity(Self::AVERAGE_BOARD_SIZE);
        let mut current_paths = vec![(Vec::with_capacity(0), start_field)];
        while shortest_paths.is_empty() && !current_paths.is_empty() {
            let mut next_paths =
                Vec::with_capacity(current_paths.len() * Self::DirectionsType::DIRECTIONS_COUNT);
            let mut new_visited_fields =
                Vec::with_capacity(current_paths.len() * Self::DirectionsType::DIRECTIONS_COUNT);
            for (current_path, previous_end) in current_paths {
                //current_path[0][0] + 0;
                let _temp = self.get_surrounding_possible_fields(previous_end);
                let surrounding_fields_directions = _temp
                    .iter()
                    .filter(|field_direction| {
                        !visited_fields
                            .iter()
                            .any(|&visited_field| visited_field == field_direction.0)
                    })
                    .collect::<Vec<_>>();
                for &(next_field, direction) in surrounding_fields_directions {
                    let mut next_path = current_path.clone();
                    next_path.push((next_field, direction));
                    if Self::is_final_field(next_field, player) {
                        shortest_paths.push(next_path);
                    } else {
                        next_paths.push((next_path, next_field));
                    }
                    new_visited_fields.push(next_field);
                }
            }
            current_paths = next_paths;
            new_visited_fields
                .iter()
                .map(|&x| visited_fields.push(x))
                .collect::<Vec<_>>();
        }
        shortest_paths
    }
    fn place_wall(
        &mut self,
        player: Self::PlayerIndexType,
        first_field: Self::FieldIndexType,
        direction: Self::DirectionsType,
        wall_direction: Self::WallDirectionType,
    ) -> Option<WallPlacmentError>;
}
