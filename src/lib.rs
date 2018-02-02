#[macro_use]
extern crate lazy_static;
extern crate num_traits;
mod board_square;
use self::board_square::*;

mod javascript_interaction;
pub use self::javascript_interaction::*;
//mod game_logic;
//pub use self::game_logic::*;

use std::sync::Mutex;

type BoardType = SquareBoard5;

const BOARDSIZE: usize = BoardType::AVERAGE_BOARD_SIZE;

impl MoveError {
    pub fn to_string(self) -> &'static str {
        use self::MoveError::*;
        match self {
            BoardBoundary => "board boundary reached",
            Wall => "wall is blocking",
            FieldsNotAdjacent => "fields are not adjacent",
        }
    }
}
impl WallPlacmentError {
    pub fn to_string(self) -> &'static str {
        use self::WallPlacmentError::*;
        match self {
            BoardBoundary => "BoardBoundary",
            WallAlreadyPlaced => "WallAlreadyPlaced",
            NoMoreWalls => "NoMoreWalls",
            PlayerBlocked => "PlayerBlocked",
            WallsAlreadyCrossing => "WallsAlreadyCrossing",
            NotConnected => "NotConnected",
        }
    }
}

pub struct GameData {
    pub board: BoardType,
    pub current_player: TwoPlayerIndices,
    pub wall_index_selected: Option<(usize, usize, bool)>,
    pub logbook: Vec<String>,
}
impl GameData {
    fn new() -> Self {
        GameData {
            board: BoardType::new(),
            current_player: TwoPlayerIndices::White,
            wall_index_selected: None,
            logbook: vec!["Game started".to_string()],
        }
    }
    fn get_current_player(&self) -> TwoPlayerIndices {
        self.current_player
    }
    fn next_player(&mut self) {
        let next_player = self.get_current_player().next_player();
        self.current_player = next_player;
    }
    fn append_logbook(&mut self, new_line: String) {
        self.logbook.push(new_line);
    }
    fn get_logbook(&self) -> &Vec<String> {
        &self.logbook
    }
    fn move_player_by_field(&mut self, field: FieldIndexSquare<usize>) -> Option<MoveError> {
        let current_field = self.board.get_current_field(self.get_current_player());
        let connected_fields = SquareBoard5::get_surrounding_fields(current_field);
        for (next_field, direction) in connected_fields {
            if next_field == field {
                if let Some(error) = self.board.move_player(self.current_player, direction) {
                    return Some(error);
                } else {
                    self.next_player();
                    return None;
                }
            }
        }
        Some(MoveError::FieldsNotAdjacent)
    }
}

lazy_static! {
  static ref DATA: Mutex<GameData> = Mutex::new(GameData::new());
}
