//mod square5;
//pub use self::square5::*;
mod uint_version;
pub use self::uint_version::*;

pub trait BoardTrait {
    type FieldIndices;
    type Directions;
    type PlayerIndices: Clone + Copy;
    type MoveErrorType;
    type PlaceWallErrorType;
    type WallPlacementDirections;
    fn get_current_player_string(&self) -> &'static str;
    fn get_player_wall_count(&self, player: Self::PlayerIndices) -> u8;
    fn get_current_player_color_string(&self) -> &'static str;
    fn next_player(&mut self);
    fn get_logbook(&self) -> &Vec<String>;
    fn append_logbook(&mut self, new_line: String);
    fn new() -> Self;
    fn get_current_player(&self) -> Self::PlayerIndices;
    fn move_is_allowed(
        &self,
        direction: Self::Directions,
        player: Self::PlayerIndices,
    ) -> Result<Self::FieldIndices, Self::MoveErrorType> {
        self.fields_connected(self.get_player_field(player), direction)
    }
    fn get_shortest_pathes(&self, player: Self::PlayerIndices) -> Vec<Vec<Self::FieldIndices>>;

    //    fn get_adjacent_fields(field: Self::FieldIndices) -> Vec<Self::FieldIndices>;
    fn get_adjacent_field(
        field: Self::FieldIndices,
        direction: Self::Directions,
    ) -> Option<Self::FieldIndices>;
    fn fields_connected(
        &self,
        first_field: Self::FieldIndices,
        direction: Self::Directions,
    ) -> Result<Self::FieldIndices, Self::MoveErrorType>;
    fn get_fields_connected(&self, field: Self::FieldIndices) -> Vec<Self::FieldIndices>;
    fn move_player(
        &mut self,
        player: Self::PlayerIndices,
        direction: Self::Directions,
    ) -> Option<Self::MoveErrorType> {
        let check = self.move_is_allowed(direction, player);
        match check {
            Ok(new_field) => {
                self.set_player_field(player, new_field);
                None
            }
            Err(error) => Some(error),
        }
    }
    fn place_wall(
        &mut self,
        player: Self::PlayerIndices,
        first_field: Self::FieldIndices,
        direction: Self::Directions,
        wall_direction: Self::WallPlacementDirections,
    ) -> Option<Self::PlaceWallErrorType>;
    fn is_final_field(field: Self::FieldIndices, player: Self::PlayerIndices) -> bool;
    fn get_player_field(&self, player: Self::PlayerIndices) -> Self::FieldIndices;
    fn set_player_field(&mut self, player: Self::PlayerIndices, new_position: Self::FieldIndices);
    fn move_player_by_field(
        &mut self,
        new_field: Self::FieldIndices,
    ) -> Option<Self::MoveErrorType>;
}
