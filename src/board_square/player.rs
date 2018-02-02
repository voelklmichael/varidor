pub trait PlayerIndexTrait: Clone + Copy + 'static {
    const PLAYER_COUNT: usize;
    type PlayerIndexArray: Iterator<Item = &'static Self>;
    fn get_player_index_array() -> Self::PlayerIndexArray;
    fn next_player(self) -> Self;
    fn to_string(self) -> &'static str;
    fn get_color_as_string(self) -> &'static str;
}

#[derive(Clone, Copy)]
pub enum TwoPlayerIndices {
    White,
    Black,
}
use std::slice::Iter;
impl PlayerIndexTrait for TwoPlayerIndices {
    const PLAYER_COUNT: usize = 2;
    type PlayerIndexArray = Iter<'static, TwoPlayerIndices>;
    fn get_player_index_array() -> Self::PlayerIndexArray {
        use self::TwoPlayerIndices::*;
        [White, Black].into_iter()
    }
    fn next_player(self) -> Self {
        match self {
            TwoPlayerIndices::White => TwoPlayerIndices::Black,
            TwoPlayerIndices::Black => TwoPlayerIndices::White,
        }
    }
    fn to_string(self) -> &'static str {
        match self {
            TwoPlayerIndices::White => "WHITE",
            TwoPlayerIndices::Black => "BLACK",
        }
    }
    fn get_color_as_string(self) -> &'static str {
        match self {
            TwoPlayerIndices::White => "White",
            TwoPlayerIndices::Black => "Black",
        }
    }
}
